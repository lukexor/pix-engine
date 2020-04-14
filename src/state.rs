use crate::{
    event::PixEvent,
    renderer::{self, Renderer},
};
use environment::Environment;
use std::{borrow::Cow, error, fmt, vec::Drain};
use window::Window;

pub mod rendering;

mod environment;
mod setting;
mod window;

/// Result type for State Errors.
pub type Result<T> = std::result::Result<T, Error>;

/// Types of errors State can return in a result.
#[derive(Debug)]
pub enum Error {
    RendererError(renderer::Error),
    InvalidWindowTarget(u32),
    Other(Cow<'static, str>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            RendererError(err) => write!(f, "renderer error: {}", &err),
            InvalidWindowTarget(t) => write!(f, "invalid window_target: {}", &t),
            Other(desc) => write!(f, "{}", &desc),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<renderer::Error> for Error {
    fn from(err: renderer::Error) -> Self {
        Self::RendererError(err)
    }
}

/// Contains all engine state and methods allowing the enclosed app to interact
/// with engine state
pub struct State {
    #[cfg(all(feature = "sdl2-renderer", not(feature = "wasm-renderer")))]
    pub(crate) renderer: renderer::sdl2::Sdl2Renderer,
    #[cfg(all(feature = "wasm-renderer", not(feature = "sdl2-renderer")))]
    pub(crate) renderer: renderer::wasm::WasmRenderer,
    pub(crate) events: Vec<PixEvent>,
    pub(crate) should_loop: bool,
    pub(crate) manual_update: u32, // Used to manually update when should_loop is false
    pub(crate) environment: Environment,
    primary_window: u32,
    window_target: u32,
    window_target_history: Vec<u32>,
    windows: Vec<Window>,
    // input_states
    // elements
    // loaded_fonts
}

impl State {
    /// Creates a new State instance
    pub(crate) fn new(title: &str, width: u32, height: u32) -> Result<Self> {
        let renderer = renderer::load_renderer(title, width, height)?;
        let primary_window = renderer.default_window_id();
        Ok(Self {
            renderer,
            events: Vec::new(),
            should_loop: true,
            manual_update: 1, // Always loop at least once on start
            environment: Environment::default(),
            primary_window,
            window_target: primary_window,
            window_target_history: Vec::new(),
            windows: vec![Window::new(primary_window, title)],
        })
    }

    /// Returns an iterator of events from the event queue
    pub fn poll_event(&mut self) -> Option<PixEvent> {
        self.events.pop()
    }
    /// Returns an iterator of PixEvents from the event queue.
    pub fn poll_iter(&mut self) -> Drain<PixEvent> {
        self.events.drain(..)
    }

    /// Sets the engine to loop and call `App::on_update()`.
    pub fn r#loop(&mut self) {
        self.should_loop = true;
    }
    /// Sets the engine to stop looping and no longer call `App::on_update()`.
    pub fn no_loop(&mut self) {
        self.should_loop = false;
    }

    /// Pushes current window settings, saving them for later use with `State::pop()`.
    pub fn push(&mut self) {
        for window in self.windows.iter_mut() {
            window.push();
        }
    }
    /// Pops previous window settings.
    pub fn pop(&mut self) {
        for window in self.windows.iter_mut() {
            window.pop();
        }
    }

    /// Calls `State::on_update()` a number of times based on the parameter passed in.
    /// Useful for updating when `State::no_loop()` is enabled.
    pub fn update(&mut self, n: u32) {
        self.manual_update = n;
    }

    /// Window Management

    /// Get a window based on the current window target
    pub(crate) fn get_window(&self) -> &Window {
        let target = self.window_target();
        self.windows
            .iter()
            .find(|w| target == w.id())
            .expect("valid window target")
    }
    /// Get a mutable window based on the current window target
    pub(crate) fn get_window_mut(&mut self) -> &mut Window {
        let target = self.window_target();
        self.windows
            .iter_mut()
            .find(|w| target == w.id())
            .expect("valid window target")
    }

    /// Get the primary window_id
    pub fn primary_window(&self) -> u32 {
        self.primary_window
    }
    /// Get the window_id of the current window target
    pub fn window_target(&self) -> u32 {
        self.window_target
    }

    /// Set a new window target. Setting None will revert to the default window.
    ///
    /// Errors if the window_id is not a valid window_id.
    pub fn set_window_target<I: Into<Option<u32>>>(&mut self, window_id: I) -> Result<()> {
        let window_id = window_id.into();
        if Some(self.window_target) != window_id {
            let id = match window_id {
                Some(id) => {
                    if self.windows.iter().any(|w| id == w.id()) {
                        self.window_target_history.push(self.window_target);
                        id
                    } else {
                        return Err(Error::InvalidWindowTarget(id));
                    }
                }
                None => self
                    .window_target_history
                    .pop()
                    .or_else(|| Some(self.windows[0].id()))
                    .unwrap(),
            };
            self.window_target = id;
            self.renderer.set_window_target(id);
        }
        Ok(())
    }

    /// Create and open a new window.
    ///
    /// Errors if the window can't be created for any reason.
    pub fn create_window(&mut self, title: &str, width: u32, height: u32) -> Result<u32> {
        let id = self.renderer.create_window(title, width, height)?;
        self.windows.push(Window::new(id, title));
        Ok(id)
    }
    /// Close the current window target.
    ///
    /// Returns true when all windows are closed.
    pub fn close_window(&mut self) -> bool {
        let target = self.window_target();
        self.windows.retain(|w| target != w.id());
        self.renderer.close_window()
    }
}
