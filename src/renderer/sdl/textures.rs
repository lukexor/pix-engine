use super::{Renderer, WindowCanvas, TTF};
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
    ) -> Result<Texture> {
        let (_, texture_creator) = self
            .canvases
            .get(&self.window_target)
            .ok_or(WindowError::InvalidWindow(self.window_target))?;
        let texture =
            texture_creator.create_texture_target(format.map(|f| f.into()), width, height)?;
        Ok(Texture::new(
            self.window_target,
            texture,
            width,
            height,
            format,
        ))
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
    unsafe fn delete_texture(&mut self, texture: Texture) -> Result<()> {
        texture.destroy();
        Ok(())
    }

    /// Update texture with pixel data.
    #[inline]
    fn update_texture<P: AsRef<[u8]>>(
        &mut self,
        texture: &mut Texture,
        rect: Option<Rect<i32>>,
        pixels: P,
        pitch: usize,
    ) -> Result<()> {
        let rect: Option<SdlRect> = rect.map(|r| r.into());
        Ok(texture.inner_mut().update(rect, pixels.as_ref(), pitch)?)
    }

    /// Draw texture canvas.
    #[inline]
    fn texture(
        &mut self,
        texture: &mut Texture,
        src: Option<Rect<i32>>,
        dst: Option<Rect<i32>>,
        angle: Scalar,
        center: Option<PointI2>,
        flipped: Option<Flipped>,
        tint: Option<Color>,
    ) -> Result<()> {
        match tint {
            Some(tint) => {
                let [r, g, b, a] = tint.channels();
                texture.inner_mut().set_color_mod(r, g, b);
                texture.inner_mut().set_alpha_mod(a);
            }
            None => {
                texture.inner_mut().set_color_mod(255, 255, 255);
                texture.inner_mut().set_alpha_mod(255);
            }
        }
        let src = src.map(|r| r.into());
        let dst = dst.map(|r| r.into());
        self.update_canvas(|canvas: &mut WindowCanvas| -> Result<()> {
            if angle > 0.0 || center.is_some() || flipped.is_some() {
                Ok(canvas.copy_ex(
                    texture.inner(),
                    src,
                    dst,
                    angle,
                    center.map(|c| c.into()),
                    matches!(flipped, Some(Flipped::Horizontal | Flipped::Both)),
                    matches!(flipped, Some(Flipped::Vertical | Flipped::Both)),
                )?)
            } else {
                Ok(canvas.copy(texture.inner(), src, dst)?)
            }
        })
    }

    /// Returns texture used as the target for drawing operations, if set.
    #[inline]
    fn texture_target(&self) -> Option<&Texture> {
        self.texture_target.map(|ptr| unsafe { &*ptr })
    }

    /// Set texture as the target for drawing operations.
    #[inline]
    fn set_texture_target(&mut self, texture: &mut Texture) {
        self.texture_target = Some(texture);
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

impl Renderer {
    pub(crate) fn update_font_cache(&mut self) -> Result<()> {
        if !self.font_cache.contains(&self.font) {
            self.font_cache
                .put(self.font.clone(), TTF.load_font(&self.font.0, self.font.1)?);
        }
        Ok(())
    }
}
