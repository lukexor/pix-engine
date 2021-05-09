//! State management for `PixEngine`.

use crate::{
    common::{PixError, PixResult},
    event::{Keycode, MouseButton},
    renderer::{Renderer, RendererError, Rendering},
    shape::Point,
};
use environment::Environment;
use settings::Settings;
use std::{borrow::Cow, collections::HashSet, error, fmt, io, result};

pub mod environment;
pub mod settings;

/// `PixState` Result
type PixStateResult<T> = result::Result<T, PixStateError>;

/// Types of errors the `AppState` trait can return in a `PixStateResult`.
#[non_exhaustive]
#[derive(Debug)]
pub enum PixStateError {
    /// IO specific errors.
    IoError(io::Error),
    /// Renderer specific errors.
    RendererError(RendererError),
    /// Unknown errors.
    Other(Cow<'static, str>),
}

/// Defines state changing operations that are called while the `PixEngine` is running.
pub trait AppState {
    /// Called once upon engine start when `PixEngine::run()` is called.
    fn on_start(&mut self, _s: &mut PixState) -> PixResult<()> {
        Ok(())
    }

    /// Called every frame based on the target_frame_rate. By default this is as often as possible.
    fn on_update(&mut self, _s: &mut PixState) -> PixResult<()>;

    /// Called once when the engine detects a close/exit event.
    fn on_stop(&mut self, _s: &mut PixState) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a key is pressed.
    fn on_key_pressed(&mut self, _s: &mut PixState, _key: Keycode) {}

    /// Called each time a key is released.
    fn on_key_released(&mut self, _s: &mut PixState, _key: Keycode) {}

    /// Called each time a key is typed. Ignores special keys like Backspace.
    fn on_key_typed(&mut self, _s: &mut PixState, _text: &str) {}

    /// Called each time a mouse button is pressed.
    fn on_mouse_dragged(&mut self, _s: &mut PixState) {}

    /// Called each time a mouse button is pressed.
    fn on_mouse_pressed(&mut self, _s: &mut PixState, _btn: MouseButton) {}

    /// Called each time a mouse button is released.
    fn on_mouse_released(&mut self, _s: &mut PixState, _btn: MouseButton) {}

    /// Called each time the mouse wheel is scrolled.
    fn on_mouse_wheel(&mut self, _s: &mut PixState, _x_delta: i32, _y_delta: i32) {}

    /// Called each time the window is resized.
    fn on_window_resized(&mut self, _s: &mut PixState) {}
}

/// Represents all engine-specific state and methods.
#[derive(Debug)]
pub struct PixState {
    pub(crate) title: String,
    pub(crate) renderer: Renderer,
    pub(crate) env: Environment,
    pub(crate) settings: Settings,
    pub(crate) mouse_pos: Point,
    pub(crate) pmouse_pos: Point,
    pub(crate) mouse_down: bool,
    pub(crate) mouse_buttons: HashSet<MouseButton>,
    pub(crate) key_down: bool,
    pub(crate) keys: HashSet<Keycode>,
}

impl PixState {
    /// Creates a new `PixState` instance with a given `Renderer`.
    pub fn init(title: &str, renderer: Renderer) -> Self {
        Self {
            title: title.to_owned(),
            renderer,
            env: Environment::default(),
            settings: Settings::default(),
            mouse_pos: (0, 0),
            pmouse_pos: (0, 0),
            mouse_down: false,
            mouse_buttons: HashSet::new(),
            key_down: false,
            keys: HashSet::new(),
        }
    }

    /// Clears the render target to the current background color set by `PixState::background()`.
    pub fn clear(&mut self) {
        let color = self.settings.background;
        self.renderer.set_draw_color(self.settings.background);
        self.renderer.clear();
        self.renderer.set_draw_color(color);
    }

    /// Get the current window title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set the current window title.
    pub fn set_title(&mut self, title: &str) -> PixStateResult<()> {
        self.title = title.to_owned();
        if self.settings.show_frame_rate {
            self.renderer
                .set_title(&format!("{} - FPS: {}", title, self.env.frame_rate))?;
        } else {
            self.renderer.set_title(title)?;
        }
        Ok(())
    }

    /// Returns the current mouse position coordinates as (x, y).
    pub fn mouse_pos(&self) -> (i32, i32) {
        self.mouse_pos
    }

    /// Returns the previous mouse position coordinates last frame as (x, y).
    pub fn pmouse_pos(&self) -> (i32, i32) {
        self.pmouse_pos
    }

    /// Returns if a mouse button is currently being held.
    pub fn mouse_pressed(&self, btn: MouseButton) -> bool {
        self.mouse_buttons.contains(&btn)
    }

    /// Returns the a list of the current mouse buttons being held.
    pub fn mouse_buttons(&self) -> &HashSet<MouseButton> {
        &self.mouse_buttons
    }

    /// Returns the a list of the current keys being held.
    pub fn keys(&self) -> &HashSet<Keycode> {
        &self.keys
    }

    /// Returns if a key is currently being held.
    pub fn key_pressed(&self, key: Keycode) -> bool {
        self.keys.contains(&key)
    }
}

impl fmt::Display for PixStateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PixStateError::*;
        match self {
            IoError(err) => err.fmt(f),
            RendererError(err) => err.fmt(f),
            Other(err) => write!(f, "Unknown error: {}", err),
        }
    }
}

impl error::Error for PixStateError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<PixStateError> for PixError {
    fn from(err: PixStateError) -> Self {
        Self::StateError(err)
    }
}
