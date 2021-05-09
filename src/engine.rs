//! The core `PixEngine` functionality.

use crate::{
    common::PixResult,
    event::{Event, WindowEvent},
    renderer::{Position, Renderer, RendererSettings, Rendering},
    state::{AppState, PixState},
};
use std::{
    path::Path,
    time::{Duration, Instant},
};

const ONE_SECOND: Duration = Duration::from_secs(1);

/// Builds a `PixEngine` instance by providing several optional modifiers.
#[derive(Debug, Default)]
pub struct PixEngineBuilder {
    settings: RendererSettings,
}

impl PixEngineBuilder {
    /// Creates a new `PixEngineBuilder` instance.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            settings: RendererSettings {
                width,
                height,
                ..Default::default()
            },
        }
    }

    /// Set a window title.
    pub fn with_title(&mut self, title: &str) -> &mut Self {
        self.settings.title = title.to_owned();
        self
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
    pub fn icon<P>(&mut self, path: P) -> &mut Self
    where
        P: AsRef<Path>,
    {
        self.settings.icon = Some(path.as_ref().to_owned());
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
    pub fn build(&self) -> PixResult<PixEngine> {
        let renderer = Renderer::init(self.settings.clone())?;
        Ok(PixEngine {
            state: PixState::init(&self.settings.title, renderer),
            last_frame_time: Instant::now(),
            frame_timer: Duration::from_secs(1),
            frame_counter: 0,
        })
    }
}

/// The core engine that maintains the frame loop, event handling, etc.
#[derive(Debug)]
pub struct PixEngine {
    state: PixState,
    frame_timer: Duration,
    last_frame_time: Instant,
    frame_counter: u64,
}

impl PixEngine {
    /// Creates a default `PixEngineBuilder` which can create a `PixEngine` instance.
    pub fn builder() -> PixEngineBuilder {
        PixEngineBuilder::default()
    }

    /// Creates a new `PixEngineBuilder` with width/height which can create a `PixEngine` instance.
    pub fn create(width: u32, height: u32) -> PixEngineBuilder {
        PixEngineBuilder::new(width, height)
    }

    /// Starts the `PixEngine` and begins executing the frame loop.
    pub fn run<A>(&mut self, app: &mut A) -> PixResult<()>
    where
        A: AppState,
    {
        self.start(app)?;
        if self.state.env.quit {
            return Ok(());
        }

        self.last_frame_time = Instant::now();
        // on_stop loop enables on_stop to prevent application close if necessary
        'on_stop: loop {
            // running loop continues until an event or on_update returns false or errors
            'running: loop {
                self.handle_events(app);
                if self.state.env.quit {
                    break 'running;
                } else if self.state.settings.paused {
                    std::thread::sleep(Duration::from_millis(16));
                    continue;
                }

                app.on_update(&mut self.state)?;
                self.state.renderer.present();
                self.update_frame_rate()?;
            }
            app.on_stop(&mut self.state)?;
            if self.state.env.quit {
                break 'on_stop;
            }
        }
        Ok(())
    }

    /// Returns a reference to `PixState`.
    pub fn state(&self) -> &PixState {
        &self.state
    }

    /// Returns a mutable reference to the `PixState`.
    pub fn state_mut(&mut self) -> &mut PixState {
        &mut self.state
    }

    /// Setup at the start of running the engine.
    fn start<A>(&mut self, app: &mut A) -> PixResult<()>
    where
        A: AppState,
    {
        // Clear and present once on start
        self.state.clear();
        self.state.renderer.present();
        // Handle events before on_start to initialize window
        self.handle_events(app);
        Ok(app.on_start(&mut self.state)?)
    }

    /// Handle events from the event pump.
    fn handle_events<A>(&mut self, app: &mut A)
    where
        A: AppState,
    {
        while let Some(event) = self.state.renderer.poll_event() {
            match event {
                Event::Quit { .. } | Event::AppTerminating { .. } => self.state.quit(),
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::FocusGained => self.state.env.focused = true,
                    WindowEvent::FocusLost => self.state.env.focused = false,
                    WindowEvent::Resized(_, _) => app.on_window_resized(&mut self.state),
                    _ => (),
                },
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    self.state.key_down = true;
                    self.state.keys.insert(key);
                    app.on_key_pressed(&mut self.state, key);
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    self.state.key_down = false;
                    self.state.keys.remove(&key);
                    app.on_key_released(&mut self.state, key);
                }
                Event::TextInput { text, .. } => {
                    app.on_key_typed(&mut self.state, &text);
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
                Event::MouseWheel { x, y, .. } => {
                    app.on_mouse_wheel(&mut self.state, x, y);
                }
                _ => (),
            }
        }
    }

    /// Updates the average frame rate and the window title if setting is enabled.
    fn update_frame_rate(&mut self) -> PixResult<()> {
        let now = Instant::now();
        self.state.env.delta_time = now - self.last_frame_time;
        self.last_frame_time = now;
        if self.state.settings.show_frame_rate {
            self.frame_timer += self.state.env.delta_time;
            self.frame_counter += 1;
            self.state.env.frame_count += 1;
            if self.frame_timer >= ONE_SECOND {
                self.state.env.frame_rate = self.frame_counter;
                let title = format!(
                    "{} - FPS: {}",
                    self.state.title(),
                    self.state.env.frame_rate
                );
                self.state.set_title(&title)?;
                self.frame_timer -= ONE_SECOND;
                self.frame_counter = 0;
            }
        }
        Ok(())
    }
}
