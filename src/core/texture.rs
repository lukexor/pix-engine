//! `Texture` functions.

use crate::{
    prelude::*,
    renderer::{RendererTexture, Result as RendererResult},
};
use num_traits::AsPrimitive;

/// `TextureId`.
pub type TextureId = usize;

/// `Texture`.
pub struct Texture {
    window_id: WindowId,
    inner: RendererTexture,
    width: u32,
    height: u32,
    format: Option<PixelFormat>,
}

impl Texture {
    /// Returns the window id this `Texture` belongs to.
    #[inline]
    pub fn window_id(&self) -> WindowId {
        self.window_id
    }

    /// Returns the `Texture` width.
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the `Texture` height.
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns the `Texture` dimensions as `(width, height)`.
    #[inline]
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Returns the center position as [Point].
    #[inline]
    pub fn center(&self) -> PointI2 {
        point!(self.width() as i32 / 2, self.height() as i32 / 2)
    }

    /// Returns the `Texture` format.
    #[inline]
    pub fn format(&self) -> Option<PixelFormat> {
        self.format
    }
}

impl Texture {
    pub(crate) fn new(
        window_id: WindowId,
        texture: RendererTexture,
        width: u32,
        height: u32,
        format: Option<PixelFormat>,
    ) -> Self {
        Self {
            window_id,
            inner: texture,
            width,
            height,
            format,
        }
    }

    pub(crate) fn inner(&self) -> &RendererTexture {
        &self.inner
    }

    pub(crate) unsafe fn destroy(self) {
        self.inner.destroy();
    }

    pub(crate) fn inner_mut(&mut self) -> &mut RendererTexture {
        &mut self.inner
    }
}

/// Trait for texture operations on the underlying `Renderer`.
pub(crate) trait TextureRenderer {
    /// Create a `Texture` to draw to.
    fn create_texture(
        &mut self,
        width: u32,
        height: u32,
        format: Option<PixelFormat>,
    ) -> RendererResult<Texture>;

    /// Delete a `Texture`.
    #[cfg(not(target_arch = "wasm32"))]
    unsafe fn delete_texture(&mut self, texture: Texture) -> RendererResult<()>;
    /// Delete a `Texture`.
    #[cfg(target_arch = "wasm32")]
    fn delete_texture(&mut self, texture: Texture) -> RendererResult<()>;

    /// Update texture with pixel data.
    fn update_texture<P: AsRef<[u8]>>(
        &mut self,
        texture: &mut Texture,
        rect: Option<Rect<i32>>,
        pixels: P,
        pitch: usize,
    ) -> RendererResult<()>;

    /// Draw texture to the curent canvas.
    #[allow(clippy::too_many_arguments)]
    fn texture(
        &mut self,
        texture: &mut Texture,
        src: Option<Rect<i32>>,
        dst: Option<Rect<i32>>,
        angle: Scalar,
        center: Option<PointI2>,
        flipped: Option<Flipped>,
        tint: Option<Color>,
    ) -> RendererResult<()>;

    /// Set texture as the target for drawing operations.
    fn set_texture_target(&mut self, texture: &mut Texture);

    /// Clear texture as the target for drawing operations.
    fn clear_texture_target(&mut self);

    /// Clear internal texture cache.
    fn clear_texture_cache(&mut self);
}

impl std::fmt::Debug for Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Texture {{}}")
    }
}

impl PixState {
    /// Draw a portion of a [Texture] to the current canvas resized to the target `dst`.
    pub fn texture<R1, R2>(&mut self, texture: &mut Texture, src: R1, dst: R2) -> PixResult<()>
    where
        R1: Into<Option<Rect<i32>>>,
        R2: Into<Option<Rect<i32>>>,
    {
        Ok(self
            .renderer
            .texture(texture, src.into(), dst.into(), 0.0, None, None, None)?)
    }

    /// Draw a transformed [Texture] to the current canvas resized to the target `rect`, optionally
    /// rotated by an `angle` about the `center` point or `flipped`. `angle` can be in either
    /// radians or degrees based on [AngleMode].
    #[allow(clippy::too_many_arguments)]
    pub fn texture_transformed<R1, R2, A, C, F, T>(
        &mut self,
        texture: &mut Texture,
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
            texture,
            src.into(),
            dst.into(),
            angle,
            center.into(),
            flipped.into(),
            tint.into(),
        )?)
    }

    /// Constructs a `Texture` to render to.
    pub fn create_texture<F>(&mut self, width: u32, height: u32, format: F) -> PixResult<Texture>
    where
        F: Into<Option<PixelFormat>>,
    {
        Ok(self.renderer.create_texture(width, height, format.into())?)
    }

    /// Delete a `Texture`.
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure that the texture to be destroyed was created with the
    /// current canvas. Currently, the only way to violate this is by creating a texture and then
    /// toggling vsync after `PixEngine` initialization. Toggling vsync requires re-creating any
    /// textures in order to safely destroy them.
    ///
    /// Destroying textures created from a dropped canvas is undefined behavior.
    #[cfg(not(target_arch = "wasm32"))]
    pub unsafe fn delete_texture(&mut self, texture: Texture) -> PixResult<()> {
        Ok(self.renderer.delete_texture(texture)?)
    }

    /// Delete a `Texture`.
    #[cfg(target_arch = "wasm32")]
    pub fn delete_texture(&mut self, texture: Texture) -> PixResult<()> {
        Ok(self.renderer.delete_texture(texture)?)
    }

    /// Update the `Texture` with a [u8] [slice] of pixel data.
    pub fn update_texture<R, P>(
        &mut self,
        texture: &mut Texture,
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
        Ok(self.renderer.update_texture(texture, rect, pixels, pitch)?)
    }

    /// Target a `Texture` for drawing operations.
    pub fn with_texture<F>(&mut self, texture: &mut Texture, f: F) -> PixResult<()>
    where
        F: FnOnce(&mut PixState) -> PixResult<()>,
    {
        self.push();
        self.renderer.set_texture_target(texture);
        let result = f(self);
        self.renderer.clear_texture_target();
        self.pop();
        result
    }
}
