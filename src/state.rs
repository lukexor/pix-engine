//! [PixState] functions for the [PixEngine] and [AppState].
//!
//! `PixState` is the global engine state and API for any application using `pix-engine`. A mutable
//! reference is passed to most [AppState] methods and allows you to modify settings, query engine
//! and input state, as well as drawing to the current render target.
//!
//! The most common use of `PixState` is in the [AppState::on_update] method.
//!
//! See the [Getting Started](crate#getting-started) section and the [PixState] page for the list
//! of available methods.
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App { checkbox: bool, text_field: String };
//! # impl AppState for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     s.fill(s.accent_color());
//!     s.rect([100, 0, 100, 100])?;
//!     if s.button("Click me")? {
//!         s.text("I was clicked!");
//!     }
//!     Ok(())
//! }
//! # }
//! ```

use crate::{
    gui::state::UiState,
    prelude::*,
    renderer::{Renderer, RendererSettings, Rendering, WindowRenderer},
    texture::TextureRenderer,
};
use environment::Environment;
use settings::Settings;
use std::collections::HashSet;

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
    pub(crate) setting_stack: Vec<(Settings, Theme)>,
    pub(crate) theme: Theme,
}

impl PixState {
    /// Constructs `PixState` with a given `Renderer`.
    #[inline]
    pub(crate) fn new(settings: RendererSettings, theme: Theme) -> PixResult<Self> {
        let show_frame_rate = settings.show_frame_rate;
        let mut renderer = Renderer::new(settings)?;
        renderer.font_size(theme.font_sizes.body)?;
        renderer.font_family(&theme.fonts.body)?;
        Ok(Self {
            renderer,
            env: Environment::default(),
            ui: UiState::default(),
            settings: Settings {
                background: theme.colors.background,
                fill: Some(theme.colors.text),
                show_frame_rate,
                ..Default::default()
            },
            setting_stack: Vec::new(),
            theme,
        })
    }

    /// Handle state changes this frame prior to calling [AppState::on_update].
    #[inline]
    pub(crate) fn pre_update(&mut self) {
        // Reset mouse cursor icon to the current setting
        // Ignore any errors, as setting cursor in the first place should have succeeded.
        let _ = self.renderer.cursor(self.settings.cursor.as_ref());
        self.ui.pre_update(&self.theme);
    }

    /// Handle state updates for this frame.
    #[inline]
    pub(crate) fn on_update(&mut self) -> PixResult<()> {
        for texture in &mut self.ui.textures.values() {
            if texture.visible {
                self.renderer.texture(
                    texture.id,
                    texture.src,
                    texture.dst,
                    0.0,
                    None,
                    None,
                    None,
                )?;
            }
        }
        Ok(())
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
        self.renderer.set_title(title.as_ref())
    }

    /// Returns the current mouse position coordinates as `(x, y)`.
    #[inline]
    pub fn mouse_pos(&self) -> PointI2 {
        self.ui.mouse_pos()
    }

    /// Returns the previous mouse position coordinates last frame as `(x, y)`.
    #[inline]
    pub fn pmouse_pos(&self) -> PointI2 {
        self.ui.pmouse_pos()
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
