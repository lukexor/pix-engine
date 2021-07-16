//! `Window` functions.

use crate::prelude::{Event, PixError};
use num_traits::AsPrimitive;
use std::{borrow::Cow, error, ffi::NulError, fmt, result};

/// The result type for `Renderer` operations.
pub type Result<T> = result::Result<T, Error>;

/// Represents a possible screen position.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Position {
    /// A positioned `(x, y)` coordinate.
    Positioned(i32),
    /// A coordinate placed in the center of the display.
    Centered,
}

impl Default for Position {
    fn default() -> Self {
        Self::Centered
    }
}

/// Window Identifier
pub type WindowId = u32;

/// Trait representing window operations.
pub(crate) trait Window {
    /// Get the primary window id.
    fn window_id(&self) -> WindowId;

    /// Set whether the cursor is shown or not.
    fn cursor(&mut self, show: bool);

    /// Returns a single event or None if the event pump is empty.
    fn poll_event(&mut self) -> Option<Event>;

    /// Get the current window title.
    fn title(&self) -> &str;

    /// Set the current window title.
    fn set_title(&mut self, title: &str) -> Result<()>;

    /// Dimensions of the primary window as `(width, height)`.
    fn dimensions(&self, id: WindowId) -> Result<(u32, u32)>;

    /// Set dimensions of the primary window as `(width, height)`.
    fn set_dimensions(&mut self, id: WindowId, dimensions: (u32, u32)) -> Result<()>;

    /// Resize the window.
    fn resize<T>(&mut self, width: T, height: T) -> Result<()>
    where
        T: AsPrimitive<u32>;

    /// Returns whether the application is fullscreen or not.
    fn fullscreen(&self) -> bool;

    /// Set the application to fullscreen or not.
    fn set_fullscreen(&mut self, val: bool);
}

/// WindowBuilder
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct WindowBuilder {
    title: String,
    width: u32,
    height: u32,
}

impl WindowBuilder {
    /// Creates a new WindowBuilder instance.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            title: String::new(),
            width,
            height,
        }
    }

    /// Set a window title.
    pub fn with_title<S>(&mut self, title: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.title = title.as_ref().to_owned();
        self
    }

    /// Create a new window from the WindowBuilder and return its id.
    ///
    /// Returns Err if any options provided are invalid.
    pub fn build(&self) -> Result<WindowId> {
        todo!("secondary windows are not yet implemented");
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self::new(400, 400)
    }
}

/// The error type for `Renderer` operations.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Invalid window title.
    InvalidTitle(&'static str, NulError),
    /// Invalid [WindowId].
    InvalidWindow(WindowId),
    /// Invalid `(x, y)` window [Position].
    InvalidPosition(Position, Position),
    /// An overflow occurred.
    Overflow(Cow<'static, str>, u32),
    /// Any other unknown error as a string.
    Other(Cow<'static, str>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            InvalidTitle(msg, err) => write!(f, "invalid title: {}, {}", msg, err),
            InvalidWindow(window_id) => write!(f, "invalid window id: {}", window_id),
            InvalidPosition(x, y) => write!(f, "invalid window position: {:?}", (x, y)),
            Overflow(err, val) => write!(f, "overflow {}: {}", err, val),
            Other(err) => write!(f, "unknown window error: {}", err),
        }
    }
}

impl error::Error for Error {}

impl From<Error> for PixError {
    fn from(err: Error) -> Self {
        Self::WindowError(err)
    }
}

impl From<NulError> for Error {
    fn from(err: NulError) -> Self {
        Self::InvalidTitle("unknown nul error", err)
    }
}