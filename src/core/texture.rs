//! `Texture` functions.

use crate::{prelude::*, renderer::Rendering};

/// `Texture` Identifier.
pub type TextureId = usize;

impl PixState {
    /// Draw the `Texture` to the current canvas.
    pub fn texture(
        &mut self,
        texture_id: TextureId,
        src: Option<Rect<i32>>,
        dst: Option<Rect<i32>>,
    ) -> PixResult<()> {
        Ok(self.renderer.texture(texture_id, src, dst)?)
    }

    /// Constructs a `Texture` to render to.
    pub fn create_texture<F>(&mut self, width: u32, height: u32, format: F) -> PixResult<TextureId>
    where
        F: Into<Option<PixelFormat>>,
    {
        Ok(self.renderer.create_texture(width, height, format.into())?)
    }

    /// Deletes a texture by [TextureId].
    pub fn delete_texture(&mut self, texture_id: TextureId) -> PixResult<()> {
        Ok(self.renderer.delete_texture(texture_id)?)
    }

    /// Update the `Texture` with a [u8] [slice] of pixel data.
    pub fn update_texture<R, P>(
        &mut self,
        texture_id: TextureId,
        rect: R,
        pixels: P,
        pitch: usize,
    ) -> PixResult<()>
    where
        R: Into<Option<Rect<i32>>>,
        P: AsRef<[u8]>,
    {
        Ok(self
            .renderer
            .update_texture(texture_id, rect.into(), pixels.as_ref(), pitch)?)
    }
}
