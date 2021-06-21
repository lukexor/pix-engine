//! `Texture` functions.

use crate::{prelude::*, renderer::Rendering};
use num_traits::AsPrimitive;

/// `Texture` Identifier.
pub type TextureId = usize;

impl PixState {
    /// Constructs a `Texture` to render to.
    pub fn create_texture<T: Into<u32>>(
        &mut self,
        format: impl Into<Option<PixelFormat>>,
        width: T,
        height: T,
    ) -> PixResult<TextureId> {
        Ok(self.renderer.create_texture(format, width, height)?)
    }

    /// Deletes a texture by [`TextureId`].
    pub fn delete_texture(&mut self, texture_id: usize) -> PixResult<()> {
        Ok(self.renderer.delete_texture(texture_id)?)
    }

    /// Update the `Texture` with a [`u8`] [`slice`] of pixel data.
    pub fn update_texture<R, T>(
        &mut self,
        texture_id: usize,
        rect: Option<R>,
        pixels: &[u8],
        pitch: usize,
    ) -> PixResult<()>
    where
        R: Into<Rect<T>>,
        T: AsPrimitive<i32> + AsPrimitive<u32>,
    {
        Ok(self
            .renderer
            .update_texture(texture_id, rect, pixels, pitch)?)
    }
}
