//! Environment related information for the [`PixEngine`].
//!
//! [`PixEngine`]: crate::prelude::PixEngine
use crate::{
    prelude::{PixResult, PixState, WindowBuilder, WindowId},
    renderer::Rendering,
    window::Window,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Environment values for [`PixState`]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct Environment {
    pub(crate) focused: bool,
    pub(crate) focused_window: Option<WindowId>,
    pub(crate) delta_time: Duration,
    pub(crate) frame_rate: Duration,
    pub(crate) target_frame_rate: u32,
    pub(crate) frame_count: usize,
    pub(crate) quit: bool,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            focused: false,
            focused_window: None,
            delta_time: Duration::default(),
            frame_rate: Duration::default(),
            target_frame_rate: 60,
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

    /// Create a new [`WindowBuilder`].
    pub fn create_window(&self, width: u32, height: u32) -> WindowBuilder {
        WindowBuilder::new(width, height)
    }

    /// Close an open window.
    pub fn close_window(&self, _window_id: WindowId) -> PixResult<()> {
        todo!("close_window");
    }

    /// The time elapsed since last frame.
    pub fn delta_time(&self) -> f64 {
        self.env.delta_time.as_secs_f64()
    }

    /// The total number of frames rendered since application start.
    pub fn frame_count(&self) -> usize {
        self.env.frame_count
    }

    /// The average frames per second rendered.
    pub fn frame_rate(&self) -> u32 {
        self.env.frame_rate.as_secs() as u32
    }

    /// Set a target frame rate to render at.
    pub fn set_frame_rate(&mut self, rate: u32) {
        self.env.target_frame_rate = rate;
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
