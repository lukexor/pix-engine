use super::Renderer;
use crate::{prelude::*, renderer::TextureRenderer};
use anyhow::Context;
use sdl2::{
    rect::Rect as SdlRect,
    render::{Canvas, Texture as SdlTexture},
};
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
        self.inner.as_ref().expect("texture has been dropped")
    }
}

impl DerefMut for RendererTexture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().expect("texture has been dropped")
    }
}

impl Drop for RendererTexture {
    fn drop(&mut self) {
        let texture = self.inner.take().expect("texture has been dropped");
        // SAFETY: A RendererTexture can only exist inside of a WindowCanvas, which contains the
        // TextureCreator that created this texture, therefore it's safe to destroy.
        unsafe { texture.destroy() };
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
    ) -> PixResult<TextureId> {
        let texture_id = self.next_texture_id;
        self.next_texture_id += 1;
        let window_canvas = self.window_canvas_mut()?;
        let texture = window_canvas
            .texture_creator
            .create_texture_target(format.map(|f| f.into()), width, height)
            .context("failed to create texture")?;
        let teture_id = TextureId(texture_id);
        window_canvas
            .textures
            .insert(teture_id, RefCell::new(RendererTexture::new(texture)));
        Ok(teture_id)
    }

    /// Delete texture.
    ///
    /// # Note
    ///
    /// It is up to the caller to ensure that the texture to be dropped was created with the
    /// current canvas. Currently, the only way to violate this is by creating a texture using
    /// [`PixState::with_window`] and calling a texture method after that window has been closed.
    #[inline]
    fn delete_texture(&mut self, texture_id: TextureId) -> PixResult<()> {
        self.window_canvas_mut()?
            .textures
            .remove(&texture_id)
            .map_or(Err(PixError::InvalidTexture(texture_id).into()), |_| Ok(()))
    }

    /// Update texture with pixel data.
    #[inline]
    fn update_texture<P: AsRef<[u8]>>(
        &mut self,
        texture_id: TextureId,
        rect: Option<Rect<i32>>,
        pixels: P,
        pitch: usize,
    ) -> PixResult<()> {
        let window = self.windows.values().find_map(|w| {
            if w.textures.contains_key(&texture_id) {
                Some(w)
            } else {
                None
            }
        });
        if let Some(window) = window {
            // We ensured there's a valid texture above
            let texture = window.textures.get(&texture_id).unwrap();
            let rect: Option<SdlRect> = rect.map(|r| r.into());
            Ok(texture
                .borrow_mut()
                .update(rect, pixels.as_ref(), pitch)
                .context("failed to update texture")?)
        } else {
            Err(PixError::InvalidTexture(texture_id).into())
        }
    }

    /// Draw texture canvas.
    #[inline]
    fn texture(
        &mut self,
        texture_id: TextureId,
        src: Option<Rect<i32>>,
        dst: Option<Rect<i32>>,
        angle: Scalar,
        center: Option<PointI2>,
        flipped: Option<Flipped>,
        tint: Option<Color>,
    ) -> PixResult<()> {
        assert_ne!(
            Some(texture_id),
            self.texture_target,
            "`texture_id` must not equal the current `texture_target`"
        );

        let target_texture = self.texture_target;
        let window = self.windows.values_mut().find_map(|w| {
            if w.textures.contains_key(&texture_id) {
                Some(w)
            } else {
                None
            }
        });
        if let Some(window) = window {
            // We ensured there's a valid texture above
            let texture = window.textures.get(&texture_id).unwrap();
            {
                let mut texture = texture.borrow_mut();
                if let Some(tint) = tint {
                    let [r, g, b, a] = tint.channels();
                    texture.set_color_mod(r, g, b);
                    texture.set_alpha_mod(a);
                } else {
                    texture.set_color_mod(255, 255, 255);
                    texture.set_alpha_mod(255);
                }
            }
            let src = src.map(|r| r.into());
            let dst = dst.map(|r| r.into());
            let update = |canvas: &mut Canvas<_>| -> PixResult<()> {
                let result = if angle > 0.0 || center.is_some() || flipped.is_some() {
                    canvas.copy_ex(
                        &texture.borrow(),
                        src,
                        dst,
                        angle,
                        center.map(|c| c.into()),
                        matches!(flipped, Some(Flipped::Horizontal | Flipped::Both)),
                        matches!(flipped, Some(Flipped::Vertical | Flipped::Both)),
                    )
                } else {
                    canvas.copy(&texture.borrow(), src, dst)
                };
                Ok(result.map_err(PixError::Renderer)?)
            };

            if let Some(texture_id) = target_texture {
                if let Some(texture) = window.textures.get(&texture_id) {
                    let mut result = Ok(());
                    window
                        .canvas
                        .with_texture_canvas(&mut texture.borrow_mut(), |canvas| {
                            result = update(canvas);
                        })
                        .with_context(|| {
                            format!("failed to update texture target {}", texture_id)
                        })?;
                    result
                } else {
                    Err(PixError::InvalidTexture(texture_id).into())
                }
            } else {
                update(&mut window.canvas)
            }
        } else {
            Err(PixError::InvalidTexture(texture_id).into())
        }
    }

    /// Returns texture used as the target for drawing operations, if set.
    #[inline]
    fn texture_target(&self) -> Option<TextureId> {
        self.texture_target
    }

    /// Set texture as the target for drawing operations.
    #[inline]
    fn set_texture_target(&mut self, texture_id: TextureId) {
        self.texture_target = Some(texture_id);
    }

    /// Set texture as the target for drawing operations.
    #[inline]
    fn clear_texture_target(&mut self) {
        self.texture_target = None;
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
