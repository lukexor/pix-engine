//! `Texture` functions.

use crate::{prelude::*, renderer::Rendering};

/// `Texture` Identifier.
pub type TextureId = usize;

impl PixState {
    /// Constructs a `Texture` to render to.
    pub fn create_texture<T, F>(&mut self, width: T, height: T, format: F) -> PixResult<TextureId>
    where
        T: Into<f64>,
        F: Into<Option<PixelFormat>>,
    {
        Ok(self.renderer.create_texture(width, height, format)?)
    }

    /// Deletes a texture by [`TextureId`].
    pub fn delete_texture(&mut self, texture_id: usize) -> PixResult<()> {
        Ok(self.renderer.delete_texture(texture_id)?)
    }

    /// Update the `Texture` with a [`u8`] [`slice`] of pixel data.
    pub fn update_texture<R, P>(
        &mut self,
        texture_id: TextureId,
        rect: R,
        pixels: P,
        pitch: usize,
    ) -> PixResult<()>
    where
        R: Into<Option<Rect<f64>>>,
        P: AsRef<[u8]>,
    {
        Ok(self
            .renderer
            .update_texture(texture_id, rect, pixels, pitch)?)
    }
}
