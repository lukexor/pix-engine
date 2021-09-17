use super::Renderer;
use crate::{core::texture::TextureRenderer, prelude::*, renderer::Result};
use sdl2::{
    rect::Rect as SdlRect,
    render::{Canvas, Texture as SdlTexture},
    video::Window as SdlWindow,
};

pub(crate) type RendererTexture = SdlTexture;

impl TextureRenderer for Renderer {
    /// Create a texture to render to.
    fn create_texture(
        &mut self,
        width: u32,
        height: u32,
        format: Option<PixelFormat>,
    ) -> Result<Texture> {
        let texture =
            self.texture_creator
                .create_texture_target(format.map(|f| f.into()), width, height)?;
        Ok(Texture::new(texture))
    }

    /// Update texture with pixel data.
    fn update_texture(
        &mut self,
        texture: &mut Texture,
        rect: Option<Rect<i32>>,
        pixels: &[u8],
        pitch: usize,
    ) -> Result<()> {
        let rect: Option<SdlRect> = rect.map(|r| r.into());
        Ok(texture.inner_mut().update(rect, pixels, pitch)?)
    }

    /// Draw texture canvas.
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
    fn set_texture_target(&mut self, texture: &mut Texture) {
        self.texture_target = Some(texture);
    }

    /// Set texture as the target for drawing operations.
    fn clear_texture_target(&mut self) {
        self.texture_target = None;
    }
}

impl Renderer {
    pub(crate) fn update<F>(&mut self, f: F) -> Result<()>
    where
        for<'r> F: FnOnce(&'r mut Canvas<SdlWindow>) -> Result<()>,
    {
        match self.texture_target {
            Some(ptr) => {
                let mut texture = unsafe { &mut (*ptr).inner };
                Ok(self.canvas.with_texture_canvas(&mut texture, |canvas| {
                    let _ = f(canvas);
                })?)
            }
            None => f(&mut self.canvas),
        }
    }
}
