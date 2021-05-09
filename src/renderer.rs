//! Generic graphics renderer interfaces

use crate::{
    color::Color, common::PixError, event::Event, image::Image, shape::Rect, state::PixStateError,
};
#[cfg(all(feature = "sdl2", not(feature = "wasm")))]
use sdl::SdlRenderer;
use std::{borrow::Cow, error, ffi::NulError, fmt, io, path::PathBuf, result};
#[cfg(all(feature = "wasm", not(feature = "sdl2")))]
use wasm::WasmRenderer;

#[cfg(all(feature = "sdl2", not(feature = "wasm")))]
pub(crate) mod sdl;
#[cfg(all(feature = "wasm", not(feature = "sdl2")))]
pub(crate) mod wasm;

/// `Renderer` Result
pub type RendererResult<T> = result::Result<T, RendererError>;

/// Types of errors the `Rendering` trait can return in a `Result`.
#[non_exhaustive]
#[derive(Debug)]
pub enum RendererError {
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

/// Wrapper around a concrete renderer.
#[cfg(all(feature = "sdl2", not(feature = "wasm")))]
pub(crate) type Renderer = SdlRenderer;
#[cfg(all(feature = "wasm", not(feature = "sdl2")))]
pub(crate) type Renderer = WasmRenderer;

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
    pub(crate) fullscreen: bool,
    pub(crate) vsync: bool,
    pub(crate) resizable: bool,
    pub(crate) scale_x: f32,
    pub(crate) scale_y: f32,
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
            fullscreen: false,
            vsync: false,
            resizable: false,
            scale_x: 1.0,
            scale_y: 1.0,
        }
    }
}

/// A common interface all renderers must implement
pub(crate) trait Rendering: Sized {
    /// Creates a new `Renderer` instance.
    fn init(settings: RendererSettings) -> RendererResult<Self>;

    /// Clears the current canvas to the given clear color.
    fn clear(&mut self);

    /// Set whether the cursor is shown or not.
    fn show_cursor(&mut self, show: bool);

    /// Sets the color used by the renderer to draw the current canvas.
    fn set_draw_color(&mut self, color: Color);

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    fn set_clip_rect(&mut self, rect: Option<Rect>);

    /// Returns a single event or None if the event pump is empty.
    fn poll_event(&mut self) -> Option<Event>;

    /// Updates the canvas from the current back buffer.
    fn present(&mut self);

    /// Get the current window title.
    fn title(&self) -> &str;

    /// Set the current window title.
    fn set_title(&mut self, title: &str) -> RendererResult<()>;

    /// Width of the current canvas.
    fn width(&self) -> u32;

    /// Height of the current canvas.
    fn height(&self) -> u32;

    /// Scale the current canvas.
    fn set_scale(&mut self, x: f32, y: f32) -> RendererResult<()>;

    /// Returns whether the application is fullscreen or not.
    fn fullscreen(&self) -> bool;

    /// Set the application to fullscreen or not.
    fn set_fullscreen(&mut self, val: bool);

    /// Create a texture to render to.
    fn create_texture(&mut self, width: u32, height: u32) -> RendererResult<usize>;

    /// Draw text to the current canvas.
    fn text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        size: u32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> RendererResult<()>;

    /// Draw a pixel to the current canvas.
    fn pixel(&mut self, x: i32, y: i32, stroke: Option<Color>) -> RendererResult<()>;

    /// Draw an array of pixels to the current canvas.
    fn pixels(&mut self, pixels: &[u8], pitch: usize) -> RendererResult<()>;

    /// Draw a line to the current canvas.
    fn line(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        stroke: Option<Color>,
    ) -> RendererResult<()>;

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
    ) -> RendererResult<()>;

    /// Draw a rectangle to the current canvas.
    fn rect(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> RendererResult<()>;

    /// Draw a ellipse to the current canvas.
    fn ellipse(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> RendererResult<()>;

    /// Draw an image to the current canvas.
    fn image(&mut self, x: i32, y: i32, img: &Image) -> RendererResult<()>;
}

impl fmt::Display for RendererError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RendererError::*;
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

impl error::Error for RendererError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<RendererError> for PixError {
    fn from(err: RendererError) -> Self {
        Self::RendererError(err)
    }
}

impl From<RendererError> for PixStateError {
    fn from(err: RendererError) -> Self {
        Self::RendererError(err)
    }
}

impl From<std::num::TryFromIntError> for RendererError {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::Other(Cow::from(err.to_string()))
    }
}
