use super::{RendererResult, RendererSettings, Rendering};
use crate::{color::Color, event::Event, image::Image, shape::Rect};

/// A Web-Assembly [`Renderer`] implementation.
pub struct Renderer {}

impl Rendering for Renderer {
    /// Creates a new `Renderer` instance.
    fn init(_s: RendererSettings) -> RendererResult<Self> {
        Ok(Self {})
    }

    /// Clears the current canvas to the given clear color.
    fn clear(&mut self) {
        todo!("clear")
    }

    /// Set whether the cursor is shown or not.
    fn cursor(&mut self, _show: bool) {
        todo!("show_cursor")
    }

    /// Sets the color used by the renderer to draw the current canvas.
    fn draw_color(&mut self, _color: Color) {
        todo!("set_draw_color")
    }

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    fn clip(&mut self, _rect: Option<Rect>) {
        todo!("set_clip_rect")
    }

    /// Returns a single event or None if the event pump is empty.
    fn poll_event(&mut self) -> Option<Event> {
        todo!("poll_event")
    }

    /// Updates the canvas from the current back buffer.
    fn present(&mut self) {
        todo!("present")
    }

    /// Get the current window title.
    fn title(&self) -> &str {
        todo!("title")
    }

    /// Set the current window title.
    fn set_title<S>(&mut self, _title: S) -> RendererResult<()>
    where
        S: AsRef<str>,
    {
        todo!("set_title")
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
    fn scale(&mut self, _x: f32, _y: f32) -> RendererResult<()> {
        todo!("set_scale")
    }

    /// Returns whether the application is fullscreen or not.
    fn is_fullscreen(&self) -> bool {
        todo!("fullscreen")
    }

    /// Set the application to fullscreen or not.
    fn fullscreen(&mut self, _val: bool) {
        todo!("set_fullscreen")
    }

    /// Draw text to the current canvas.
    fn text<S>(
        &mut self,
        text: S,
        x: i32,
        y: i32,
        size: u32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> RendererResult<()>
    where
        S: AsRef<str>,
    {
        todo!("text")
    }

    /// Draw a pixel to the current canvas.
    fn point(&mut self, x: i32, y: i32, stroke: Option<Color>) -> RendererResult<()> {
        todo!("pixel")
    }

    /// Draw an array of pixels to the current canvas.
    fn points(&mut self, pixels: &[u8], pitch: usize) -> RendererResult<()> {
        todo!("pixels")
    }

    /// Draw a line to the current canvas.
    fn line(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        stroke: Option<Color>,
    ) -> RendererResult<()> {
        todo!("line")
    }

    /// Draw a triangle to the current canvas.
    #[allow(clippy::too_many_arguments)]
    fn triangle(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> RendererResult<()> {
        todo!("triangle")
    }

    /// Draw a rectangle to the current canvas.
    fn rect(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> RendererResult<()> {
        todo!("rect")
    }

    /// Draw a ellipse to the current canvas.
    fn ellipse(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> RendererResult<()> {
        todo!("ellipse")
    }

    /// Draw an image to the current canvas.
    fn image(&mut self, x: i32, y: i32, img: &Image) -> RendererResult<()> {
        todo!("image")
    }
}

impl std::fmt::Debug for Renderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Add some fields
        write!(f, "WasmRenderer {{}}")
    }
}