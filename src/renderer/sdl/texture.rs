use super::Renderer;
use crate::renderer::*;
use anyhow::Context;
use sdl2::{
    rect::Rect as SdlRect,
    render::{Canvas, Texture as SdlTexture},
};

pub(crate) type RendererTexture = SdlTexture;

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
        window_canvas.textures.insert(texture_id, texture);
        Ok(texture_id)
    }

    /// Delete texture.
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure that the texture to be destroyed was created with the
    /// current canvas. Currently, the only way to violate this is by creating a texture and then
    /// toggling vsync after `PixEngine` initialization. Toggling vsync requires re-creating any
    /// textures in order to safely destroy them.
    ///
    /// Destroying textures created from a dropped canvas is undefined behavior.
    #[inline]
    fn delete_texture(&mut self, texture_id: TextureId) -> PixResult<()> {
        let window_canvas = self.window_canvas_mut()?;
        if let Some(texture) = window_canvas.textures.remove(&texture_id) {
            // SAFETY: If we have a valid texture entry, it's safe to destroy since the
            // texture_creator is still alive inside window_canvas.
            unsafe { texture.destroy() };
            Ok(())
        } else {
            Err(PixError::InvalidTexture(texture_id).into())
        }
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
        let window_canvas = self.window_canvas_mut()?;
        if let Some(texture) = window_canvas.textures.get_mut(&texture_id) {
            let rect: Option<SdlRect> = rect.map(|r| r.into());
            Ok(texture
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
        // Hairy bit to get multiple mutable exclusive borrows from our texture hashmap in order to
        // allow rendering one texture into another
        // TODO: Clean up
        let (texture, texture_target) = unsafe {
            let t1 = self
                .window_canvas_mut()?
                .textures
                .get_mut(&texture_id)
                .map(|t| {
                    let t: *mut _ = t;
                    t
                });
            let t2 = if let Some(target) = self.texture_target {
                self.window_canvas_mut()?
                    .textures
                    .get_mut(&target)
                    .map(|t| {
                        let t: *mut _ = t;
                        t
                    })
            } else {
                None
            };
            assert_ne!(
                t1, t2,
                "texture_id must not be set as the current texture_target"
            );
            (t1.map(|t| &mut *t), t2.map(|t| &mut *t))
        };
        if let Some(texture) = texture {
            match tint {
                Some(tint) => {
                    let [r, g, b, a] = tint.channels();
                    texture.set_color_mod(r, g, b);
                    texture.set_alpha_mod(a);
                }
                None => {
                    texture.set_color_mod(255, 255, 255);
                    texture.set_alpha_mod(255);
                }
            }
            let src = src.map(|r| r.into());
            let dst = dst.map(|r| r.into());
            let update = |canvas: &mut Canvas<_>| -> PixResult<()> {
                let result = if angle > 0.0 || center.is_some() || flipped.is_some() {
                    canvas.copy_ex(
                        texture,
                        src,
                        dst,
                        angle,
                        center.map(|c| c.into()),
                        matches!(flipped, Some(Flipped::Horizontal | Flipped::Both)),
                        matches!(flipped, Some(Flipped::Vertical | Flipped::Both)),
                    )
                } else {
                    canvas.copy(texture, src, dst)
                };
                Ok(result.map_err(PixError::Renderer)?)
            };
            let canvas = self.canvas_mut()?;
            if let Some(texture) = texture_target {
                let mut result = Ok(());
                canvas
                    .with_texture_canvas(texture, |canvas| {
                        result = update(canvas);
                    })
                    .with_context(|| format!("failed to update texture target {}", texture_id))?;
                result
            } else {
                update(canvas)
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
