use crate::{event::PixEvent, renderer};
use environment::Environment;
use setting::Setting;
use std::{borrow::Cow, error, fmt, vec::Drain};

pub mod rendering;

mod environment;
mod setting;

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

// TODO Add SDL2 specific functions to manage multiple windows

/// Contains all engine state and methods allowing the enclosed app to interact
/// with engine state
pub struct State {
    pub(crate) title: String,
    #[cfg(all(feature = "sdl2-renderer", not(feature = "wasm-renderer")))]
    pub(crate) renderer: renderer::sdl2::Sdl2Renderer,
    #[cfg(all(feature = "wasm-renderer", not(feature = "sdl2-renderer")))]
    pub(crate) renderer: renderer::wasm::WasmRenderer,
    pub(crate) events: Vec<PixEvent>,
    pub(crate) should_loop: bool,
    pub(crate) manual_update: u32, // Used to manually update when should_loop is false
    environment: Environment,
    // input_states
    // elements
    // loaded_fonts
    settings: Setting,
    settings_stack: Vec<Setting>,
}

impl State {
    /// Creates a new State instance
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self> {
        let renderer = renderer::load_renderer(title, width, height)?;
        Ok(Self {
            title: title.to_owned(),
            renderer,
            events: Vec::new(),
            should_loop: true,
            manual_update: 1, // Always loop at least once on start
            environment: Environment::default(),
            settings: Setting::default(),
            settings_stack: Vec::new(),
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
        self.settings_stack.push(self.settings.clone());
    }
    /// Pops previous window settings.
    pub fn pop(&mut self) {
        if let Some(settings) = self.settings_stack.pop() {
            self.settings = settings;
        }
    }

    /// Calls `State::on_update()` a number of times based on the parameter passed in.
    /// Useful for updating when `State::no_loop()` is enabled.
    pub fn update(&mut self, n: u32) {
        self.manual_update = n;
    }
}
