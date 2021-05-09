use super::{Position, RendererError, RendererResult, RendererSettings, Rendering};
use crate::{
    color::Color,
    event::{Axis, Button, Event, Keycode, MouseButton, WindowEvent},
    image::Image,
    shape::Rect,
};

/// A Web-Assembly [`Renderer`] implementation.
pub struct WasmRenderer {
    title: String,
}

impl Rendering for WasmRenderer {
    /// Creates a new `Renderer` instance.
    fn init(s: &RendererSettings) -> RendererResult<Self> {
        Ok(Self {
            title: s.title.to_owned(),
        })
    }

    /// Clears the current canvas to the given clear color.
    fn clear(&mut self) {
        todo!("clear")
    }

    /// Set whether the cursor is shown or not.
    fn show_cursor(&mut self, show: bool) {
        todo!("show_cursor")
    }

    /// Sets the color used by the renderer to draw the current canvas.
    fn set_draw_color(&mut self, color: Color) {
        todo!("set_draw_color")
    }

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    fn set_clip_rect(&mut self, rect: Option<Rect>) {
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
        &self.title
    }

    /// Set the current window title.
    fn set_title(&mut self, title: &str) -> RendererResult<()> {
        self.title = title.to_owned();
        Ok(())
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
    fn set_scale(&mut self, x: f32, y: f32) -> RendererResult<()> {
        todo!("set_scale")
    }

    /// Returns whether the application is fullscreen or not.
    fn fullscreen(&self) -> bool {
        todo!("fullscreen")
    }

    /// Set the application to fullscreen or not.
    fn set_fullscreen(&mut self, val: bool) {
        todo!("set_fullscreen")
    }

    /// Create a texture to render to.
    fn create_texture(&mut self, width: u32, height: u32) -> RendererResult<usize> {
        todo!("create_texture")
    }

    /// Draw text to the current canvas.
    fn text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        size: u32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> RendererResult<()> {
        todo!("text")
    }

    /// Draw a pixel to the current canvas.
    fn pixel(&mut self, x: i32, y: i32, stroke: Option<Color>) -> RendererResult<()> {
        todo!("pixel")
    }

    /// Draw an array of pixels to the current canvas.
    fn pixels(&mut self, pixels: &[u8], pitch: usize) -> RendererResult<()> {
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

impl std::fmt::Debug for WasmRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Add some fields
        write!(f, "WasmRenderer {{}}")
    }
}
