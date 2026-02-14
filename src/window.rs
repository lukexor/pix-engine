//! `Window` methods.
//!
//! Provides window creation and manipulation methods on [`PixState`].
//!
//! Provided methods:
//!
//! - [`PixState::window_id`]: Get current window target ID.
//! - [`PixState::window`]: Create a [`WindowBuilder`] to open a new window.
//! - [`PixState::close_window`]: Close a window by ID.
//! - [`PixState::dimensions`]: Get the current render target (window or texture) dimensions as
//!   `(width, height)`.
//! - [`PixState::window_dimensions`]: Get the current window target dimensions as `(width, height)`.
//! - [`PixState::set_window_dimensions`]: Set the current window target dimensions.
//! - [`PixState::viewport`]: Get the current render target (window or texture) viewport.
//! - [`PixState::set_viewport`]: Set the current render target (window or texture) viewport.
//! - [`PixState::clear_viewport`]: Clear the current render target (window or texture) viewport
//!   back to the entire render size.
//! - [`PixState::width`]: Get the current render target (window or texture) width.
//! - [`PixState::window_width`]: Get the current window target width.
//! - [`PixState::set_window_width`]: Set the current window target width.
//! - [`PixState::height`]: Get the current render target (window or texture) height.
//! - [`PixState::window_height`]: Get the current window target height.
//! - [`PixState::set_window_height`]: Set the current window target height.
//! - [`PixState::center`]: Get the current render target (window or texture) center.
//! - [`PixState::window_center`]: Get the current window target center.
//! - [`PixState::display_dimensions`]: Get the primary display dimensions as `(width, height)`.
//! - [`PixState::display_width`]: Get the primary display width.
//! - [`PixState::display_height`]: Get the primary display height.
//! - [`PixState::show_window`]: Show the current window target if it is hidden.
//! - [`PixState::hide_window`]: Hide the current window target if it is shown.
//! - [`PixState::set_window_target`]: Set a window as the primary target for drawing operations.
//! - [`PixState::reset_window_target`]: Reset window target back to the primary window for drawing
//!   operations.

use crate::{
    image::Icon,
    ops::clamp_dimensions,
    prelude::*,
    renderer::{Renderer, RendererSettings},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

/// Represents a possible screen position.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Position {
    /// A positioned `(x, y)` coordinate.
    Positioned(i32),
    /// A coordinate placed in the center of the display.
    #[default]
    Centered,
}

/// Window Identifier.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WindowId(pub(crate) u32);

impl fmt::Display for WindowId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for WindowId {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WindowId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A window cursor indicating the position of the mouse.
#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(target_arch = "wasm32", derive(Copy))]
#[allow(variant_size_differences)]
pub enum Cursor {
    /// A system supported cursor. e.g. Arrow, Hand, etc.
    System(SystemCursor),
    #[cfg(not(target_arch = "wasm32"))]
    /// A custom cursor from a image path starting at `(x, y)`.
    Image(PathBuf, (i32, i32)),
}

impl Default for Cursor {
    fn default() -> Self {
        Self::System(SystemCursor::Arrow)
    }
}

impl Cursor {
    /// Constructs a `Cursor` from a file path.
    #[inline]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new<P: Into<PathBuf>>(path: P, x: i32, y: i32) -> Self {
        Self::Image(path.into(), (x, y))
    }

    /// Constructs a `Cursor` with `SystemCursor::Arrow`.
    #[inline]
    #[must_use]
    pub const fn arrow() -> Self {
        Self::System(SystemCursor::Arrow)
    }

    /// Constructs a `Cursor` with `SystemCursor::IBeam`.
    #[inline]
    #[must_use]
    pub const fn ibeam() -> Self {
        Self::System(SystemCursor::IBeam)
    }

    /// Constructs a `Cursor` with `SystemCursor::No`.
    #[inline]
    #[must_use]
    pub const fn no() -> Self {
        Self::System(SystemCursor::No)
    }

    /// Constructs a `Cursor` with `SystemCursor::Hand`.
    #[inline]
    #[must_use]
    pub const fn hand() -> Self {
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
    /// Get the count of open windows.
    fn window_count(&self) -> usize;

    /// Get the primary window ID.
    fn primary_window_id(&self) -> WindowId;

    /// Get the current window target ID.
    fn window_id(&self) -> WindowId;

    /// Create a new window.
    fn create_window(&mut self, s: &mut RendererSettings) -> PixResult<WindowId>;

    /// Close a window.
    fn close_window(&mut self, id: WindowId) -> PixResult<()>;

    /// Set the mouse cursor to a predefined symbol or image, or hides cursor if `None`.
    fn cursor(&mut self, cursor: Option<&Cursor>) -> PixResult<()>;

    /// Returns a single event or None if the event pump is empty.
    fn poll_event(&mut self) -> Option<Event>;

    /// Get the current window title.
    fn title(&self) -> &str;

    /// Set the current window title.
    fn set_title(&mut self, title: &str) -> PixResult<()>;

    /// Set the average frames-per-second rendered.
    fn set_fps(&mut self, fps: f32) -> PixResult<()>;

    /// Dimensions of the current render target as `(width, height)`.
    fn dimensions(&self) -> PixResult<(u32, u32)>;

    /// Dimensions of the current window target as `(width, height)`.
    fn window_dimensions(&self) -> PixResult<(u32, u32)>;

    /// Position of the current window target as `(x, y)`.
    fn window_position(&self) -> PixResult<(i32, i32)>;

    /// Set dimensions of the current window target as `(width, height)`.
    fn set_window_dimensions(&mut self, dimensions: (u32, u32)) -> PixResult<()>;

    /// Returns the rendering viewport of the current render target.
    fn viewport(&self) -> PixResult<Rect<i32>>;

    /// Set the rendering viewport of the current render target.
    fn set_viewport(&mut self, rect: Option<Rect<i32>>) -> PixResult<()>;

    /// Dimensions of the primary display as `(width, height)`.
    fn display_dimensions(&self) -> PixResult<(u32, u32)>;

    /// Returns whether the application is fullscreen or not.
    fn fullscreen(&self) -> PixResult<bool>;

    /// Set the application to fullscreen or not.
    fn set_fullscreen(&mut self, val: bool) -> PixResult<()>;

    /// Returns whether the window synchronizes frame rate to the screens refresh rate.
    fn vsync(&self) -> bool;

    /// Set the window to synchronize frame rate to the screens refresh rate.
    ///
    /// # Note
    ///
    /// Due to the current limitation with changing `VSync` at runtime, this method creates a new
    /// window using the properties of the current window and returns the new `WindowId`.
    ///
    /// If you are storing and interacting with this window using the `WindowId`, make sure to
    /// use the newly returned `WindowId`.
    fn set_vsync(&mut self, val: bool) -> PixResult<WindowId>;

    /// Set window as the target for drawing operations.
    fn set_window_target(&mut self, id: WindowId) -> PixResult<()>;

    /// Reset main window as the target for drawing operations.
    fn reset_window_target(&mut self);

    /// Show the current window target.
    fn show(&mut self) -> PixResult<()>;

    /// Hide the current window target.
    fn hide(&mut self) -> PixResult<()>;
}

/// Opens a new window by providing several window configuration functions.
///
/// In addition to the primary window created for you when calling [`Engine::run`], you can open
/// additional windows with various configurations and render to them using the
/// [`PixState::set_window_target`] method.
///
/// # Example
///
/// ```
/// # use pix_engine::prelude::*;
/// # struct App { windows: Vec<WindowId> };
/// # impl PixEngine for App {
/// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
/// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
///     if let Key::O = event.key {
///         let window_id = s.window()
///             .title("New Window")
///             .dimensions(800, 600)
///             .position(10, 10)
///             .resizable()
///             .borderless()
///             .build()?;
///         self.windows.push(window_id);
///         return Ok(true);
///     }
///     Ok(false)
/// }
/// # }
/// ```
#[must_use]
#[derive(Debug)]
pub struct WindowBuilder<'a> {
    renderer: &'a mut Renderer,
    settings: RendererSettings,
}

impl<'a> WindowBuilder<'a> {
    /// Creates a new `WindowBuilder` instance.
    #[inline]
    pub(crate) fn new(renderer: &'a mut Renderer) -> Self {
        let vsync = renderer.vsync();
        Self {
            renderer,
            settings: RendererSettings {
                vsync,
                ..RendererSettings::default()
            },
        }
    }

    /// Set window dimensions.
    #[inline]
    pub fn dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.settings.width = width;
        self.settings.height = height;
        self
    }

    /// Set a window title.
    #[inline]
    pub fn title<S: Into<String>>(&mut self, title: S) -> &mut Self {
        self.settings.title = title.into();
        self
    }

    /// Position the window at the given `(x, y)` coordinates of the display.
    #[inline]
    pub fn position(&mut self, x: i32, y: i32) -> &mut Self {
        self.settings.x = Position::Positioned(x);
        self.settings.y = Position::Positioned(y);
        self
    }

    /// Position the window in the center of the display.
    #[inline]
    pub fn position_centered(&mut self) -> &mut Self {
        self.settings.x = Position::Centered;
        self.settings.y = Position::Centered;
        self
    }

    /// Start window in fullscreen mode.
    #[inline]
    pub fn fullscreen(&mut self) -> &mut Self {
        self.settings.fullscreen = true;
        self
    }

    /// Allow window resizing.
    #[inline]
    pub fn resizable(&mut self) -> &mut Self {
        self.settings.resizable = true;
        self
    }

    #[inline]
    /// Removes the window decoration.
    pub fn borderless(&mut self) -> &mut Self {
        self.settings.borderless = true;
        self
    }

    /// Set a window icon.
    #[inline]
    pub fn icon<I>(&mut self, icon: I) -> &mut Self
    where
        I: Into<Icon>,
    {
        self.settings.icon = Some(icon.into());
        self
    }

    /// Create a new window from the `WindowBuilder` and return its id.
    ///
    /// # Errors
    ///
    /// If the renderer fails to create a new window, then an error is returned.
    ///
    /// Possible errors include the title containing a `nul` character, the position or dimensions
    /// being invalid values or overlowing and an internal renderer error such as running out of
    /// memory or a software driver issue.
    pub fn build(&mut self) -> PixResult<WindowId> {
        self.renderer.create_window(&mut self.settings)
    }
}

impl PixState {
    /// Get the current window target ID.
    #[inline]
    #[must_use]
    pub fn window_id(&self) -> WindowId {
        self.renderer.window_id()
    }

    /// Create a new [`WindowBuilder`].
    #[inline]
    pub fn window(&mut self) -> WindowBuilder<'_> {
        WindowBuilder::new(&mut self.renderer)
    }

    /// Close a window.
    ///
    /// # Errors
    ///
    /// If the window has already been closed or is invalid, then an error is returned.
    #[inline]
    pub fn close_window(&mut self, id: WindowId) -> PixResult<()> {
        if id == self.renderer.primary_window_id() || self.renderer.window_count() == 1 {
            self.quit();
            return Ok(());
        }
        self.renderer.close_window(id)
    }

    /// The dimensions of the current render target (window or texture) as `(width, height)`.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn dimensions(&self) -> PixResult<(u32, u32)> {
        self.renderer.dimensions()
    }

    /// The dimensions of the current window target as `(width, height)`.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn window_dimensions(&self) -> PixResult<(u32, u32)> {
        self.renderer.window_dimensions()
    }

    /// Set the dimensions of the current window target from `(width, height)`.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn set_window_dimensions(&mut self, dimensions: (u32, u32)) -> PixResult<()> {
        self.renderer.set_window_dimensions(dimensions)
    }

    /// The position of the current window target as `(x, y)`.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn window_position(&self) -> PixResult<(i32, i32)> {
        self.renderer.window_position()
    }

    /// Returns the rendering viewport of the current render target.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn viewport(&mut self) -> PixResult<Rect<i32>> {
        self.renderer.viewport()
    }

    /// Set the rendering viewport of the current render target.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn set_viewport<R>(&mut self, rect: R) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
    {
        self.renderer.set_viewport(Some(rect.into()))
    }

    /// Clears the rendering viewport of the current render target back to the entire target.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn clear_viewport(&mut self) -> PixResult<()> {
        self.renderer.set_viewport(None)
    }

    /// The width of the current render target.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn width(&self) -> PixResult<u32> {
        let (width, _) = self.dimensions()?;
        Ok(width)
    }

    /// The width of the current window.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn window_width(&self) -> PixResult<u32> {
        let (width, _) = self.window_dimensions()?;
        Ok(width)
    }

    /// Set the width of the current window.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn set_window_width(&mut self, width: u32) -> PixResult<()> {
        let (_, height) = self.window_dimensions()?;
        self.renderer.set_window_dimensions((width, height))
    }

    /// The height of the current render target.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn height(&self) -> PixResult<u32> {
        let (_, height) = self.dimensions()?;
        Ok(height)
    }

    /// The height of the current window.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn window_height(&self) -> PixResult<u32> {
        let (_, height) = self.window_dimensions()?;
        Ok(height)
    }

    /// Set the height of the current window.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn set_window_height(&mut self, height: u32) -> PixResult<()> {
        let (width, _) = self.window_dimensions()?;
        self.renderer.set_window_dimensions((width, height))
    }

    /// The x of the current window.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn window_x(&self) -> PixResult<i32> {
        let (x, _) = self.window_position()?;
        Ok(x)
    }

    /// The y of the current window.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn window_y(&self) -> PixResult<i32> {
        let (_, y) = self.window_position()?;
        Ok(y)
    }

    /// The center [Point] of the current render target.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn center(&self) -> PixResult<Point<i32>> {
        let (width, height) = self.dimensions()?;
        let (width, height) = clamp_dimensions(width, height);
        Ok(point![width / 2, height / 2])
    }

    /// The center [Point] of the current window.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn window_center(&self) -> PixResult<Point<i32>> {
        let (width, height) = self.window_dimensions()?;
        let (width, height) = clamp_dimensions(width, height);
        Ok(point![width / 2, height / 2])
    }

    /// The dimensions of the primary display as `(width, height)`.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn display_dimensions(&self) -> PixResult<(u32, u32)> {
        self.renderer.display_dimensions()
    }

    /// The width of the primary display.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn display_width(&self) -> PixResult<u32> {
        let (width, _) = self.display_dimensions()?;
        Ok(width)
    }

    /// The height of the primary display.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn display_height(&self) -> PixResult<u32> {
        let (_, height) = self.display_dimensions()?;
        Ok(height)
    }

    /// Show the current window target if it is hidden.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn show_window(&mut self) -> PixResult<()> {
        self.renderer.show()
    }

    /// Hide the current window target if it is shown.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn hide_window(&mut self) -> PixResult<()> {
        self.renderer.hide()
    }

    /// Set a `Window` as the primary target for drawing operations. Pushes current settings and UI
    /// cursor to the stack, so any changes made while a window target is set will be in effect
    /// until [`PixState::reset_window_target`] is called.
    ///
    /// # Errors
    ///
    /// If the window has been closed or is invalid, then an error is returned.
    pub fn set_window_target(&mut self, id: WindowId) -> PixResult<()> {
        if id != self.renderer.primary_window_id() {
            self.push();
            self.ui.push_cursor();
            self.set_cursor_pos(self.theme.spacing.frame_pad);
            self.renderer.set_window_target(id)
        } else {
            Ok(())
        }
    }

    /// Reset `Window` target back to the primary window for drawing operations. Pops previous
    /// settings and UI cursor off the stack, so that changes made while window target was set are
    /// reverted.
    pub fn reset_window_target(&mut self) {
        if self.window_id() != self.renderer.primary_window_id() {
            self.renderer.reset_window_target();
            self.ui.pop_cursor();
            self.pop();
        }
    }
}
