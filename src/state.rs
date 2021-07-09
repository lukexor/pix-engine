//! [PixState] functions for the [PixEngine] and [AppState].

use crate::{
    prelude::*,
    renderer::{Error as RendererError, Renderer},
    window::Window,
};
use environment::Environment;
use settings::Settings;
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    error, fmt, io, result,
    time::Instant,
};

pub mod environment;
pub mod settings;

/// The result type for [PixState] operations.
pub type Result<T> = result::Result<T, Error>;

/// Represents all state and methods for updating and interacting with the [PixEngine].
#[non_exhaustive]
#[derive(Debug)]
pub struct PixState {
    pub(super) title: String,
    pub(super) renderer: Renderer,
    pub(super) env: Environment,
    pub(super) settings: Settings,
    pub(super) mouse: MouseState,
    pub(super) pmouse: MouseState,
    pub(super) keys: KeyState,
    pub(super) setting_stack: Vec<Settings>,
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
            mouse: MouseState::default(),
            pmouse: MouseState::default(),
            keys: KeyState::default(),
            setting_stack: Vec::new(),
        }
    }

    /// Get the current window title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set the current window title.
    #[inline]
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

    /// Returns the current mouse position coordinates as `(x, y)`.
    pub fn mouse_pos(&self) -> Point<i32> {
        self.mouse.pos
    }

    /// Returns the previous mouse position coordinates last frame as `(x, y)`.
    pub fn pmouse_pos(&self) -> Point<i32> {
        self.pmouse.pos
    }

    /// Returns if any [Mouse] button is currently being held.
    pub fn mouse_pressed(&self) -> bool {
        self.mouse.is_pressed()
    }

    /// Returns if a specific [Mouse] button is currently being held.
    pub fn mouse_down(&self, btn: Mouse) -> bool {
        self.mouse.is_down(btn)
    }

    /// Returns the a list of the current mouse buttons being held.
    pub fn mouse_buttons(&self) -> &HashSet<Mouse> {
        &self.mouse.pressed
    }

    /// Returns the a list of the current keys being held.
    pub fn keys(&self) -> &HashSet<Key> {
        &self.keys.pressed
    }

    /// Returns if any [Key] is currently being held.
    pub fn key_pressed(&self) -> bool {
        self.keys.is_pressed()
    }

    /// Returns if a specific [Key] is currently being held.
    pub fn key_down(&self, key: Key) -> bool {
        self.keys.is_down(key)
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub(super) struct MouseState {
    pub(super) pos: Point<i32>,
    pub(super) pressed: HashSet<Mouse>,
    pub(super) last_clicked: HashMap<Mouse, Instant>,
}

impl MouseState {
    /// Whether any [Mouse] buttons are pressed.
    pub(super) fn is_pressed(&self) -> bool {
        !self.pressed.is_empty()
    }

    /// Returns if a specific [Mouse] button is currently being held.
    pub(super) fn is_down(&self, btn: Mouse) -> bool {
        self.pressed.contains(&btn)
    }

    /// Store a pressed [Mouse] button.
    pub(super) fn press(&mut self, btn: Mouse) {
        self.pressed.insert(btn);
    }

    /// Remove a pressed [Mouse] button.
    pub(super) fn release(&mut self, btn: &Mouse) {
        self.pressed.remove(btn);
    }

    /// Store last time a [Mouse] button was clicked.
    pub(super) fn click(&mut self, btn: Mouse, time: Instant) {
        self.last_clicked.insert(btn, time);
    }

    /// Get last time a [Mouse] button was clicked.
    pub(super) fn last_clicked(&mut self, btn: &Mouse) -> Option<&Instant> {
        self.last_clicked.get(&btn)
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub(super) struct KeyState {
    pub(super) pressed: HashSet<Key>,
}

impl KeyState {
    /// Returns if any [Key] is currently being held.
    pub(super) fn is_pressed(&self) -> bool {
        !self.pressed.is_empty()
    }

    /// Returns if a specific [Key] is currently being held.
    pub(super) fn is_down(&self, key: Key) -> bool {
        self.pressed.contains(&key)
    }

    /// Store a pressed [Key].
    pub(super) fn press(&mut self, key: Key) {
        self.pressed.insert(key);
    }

    /// Remove a pressed [Key].
    pub(super) fn release(&mut self, key: &Key) {
        self.pressed.remove(key);
    }
}

/// The error type for [PixState] operations.
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            Other(err) => write!(f, "image error: {}", err),
            err => err.fmt(f),
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
