//! Core [`PixEngine`] functions.

use crate::{
    common::Result,
    event::{Event, KeyEvent, WindowEvent},
    renderer::{Renderer, RendererSettings, Rendering},
    state::{AppState, PixState},
    window::Position,
    window::Window,
};
#[cfg(not(target_arch = "wasm32"))]
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

#[cfg(not(target_arch = "wasm32"))]
const ONE_SECOND: Duration = Duration::from_secs(1);

/// Builds a [`PixEngine`] instance by providing several configration functions.
#[non_exhaustive]
#[derive(Default, Debug, Clone)]
pub struct PixEngineBuilder {
    settings: RendererSettings,
}

impl PixEngineBuilder {
    /// Constructs a `PixEngineBuilder`.
    pub fn new() -> Self {
        Self {
            settings: RendererSettings::default(),
        }
    }

    /// Set window dimensions.
    pub fn with_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.settings.width = width;
        self.settings.height = height;
        self
    }

    /// Set a window title.
    pub fn with_title<S>(&mut self, title: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.settings.title = title.into();
        self
    }

    /// Enable average frame rate (FPS) in title.
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
    #[cfg(not(target_arch = "wasm32"))]
    pub fn icon<P>(&mut self, path: P) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.settings.icon = Some(path.into());
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

    /// Convert [`PixEngineBuilder`] to a [`PixEngine`] instance.
    pub fn build(&self) -> PixEngine {
        PixEngine {
            settings: self.settings.clone(),
            #[cfg(not(target_arch = "wasm32"))]
            last_frame_time: Instant::now(),
            #[cfg(not(target_arch = "wasm32"))]
            frame_timer: Duration::from_secs(1),
            frame_counter: 0,
        }
    }
}

/// The core engine that maintains the render loop, state, drawing functions, event handling, etc.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct PixEngine {
    settings: RendererSettings,
    #[cfg(not(target_arch = "wasm32"))]
    frame_timer: Duration,
    #[cfg(not(target_arch = "wasm32"))]
    last_frame_time: Instant,
    frame_counter: u64,
}

impl PixEngine {
    /// Constructs a default [`PixEngineBuilder`] which can build a `PixEngine` instance.
    pub fn builder() -> PixEngineBuilder {
        PixEngineBuilder::default()
    }

    /// Starts the `PixEngine` application and begins executing the frame loop.
    pub fn run<A>(&mut self, app: &mut A) -> Result<()>
    where
        A: AppState,
    {
        let renderer = Renderer::new(self.settings.clone())?;
        let mut state = PixState::new(&self.settings.title, renderer);
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

        // on_stop loop enables on_stop to prevent application close if necessary
        'on_stop: loop {
            // running loop continues until an event or on_update returns false or errors
            'running: loop {
                self.handle_events(&mut state, app)?;
                if state.env.quit {
                    break 'running;
                } else if state.settings.paused {
                    self.sleep();
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

    /// Handle user and system events.
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
                    state.mouse_pos = [x, y].into();
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

    #[cfg(target_arch = "wasm32")]
    fn sleep(&mut self) {
        todo!("wasm32: sleep");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn sleep(&mut self) {
        std::thread::sleep(Duration::from_millis(16));
    }

    #[cfg(target_arch = "wasm32")]
    fn update_frame_rate(&mut self, _state: &mut PixState) -> Result<()> {
        todo!("wasm32: update_frame_rate");
    }

    /// Updates the average frame rate and the window title if setting is enabled.
    #[cfg(not(target_arch = "wasm32"))]
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

impl Default for PixEngine {
    fn default() -> Self {
        PixEngine::builder().build()
    }
}
