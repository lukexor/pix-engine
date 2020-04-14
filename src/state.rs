use crate::{event::PixEvent, renderer};
use environment::StateEnvironment;
use setting::StateSetting;
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
    Other(Cow<'static, str>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            RendererError(err) => write!(f, "renderer error: {}", &err),
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
    pub(crate) title: String,
    #[cfg(all(feature = "sdl2-renderer", not(feature = "wasm-renderer")))]
    pub(crate) renderer: renderer::sdl2::Sdl2Renderer,
    #[cfg(all(feature = "wasm-renderer", not(feature = "sdl2-renderer")))]
    pub(crate) renderer: renderer::wasm::WasmRenderer,
    pub(crate) events: Vec<PixEvent>,
    pub(crate) should_loop: bool,
    // input_states
    // elements
    // loaded_fonts
    setting_stack: Vec<StateSetting>,
    environment: StateEnvironment,
    settings: StateSetting,
}

impl State {
    /// Creates a new State instance with default settings
    pub(crate) fn new(title: &str, width: u32, height: u32) -> Result<Self> {
        let renderer = renderer::load_renderer(title, width, height)?;
        Ok(Self {
            renderer,
            events: Vec::new(),
            title: title.to_owned(),
            should_loop: true,
            setting_stack: Vec::new(),
            environment: StateEnvironment::default(),
            settings: StateSetting::default(),
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
}
