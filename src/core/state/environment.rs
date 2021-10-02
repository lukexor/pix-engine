//! Environment information for the [PixEngine].
//!
//! [PixEngine]: crate::prelude::PixEngine
use crate::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Environment values for [PixState]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct Environment {
    pub(crate) focused: bool,
    pub(crate) focused_window: Option<WindowId>,
    pub(crate) delta_time: Scalar,
    pub(crate) frame_rate: usize,
    pub(crate) target_frame_rate: Option<Scalar>,
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

    /// Get the target frame rate to render at.
    pub fn target_frame_rate(&mut self) -> Option<usize> {
        self.env.target_frame_rate.map(|rate| rate as usize)
    }

    /// Set a target frame rate to render at, controls how often
    /// [on_update](crate::prelude::AppState::on_update) is called.
    pub fn set_frame_rate(&mut self, rate: usize) {
        self.env.target_frame_rate = Some(rate as Scalar);
    }

    /// Remove target frame rate and call [on_update](crate::prelude::AppState::on_update) as often
    /// as possible.
    pub fn clear_frame_rate(&mut self) {
        self.env.target_frame_rate = None;
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
