//! Generic graphics renderer interfaces

use crate::{
    color::Color,
    common,
    event::{Event, EventIterator},
    image::Image,
    shape::Rect,
};
use sdl::SdlRenderer;
use std::borrow::Cow;

pub(crate) mod sdl;

/// `Renderer` Result
pub type Result<T> = std::result::Result<T, Error>;

/// Types of errors the `Rendering` trait can return in a `Result`.
#[derive(Debug)]
pub enum Error {
    /// Indicates an invalid `Renderer` setting.
    InvalidSetting,
    /// Renderer error
    Renderer,
    /// Error when an invalid (x, y) screen position is encountered.
    InvalidPosition,
    /// Any other unknown error as a string.
    Other(Cow<'static, str>),
}

/// Wrapper around a concrete renderer.
pub(crate) type Renderer = SdlRenderer;

/// Represents a possible screen position.
#[derive(Debug, Copy, Clone)]
pub(crate) enum Position {
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
#[derive(Debug)]
pub(crate) struct RendererSettings {
    pub(crate) title: String,
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
    fn init(settings: &RendererSettings) -> Result<Self>;

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

    /// Returns an iterator over events from the event pump.
    fn poll_event_iter(&mut self) -> EventIterator<'_>;

    /// Updates the canvas from the current back buffer.
    fn present(&mut self);

    /// Get the current window title.
    fn title(&self) -> &str;

    /// Set the current window title.
    fn set_title(&mut self, title: &str) -> Result<()>;

    /// Width of the current canvas.
    fn width(&self) -> u32;

    /// Height of the current canvas.
    fn height(&self) -> u32;

    /// Scale the current canvas.
    fn set_scale(&mut self, x: f32, y: f32) -> Result<()>;

    /// Returns whether the application is fullscreen or not.
    fn fullscreen(&self) -> bool;

    /// Set the application to fullscreen or not.
    fn set_fullscreen(&mut self, val: bool);

    /// Create a texture to render to.
    fn create_texture(&mut self, width: u32, height: u32) -> Result<usize>;

    /// Draw an array of pixels to the current canvas.
    fn draw_pixels(&mut self, pixels: &[u8], pitch: usize) -> Result<()>;

    /// Draw text to the current canvas.
    fn text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()>;

    /// Draw a pixel to the current canvas.
    fn pixel(&mut self, x: i32, y: i32, stroke: Option<Color>) -> Result<()>;

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
}

impl Error {
    /// Creates a renderer error from anything that implements Display.
    pub fn renderer<E: std::fmt::Display>(err: E) -> Self {
        Self::Other(Cow::from(err.to_string()))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            InvalidSetting => write!(f, "invalid Renderer setting"), // TODO add setting to this
            Renderer => write!(f, "Renderer error"),                 // TODO: make this more robust
            InvalidPosition => write!(f, "Invalid window position"),
            Other(e) => write!(f, "Renderer Error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<Error> for common::Error {
    fn from(err: Error) -> Self {
        Self::RendererError(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::Other(Cow::from(err))
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::Other(Cow::from(err.to_string()))
    }
}
