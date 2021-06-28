use crate::{
    audio::Audio,
    prelude::*,
    renderer::{RendererSettings, Rendering, Result},
    window::{Result as WindowResult, Window},
};
use num_traits::AsPrimitive;

/// A Web-Assembly [Renderer] implementation.
pub(crate) struct Renderer {}

impl Audio for Renderer {
    /// Add audio samples to the audio buffer queue.
    fn enqueue_audio(&mut self, samples: &[f32]) {
        todo!("enqueue_audio")
    }
}

impl Window for Renderer {
    /// Get the primary window id.
    fn window_id(&self) -> WindowId {
        todo!("window_id")
    }

    /// Set whether the cursor is shown or not.
    fn cursor(&mut self, show: bool) {
        todo!("show_cursor")
    }

    /// Returns a single event or None if the event pump is empty.
    fn poll_event(&mut self) -> Option<Event> {
        todo!("poll_event")
    }

    /// Get the current window title.
    fn title(&self) -> &str {
        todo!("title")
    }

    /// Set the current window title.
    fn set_title(&mut self, title: &str) -> WindowResult<()> {
        todo!("set_title")
    }

    /// Width of the window.
    fn window_width(&self) -> WindowResult<u32> {
        todo!("window_width")
    }

    /// Height of the window.
    fn window_height(&self) -> WindowResult<u32> {
        todo!("window_height")
    }

    /// Resize the window.
    fn resize(&mut self, width: u32, height: u32) -> WindowResult<()> {
        todo!("resize")
    }

    /// Returns whether the application is fullscreen or not.
    fn fullscreen(&self) -> bool {
        todo!("fullscreen")
    }

    /// Set the application to fullscreen or not.
    fn set_fullscreen(&mut self, val: bool) {
        todo!("set_fullscreen")
    }
}

impl Rendering for Renderer {
    /// Creates a new Renderer instance.
    fn new(s: &RendererSettings) -> Result<Self> {
        Ok(Self {})
    }

    /// Clears the current canvas to the given clear color.
    fn clear(&mut self) {
        todo!("clear")
    }

    /// Sets the color used by the renderer to draw the current canvas.
    fn set_draw_color(&mut self, color: impl Into<Color>) {
        todo!("set_draw_color")
    }

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    fn clip<T, R>(&mut self, rect: R)
    where
        T: AsPrimitive<Scalar>,
        R: Into<Option<Rect<T>>>,
    {
        todo!("set_clip_rect")
    }

    /// Sets the blend mode used by the renderer to draw textures.
    fn blend_mode(&mut self, mode: BlendMode) {
        todo!("blend_mode")
    }

    /// Updates the canvas from the current back buffer.
    fn present(&mut self) {
        todo!("present")
    }

    /// Width of the current canvas.
    fn width(&self) -> u32 {
        todo!("width")
    }

    /// Height of the current canvas.
    fn height(&self) -> u32 {
        todo!("height")
    }

    /// Scale the current canvas.
    fn scale<T: AsPrimitive<f32>>(&mut self, x: T, y: T) -> Result<()> {
        todo!("set_scale")
    }

    /// Create a texture to draw to.
    fn create_texture<T, F>(&mut self, width: T, height: T, format: F) -> Result<TextureId>
    where
        T: Into<Scalar>,
        F: Into<Option<PixelFormat>>,
    {
        todo!("create_teture")
    }

    /// Delete a texture.
    fn delete_texture(&mut self, texture_id: TextureId) -> Result<()> {
        todo!("delete_texture");
    }

    /// Update texture with pixel data.
    fn update_texture<R, P>(
        &mut self,
        texture_id: TextureId,
        rect: R,
        pixels: P,
        pitch: usize,
    ) -> Result<()>
    where
        R: Into<Option<Rect<Scalar>>>,
        P: AsRef<[u8]>,
    {
        todo!("update_texture")
    }

    /// Draw texture canvas.
    fn texture<R>(&mut self, texture_id: usize, src: R, dst: R) -> Result<()>
    where
        R: Into<Option<Rect<Scalar>>>,
    {
        todo!("texture")
    }

    /// Draw text to the current canvas.
    fn text<P, T, C>(&mut self, position: P, text: T, size: u32, fill: C, _stroke: C) -> Result<()>
    where
        P: Into<Point<Scalar>>,
        T: AsRef<str>,
        C: Into<Option<Color>>,
    {
        todo!("text")
    }

    /// Draw a pixel to the current canvas.
    fn point<P, C>(&mut self, p: P, color: C) -> Result<()>
    where
        P: Into<Point<Scalar>>,
        C: Into<Option<Color>>,
    {
        todo!("pixels")
    }

    /// Draw a line to the current canvas.
    fn line<L, C>(&mut self, line: L, color: C) -> Result<()>
    where
        L: Into<Line<Scalar>>,
        C: Into<Option<Color>>,
    {
        todo!("line")
    }

    /// Draw a triangle to the current canvas.
    fn triangle<T, C>(&mut self, tri: T, fill: C, stroke: C) -> Result<()>
    where
        T: Into<Triangle<Scalar>>,
        C: Into<Option<Color>>,
    {
        todo!("triangle")
    }

    /// Draw a rectangle to the current canvas.
    fn rect<R, C>(&mut self, rect: R, fill: C, stroke: C) -> Result<()>
    where
        R: Into<Rect<Scalar>>,
        C: Into<Option<Color>>,
    {
        todo!("rect")
    }

    /// Draw a polygon to the current canvas.
    fn polygon<C, V>(&mut self, vx: V, vy: V, fill: C, stroke: C) -> Result<()>
    where
        C: Into<Option<Color>>,
        V: AsRef<[Scalar]>,
    {
        todo!("polygon")
    }

    /// Draw a ellipse to the current canvas.
    fn ellipse<E, C>(&mut self, ellipse: E, fill: C, stroke: C) -> Result<()>
    where
        E: Into<Ellipse<Scalar>>,
        C: Into<Option<Color>>,
    {
        todo!("ellipse")
    }

    /// Draw an image to the current canvas.
    fn image<P>(&mut self, position: P, img: &Image) -> Result<()>
    where
        P: Into<Point<Scalar>>,
    {
        todo!("image")
    }

    /// Draw a resized image to the current canvas.
    fn image_resized<R>(&mut self, dst_rect: R, img: &Image) -> Result<()>
    where
        R: Into<Rect<Scalar>>,
    {
        todo!("image_resized")
    }
}

impl std::fmt::Debug for Renderer {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("WasmRenderer {{}}")
    }
}
