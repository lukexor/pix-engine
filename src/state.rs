//! [`PixState`] functions for the [`PixEngine`] and [`AppState`].

use crate::{
    prelude::*,
    renderer::{Error as RendererError, Renderer, Rendering},
    window::Window,
};
use environment::Environment;
use settings::Settings;
use std::{borrow::Cow, collections::HashSet, error, fmt, io, result};

pub mod environment;
pub mod settings;

/// The result type for [`PixState`] operations.
pub type Result<T> = result::Result<T, Error>;

/// The error type for [`PixState`] operations.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// IO specific errors.
    IoError(io::Error),
    /// Renderer specific errors.
    RendererError(RendererError),
    /// Unknown errors.
    Other(Cow<'static, str>),
}

/// Defines state changing operations that are called while the PixEngine is running.
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
    fn on_key_pressed(&mut self, _s: &mut PixState, _event: KeyEvent) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a key is released.
    fn on_key_released(&mut self, _s: &mut PixState, _event: KeyEvent) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a key is typed. Ignores special keys like Backspace.
    fn on_key_typed(&mut self, _s: &mut PixState, _text: &str) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a mouse button is pressed.
    fn on_mouse_dragged(&mut self, _s: &mut PixState) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a mouse button is pressed.
    fn on_mouse_pressed(&mut self, _s: &mut PixState, _btn: Mouse) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a mouse button is released.
    fn on_mouse_released(&mut self, _s: &mut PixState, _btn: Mouse) -> PixResult<()> {
        Ok(())
    }

    /// Called each time the mouse wheel is scrolled.
    fn on_mouse_wheel(&mut self, _s: &mut PixState, _x_delta: i32, _y_delta: i32) -> PixResult<()> {
        Ok(())
    }

    /// Called each time the window is resized.
    fn on_window_resized(&mut self, _s: &mut PixState) -> PixResult<()> {
        Ok(())
    }
}

/// Represents all engine-specific state and methods.
#[non_exhaustive]
#[derive(Debug)]
pub struct PixState {
    pub(crate) title: String,
    pub(crate) renderer: Renderer,
    pub(crate) env: Environment,
    pub(crate) settings: Settings,
    pub(crate) mouse_pos: Point<i32>,
    pub(crate) pmouse_pos: Point<i32>,
    pub(crate) mouse_down: bool,
    pub(crate) mouse_buttons: HashSet<Mouse>,
    pub(crate) key_down: bool,
    pub(crate) keys: HashSet<Key>,
    pub(crate) setting_stack: Vec<Settings>,
}

impl PixState {
    /// Constructs `PixState` with a given `Renderer`.
    pub(super) fn new<S>(title: S, renderer: Renderer) -> Self
    where
        S: Into<String>,
    {
        Self {
            title: title.into(),
            renderer,
            env: Environment::default(),
            settings: Settings::default(),
            mouse_pos: Point::default(),
            pmouse_pos: Point::default(),
            mouse_down: false,
            mouse_buttons: HashSet::new(),
            key_down: false,
            keys: HashSet::new(),
            setting_stack: Vec::new(),
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
    pub fn set_title<S>(&mut self, title: S) -> PixResult<()>
    where
        S: Into<String>,
    {
        self.title = title.into();
        if self.settings.show_frame_rate {
            self.renderer
                .set_title(&format!("{} - FPS: {}", self.title, self.env.frame_rate))?;
        } else {
            self.renderer.set_title(&self.title)?;
        }
        Ok(())
    }

    /// Returns the current mouse position coordinates as (x, y).
    pub fn mouse_pos(&self) -> Point<i32> {
        self.mouse_pos
    }

    /// Returns the previous mouse position coordinates last frame as (x, y).
    pub fn pmouse_pos(&self) -> Point<i32> {
        self.pmouse_pos
    }

    /// Returns if a mouse button is currently being held.
    pub fn mouse_pressed(&self, btn: Mouse) -> bool {
        self.mouse_buttons.contains(&btn)
    }

    /// Returns the a list of the current mouse buttons being held.
    pub fn mouse_buttons(&self) -> &HashSet<Mouse> {
        &self.mouse_buttons
    }

    /// Returns the a list of the current keys being held.
    pub fn keys(&self) -> &HashSet<Key> {
        &self.keys
    }

    /// Returns if a key is currently being held.
    pub fn key_pressed(&self, key: Key) -> bool {
        self.keys.contains(&key)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            IoError(err) => err.fmt(f),
            RendererError(err) => err.fmt(f),
            Other(err) => write!(f, "image error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            IoError(err) => err.source(),
            RendererError(err) => err.source(),
            _ => None,
        }
    }
}

impl From<Error> for PixError {
    fn from(err: Error) -> Self {
        Self::StateError(err)
    }
}
