use crate::{
    color::Color,
    event::PixEvent,
    shape::{Point, Rect},
    state::rendering::BlendMode,
};
use std::{borrow::Cow, error, ffi::NulError, fmt};

/// Result type for Renderer Errors.
pub type Result<T> = std::result::Result<T, Error>;

/// Types of errors Renderers can return in a result.
#[derive(Debug)]
pub enum Error {
    IntegerOverflows(Cow<'static, str>, u32),
    InvalidWindowTarget(u32),
    InvalidWidth(u32),
    InvalidHeight(u32),
    InvalidString(NulError),
    Other(Cow<'static, str>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            IntegerOverflows(err, val) => write!(f, "integer overflowed {}: {}", val, err),
            InvalidWindowTarget(t) => write!(f, "invalid window_target: {}", &t),
            InvalidWidth(w) => write!(f, "invalid width: {}", &w),
            InvalidHeight(h) => write!(f, "invalid height: {}", &h),
            InvalidString(err) => write!(f, "invalid string: {}", &err),
            Other(desc) => write!(f, "{}", &desc),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Other(err.into())
    }
}

#[cfg(all(feature = "sdl2-renderer", not(feature = "wasm-renderer")))]
pub(super) mod sdl2;
#[cfg(all(feature = "wasm-renderer", not(feature = "sdl2-renderer")))]
pub(super) mod wasm;

#[cfg(all(feature = "sdl2-renderer", not(feature = "wasm-renderer")))]
pub(crate) fn load_renderer(title: &str, width: u32, height: u32) -> Result<sdl2::Sdl2Renderer> {
    sdl2::Sdl2Renderer::new(title, width, height)
}
#[cfg(all(feature = "wasm-renderer", not(feature = "sdl2-renderer")))]
pub(crate) fn load_renderer(title: &str, width: u32, height: u32) -> Result<wasm::WasmRenderer> {
    wasm::WasmRenderer::new(title, width, height)
}

pub(crate) trait Renderer {
    /// Settings

    /// Set title for the current window target.
    ///
    /// Errors if the title contains a nul byte.
    fn title(&mut self, _title: &str) -> Result<()>;

    /// Sets the audio sample rate for the audio playback in Hz.
    fn audio_sample_rate(&mut self, rate: i32) -> Result<()>;

    /// Set draw color for the clear operation on the current window target.
    fn background<C: Into<Color>>(&mut self, _color: C);

    /// Set draw color for the fill operations on the current window target.
    fn fill<C: Into<Option<Color>>>(&mut self, _color: C);

    /// Set draw color for the drawing outlines on the current window target.
    fn stroke<C: Into<Option<Color>>>(&mut self, _color: C);

    /// Get the blending mode for the current window target.
    fn get_blend_mode(&self) -> BlendMode;

    /// Set the blending mode for drawing operations on the current window target.
    fn blend_mode(&mut self, _mode: BlendMode);

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
    fn get_scale(&self) -> (f32, f32);

    /// Set the scale_x and scale_y factors for the current window target.
    fn scale(&mut self, _scale_x: f32, _scale_y: f32) -> Result<()>;

    /// Get the clipping rectangle for the current window target.
    fn get_clip_rect(&self) -> Option<Rect>;

    /// Set the clipping rectangle for the current window target.
    fn clip_rect<R: Into<Option<Rect>>>(&mut self, _rect: R);

    /// Get the viewport rectangle for the current window target.
    fn get_viewport(&self) -> Rect;

    /// Set the viewport rectangle for the current window target.
    fn viewport<R: Into<Option<Rect>>>(&mut self, _rect: R);

    /// Drawing

    /// Draw a point on the current window target.
    fn draw_point<P: Into<Point>>(&mut self, _point: P) -> Result<()>;

    /// Draw multiple points on the current window target.
    fn draw_points<'a, P: Into<&'a [Point]>>(&mut self, _points: P) -> Result<()>;

    /// Draw a line on the current window target.
    fn draw_line<P1: Into<Point>, P2: Into<Point>>(&mut self, _start: P1, _end: P2) -> Result<()>;

    /// Draw a series of lines on the current window target.
    fn draw_lines<'a, P: Into<&'a [Point]>>(&mut self, _points: P) -> Result<()>;

    /// Draw a rectangle on the current window target.
    fn draw_rect<R: Into<Rect>>(&mut self, _rect: R) -> Result<()>;

    /// Draw multiple rectangles on the current window target.
    fn draw_rects<'a, R: Into<&'a [Rect]>>(&mut self, _rects: R) -> Result<()>;

    /// Draw a filled rectangle on the current window target. Passing None will fill the entire
    /// rendering target.
    fn fill_rect<R: Into<Option<Rect>>>(&mut self, _rect: R) -> Result<()>;

    /// Draw multiple filled rectangles on the current window target.
    fn fill_rects<'a, R: Into<&'a [Rect]>>(&mut self, _rects: R) -> Result<()>;

    // /// Reads pixels from the current window target.
    // /// # Remarks
    // /// WARNING: This is a very slow operation, and should not be used frequently.
    // fn read_pixels<R: Into<Option<Rect>>>(&self, _rect: R) -> Result<()>;

    // Textures
    // TODO
    // copy
    // copy_ex
}
