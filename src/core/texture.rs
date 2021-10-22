//! `Texture` functions.

use crate::{prelude::*, renderer::Result as RendererResult};
use num_traits::AsPrimitive;

/// `TextureId`.
pub type TextureId = usize;

/// Trait for texture operations on the underlying `Renderer`.
pub(crate) trait TextureRenderer {
    /// Create a `Texture` to draw to.
    fn create_texture(
        &mut self,
        width: u32,
        height: u32,
        format: Option<PixelFormat>,
    ) -> RendererResult<TextureId>;

    /// Delete a `Texture`.
    fn delete_texture(&mut self, texture_id: TextureId) -> RendererResult<()>;

    /// Update texture with pixel data.
    fn update_texture<P: AsRef<[u8]>>(
        &mut self,
        texture_id: TextureId,
        rect: Option<Rect<i32>>,
        pixels: P,
        pitch: usize,
    ) -> RendererResult<()>;

    /// Draw texture to the curent canvas.
    #[allow(clippy::too_many_arguments)]
    fn texture(
        &mut self,
        texture_id: TextureId,
        src: Option<Rect<i32>>,
        dst: Option<Rect<i32>>,
        angle: Scalar,
        center: Option<PointI2>,
        flipped: Option<Flipped>,
        tint: Option<Color>,
    ) -> RendererResult<()>;

    /// Returns texture used as the target for drawing operations, if set.
    fn texture_target(&self) -> Option<TextureId>;

    /// Set texture as the target for drawing operations.
    fn set_texture_target(&mut self, texture_id: TextureId);

    /// Clear texture as the target for drawing operations.
    fn clear_texture_target(&mut self);

    /// Clear internal texture cache.
    fn clear_texture_cache(&mut self);
}

impl PixState {
    /// Draw a portion of a [Texture] to the current canvas resized to the target `dst`.
    pub fn texture<R1, R2>(&mut self, texture_id: TextureId, src: R1, dst: R2) -> PixResult<()>
    where
        R1: Into<Option<Rect<i32>>>,
        R2: Into<Option<Rect<i32>>>,
    {
        Ok(self
            .renderer
            .texture(texture_id, src.into(), dst.into(), 0.0, None, None, None)?)
    }

    /// Draw a transformed [Texture] to the current canvas resized to the target `rect`, optionally
    /// rotated by an `angle` about the `center` point or `flipped`. `angle` can be in either
    /// radians or degrees based on [AngleMode].
    #[allow(clippy::too_many_arguments)]
    pub fn texture_transformed<R1, R2, A, C, F, T>(
        &mut self,
        texture_id: TextureId,
        src: R1,
        dst: R2,
        angle: A,
        center: C,
        flipped: F,
        tint: T,
    ) -> PixResult<()>
    where
        R1: Into<Option<Rect<i32>>>,
        R2: Into<Option<Rect<i32>>>,
        A: AsPrimitive<Scalar>,
        C: Into<Option<PointI2>>,
        F: Into<Option<Flipped>>,
        T: Into<Option<Color>>,
    {
        let s = &self.settings;
        let mut angle: Scalar = angle.as_();
        if let AngleMode::Radians = s.angle_mode {
            angle = angle.to_degrees();
        };
        Ok(self.renderer.texture(
            texture_id,
            src.into(),
            dst.into(),
            angle,
            center.into(),
            flipped.into(),
            tint.into(),
        )?)
    }

    /// Constructs a `Texture` to render to.
    pub fn create_texture<F>(&mut self, width: u32, height: u32, format: F) -> PixResult<TextureId>
    where
        F: Into<Option<PixelFormat>>,
    {
        Ok(self.renderer.create_texture(width, height, format.into())?)
    }

    /// Delete a `Texture`.
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
        let rect = rect.into();
        let pixels = pixels.as_ref();
        Ok(self
            .renderer
            .update_texture(texture_id, rect, pixels, pitch)?)
    }

    /// Target a `Texture` for drawing operations.
    pub fn with_texture<F>(&mut self, texture_id: TextureId, f: F) -> PixResult<()>
    where
        F: FnOnce(&mut PixState) -> PixResult<()>,
    {
        self.push();
        self.ui.push_cursor();
        self.set_cursor_pos([0, 0]);

        self.renderer.set_texture_target(texture_id);
        let result = f(self);
        self.renderer.clear_texture_target();

        self.ui.pop_cursor();
        self.pop();
        result
    }
}
