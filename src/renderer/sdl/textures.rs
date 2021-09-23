use super::{Renderer, TTF};
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
        let texture =
            self.texture_creator
                .create_texture_target(format.map(|f| f.into()), width, height)?;
        Ok(Texture::new(texture, width, height, format))
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
        texture: &Texture,
        src: Option<Rect<i32>>,
        dst: Option<Rect<i32>>,
    ) -> Result<()> {
        let src = src.map(|r| r.into());
        let dst = dst.map(|r| r.into());
        Ok(self.canvas.copy(texture.inner(), src, dst)?)
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
}

impl Renderer {
    pub(crate) fn update_font_cache(&mut self) -> Result<()> {
        if self.font_cache.get(&self.font).is_none() {
            self.font_cache
                .insert(self.font.clone(), TTF.load_font(&self.font.0, self.font.1)?);
        }
        Ok(())
    }
}
