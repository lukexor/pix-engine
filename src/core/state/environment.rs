//! Environment information for the [PixEngine].
//!
//! [PixEngine]: crate::prelude::PixEngine
use crate::{core::window::Window, prelude::*};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Environment values for [PixState]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct Environment {
    pub(crate) focused: bool,
    pub(crate) focused_window: Option<WindowId>,
    pub(crate) delta_time: f64,
    pub(crate) frame_rate: usize,
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
            frame_rate: 0,
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
    pub fn create_window(&mut self, width: Primitive, height: Primitive) -> WindowBuilder {
        WindowBuilder::new(width, height)
    }

    /// Close an open window.
    pub fn close_window(&mut self, window_id: WindowId) -> PixResult<()> {
        if window_id == self.renderer.window_id() {
            self.env.quit = true;
        } else {
            todo!("secondary windows are not yet implemented");
        }
        Ok(())
    }

    /// The time elapsed since last frame in seconds.
    pub fn delta_time(&self) -> Scalar {
        self.env.delta_time
    }

    /// The total number of frames rendered since application start.
    pub fn frame_count(&self) -> usize {
        self.env.frame_count
    }

    /// The average frames per second rendered.
    pub fn frame_rate(&self) -> usize {
        self.env.frame_rate
    }

    /// Set a target frame rate to render at, controls how often
    /// [on_update](crate::prelude::AppState::on_update) is called.
    pub fn set_frame_rate(&mut self, rate: usize) {
        self.env.target_frame_rate = Some(rate as Scalar);
    }

    /// The dimensions of the primary window as `(width, height)`.
    pub fn dimensions(&self) -> (Primitive, Primitive) {
        // SAFETY: Primary window_id should always exist
        let window_id = self.window_id();
        self.renderer.dimensions(window_id).unwrap()
    }

    /// Set the dimensions of the primary window from `(width, height)`.
    pub fn set_dimensions(&mut self, dimensions: (Primitive, Primitive)) {
        // SAFETY: Primary window_id should always exist
        let window_id = self.window_id();
        self.renderer.set_dimensions(window_id, dimensions).unwrap()
    }

    /// The width of the primary window.
    pub fn width(&self) -> Primitive {
        // SAFETY: Primary window_id should always exist
        let window_id = self.window_id();
        let (width, _) = self.renderer.dimensions(window_id).unwrap();
        width
    }

    /// Set the width of the primary window.
    pub fn set_width(&mut self, width: Primitive) {
        let window_id = self.window_id();
        // SAFETY: Primary window_id should always exist
        let (_, height) = self.renderer.dimensions(window_id).unwrap();
        self.renderer
            .set_dimensions(window_id, (width, height))
            .unwrap();
    }

    /// The height of the primary window.
    pub fn height(&self) -> Primitive {
        // SAFETY: Primary window_id should always exist
        let (_, height) = self.renderer.dimensions(self.window_id()).unwrap();
        height
    }

    /// Set the height of the primary window.
    pub fn set_height(&mut self, height: Primitive) {
        let window_id = self.window_id();
        // SAFETY: Primary window_id should always exist
        let (width, _) = self.renderer.dimensions(window_id).unwrap();
        self.renderer
            .set_dimensions(window_id, (width, height))
            .unwrap();
    }

    /// Trigger exiting of the game loop.
    pub fn quit(&mut self) {
        self.env.quit = true;
    }

    /// Abort exiting of the game loop.
    pub fn abort_quit(&mut self) {
        self.env.quit = false;
    }
}
