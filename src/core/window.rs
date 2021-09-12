//! `Window` functions.

use crate::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, error, ffi::NulError, fmt, path::PathBuf, result};

/// The result type for `Renderer` operations.
pub type Result<T> = result::Result<T, Error>;

/// Represents a possible screen position.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

/// Window Identifier.
pub type WindowId = usize;

#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[allow(variant_size_differences)]
pub enum Cursor {
    System(SystemCursor),
    Image(PathBuf),
}

impl Default for Cursor {
    fn default() -> Self {
        Self::System(SystemCursor::Arrow)
    }
}

impl Cursor {
    /// Constructs a `Cursor` from a file path.
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self::Image(path.into())
    }

    /// Constructs a `Cursor` with `SystemCursor::Arrow`.
    pub fn arrow() -> Self {
        Self::System(SystemCursor::Arrow)
    }

    /// Constructs a `Cursor` with `SystemCursor::IBeam`.
    pub fn ibeam() -> Self {
        Self::System(SystemCursor::IBeam)
    }

    /// Constructs a `Cursor` with `SystemCursor::Hand`.
    pub fn hand() -> Self {
        Self::System(SystemCursor::Hand)
    }
}

/// System Cursor Icon.
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SystemCursor {
    Arrow,
    IBeam,
    Wait,
    Crosshair,
    WaitArrow,
    SizeNWSE,
    SizeNESW,
    SizeWE,
    SizeNS,
    SizeAll,
    No,
    Hand,
}

/// Trait representing window operations.
pub(crate) trait Window {
    /// Get the primary window id.
    fn window_id(&self) -> WindowId;

    /// Set the mouse cursor to a predefined symbol or image, or hides cursor if `None`.
    fn cursor(&mut self, cursor: Option<&Cursor>) -> Result<()>;

    /// Returns a single event or None if the event pump is empty.
    fn poll_event(&mut self) -> Option<Event>;

    /// Get the current window title.
    fn title(&self) -> &str;

    /// Set the current window title.
    fn set_title(&mut self, title: &str) -> Result<()>;

    /// Set the current window title with FPS appended.
    fn set_fps_title(&mut self, fps: usize) -> Result<()>;

    /// Dimensions of the primary window as `(width, height)`.
    fn dimensions(&self, id: WindowId) -> Result<(u32, u32)>;

    /// Set dimensions of the primary window as `(width, height)`.
    fn set_dimensions(&mut self, id: WindowId, dimensions: (u32, u32)) -> Result<()>;

    /// Returns whether the application is fullscreen or not.
    fn fullscreen(&self) -> bool;

    /// Set the application to fullscreen or not.
    fn set_fullscreen(&mut self, val: bool);

    /// Returns whether the window synchronizes frame rate to the screens refresh rate.
    fn vsync(&self) -> bool;

    /// Set the window to synchronize frame rate to the screens refresh rate.
    fn set_vsync(&mut self, val: bool) -> Result<()>;
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
    pub fn with_title(&mut self, title: String) -> &mut Self {
        self.title = title;
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
    /// Invalid text.
    InvalidText(&'static str, NulError),
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
            InvalidText(msg, err) => write!(f, "invalid text: {}, {}", msg, err),
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

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::Other(Cow::from(err))
    }
}

impl From<NulError> for Error {
    fn from(err: NulError) -> Self {
        Self::InvalidTitle("unknown nul error", err)
    }
}
