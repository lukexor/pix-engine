//! Environment related information of the engine.

use super::PixState;
use crate::renderer::Rendering;
use std::time::Duration;

#[derive(Debug, Clone)]
pub(crate) struct Environment {
    pub(crate) focused: bool,
    pub(crate) delta_time: Duration,
    pub(crate) frame_rate: u64,
    pub(crate) target_frame_rate: u64,
    pub(crate) frame_count: u64,
    pub(crate) quit: bool,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            focused: false,
            delta_time: Duration::from_secs(0),
            frame_rate: 0,
            target_frame_rate: 60, // TODO: target_frame_rate
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
