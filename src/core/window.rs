//! `Window` functions.

use crate::{prelude::*, renderer::RendererSettings};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, error, ffi::NulError, fmt, path::PathBuf, result};

/// The result type for `WindowRenderer` operations.
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

/// A window cursor indicating the position of the mouse.
#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[allow(variant_size_differences)]
pub enum Cursor {
    /// A system supported cursor. e.g. Arrow, Hand, etc.
    System(SystemCursor),
    /// A custom cursor from a image path.
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

    /// Constructs a `Cursor` with `SystemCursor::No`.
    pub fn no() -> Self {
        Self::System(SystemCursor::No)
    }

    /// Constructs a `Cursor` with `SystemCursor::Hand`.
    pub fn hand() -> Self {
        Self::System(SystemCursor::Hand)
    }
}

/// System Cursor Icon.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SystemCursor {
    /// Default arrow cursor.
    Arrow,
    /// Vertical I-Beam icon, typically used for text input position.
    IBeam,
    /// Wait hour-glass icon, typically used as a loading indicator.
    Wait,
    /// Cross-hair icon.
    Crosshair,
    /// Wait hour-glass + Arrow combined.
    WaitArrow,
    /// Resize icon with arrows oriented North-West to South-East.
    SizeNWSE,
    /// Resize icon with arrows oriented North-East to South-West.
    SizeNESW,
    /// Resize icon with arrows oriented West to East.
    SizeWE,
    /// Resize icon with arrows oriented North to South.
    SizeNS,
    /// Resize icon with arrows in all cardinal directions.
    SizeAll,
    /// Circle with a line through it.
    No,
    /// Hand icon, typically used as a clickable indicator.
    Hand,
}

/// Trait representing window operations.
pub(crate) trait WindowRenderer {
    /// Get the primary window ID.
    fn primary_window_id(&self) -> WindowId;

    /// Get the current window target ID.
    fn window_id(&self) -> WindowId;

    /// Create a new window.
    fn create_window(&mut self, s: &RendererSettings) -> Result<WindowId>;

    /// Close a window.
    fn close_window(&mut self, id: WindowId) -> Result<()>;

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
    fn dimensions(&self) -> Result<(u32, u32)>;

    /// Set dimensions of the primary window as `(width, height)`.
    fn set_dimensions(&mut self, dimensions: (u32, u32)) -> Result<()>;

    /// Dimensions of the primary display as `(width, height)`.
    fn display_dimensions(&self) -> Result<(u32, u32)>;

    /// Returns whether the application is fullscreen or not.
    fn fullscreen(&self) -> Result<bool>;

    /// Set the application to fullscreen or not.
    fn set_fullscreen(&mut self, val: bool) -> Result<()>;

    /// Returns whether the window synchronizes frame rate to the screens refresh rate.
    fn vsync(&self) -> bool;

    /// Set the window to synchronize frame rate to the screens refresh rate.
    fn set_vsync(&mut self, val: bool) -> Result<()>;

    /// Set window as the target for drawing operations.
    fn set_window_target(&mut self, id: WindowId) -> Result<()>;

    /// Reset main window as the target for drawing operations.
    fn reset_window_target(&mut self);

    /// Show the current window target.
    fn show(&mut self) -> Result<()>;

    /// Hide the current window target.
    fn hide(&mut self) -> Result<()>;
}

/// WindowBuilder
#[derive(Debug)]
pub struct WindowBuilder<'a> {
    state: &'a mut PixState,
    settings: RendererSettings,
}

impl<'a> WindowBuilder<'a> {
    /// Creates a new WindowBuilder instance.
    pub fn new(s: &'a mut PixState) -> Self {
        let vsync = s.vsync();
        Self {
            state: s,
            settings: RendererSettings {
                vsync,
                ..RendererSettings::default()
            },
        }
    }

    /// Set window dimensions.
    pub fn with_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.settings.width = width;
        self.settings.height = height;
        self
    }

    /// Set a window title.
    pub fn with_title<S: Into<String>>(&mut self, title: S) -> &mut Self {
        self.settings.title = title.into();
        self
    }

    /// Position the window at the given `(x, y)` coordinates of the display.
    pub fn position(&mut self, x: i32, y: i32) -> &mut Self {
        self.settings.x = Position::Positioned(x);
        self.settings.y = Position::Positioned(y);
        self
    }

    /// Position the window in the center of the display.
    pub fn position_centered(&mut self) -> &mut Self {
        self.settings.x = Position::Centered;
        self.settings.y = Position::Centered;
        self
    }

    /// Start window in fullscreen mode.
    pub fn fullscreen(&mut self) -> &mut Self {
        self.settings.fullscreen = true;
        self
    }

    /// Allow window resizing.
    pub fn resizable(&mut self) -> &mut Self {
        self.settings.resizable = true;
        self
    }

    /// Removes the window decoration.
    pub fn borderless(&mut self) -> &mut Self {
        self.settings.borderless = true;
        self
    }

    /// Scales the window.
    pub fn scale(&mut self, x: f32, y: f32) -> &mut Self {
        self.settings.scale_x = x;
        self.settings.scale_y = y;
        self
    }

    /// Set a window icon.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn icon<P>(&mut self, path: P) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.settings.icon = Some(path.into());
        self
    }

    /// Create a new window from the WindowBuilder and return its id.
    ///
    /// Returns Err if any options provided are invalid.
    pub fn build(&mut self) -> Result<WindowId> {
        self.state.renderer.create_window(&self.settings)
    }
}

impl PixState {
    /// Whether the application has focus or not.
    pub fn focused(&self) -> bool {
        matches!(self.env.focused_window, Some(id) if id == self.renderer.window_id())
    }

    /// Get the primary window ID.
    pub fn primary_window_id(&self) -> WindowId {
        self.renderer.primary_window_id()
    }

    /// Get the current window target ID.
    pub fn window_id(&self) -> WindowId {
        self.renderer.window_id()
    }

    /// Create a new [WindowBuilder].
    pub fn window(&mut self) -> WindowBuilder<'_> {
        WindowBuilder::new(self)
    }

    /// Close a window.
    pub fn close_window(&mut self, id: WindowId) -> Result<()> {
        if id == self.window_id() {
            self.env.quit = true;
            return Ok(());
        }
        self.renderer.close_window(id)
    }

    /// The dimensions of the primary window as `(width, height)`.
    pub fn dimensions(&self) -> (u32, u32) {
        self.renderer
            .dimensions()
            .expect("primary window should exist")
    }

    /// Set the dimensions of the primary window from `(width, height)`.
    pub fn set_dimensions(&mut self, dimensions: (u32, u32)) {
        self.renderer
            .set_dimensions(dimensions)
            .expect("primary window should exist")
    }

    /// The width of the primary window.
    pub fn width(&self) -> u32 {
        let (width, _) = self
            .renderer
            .dimensions()
            .expect("primary window should exist");
        width
    }

    /// Set the width of the primary window.
    pub fn set_width(&mut self, width: u32) {
        let (_, height) = self
            .renderer
            .dimensions()
            .expect("primary window should exist");
        self.renderer
            .set_dimensions((width, height))
            .expect("primary window should exist");
    }

    /// The height of the primary window.
    pub fn height(&self) -> u32 {
        let (_, height) = self
            .renderer
            .dimensions()
            .expect("primary window should exist");
        height
    }

    /// Set the height of the primary window.
    pub fn set_height(&mut self, height: u32) {
        let (width, _) = self
            .renderer
            .dimensions()
            .expect("primary window should exist");
        self.renderer
            .set_dimensions((width, height))
            .expect("primary window should exist");
    }

    /// The dimensions of the primary display as `(width, height)`.
    pub fn display_dimensions(&self) -> (u32, u32) {
        self.renderer
            .display_dimensions()
            .expect("primary window should exist")
    }

    /// The width of the primary display.
    pub fn display_width(&self) -> u32 {
        let (width, _) = self
            .renderer
            .display_dimensions()
            .expect("primary window should exist");
        width
    }

    /// The height of the primary display.
    pub fn display_height(&self) -> u32 {
        let (_, height) = self
            .renderer
            .display_dimensions()
            .expect("primary window should exist");
        height
    }

    /// Show the current window target.
    pub fn show_window(&mut self) -> PixResult<()> {
        Ok(self.renderer.show()?)
    }

    /// Hide the current window target.
    pub fn hide_window(&mut self) -> PixResult<()> {
        Ok(self.renderer.hide()?)
    }

    /// Target a `Window` for drawing operations.
    pub fn with_window<F>(&mut self, id: WindowId, f: F) -> PixResult<()>
    where
        for<'r> F: FnOnce(&'r mut PixState) -> PixResult<()>,
    {
        self.push();
        self.renderer.set_window_target(id)?;
        let result = f(self);
        self.renderer.reset_window_target();
        self.pop();
        result
    }
}

/// The error type for `Renderer` operations.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            InvalidTitle(_, err) | InvalidText(_, err) => err.source(),
            _ => None,
        }
    }
}

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
