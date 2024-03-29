use super::Renderer;
use crate::{
    error::{Error, Result},
    prelude::*,
    renderer::TextureRenderer,
};
use anyhow::{anyhow, Context};
use sdl2::render::{Canvas, Texture as SdlTexture};
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

pub(crate) struct RendererTexture {
    inner: Option<SdlTexture>,
}

impl RendererTexture {
    pub(crate) const fn new(texture: SdlTexture) -> Self {
        Self {
            inner: Some(texture),
        }
    }
}

impl Deref for RendererTexture {
    type Target = SdlTexture;
    fn deref(&self) -> &Self::Target {
        #[allow(clippy::expect_used)]
        self.inner.as_ref().expect("texture has been dropped")
    }
}

impl DerefMut for RendererTexture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[allow(clippy::expect_used)]
        self.inner.as_mut().expect("texture has been dropped")
    }
}

impl Drop for RendererTexture {
    fn drop(&mut self) {
        if let Some(texture) = self.inner.take() {
            // SAFETY: A RendererTexture can only exist inside of a WindowCanvas, which contains the
            // Canvas that created this texture, therefore it's safe to destroy.
            unsafe { texture.destroy() };
        }
    }
}

impl TextureRenderer for Renderer {
    /// Create a texture to render to.
    #[inline]
    fn create_texture(
        &mut self,
        width: u32,
        height: u32,
        format: Option<PixelFormat>,
    ) -> Result<TextureId> {
        let texture_id = self.next_texture_id;
        self.next_texture_id += 1;
        let window_canvas = self.window_canvas_mut()?;
        let texture = window_canvas
            .canvas
            .create_texture_target(format.map(Into::into), width, height)
            .context("failed to create texture")?;
        let texture_id = TextureId(texture_id);
        window_canvas
            .textures
            .insert(texture_id, RefCell::new(RendererTexture::new(texture)));
        Ok(texture_id)
    }

    /// Delete texture.
    ///
    /// # Note
    ///
    /// It is up to the caller to ensure that the texture to be dropped was created with the
    /// current canvas. Currently, the only way to violate this is by creating a texture using
    /// [`PixState::set_window_target`] and calling a texture method after that window has been
    /// closed.
    #[inline]
    fn delete_texture(&mut self, texture_id: TextureId) -> Result<()> {
        self.window_canvas_mut()?
            .textures
            .remove(&texture_id)
            .map_or(Err(Error::InvalidTexture(texture_id).into()), |_| Ok(()))
    }

    /// Update texture with pixel data.
    #[inline]
    fn update_texture<P: AsRef<[u8]>>(
        &mut self,
        texture_id: TextureId,
        rect: Option<Rect<i32>>,
        pixels: P,
        pitch: usize,
    ) -> Result<()> {
        let window = self
            .windows
            .values()
            .find(|w| w.textures.contains_key(&texture_id));
        if let Some(window) = window {
            // We ensured there's a valid texture above
            let texture = window
                .textures
                .get(&texture_id)
                .ok_or_else(|| anyhow!(Error::InvalidTexture(texture_id)))?;
            Ok(texture
                .borrow_mut()
                .update(rect.map(Into::into), pixels.as_ref(), pitch)
                .context("failed to update texture")?)
        } else {
            Err(Error::InvalidTexture(texture_id).into())
        }
    }

    /// Draw texture canvas.
    #[inline]
    fn texture(
        &mut self,
        texture_id: TextureId,
        src: Option<Rect<i32>>,
        dst: Option<Rect<i32>>,
        angle: f64,
        center: Option<Point<i32>>,
        flipped: Option<Flipped>,
        tint: Option<Color>,
    ) -> Result<()> {
        assert_ne!(
            Some(texture_id),
            self.texture_target,
            "`texture_id` must not equal the current `texture_target`"
        );

        let target_texture = self.texture_target;
        let window = self
            .windows
            .values_mut()
            .find(|w| w.textures.contains_key(&texture_id));
        if let Some(window) = window {
            // We ensured there's a valid texture above
            let texture = window
                .textures
                .get(&texture_id)
                .ok_or_else(|| anyhow!(Error::InvalidTexture(texture_id)))?;
            {
                let mut texture = texture.borrow_mut();
                let [r, g, b, a] = tint.map_or([255; 4], |t| t.channels());
                texture.set_color_mod(r, g, b);
                texture.set_alpha_mod(a);
                texture.set_blend_mode(self.blend_mode);
            }
            let src = src.map(Into::into);
            let dst = dst.map(Into::into);
            let update = |canvas: &mut Canvas<_>| -> Result<()> {
                let result = if angle > 0.0 || center.is_some() || flipped.is_some() {
                    canvas.copy_ex(
                        &texture.borrow(),
                        src,
                        dst,
                        angle,
                        center.map(Into::into),
                        matches!(flipped, Some(Flipped::Horizontal | Flipped::Both)),
                        matches!(flipped, Some(Flipped::Vertical | Flipped::Both)),
                    )
                } else {
                    canvas.copy(&texture.borrow(), src, dst)
                };
                Ok(result.map_err(Error::Renderer)?)
            };

            if let Some(texture_id) = target_texture {
                if let Some(texture) = window.textures.get(&texture_id) {
                    let mut result = Ok(());
                    window
                        .canvas
                        .with_texture_canvas(&mut texture.borrow_mut(), |canvas| {
                            result = update(canvas);
                        })
                        .with_context(|| format!("failed to update texture target {texture_id}"))?;
                    result
                } else {
                    Err(Error::InvalidTexture(texture_id).into())
                }
            } else {
                update(&mut window.canvas)
            }
        } else {
            Err(Error::InvalidTexture(texture_id).into())
        }
    }

    /// Returns texture used as the target for drawing operations, if set.
    #[inline]
    fn texture_target(&self) -> Option<TextureId> {
        self.texture_target
    }

    /// Set a `Texture` as the primary target for drawing operations instead of the window
    /// target canvas.
    ///
    /// # Errors
    ///
    /// If the texture has been dropped or is invalid, then an error is returned.
    #[inline]
    fn set_texture_target(&mut self, id: TextureId) -> Result<()> {
        self.windows
            .values()
            .find(|window| window.textures.contains_key(&id))
            .map(|_| self.texture_target = Some(id))
            .ok_or_else(|| anyhow!(Error::InvalidTexture(id)))
    }

    /// Clear `Texture` target back to the window target canvas for drawing operations.
    #[inline]
    fn clear_texture_target(&mut self) {
        self.texture_target = None;
    }

    /// Returns whether a texture is set as the target for drawing operations.
    #[inline]
    fn has_texture_target(&self) -> bool {
        self.texture_target.is_some()
    }

    /// Clear internal texture cache.
    #[inline]
    fn clear_texture_cache(&mut self) {
        self.loaded_fonts.clear();
        for window_canvas in self.windows.values_mut() {
            window_canvas.text_cache.clear();
            window_canvas.image_cache.clear();
        }
    }
}
