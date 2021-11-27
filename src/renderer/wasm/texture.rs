use super::Renderer;
use crate::{prelude::*, renderer::TextureRenderer};

impl TextureRenderer for Renderer {
    /// Create a `Texture` to draw to.
    ///
    /// # Errors
    ///
    /// If the current window target is closed or invalid, or the texture dimensions are invalid,
    /// then an error is returned.
    #[inline]
    fn create_texture(
        &mut self,
        width: u32,
        height: u32,
        format: Option<PixelFormat>,
    ) -> PixResult<TextureId> {
        todo!()
    }

    /// Delete a `Texture`.
    ///
    /// # Errors
    ///
    /// If the current window target is closed or invalid, or the texture has already been dropped,
    /// then an error is returned.
    ///
    #[inline]
    fn delete_texture(&mut self, texture_id: TextureId) -> PixResult<()> {
        todo!()
    }

    /// Update texture with pixel data.
    ///
    /// # Errors
    ///
    /// If the current window target is closed or invalid, or the renderer fails to update to the
    /// texture, then an error is returned.
    #[inline]
    fn update_texture<P: AsRef<[u8]>>(
        &mut self,
        texture_id: TextureId,
        rect: Option<Rect<i32>>,
        pixels: P,
        pitch: usize,
    ) -> PixResult<()> {
        todo!()
    }

    /// Draw texture to the curent canvas.
    ///
    /// # Errors
    ///
    /// Returns an error for any of the following:
    ///     - The current render target is closed or dropped.
    ///     - The texture being rendered has been dropped.
    ///     - The target texture is the same as the texture being rendered.
    ///     - The renderer fails to draw to the texture.
    ///
    #[allow(clippy::too_many_arguments)]
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
        todo!()
    }

    /// Returns texture used as the target for drawing operations, if set.
    #[inline]
    fn texture_target(&self) -> Option<TextureId> {
        todo!()
    }

    /// Set texture as the target for drawing operations.
    #[inline]
    fn set_texture_target(&mut self, texture_id: TextureId) {
        todo!()
    }

    /// Clear texture as the target for drawing operations.
    #[inline]
    fn clear_texture_target(&mut self) {
        todo!()
    }

    /// Returns whether a texture is set as the target for drawing operations.
    #[inline]
    fn has_texture_target(&self) -> bool {
        todo!()
    }

    /// Clear internal texture cache.
    #[inline]
    fn clear_texture_cache(&mut self) {
        todo!()
    }
}
