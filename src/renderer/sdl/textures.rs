use super::Renderer;
use crate::renderer::*;
use sdl2::{rect::Rect as SdlRect, render::Texture as SdlTexture};

pub(crate) type RendererTexture = SdlTexture;

impl TextureRenderer for Renderer {
    /// Create a texture to render to.
    #[inline]
    fn create_texture(
        &mut self,
        width: u32,
        height: u32,
        format: Option<PixelFormat>,
    ) -> Result<TextureId> {
        let (_, texture_creator) = self
            .canvases
            .get(&self.window_target)
            .ok_or(WindowError::InvalidWindow(self.window_target))?;
        let texture =
            texture_creator.create_texture_target(format.map(|f| f.into()), width, height)?;
        let texture_id = self.textures.len();
        self.textures.push((self.window_target, texture));
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
    fn delete_texture(&mut self, texture_id: TextureId) -> Result<()> {
        if self.textures.len() > texture_id {
            let (_, texture) = self.textures.remove(texture_id);
            // SAFETY: If we have a valid texture entry, it's safe to destroy as long as all
            // other methods in this crate that remove canvases or closing windows handle
            // removing textures created for that canvas.
            unsafe { texture.destroy() };
            Ok(())
        } else {
            Err(Error::InvalidTexture(texture_id))
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
    ) -> Result<()> {
        if let Some((_, texture)) = self.textures.get_mut(texture_id) {
            let rect: Option<SdlRect> = rect.map(|r| r.into());
            Ok(texture.update(rect, pixels.as_ref(), pitch)?)
        } else {
            Err(Error::InvalidTexture(texture_id))
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
    ) -> Result<()> {
        if let Some((_, texture)) = self.textures.get_mut(texture_id) {
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

            let (canvas, _) = self
                .canvases
                .get_mut(&self.window_target)
                .ok_or(WindowError::InvalidWindow(self.window_target))?;
            if angle > 0.0 || center.is_some() || flipped.is_some() {
                canvas.copy_ex(
                    texture,
                    src,
                    dst,
                    angle,
                    center.map(|c| c.into()),
                    matches!(flipped, Some(Flipped::Horizontal | Flipped::Both)),
                    matches!(flipped, Some(Flipped::Vertical | Flipped::Both)),
                )?;
            } else {
                canvas.copy(texture, src, dst)?;
            }
            Ok(())
        } else {
            Err(Error::InvalidTexture(texture_id))
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
        self.font_cache.clear();
        self.text_cache.clear();
        self.image_cache.clear();
    }
}
