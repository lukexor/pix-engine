//! The core `PixEngine` functionality.

use crate::{
    common::{Error, Result},
    event::{Event, WindowEvent},
    renderer::{self, Position, Renderer, RendererSettings, Rendering},
    state::{self, State, Stateful},
};
use std::{
    path::Path,
    time::{Duration, Instant},
};

/// Builds a `PixEngine` instance by providing several optional modifiers.
#[derive(Debug, Default)]
pub struct PixEngineBuilder {
    settings: RendererSettings,
}

impl PixEngineBuilder {
    /// Creates a new `PixEngineBuilder` instance.
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            settings: RendererSettings {
                title: title.to_owned(),
                width,
                height,
                ..Default::default()
            },
        }
    }

    /// Position the window at the given (x, y) coordinates of the display.
    pub fn position(&mut self, x: i32, y: i32) -> &mut Self {
        self.settings.x = Position::Positioned(x);
        self.settings.y = Position::Positioned(y);
        self
    }

    /// Position the window in the center of the display.
    pub fn position_centered(&mut self) -> &mut Self {
        self.settings.x = Position::Centered;
        self.settings.y = Position::Centered;
        self
    }

    /// Set a window icon.
    pub fn icon<P: AsRef<Path>>(&mut self, _path: P) -> &mut Self {
        // TODO icon
        self
    }

    /// Start window in fullscreen mode.
    pub fn fullscreen(&mut self) -> &mut Self {
        self.settings.fullscreen = true;
        self
    }

    /// Allow window resizing.
    pub fn resizable(&mut self) -> &mut Self {
        self.settings.resizable = true;
        self
    }

    /// Scales the window.
    pub fn scale(&mut self, x: f32, y: f32) -> &mut Self {
        self.settings.scale_x = x;
        self.settings.scale_y = y;
        self
    }

    /// Enable VSync.
    pub fn vsync_enabled(&mut self) -> &mut Self {
        self.settings.vsync = true;
        self
    }

    /// Convert the `PixEngineBuilder` to a `PixEngine` instance.
    ///
    /// Returns `Err` if any options provided are invalid.
    pub fn build(&self) -> Result<PixEngine> {
        let renderer = Renderer::init(&self.settings)?;
        Ok(PixEngine {
            state: State::init(renderer)?,
            last_frame_time: Instant::now(),
            frame_timer: Duration::from_secs(1),
            frame_counter: 0,
        })
    }
}

/// The core engine that maintains the frame loop, event handling, etc.
#[derive(Debug)]
pub struct PixEngine {
    state: State,
    frame_timer: Duration,
    last_frame_time: Instant,
    frame_counter: u64,
}

impl PixEngine {
    /// Creates a new `PixEngineBuilder` which can create a `PixEngine` instance.
    pub fn create(title: &str, width: u32, height: u32) -> PixEngineBuilder {
        PixEngineBuilder::new(title, width, height)
    }

    /// Starts the `PixEngine` and begins executing the frame loop.
    pub fn run<A: Stateful>(&mut self, app: &mut A) -> Result<()> {
        if self.start(app)? {
            self.last_frame_time = Instant::now();
            // on_stop loop enables on_stop to prevent application close if necessary
            'on_stop: loop {
                // running loop continues until an event or on_update returns false or errors
                'running: loop {
                    // TODO: Add state.loop() and state.no_loop() checks
                    // Ensure update is run at least once
                    self.state.clear();
                    if !self.handle_events(app)? || !app.on_update(&mut self.state)? {
                        break 'running;
                    }
                    self.state.renderer.present();
                    self.update_frame_rate()?;
                }
                if app.on_stop(&mut self.state)? {
                    break 'on_stop;
                }
            }
        }
        Ok(())
    }

    /// Returns a reference to the `PixEngine` `State`.
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Returns a mutable reference to the `PixEngine` `State`.
    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    /// Setup at the start of running the engine.
    fn start<A: Stateful>(&mut self, app: &mut A) -> Result<bool> {
        // Clear and present once on start
        self.state.clear();
        self.state.renderer.present();
        // Handle events before on_start to initialize window
        let _ = self.handle_events(app)?;
        Ok(app.on_start(&mut self.state)?)
    }

    /// Handle events from the event pump.
    fn handle_events<A: Stateful>(&mut self, app: &mut A) -> Result<bool> {
        // TODO Clear/reset key/mouse states for this frame
        while let Some(event) = self.state.renderer.poll_event() {
            match event {
                Event::Quit { .. } | Event::AppTerminating { .. } => return Ok(false),
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::FocusGained => self.state.env.focused = true,
                    WindowEvent::FocusLost => self.state.env.focused = false,
                    WindowEvent::Resized(_, _) => {
                        // TODO: Handle window resized
                    }
                    _ => (),
                },
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode {
                        self.state.key_down = true;
                        self.state.keys.insert(key);
                        app.on_key_pressed(&mut self.state, key);
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = keycode {
                        self.state.key_down = false;
                        self.state.keys.remove(&key);
                        app.on_key_released(&mut self.state, key);
                    }
                }
                Event::TextEditing { .. } => {
                    // TODO: Handle text editing
                    // input text boxes, text areas
                }
                Event::TextInput { .. } => {
                    // TODO: Handle text input
                    // input text boxes, text areas
                }
                Event::MouseMotion { x, y, .. } => {
                    self.state.pmouse_pos = self.state.mouse_pos;
                    self.state.mouse_pos = (x, y);
                    if self.state.mouse_down {
                        app.on_mouse_dragged(&mut self.state);
                    }
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    self.state.mouse_down = true;
                    self.state.mouse_buttons.insert(mouse_btn);
                    app.on_mouse_pressed(&mut self.state, mouse_btn);
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    self.state.mouse_down = false;
                    self.state.mouse_buttons.remove(&mouse_btn);
                    app.on_mouse_released(&mut self.state, mouse_btn);
                }
                Event::MouseWheel { .. } => {
                    // TODO: Handle mouse wheel
                    // scrolling
                }
                _ => (),
            }
        }
        Ok(true)
    }

    /// Updates the average frame rate and the window title if setting is enabled.
    fn update_frame_rate(&mut self) -> Result<()> {
        let now = Instant::now();
        let one_second = Duration::from_secs(1);
        self.state.env.delta_time = now - self.last_frame_time;
        self.last_frame_time = now;
        if self.state.settings.show_frame_rate {
            self.frame_timer += self.state.env.delta_time;
            self.frame_counter += 1;
            self.state.env.frame_count += 1;
            if self.frame_timer >= one_second {
                self.state.env.frame_rate = self.frame_counter;
                let title = format!(
                    "{} - FPS: {}",
                    self.state.title(),
                    self.state.env.frame_rate
                );
                self.state.set_title(&title)?;
                self.frame_timer -= one_second;
                self.frame_counter = 0;
            }
        }
        Ok(())
    }
}

impl From<state::Error> for Error {
    fn from(err: state::Error) -> Error {
        Error::StateError(err)
    }
}
impl From<renderer::Error> for Error {
    fn from(err: renderer::Error) -> Error {
        Error::RendererError(err)
    }
}
