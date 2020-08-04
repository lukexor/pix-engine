//! Environment related information of the engine.

use super::State;
use crate::{
    event::{Keycode, MouseButton},
    renderer::Rendering,
};
use std::{collections::HashSet, time::Duration};

#[derive(Debug, Clone)]
pub(crate) struct Environment {
    pub(crate) focused: bool,
    pub(crate) delta_time: Duration,
    pub(crate) frame_rate: u64,
    pub(crate) frame_count: u64,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            focused: false,
            delta_time: Duration::from_secs(0),
            frame_rate: 0,
            frame_count: 0,
        }
    }
}

impl State {
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

    /// Returns whether the application is fullscreen or not.
    pub fn fullscreen(&self) -> bool {
        self.renderer.fullscreen()
    }

    /// Set the application to fullscreen or not.
    pub fn set_fullscreen(&mut self, val: bool) {
        self.renderer.set_fullscreen(val)
    }

    /// Returns the a list of the current keys being held.
    pub fn keys(&self) -> &HashSet<Keycode> {
        &self.keys
    }

    /// Returns if a key is currently being held.
    pub fn key_pressed(&self, key: Keycode) -> bool {
        self.keys.contains(&key)
    }

    /// Returns the a list of the current mouse buttons being held.
    pub fn mouse_buttons(&self) -> &HashSet<MouseButton> {
        &self.mouse_buttons
    }

    /// Returns if a mouse button is currently being held.
    pub fn mouse_pressed(&self, btn: MouseButton) -> bool {
        self.mouse_buttons.contains(&btn)
    }

    /// Returns the current mouse position coordinates as (x, y).
    pub fn mouse_pos(&self) -> (i32, i32) {
        self.mouse_pos
    }

    /// Returns the previous mouse position coordinates last frame as (x, y).
    pub fn pmouse_pos(&self) -> (i32, i32) {
        self.pmouse_pos
    }
}
