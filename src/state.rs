//! [`PixState`] methods for the [`PixEngine`] and [`AppState`].
//!
//! `PixState` is the global engine state and API for any application using `pix-engine`. A mutable
//! reference is passed to most [`AppState`] methods and allows you to modify settings, query engine
//! and input state, as well as drawing to the current render target.
//!
//! The most common use of `PixState` is in the [`AppState::on_update`] method.
//!
//! See the [Getting Started](crate#getting-started) section and the [`PixState`] page for the list
//! of available methods.
//!
//! Provided [`PixState`] methods:
//!
//! - [`PixState::title`]: Current window title.
//! - [`PixState::set_title`]: Set new window title.
//! - [`PixState::mouse_pos`]: [Mouse] position this frame.
//! - [`PixState::pmouse_pos`]: [Mouse] position previous frame.
//! - [`PixState::mouse_pressed`]: Whether any [Mouse] button was pressed this frame.
//! - [`PixState::mouse_clicked`]: Whether a given [Mouse] button was clicked this frame.
//! - [`PixState::mouse_down`]: Whether a given [Mouse] button was pressed this frame.
//! - [`PixState::mouse_buttons`]: A [`HashSet`] of [Mouse] buttons pressed this frame.
//! - [`PixState::key_pressed`]: Whether a given [Key] was pressed this frame.
//! - [`PixState::key_down`]: Whether a given [Key] was pressed this frame.
//! - [`PixState::keys`]: Whether any [Key] was pressed this frame.
//! - [`PixState::keymod_down`]: Whether a given [key modifier][`KeyMod`] was pressed this frame.
//! - [`PixState::keymod`]: The [`KeyMod`]s pressed this frame.
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App { checkbox: bool, text_field: String };
//! # impl AppState for App {
//! fn on_update(&mut self, s: &mut PixState) -> Result<()> {
//!     s.fill(s.theme().colors.primary);
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
use std::{collections::HashSet, mem, time::Instant};

pub mod environment;
pub mod settings;

/// Represents all state and methods for updating and interacting with the [`PixEngine`].
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
    /// Get the current window title.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.text(format!("Window title: {}", s.title()))?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn title(&self) -> &str {
        self.renderer.title()
    }

    /// Set the current window title.
    ///
    /// # Errors
    ///
    /// If the current window target is closed or invalid or the string contains a `nul` byte, then
    /// an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     if s.button("Change title")? {
    ///         s.set_title("Title changed!")?;
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn set_title<S: AsRef<str>>(&mut self, title: S) -> Result<()> {
        self.renderer.set_title(title.as_ref())
    }

    /// Returns the current mouse position coordinates this frame as `(x, y)`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     // Draw 100x100 rectangle that follows the mouse
    ///     s.rect(rect![s.mouse_pos(), 100, 100])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn mouse_pos(&self) -> Point<i32> {
        self.ui.mouse_pos()
    }

    /// Returns the previous mouse position coordinates last frame as `(x, y)`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     // Draw 100x100 rectangle that follows the mouse last frame
    ///     // Creates a yoyo-like effect
    ///     s.rect(rect![s.pmouse_pos(), 100, 100])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn pmouse_pos(&self) -> Point<i32> {
        self.ui.pmouse_pos()
    }

    /// Returns if any [Mouse] button was pressed this frame.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     if s.mouse_pressed() {
    ///         s.background(Color::random());
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn mouse_pressed(&self) -> bool {
        self.ui.mouse_pressed()
    }

    /// Returns if the [Mouse] was clicked (pressed and released) this frame.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     if s.mouse_clicked(Mouse::Left) {
    ///         s.background(Color::random());
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn mouse_clicked(&self, btn: Mouse) -> bool {
        self.ui.mouse_clicked(btn)
    }

    /// Returns if the [Mouse] was double clicked (pressed and released) this frame.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     if s.mouse_dbl_clicked(Mouse::Left) {
    ///         s.background(Color::random());
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn mouse_dbl_clicked(&self, btn: Mouse) -> bool {
        self.ui.mouse_dbl_clicked(btn)
    }

    /// Returns if a specific [Mouse] button was pressed this frame.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     if s.mouse_down(Mouse::Left) {
    ///         s.background(Color::random());
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn mouse_down(&self, btn: Mouse) -> bool {
        self.ui.mouse_down(btn)
    }

    /// Returns a list of the current mouse buttons pressed this frame.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     // Only trigger if both buttons are pressed
    ///     if s.mouse_buttons().contains(&Mouse::Left)
    ///        && s.mouse_buttons().contains(&Mouse::Right)
    ///     {
    ///         s.background(Color::random());
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub const fn mouse_buttons(&self) -> &HashSet<Mouse> {
        self.ui.mouse_buttons()
    }

    /// Returns if any [Key] was pressed this frame.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     if s.key_pressed() {
    ///         s.background(Color::random());
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn key_pressed(&self) -> bool {
        self.ui.key_pressed()
    }

    /// Returns if a specific [Key] was pressed this frame.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     if s.key_down(Key::Space) {
    ///         s.background(Color::random());
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn key_down(&self, key: Key) -> bool {
        self.ui.key_down(key)
    }

    /// Returns a list of the current keys pressed this frame.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     if s.keys().contains(&Key::Space) && s.keys().contains(&Key::Up) {
    ///         s.background(Color::random());
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub const fn keys(&self) -> &HashSet<Key> {
        self.ui.keys()
    }

    /// Returns if a specific [`KeyMod`] was pressed this frame.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     if s.keymod_down(KeyMod::CTRL) && s.key_down(Key::Space) {
    ///         s.background(Color::random());
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub const fn keymod_down(&self, keymod: KeyMod) -> bool {
        self.ui.keymod_down(keymod)
    }

    /// Returns a list of the current key modifiers pressed this frame.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     if s.keymod().intersects(KeyMod::SHIFT | KeyMod::CTRL)
    ///         && s.key_down(Key::Space)
    ///     {
    ///         s.background(Color::random());
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub const fn keymod(&self) -> &KeyMod {
        self.ui.keymod()
    }
}

impl PixState {
    /// Constructs `PixState` with a given [Renderer].
    #[inline]
    pub(crate) fn new(settings: RendererSettings, theme: Theme) -> Result<Self> {
        let show_frame_rate = settings.show_frame_rate;
        let target_frame_rate = settings.target_frame_rate;
        let renderer = Renderer::new(settings)?;
        let mut state = Self {
            renderer,
            env: Environment::default(),
            ui: UiState::default(),
            settings: Settings::default(),
            setting_stack: Vec::new(),
            theme: theme.clone(),
        };
        state.background(theme.colors.background);
        state.fill(theme.colors.on_background());
        state.show_frame_rate(show_frame_rate);
        state.frame_rate(target_frame_rate);
        state.font_size(theme.font_size)?;
        state.font_style(theme.styles.body);
        state.font_family(theme.fonts.body)?;
        Ok(state)
    }

    /// Handle state changes this frame prior to calling [`AppState::on_update`].
    #[inline]
    pub(crate) fn pre_update(&mut self) {
        // Reset mouse cursor icon to the current setting
        // Ignore any errors, as setting cursor in the first place should have succeeded.
        let _ignore_result = self.renderer.cursor(self.settings.cursor.as_ref());
        self.ui.pre_update(&self.theme);
    }

    /// Handle state updates for this frame.
    #[inline]
    pub(crate) fn on_update(&mut self) -> Result<()> {
        for texture in self.ui.textures.iter_mut().filter(|t| t.visible) {
            self.renderer
                .texture(texture.id, texture.src, texture.dst, 0.0, None, None, None)?;
        }
        Ok(())
    }

    /// Handle state changes this frame after calling [`AppState::on_update`].
    #[inline]
    pub(crate) fn post_update(&mut self) {
        self.ui.post_update();
    }

    /// Takes a [Rect] and returns a modified [Rect] based on the current [`RectMode`].
    #[inline]
    pub(crate) fn get_rect<R>(&self, rect: R) -> Rect<i32>
    where
        R: Into<Rect<i32>>,
    {
        let mut rect = rect.into();
        if self.settings.rect_mode == RectMode::Center {
            rect.center_on(rect.top_left());
        }
        rect
    }

    /// Takes an [Ellipse] and returns a modified [Ellipse] based on the current [`EllipseMode`].
    #[inline]
    pub(crate) fn get_ellipse<E>(&self, ellipse: E) -> Ellipse<i32>
    where
        E: Into<Ellipse<i32>>,
    {
        let mut ellipse = ellipse.into();
        if self.settings.ellipse_mode == RectMode::Corner {
            ellipse.center_on(ellipse.bottom_right());
        }
        ellipse
    }

    /// Updates the mouse position state this frame.
    #[inline]
    pub(crate) fn on_mouse_motion(&mut self, pos: Point<i32>) {
        self.ui.pmouse.pos = self.ui.mouse.pos;
        self.ui.mouse.pos = pos;
    }

    /// Updates the mouse click state this frame.
    #[inline]
    pub(crate) fn on_mouse_click(&mut self, btn: Mouse, time: Instant) {
        self.ui.pmouse.clicked = mem::take(&mut self.ui.mouse.clicked);
        self.ui.pmouse.last_clicked = mem::take(&mut self.ui.mouse.last_clicked);
        self.ui.mouse.click(btn, time);
    }

    /// Updates the mouse double click state this frame.
    #[inline]
    pub(crate) fn on_mouse_dbl_click(&mut self, btn: Mouse, time: Instant) {
        self.ui.pmouse.last_dbl_clicked = mem::take(&mut self.ui.mouse.last_dbl_clicked);
        self.ui.mouse.dbl_click(btn, time);
    }

    /// Updates the mouse pressed state this frame.
    #[inline]
    pub(crate) fn on_mouse_pressed(&mut self, btn: Mouse) {
        self.ui.pmouse.pressed = mem::take(&mut self.ui.mouse.pressed);
        self.ui.mouse.press(btn);
    }

    /// Updates the mouse released state this frame.
    #[inline]
    pub(crate) fn on_mouse_released(&mut self, btn: Mouse) {
        self.ui.pmouse.pressed = mem::take(&mut self.ui.mouse.pressed);
        self.ui.mouse.release(btn);
    }

    /// Updates the mouse wheel state this frame.
    #[inline]
    pub(crate) fn on_mouse_wheel(&mut self, x: i32, y: i32) {
        self.ui.pmouse.xrel = self.ui.mouse.xrel;
        self.ui.pmouse.yrel = self.ui.mouse.yrel;
        self.ui.mouse.wheel(x, y);
    }

    /// Polls for events from the underlying renderer.
    #[inline]
    pub fn poll_event(&mut self) -> Option<Event> {
        self.renderer.poll_event()
    }

    /// Open a controller with a given ID to start handling events.
    ///
    /// # Errors
    ///
    /// If the `ControllerId` is invalid or the renderer fails to open the controller, then an
    /// error is returned.
    #[inline]
    pub fn open_controller(&mut self, id: ControllerId) -> Result<()> {
        self.renderer.open_controller(id)
    }

    /// Close a controller with a given ID to stop handling events.
    #[inline]
    pub fn close_controller(&mut self, id: ControllerId) {
        self.renderer.close_controller(id);
    }
}
