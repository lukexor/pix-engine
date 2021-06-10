//! Environment related information of the engine.

use super::PixState;
use crate::{common::Result, renderer::Rendering};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Environment values for [PixState]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct Environment {
    pub(crate) focused: bool,
    pub(crate) focused_window: Option<WindowId>,
    pub(crate) delta_time: Duration,
    pub(crate) frame_rate: u64,
    pub(crate) frame_count: u64,
    pub(crate) quit: bool,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            focused: false,
            focused_window: None,
            delta_time: Duration::from_secs(0),
            frame_rate: 0,
            frame_count: 0,
            quit: false,
        }
    }
}

impl PixState {
    /// Whether the application has focus or not.
    pub fn focused(&self) -> bool {
        self.env.focused
    }

    /// Whether the given window has focus or not.
    pub fn focused_window(&self, window_id: WindowId) -> bool {
        self.env
            .focused_window
            .map(|id| id == window_id)
            .unwrap_or(false)
    }

    /// Get the primary `Window` id.
    pub fn window_id(&self) -> WindowId {
        self.renderer.window_id()
    }

    /// Create a new [WindowBuilder].
    pub fn create_window(&self, width: u32, height: u32) -> WindowBuilder {
        WindowBuilder::new(width, height)
    }

    /// Close an open window.
    pub fn close_window(&self, _window_id: WindowId) -> Result<()> {
        todo!("close_window");
    }

    /// The time elapsed since last frame.
    pub fn delta_time(&self) -> f64 {
        self.env.delta_time.as_secs_f64()
    }

    /// The total number of frames rendered since application start.
    pub fn frame_count(&self) -> u64 {
        self.env.frame_count
    }

    /// The average frames per second rendered.
    pub fn frame_rate(&self) -> u64 {
        self.env.frame_rate
    }

    /// The dimensions of the current canvas as a tuple of (width, height).
    pub fn dimensions(&self) -> (u32, u32) {
        (self.renderer.width(), self.renderer.height())
    }

    /// The width of the current canvas.
    pub fn width(&self) -> u32 {
        self.renderer.width()
    }

    /// The height of the current canvas.
    pub fn height(&self) -> u32 {
        self.renderer.height()
    }

    /// Trigger exiting of the game loop.
    pub fn quit(&mut self) {
        self.env.quit = true;
    }

    /// Abort exiting of the game loop.
    pub fn abort_quit(&mut self) {
        self.env.quit = false;
    }

    /// The display width of the primary monitor.
    pub fn display_width(&self) -> u32 {
        todo!("display_width")
    }

    /// The display height of the primary monitor.
    pub fn display_height(&self) -> u32 {
        todo!("display_height")
    }

    /// The display pixel density of the primary monitor.
    pub fn display_density(&self) -> u32 {
        todo!("display_density")
    }
}

/// Window Identifier
pub type WindowId = u32;

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
            width,
            height,
            ..Default::default()
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
        todo!("WindowBuilder::build");
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            title: String::new(),
            width: 400,
            height: 400,
        }
    }
}
