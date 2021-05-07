//! State management for the engine.

use crate::{
    common::{PixError, PixResult},
    event::{Keycode, MouseButton},
    renderer::{Renderer, RendererError, Rendering},
};
use environment::Environment;
use settings::Settings;
use std::{borrow::Cow, collections::HashSet, error, fmt, io, result};

pub mod environment;
pub mod settings;

/// `State` Result
type StateResult<T> = result::Result<T, StateError>;

/// Types of errors the `Stateful` trait can return in a `StateResult`.
#[derive(Debug)]
pub enum StateError {
    /// IO specific errors.
    IoError(io::Error),
    /// Renderer specific errors.
    RendererError(RendererError),
    /// Unknown errors.
    Other(Cow<'static, str>),
}

/// Defines state changing operations that are called while the `PixEngine` is running.
pub trait Stateful {
    /// Called once upon engine start when `PixEngine::run()` is called.
    ///
    /// Return `Ok(true)` to continue running.
    /// Return `Err` or `Ok(false)` to shutdown the engine and close the application.
    fn on_start(&mut self, _s: &mut State) -> PixResult<bool> {
        Ok(true)
    }

    /// Called every frame based on the target_frame_rate. By default this is as often as possible.
    ///
    /// Return `Ok(true)` to continue running.
    /// Return `Err` or `Ok(false)` to shutdown the engine and close the application.
    fn on_update(&mut self, _s: &mut State) -> PixResult<bool>;

    /// Called once when the engine detects a close/exit event.
    ///
    /// Return `Ok(true)` to continue shutting down the engine and closing the application.
    /// Return `Err` or `Ok(false)` to abort exiting.
    fn on_stop(&mut self, _s: &mut State) -> PixResult<bool> {
        Ok(true)
    }

    /// Called each time a key is pressed.
    fn on_key_pressed(&mut self, _s: &mut State, _key: Keycode) {}

    /// Called each time a key is released.
    fn on_key_released(&mut self, _s: &mut State, _key: Keycode) {}

    /// Called each time a mouse button is pressed.
    fn on_mouse_dragged(&mut self, _s: &mut State) {}

    /// Called each time a mouse button is pressed.
    fn on_mouse_pressed(&mut self, _s: &mut State, _btn: MouseButton) {}

    /// Called each time a mouse button is released.
    fn on_mouse_released(&mut self, _s: &mut State, _btn: MouseButton) {}
}

/// Represents all engine-specific state and methods.
#[derive(Debug)]
pub struct State {
    pub(crate) renderer: Renderer,
    pub(crate) env: Environment,
    pub(crate) settings: Settings,
    pub(crate) key_down: bool,
    pub(crate) mouse_down: bool,
    pub(crate) mouse_pos: (i32, i32),
    pub(crate) pmouse_pos: (i32, i32),
    pub(crate) keys: HashSet<Keycode>,
    pub(crate) mouse_buttons: HashSet<MouseButton>,
}

impl State {
    /// Creates a new `State` instance with a given `Renderer`.
    pub fn init(renderer: Renderer) -> Self {
        Self {
            renderer,
            env: Environment::default(),
            settings: Settings::default(),
            key_down: false,
            mouse_down: false,
            mouse_pos: (0, 0),
            pmouse_pos: (0, 0),
            keys: HashSet::new(),
            mouse_buttons: HashSet::new(),
        }
    }

    /// Clears the render target to the current background color set by `State::background()`.
    pub fn clear(&mut self) {
        self.renderer.set_draw_color(self.settings.background);
        self.renderer.clear();
    }

    /// Get the current window title.
    pub fn title(&self) -> &str {
        self.renderer.title()
    }

    /// Set the current window title.
    pub fn set_title(&mut self, title: &str) -> StateResult<()> {
        self.renderer.set_title(title)?;
        Ok(())
    }
}

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StateError::*;
        match self {
            IoError(err) => err.fmt(f),
            RendererError(err) => err.fmt(f),
            Other(err) => write!(f, "Unknown error: {}", err),
        }
    }
}

impl error::Error for StateError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<StateError> for PixError {
    fn from(err: StateError) -> Self {
        Self::StateError(err)
    }
}
