//! The core `PixEngine` functionality.

use crate::{
    common::Result,
    event::{Event, KeyEvent, WindowEvent},
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
    pub fn with_title<S>(&mut self, title: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.settings.title = title.as_ref().to_owned();
        self
    }

    /// Enables frame rate in title.
    pub fn with_frame_rate(&mut self) -> &mut Self {
        self.settings.show_frame_rate = true;
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

    /// Set audio sample rate.
    pub fn audio_sample_rate(&mut self, sample_rate: i32) -> &mut Self {
        self.settings.audio_sample_rate = sample_rate;
        self
    }

    /// Convert the `PixEngineBuilder` to a `PixEngine` instance.
    ///
    /// Returns `Err` if any options provided are invalid.
    pub fn build(&self) -> Result<PixEngine> {
        Ok(PixEngine {
            settings: self.settings.clone(),
            last_frame_time: Instant::now(),
            frame_timer: Duration::from_secs(1),
            frame_counter: 0,
        })
    }
}

/// The core engine that maintains the frame loop, event handling, etc.
#[derive(Debug)]
pub struct PixEngine {
    settings: RendererSettings,
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
    pub fn run<A>(&mut self, app: &mut A) -> Result<()>
    where
        A: AppState,
    {
        let renderer = Renderer::init(self.settings.clone())?;
        let mut state = PixState::init(&self.settings.title, renderer);
        state.show_frame_rate(self.settings.show_frame_rate);

        // Clear and present once on start
        state.clear();
        state.renderer.present();

        // Handle events before on_start to initialize window
        self.handle_events(&mut state, app)?;

        app.on_start(&mut state)?;
        if state.env.quit {
            return Ok(());
        }

        self.last_frame_time = Instant::now();
        // on_stop loop enables on_stop to prevent application close if necessary
        'on_stop: loop {
            // running loop continues until an event or on_update returns false or errors
            'running: loop {
                self.handle_events(&mut state, app)?;
                if state.env.quit {
                    break 'running;
                } else if state.settings.paused {
                    std::thread::sleep(Duration::from_millis(16));
                    continue;
                }

                app.on_update(&mut state)?;
                state.renderer.present();
                self.update_frame_rate(&mut state)?;
            }
            app.on_stop(&mut state)?;
            if state.env.quit {
                break 'on_stop;
            }
        }
        Ok(())
    }

    /// Handle events from the event pump.
    fn handle_events<A>(&mut self, state: &mut PixState, app: &mut A) -> Result<()>
    where
        A: AppState,
    {
        while let Some(event) = state.renderer.poll_event() {
            match event {
                Event::Quit { .. } | Event::AppTerminating { .. } => state.quit(),
                Event::Window {
                    window_id,
                    win_event,
                } => match win_event {
                    WindowEvent::FocusGained => {
                        state.env.focused = true;
                        state.env.focused_window = Some(window_id);
                    }
                    WindowEvent::FocusLost => {
                        state.env.focused = false;
                        state.env.focused_window = None;
                    }
                    WindowEvent::Resized(_, _) | WindowEvent::SizeChanged(_, _) => {
                        app.on_window_resized(state)?
                    }
                    _ => (),
                },
                Event::KeyDown {
                    key: Some(key),
                    keymod,
                    repeat,
                } => {
                    state.key_down = true;
                    state.keys.insert(key);
                    app.on_key_pressed(
                        state,
                        KeyEvent {
                            key,
                            keymod,
                            pressed: true,
                            repeat,
                        },
                    )?;
                }
                Event::KeyUp {
                    key: Some(key),
                    keymod,
                    repeat,
                } => {
                    state.key_down = false;
                    state.keys.remove(&key);
                    app.on_key_released(
                        state,
                        KeyEvent {
                            key,
                            keymod,
                            pressed: false,
                            repeat,
                        },
                    )?;
                }
                Event::TextInput { text, .. } => {
                    app.on_key_typed(state, &text)?;
                }
                Event::MouseMotion { x, y, .. } => {
                    state.pmouse_pos = state.mouse_pos;
                    state.mouse_pos = (x, y).into();
                    if state.mouse_down {
                        app.on_mouse_dragged(state)?;
                    }
                }
                Event::MouseDown { button, .. } => {
                    state.mouse_down = true;
                    state.mouse_buttons.insert(button);
                    app.on_mouse_pressed(state, button)?;
                }
                Event::MouseUp { button, .. } => {
                    state.mouse_down = false;
                    state.mouse_buttons.remove(&button);
                    app.on_mouse_released(state, button)?;
                }
                Event::MouseWheel { x, y, .. } => {
                    app.on_mouse_wheel(state, x, y)?;
                }
                _ => (),
            }
        }
        Ok(())
    }

    /// Updates the average frame rate and the window title if setting is enabled.
    fn update_frame_rate(&mut self, state: &mut PixState) -> Result<()> {
        let now = Instant::now();
        state.env.delta_time = now - self.last_frame_time;
        self.last_frame_time = now;
        if state.settings.show_frame_rate {
            self.frame_timer += state.env.delta_time;
            self.frame_counter += 1;
            state.env.frame_count += 1;
            if self.frame_timer >= ONE_SECOND {
                state.env.frame_rate = self.frame_counter;
                let title = format!("{} - FPS: {}", state.title(), state.env.frame_rate);
                state.renderer.set_title(&title)?;
                self.frame_timer -= ONE_SECOND;
                self.frame_counter = 0;
            }
        }
        Ok(())
    }
}
