use crate::{
    color::Color,
    event::PixEvent,
    shape::{Line, Point, Rect},
    state::rendering::{BlendMode, Texture},
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
    /// # Settings

    /// Set title for the current window target.
    ///
    /// Errors if the title contains a nul byte.
    fn title(&mut self, title: &str) -> Result<()>;

    /// Sets the audio sample rate for the audio playback in Hz.
    fn audio_sample_rate(&mut self, rate: i32) -> Result<()>;

    /// Set draw color for the clear operation on the current window target.
    fn background(&mut self, color: Color);

    /// Set draw color for the fill operations on the current window target.
    fn fill(&mut self, color: Option<Color>);

    /// Set draw color for the drawing outlines on the current window target.
    fn stroke(&mut self, color: Option<Color>);

    /// Get the blending mode for the current window target.
    fn get_blend_mode(&self) -> BlendMode;

    /// Set the blending mode for drawing operations on the current window target.
    fn blend_mode(&mut self, mode: BlendMode);

    /// Get the scale_x and scale_y factors for the current window target.
    fn get_scale(&self) -> (f32, f32);

    /// Set the scale_x and scale_y factors for the current window target.
    fn scale(&mut self, scale_x: f32, scale_y: f32) -> Result<()>;

    /// # Input

    /// Returns a list of events from the event queue since last time poll_events
    /// was called.
    fn poll_events(&mut self) -> Vec<PixEvent>;

    /// # Rendering

    /// Presents changes made to the canvas on the current window target since present was last
    /// called.
    fn present(&mut self);

    /// Presents changes made to the canvases of all windows since present was last called.
    fn present_all(&mut self);

    /// Clears the canvas on the current window target to the current draw color.
    fn clear(&mut self);

    /// Clears all canvases of all windows to their current draw colors.
    fn clear_all(&mut self);

    /// Get the clipping rectangle for the current window target.
    fn get_clip_rect(&self) -> Option<Rect>;

    /// Set the clipping rectangle for the current window target.
    fn clip_rect(&mut self, rect: Option<Rect>);

    /// Get the viewport rectangle for the current window target.
    fn get_viewport(&self) -> Rect;

    /// Set the viewport rectangle for the current window target.
    fn viewport(&mut self, rect: Option<Rect>);

    /// # Drawing

    /// Draw a point on the current window target.
    fn point(&mut self, point: Point) -> Result<()>;

    /// Draw a line on the current window target.
    fn line(&mut self, line: Line) -> Result<()>;

    /// Draw a rectangle on the current window target.
    fn rect(&mut self, rect: Rect) -> Result<()>;

    /// Reads pixels from the current window target.
    ///
    /// # Remarks
    /// WARNING: This is a very slow operation, and should not be used frequently.
    fn read_pixels(&self, rect: Rect) -> Result<Vec<u8>>;

    /// # Textures

    /// Copy all or a portion of a texture to the current window target.
    ///
    /// - If `src` is `None`, the entire texture is copied.
    /// - If `dst` is `None`, the texture will be stretched to fill the entire target.
    fn copy(&mut self, texture: Texture, src: Option<Rect>, dst: Option<Rect>) -> Result<()>;
}
