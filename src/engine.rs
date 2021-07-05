//! [PixEngine] functions.

use crate::{
    prelude::*,
    renderer::{Renderer, RendererSettings, Rendering},
    window::Window,
};
use std::time::Instant;

#[cfg(not(target_arch = "wasm32"))]
use crate::{ASSETS, ASSET_DIR};
#[cfg(not(target_arch = "wasm32"))]
use std::{fs, io, path::PathBuf};

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

    /// Set a True-Type Font for text rendering.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn with_font<P>(&mut self, path: P, size: u16) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.settings.font = path.into();
        self.settings.font_size = size;
        self
    }

    /// Set font for text rendering.
    #[cfg(target_arch = "wasm32")]
    pub fn with_font<S>(&mut self, font: S, size: u16) -> &mut Self
    where
        S: Into<String>,
    {
        self.settings.font = font.into();
        self.settings.font_size = size;
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
            last_frame_time: Instant::now(),
            frame_timer: 1000.0,
        }
    }
}

/// The core engine that maintains the render loop, state, drawing functions, event handling, etc.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct PixEngine {
    settings: RendererSettings,
    frame_timer: f64,
    last_frame_time: Instant,
}

impl PixEngine {
    /// Constructs a default [`PixEngineBuilder`] which can build a `PixEngine` instance.
    pub fn builder() -> PixEngineBuilder {
        PixEngineBuilder::default()
    }

    /// Starts the `PixEngine` application and begins executing the frame loop.
    pub fn run<A>(&mut self, app: &mut A) -> PixResult<()>
    where
        A: AppState,
    {
        #[cfg(not(target_arch = "wasm32"))]
        fs::create_dir_all(ASSET_DIR)?;
        #[cfg(not(target_arch = "wasm32"))]
        match ASSETS.extract(ASSET_DIR) {
            Err(e) if e.kind() != io::ErrorKind::AlreadyExists => return Err(e.into()),
            _ => (),
        }

        let renderer = Renderer::new(&self.settings)?;
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
                }

                let now = Instant::now();
                let time_since_last = (now - self.last_frame_time).as_millis() as f64;
                self.frame_timer += time_since_last;
                let target_delta_time = state
                    .env
                    .target_frame_rate
                    .map(|rate| 1000.0 / rate)
                    .unwrap_or(0.0);

                if state.settings.paused || time_since_last >= target_delta_time {
                    state.env.frame_rate = 1000.0 / time_since_last;
                    state.env.delta_time = (now - self.last_frame_time).as_secs_f64();
                    self.last_frame_time = now;

                    if !state.settings.paused {
                        app.on_update(&mut state)?;
                        state.renderer.present();
                    }
                }

                if state.settings.show_frame_rate && self.frame_timer >= 1000.0 {
                    self.frame_timer -= 1000.0;
                    let title = format!("{} - FPS: {:#.2}", state.title(), state.env.frame_rate);
                    state.renderer.set_title(&title)?;
                }
            }

            app.on_stop(&mut state)?;
            if state.env.quit {
                break 'on_stop;
            }
        }
        Ok(())
    }

    /// Handle user and system events.
    fn handle_events<A>(&mut self, state: &mut PixState, app: &mut A) -> PixResult<()>
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
                    state.mouse_pos = point!(x, y);
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
}

impl Default for PixEngine {
    fn default() -> Self {
        PixEngine::builder().build()
    }
}
