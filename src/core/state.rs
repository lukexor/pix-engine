//! [PixState] functions for the [PixEngine] and [AppState].

use crate::{
    gui::state::UiState,
    prelude::*,
    renderer::{Error as RendererError, Renderer, WindowRenderer},
};
use environment::Environment;
use settings::Settings;
use std::{borrow::Cow, collections::HashSet, error, fmt, io};

pub mod environment;
pub mod settings;

/// Represents all state and methods for updating and interacting with the [PixEngine].
#[non_exhaustive]
#[derive(Debug)]
pub struct PixState {
    pub(crate) renderer: Renderer,
    pub(crate) env: Environment,
    pub(crate) ui: UiState,
    pub(crate) settings: Settings,
    pub(crate) setting_stack: Vec<Settings>,
    pub(crate) theme: Theme,
}

impl PixState {
    /// Constructs `PixState` with a given `Renderer`.
    #[inline]
    pub(crate) fn new(renderer: Renderer, theme: Theme) -> Self {
        Self {
            renderer,
            env: Environment::default(),
            ui: UiState::default(),
            settings: Settings::default(),
            setting_stack: Vec::new(),
            theme,
        }
    }

    /// Handle state changes this frame prior to calling [AppState::on_update].
    #[inline]
    pub(crate) fn pre_update(&mut self) {
        // Reset mouse cursor icon to the current setting
        // Ignore any errors, as setting cursor in the first place should have succeeded.
        let _ = self.renderer.cursor(self.settings.cursor.as_ref());
        self.ui.pre_update();
        self.ui.set_cursor(self.theme.style.frame_pad);
    }

    /// Handle state changes this frame after calling [AppState::on_update].
    #[inline]
    pub(crate) fn post_update(&mut self) {
        self.ui.post_update();
    }

    #[inline]
    pub(crate) fn get_rect<R>(&self, rect: R) -> Rect<i32>
    where
        R: Into<Rect<i32>>,
    {
        let mut rect = rect.into();
        if let RectMode::Center = self.settings.rect_mode {
            rect.center_on(rect.top_left());
        }
        rect
    }

    #[inline]
    pub(crate) fn get_ellipse<E>(&self, ellipse: E) -> Ellipse<i32>
    where
        E: Into<Ellipse<i32>>,
    {
        let mut ellipse = ellipse.into();
        if let RectMode::Center = self.settings.ellipse_mode {
            ellipse.center_on(ellipse.top_left());
        }
        ellipse
    }
}

impl PixState {
    /// Get the current window title.
    #[inline]
    pub fn title(&self) -> &str {
        self.renderer.title()
    }

    /// Set the current window title.
    #[inline]
    pub fn set_title<S: AsRef<str>>(&mut self, title: S) -> PixResult<()> {
        Ok(self.renderer.set_title(title.as_ref())?)
    }

    /// Returns the current mouse position coordinates as `(x, y)`.
    #[inline]
    pub fn mouse_pos(&self) -> PointI2 {
        self.ui.mouse.pos
    }

    /// Returns the previous mouse position coordinates last frame as `(x, y)`.
    #[inline]
    pub fn pmouse_pos(&self) -> PointI2 {
        self.ui.pmouse.pos
    }

    /// Returns if any [Mouse] button is currently being held.
    #[inline]
    pub fn mouse_pressed(&self) -> bool {
        self.ui.mouse.is_pressed()
    }

    /// Returns if a specific [Mouse] button is currently being held.
    #[inline]
    pub fn mouse_down(&self, btn: Mouse) -> bool {
        self.ui.mouse.is_down(btn)
    }

    /// Returns the a list of the current mouse buttons being held.
    #[inline]
    pub fn mouse_buttons(&self) -> &HashSet<Mouse> {
        &self.ui.mouse.pressed
    }

    /// Returns the a list of the current keys being held.
    #[inline]
    pub fn keys(&self) -> &HashSet<Key> {
        &self.ui.keys.pressed
    }

    /// Returns the a list of the current key modifiers being held.
    #[inline]
    pub fn keymods(&self) -> &HashSet<KeyMod> {
        &self.ui.keys.mods_pressed
    }

    /// Returns if any [Key] is currently being held.
    #[inline]
    pub fn key_pressed(&self) -> bool {
        self.ui.keys.is_pressed()
    }

    /// Returns if a specific [Key] is currently being held.
    #[inline]
    pub fn key_down(&self, key: Key) -> bool {
        self.ui.keys.is_down(key)
    }

    /// Returns if a specific [KeyMod] is currently being held.
    #[inline]
    pub fn keymod_down(&self, keymod: KeyMod) -> bool {
        self.ui.keys.mod_down(keymod)
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
