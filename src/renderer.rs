//! Generic graphics renderer interfaces

use crate::{
    color::Color,
    common,
    event::Event,
    image::Image,
    shape::Rect,
    state::{self, environment::WindowId, settings::BlendMode},
};
use std::{borrow::Cow, error, ffi::NulError, fmt, io, path::PathBuf, result};

#[cfg_attr(feature = "sdl2", path = "renderer/sdl.rs")]
#[cfg_attr(feature = "wasm", path = "renderer/wasm.rs")]
pub(crate) mod renderer_impl;

pub use renderer_impl::Renderer;

/// The result type for [`Renderer`] operations.
pub type Result<T> = result::Result<T, Error>;

/// The error type for [`Renderer`] operations.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Renderer initialization errors.
    InitError,
    /// Renderer I/O errors.
    IoError(io::Error),
    /// Invalid text.
    InvalidText(&'static str, NulError),
    /// Invalid (x, y) window position.
    InvalidPosition(Position, Position),
    /// An overflow occurred
    Overflow(Cow<'static, str>, u32),
    /// Any other unknown error as a string.
    Other(Cow<'static, str>),
}

/// Represents a possible screen position.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Position {
    /// A positioned (x, y) coordinate.
    Positioned(i32),
    /// A coordinate placed in the center of the display.
    Centered,
}

impl Default for Position {
    fn default() -> Self {
        Self::Centered
    }
}

/// Settings used to set up the renderer.
#[derive(Debug, Clone)]
pub(crate) struct RendererSettings {
    pub(crate) title: String,
    pub(crate) icon: Option<PathBuf>,
    pub(crate) x: Position,
    pub(crate) y: Position,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) scale_x: f32,
    pub(crate) scale_y: f32,
    pub(crate) audio_sample_rate: i32,
    pub(crate) fullscreen: bool,
    pub(crate) vsync: bool,
    pub(crate) resizable: bool,
    pub(crate) show_frame_rate: bool,
}

impl Default for RendererSettings {
    fn default() -> Self {
        Self {
            title: String::new(),
            icon: None,
            x: Position::default(),
            y: Position::default(),
            width: 400,
            height: 400,
            scale_x: 1.0,
            scale_y: 1.0,
            audio_sample_rate: 44_100,
            fullscreen: false,
            vsync: false,
            resizable: false,
            show_frame_rate: false,
        }
    }
}

/// A common interface all renderers must implement
pub(crate) trait Rendering: Sized {
    /// Creates a new `Renderer` instance.
    fn init(settings: RendererSettings) -> Result<Self>;

    /// Get the primary window id.
    fn window_id(&self) -> WindowId;

    /// Clears the current canvas to the given clear color.
    fn clear(&mut self);

    /// Set whether the cursor is shown or not.
    fn cursor(&mut self, show: bool);

    /// Sets the color used by the renderer to draw the current canvas.
    fn draw_color(&mut self, color: Color);

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    fn clip(&mut self, rect: Option<Rect>);

    /// Sets the blend mode used by the renderer to draw textures.
    fn blend_mode(&mut self, mode: BlendMode);

    /// Returns a single event or None if the event pump is empty.
    fn poll_event(&mut self) -> Option<Event>;

    /// Returns an iterator of events from the event pump.
    fn poll_events(&mut self) -> Vec<Event>;

    /// Updates the canvas from the current back buffer.
    fn present(&mut self);

    /// Get the current window title.
    fn title(&self) -> &str;

    /// Set the current window title.
    fn set_title<S>(&mut self, title: S) -> Result<()>
    where
        S: AsRef<str>;

    /// Width of the current canvas.
    fn width(&self) -> u32;

    /// Height of the current canvas.
    fn height(&self) -> u32;

    /// Scale the current canvas.
    fn scale(&mut self, x: f32, y: f32) -> Result<()>;

    /// Returns whether the application is fullscreen or not.
    fn is_fullscreen(&self) -> bool;

    /// Set the application to fullscreen or not.
    fn fullscreen(&mut self, val: bool);

    /// Draw text to the current canvas.
    fn text<S>(
        &mut self,
        text: S,
        x: i32,
        y: i32,
        size: u32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()>
    where
        S: AsRef<str>;

    /// Draw a pixel to the current canvas.
    fn point(&mut self, x: i32, y: i32, stroke: Option<Color>) -> Result<()>;

    /// Draw an array of pixels to the current canvas.
    fn points(&mut self, pixels: &[u8], pitch: usize) -> Result<()>;

    /// Draw a line to the current canvas.
    fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, stroke: Option<Color>) -> Result<()>;

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
    ) -> Result<()>;

    /// Draw a rectangle to the current canvas.
    fn rect(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()>;

    /// Draw a ellipse to the current canvas.
    fn ellipse(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()>;

    /// Draw an image to the current canvas.
    fn image(&mut self, x: i32, y: i32, img: &Image) -> Result<()>;

    /// Draw a resized image to the current canvas.
    fn image_resized(&mut self, x: i32, y: i32, w: u32, h: u32, img: &Image) -> Result<()>;

    /// Add audio samples to the audio buffer queue.
    fn enqueue_audio(&mut self, samples: &[f32]);
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            InitError => write!(f, "Renderer initialization error"),
            IoError(err) => err.fmt(f),
            InvalidText(msg, err) => write!(f, "Invalid text: {}, {}", msg, err),
            InvalidPosition(x, y) => write!(f, "Invalid window position: {:?}", (x, y)),
            Overflow(err, val) => write!(f, "{}: {}", err, val),
            Other(err) => write!(f, "Unknown renderer error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            IoError(err) => err.source(),
            _ => None,
        }
    }
}

impl From<Error> for common::Error {
    fn from(err: Error) -> Self {
        Self::RendererError(err)
    }
}

impl From<Error> for state::Error {
    fn from(err: Error) -> Self {
        Self::RendererError(err)
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::Other(Cow::from(err.to_string()))
    }
}
