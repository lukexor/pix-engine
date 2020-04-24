use crate::{event::*, renderer, shape::Point};
use environment::Environment;
use setting::Setting;
use std::{borrow::Cow, collections::HashSet, error, fmt, vec::Drain};

pub use rendering::Drawable;

pub mod rendering;

mod environment;
mod setting;

/// Result type for State Errors.
pub type StateResult<T> = std::result::Result<T, Error>;

/// Types of errors State can return in a result.
#[derive(Debug)]
pub enum Error {
    RendererError(renderer::Error),
    InvalidWindowTarget(u32),
    ConversionError(Cow<'static, str>),
    Other(Cow<'static, str>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            RendererError(err) => write!(f, "renderer error: {}", &err),
            InvalidWindowTarget(t) => write!(f, "invalid window_target: {}", &t),
            Other(desc) | ConversionError(desc) => write!(f, "{}", &desc),
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

/// Contains all engine state and methods allowing the enclosed application which implements the
/// `PixApp` trait to interact with the engine.
pub struct State {
    pub(crate) title: String,
    width: u32,
    height: u32,
    #[cfg(all(feature = "sdl2-renderer", not(feature = "wasm-renderer")))]
    pub(crate) renderer: renderer::sdl2::Sdl2Renderer,
    #[cfg(all(feature = "wasm-renderer", not(feature = "sdl2-renderer")))]
    pub(crate) renderer: renderer::wasm::WasmRenderer,
    pub(crate) events: Vec<PixEvent>,
    pub(crate) should_loop: bool,
    pub(crate) manual_update: u32, // Used to manually update when should_loop is false
    pub(crate) mouse_pos: Point,   // Mouse position
    pub(crate) pmouse_pos: Point,  // Previous mouse position
    pub(crate) mouse_is_pressed: bool,
    pub(crate) mouse_buttons: HashSet<MouseButton>, // List of pressed mouse buttons
    environment: Environment,
    settings: Setting,
    settings_stack: Vec<Setting>,
}

impl State {
    /// Creates a new `State` instance.
    pub fn new(title: &str, width: u32, height: u32) -> StateResult<Self> {
        let renderer = renderer::load_renderer(title, width, height)?;
        Ok(Self {
            title: title.to_owned(),
            width,
            height,
            renderer,
            events: Vec::new(),
            should_loop: true,
            manual_update: 1, // Always loop at least once on start
            mouse_pos: Point::default(),
            pmouse_pos: Point::default(),
            mouse_is_pressed: false,
            mouse_buttons: HashSet::new(),
            environment: Environment::new(),
            settings: Setting::new(),
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

    /// Gets the width of the window.
    pub fn width(&self) -> u32 {
        self.width
    }
    /// Gets the height of the window.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Gets the current mouse position as a `Point`.
    pub fn mouse_pos(&self) -> Point {
        self.mouse_pos
    }

    /// Gets the position of the mouse from the previous frame as a `Point`.
    pub fn pmouse_pos(&self) -> Point {
        self.pmouse_pos
    }

    /// Gets whether a mouse button is currently pressed.
    pub fn mouse_is_pressed(&self) -> bool {
        self.mouse_is_pressed
    }

    /// Gets a list of currently pressed `MouseButton`s.
    pub fn mouse_buttons(&self) -> &HashSet<MouseButton> {
        &self.mouse_buttons
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
