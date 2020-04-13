use crate::{color::Color, event::PixEvent, state::rendering::BlendMode, PixEngineResult};

#[cfg(all(feature = "sdl2-renderer", not(feature = "wasm-renderer")))]
pub(super) mod sdl2;
#[cfg(all(feature = "wasm-renderer", not(feature = "sdl2-renderer")))]
pub(super) mod wasm;

#[cfg(all(feature = "sdl2-renderer", not(feature = "wasm-renderer")))]
pub(crate) fn load_renderer(
    title: &str,
    width: u32,
    height: u32,
) -> PixEngineResult<sdl2::Sdl2Renderer> {
    sdl2::Sdl2Renderer::new(title, width, height)
}
#[cfg(all(feature = "wasm-renderer", not(feature = "sdl2-renderer")))]
pub(crate) fn load_renderer(
    title: &str,
    width: u32,
    height: u32,
) -> PixEngineResult<wasm::WasmRenderer> {
    wasm::WasmRenderer::new(title, width, height)
}

pub(crate) trait Renderer {
    /// Settings

    /// Set title for the current window target.
    ///
    /// Errors if the title contains a nul byte.
    fn set_title(&mut self, _title: &str) -> PixEngineResult<()>;

    /// Get draw color for the current window target.
    fn draw_color(&self) -> Color;

    /// Set draw color for drawing operations on the current window target.
    fn set_draw_color<C: Into<Color>>(&mut self, _color: C);

    /// Get the blending mode for the current window target.
    fn blend_mode(&self) -> BlendMode;

    /// Set the blending mode for drawing operations on the current window target.
    fn set_blend_mode(&mut self, _mode: BlendMode);

    /// Returns a list of events from the event queue since last time poll_events
    /// was called.
    fn poll_events(&mut self) -> Vec<PixEvent>;

    /// Rendering

    /// Presents changes made to the canvas on the current window target since present was last
    /// called.
    fn present(&mut self);

    /// Presents changes made to the canvases of all windows since present was last called.
    fn present_all(&mut self);

    /// Clears the canvas on the current window target to the current draw color.
    fn clear(&mut self);

    /// Clears all canvases of all windows to their current draw colors.
    fn clear_all(&mut self);

    /// Get the scale_x and scale_y factors for the current window target.
    // fn scale(&self) -> (f32, f32);

    /// Set the scale_x and scale_y factors for the current window target.
    // fn set_scale(&mut self, _scale_x: f32, _scale_y: f32);

    /// Get the clipping rectangle for the current window target.
    // fn clip_rect(&self);

    /// Set the clipping rectangle for the current window target.
    // fn set_clip_rect<R: Into<Option<Rect>>>(&mut self, _rect: R);

    /// Get the viewport rectangle for the current window target.
    // fn viewport(&self) -> Rect;

    /// Set the viewport rectangle for the current window target.
    // fn set_viewport<R: Into<Option<Rect>>>(&mut self, _rect: R);

    /// Drawing

    /// Draw a point on the current window target.
    // fn draw_point<P: Into<Point>>(&mut self, _point: P);

    /// Draw multiple points on the current window target.
    // fn draw_points<'a, P: Into<&'a [Point]>>(&mut self, _points: P);

    /// Draw a line on the current window target.
    // fn draw_line<P1: Into<Point>, P2: Into<Point>>(&mut self, _start: P1, _end: P2);

    /// Draw a series of lines on the current window target.
    // fn draw_lines<'a, P: Into<&'a [Point]>>(&mut self, _points: P);

    /// Draw a rectangle on the current window target.
    // fn draw_rect<R: Into<Rect>>(&mut self, _rect: R);

    /// Draw multiple rectangles on the current window target.
    // fn draw_rects<'a, R: Into<&'a [Rect]>>(&mut self, _rects: R)

    /// Draw a filled rectangle on the current window target. Passing None will fill the entire
    /// rendering target.
    // fn fill_rect<R: Into<Option<Rect>>>(&mut self, _rect: R);

    /// Draw multiple filled rectangles on the current window target.
    // fn fill_rects<'a, R: Into<&'a [Rect]>>(&mut self, _rects: R)

    /// Reads pixels from the current window target.
    /// # Remarks
    /// WARNING: This is a very slow operation, and should not be used frequently.
    // fn read_pixels<R: Into<Option<Rect>>>(&self, _rect: R)

    /// Textures
    // TODO
    // copy
    // copy_ex
    //

    /// Window Management

    /// Set a new window target.
    ///
    /// Errors if the window_id is not a valid window_id.
    fn push_window_target(&mut self, _window_id: u32) -> PixEngineResult<()>;

    /// Removes the current window target and switches it to the previous
    /// current window target.
    ///
    /// Will not remove the last window target (the one created upon engine creation).
    fn pop_window_target(&mut self) -> Option<u32>;

    /// Returns the window_id of the current window target
    fn current_window_target(&self) -> u32;

    /// Create and open a new window.
    ///
    /// Errors if the window can't be created for any reason.
    fn create_window(&mut self, _title: &str, _width: u32, _height: u32) -> PixEngineResult<u32>;

    /// Close the current window target.
    ///
    /// Returns true when all windows are closed.
    fn close_window(&mut self) -> bool;
}
