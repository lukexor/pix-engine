//! Environment information for the [PixEngine].
//!
//! [PixEngine]: crate::prelude::PixEngine
use crate::{
    prelude::{PixResult, PixState, WindowBuilder, WindowId},
    renderer::Rendering,
    window::Window,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Environment values for [PixState]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct Environment {
    pub(crate) focused: bool,
    pub(crate) focused_window: Option<WindowId>,
    pub(crate) delta_time: f64,
    pub(crate) frame_rate: f64,
    pub(crate) target_frame_rate: Option<f64>,
    pub(crate) frame_count: usize,
    pub(crate) quit: bool,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            focused: false,
            focused_window: None,
            delta_time: 0.0,
            frame_rate: 0.0,
            target_frame_rate: None,
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
    pub fn close_window(&self, _window_id: WindowId) -> PixResult<()> {
        todo!("close_window");
    }

    /// The time elapsed since last frame in seconds.
    pub fn delta_time(&self) -> f64 {
        self.env.delta_time
    }

    /// The total number of frames rendered since application start.
    pub fn frame_count(&self) -> usize {
        self.env.frame_count
    }

    /// The average frames per second rendered.
    pub fn frame_rate(&self) -> f64 {
        self.env.frame_rate
    }

    /// Set a target frame rate to render at, controls how often
    /// [on_update](crate::prelude::AppState::on_update) is called.
    pub fn set_frame_rate(&mut self, rate: f64) {
        self.env.target_frame_rate = Some(rate);
    }

    /// The dimensions of the current canvas as `(width, height)`.
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
