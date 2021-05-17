//! State management for `PixEngine`.

use crate::{
    common,
    event::{Event, Key, Mouse},
    renderer::{self, Renderer, Rendering},
    shape::Point,
};
use environment::Environment;
use settings::Settings;
use std::{borrow::Cow, collections::HashSet, error, fmt, io, result};

pub mod environment;
pub mod settings;

/// The result type for [`PixState`] operations.
type Result<T> = result::Result<T, Error>;

/// The error type for [`PixState`] operations.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// IO specific errors.
    IoError(io::Error),
    /// Renderer specific errors.
    RendererError(renderer::Error),
    /// Unknown errors.
    Other(Cow<'static, str>),
}

/// Defines state changing operations that are called while the `PixEngine` is running.
pub trait AppState {
    /// Called once upon engine start when `PixEngine::run()` is called.
    fn on_start(&mut self, _s: &mut PixState) -> common::Result<()> {
        Ok(())
    }

    /// Called every frame based on the target_frame_rate. By default this is as often as possible.
    fn on_update(&mut self, _s: &mut PixState) -> common::Result<()>;

    /// Called once when the engine detects a close/exit event.
    fn on_stop(&mut self, _s: &mut PixState) -> common::Result<()> {
        Ok(())
    }

    // TODO: Make on calls return result
    /// Called each time a key is pressed.
    fn on_key_pressed(
        &mut self,
        _s: &mut PixState,
        _key: Key,
        _repeat: bool,
    ) -> common::Result<()> {
        Ok(())
    }

    /// Called each time a key is released.
    fn on_key_released(
        &mut self,
        _s: &mut PixState,
        _key: Key,
        _repeat: bool,
    ) -> common::Result<()> {
        Ok(())
    }

    /// Called each time a key is typed. Ignores special keys like Backspace.
    fn on_key_typed(&mut self, _s: &mut PixState, _text: &str) -> common::Result<()> {
        Ok(())
    }

    /// Called each time a mouse button is pressed.
    fn on_mouse_dragged(&mut self, _s: &mut PixState) -> common::Result<()> {
        Ok(())
    }

    /// Called each time a mouse button is pressed.
    fn on_mouse_pressed(&mut self, _s: &mut PixState, _btn: Mouse) -> common::Result<()> {
        Ok(())
    }

    // TODO: on_mouse_clicked - Press followed by release
    // TODO: on_mouse_dbl_clicked - 2 clicks
    // TODO: on_mouse_motion

    /// Called each time a mouse button is released.
    fn on_mouse_released(&mut self, _s: &mut PixState, _btn: Mouse) -> common::Result<()> {
        Ok(())
    }

    /// Called each time the mouse wheel is scrolled.
    fn on_mouse_wheel(
        &mut self,
        _s: &mut PixState,
        _x_delta: i32,
        _y_delta: i32,
    ) -> common::Result<()> {
        Ok(())
    }

    /// Called each time the window is resized.
    fn on_window_resized(&mut self, _s: &mut PixState) -> common::Result<()> {
        Ok(())
    }
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
    pub(crate) mouse_buttons: HashSet<Mouse>,
    pub(crate) key_down: bool,
    pub(crate) keys: HashSet<Key>,
    pub(crate) perlin: Option<Vec<f64>>,
    // TODO: state_stack for push/pop
}

impl PixState {
    // TODO:
    // save_canvas<P: AsRef<Path>>(filename: P)

    /// Creates a new `PixState` instance with a given `Renderer`.
    pub fn init(title: &str, renderer: Renderer) -> Self {
        Self {
            title: title.to_owned(),
            renderer,
            env: Environment::default(),
            settings: Settings::default(),
            mouse_pos: Point::default(),
            pmouse_pos: Point::default(),
            mouse_down: false,
            mouse_buttons: HashSet::new(),
            key_down: false,
            keys: HashSet::new(),
            // TODO: move to cache object
            perlin: None,
        }
    }

    /// Clears the render target to the current background color set by `PixState::background()`.
    pub fn clear(&mut self) {
        let color = self.settings.background;
        self.renderer.draw_color(self.settings.background);
        self.renderer.clear();
        self.renderer.draw_color(color);
    }

    /// Get the current window title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set the current window title.
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        self.title = title.to_owned();
        if self.settings.show_frame_rate {
            self.renderer
                .set_title(&format!("{} - FPS: {}", title, self.env.frame_rate))?;
        } else {
            self.renderer.set_title(title)?;
        }
        Ok(())
    }

    // TODO:
    // set_dimensions
    // set_width
    // set_height

    /// Manually run on_update `n` times.
    pub fn update(&mut self, _n: usize) {
        // self.update_count = Some(n);
        // self.settings.paused = false;
        todo!("update")
    }

    /// Returns the current mouse position coordinates as (x, y).
    pub fn mouse_pos(&self) -> Point {
        self.mouse_pos
    }

    /// Returns the previous mouse position coordinates last frame as (x, y).
    pub fn pmouse_pos(&self) -> Point {
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

    /// Returns a single event or None if the event pump is empty.
    pub fn poll_event(&mut self) -> Option<Event> {
        self.renderer.poll_event()
    }

    /// Returns an iterator of events from the event pump.
    pub fn poll_events(&mut self) -> Vec<Event> {
        self.renderer.poll_events()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            IoError(err) => err.fmt(f),
            RendererError(err) => err.fmt(f),
            Other(err) => write!(f, "Image error: {}", err),
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

impl From<Error> for common::Error {
    fn from(err: Error) -> Self {
        Self::StateError(err)
    }
}
