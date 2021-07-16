//! [PixEngine] functions.

use crate::{
    prelude::*,
    renderer::{Renderer, RendererSettings, Rendering},
    window::Window,
};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

#[cfg(not(target_arch = "wasm32"))]
use crate::ASSETS;
#[cfg(not(target_arch = "wasm32"))]
use std::{fs, io, path::PathBuf};

const ONE_SECOND: Duration = Duration::from_secs(1);

/// Builds a [PixEngine] instance by providing several configration functions.
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

    /// Position the window at the given `(x, y)` coordinates of the display.
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

    /// Set a window icon.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn icon<P>(&mut self, path: P) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.settings.icon = Some(path.into());
        self
    }

    /// Set the temporary directory for extraction of static library assets.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn asset_dir<P>(&mut self, path: P) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.settings.asset_dir = path.into();
        self
    }

    /// Convert [PixEngineBuilder] to a [PixEngine] instance.
    pub fn build(&self) -> PixEngine {
        PixEngine {
            settings: self.settings.clone(),
            frames: VecDeque::with_capacity(128),
            last_frame_time: Instant::now(),
            frame_timer: Duration::from_secs(1),
        }
    }
}

/// The core engine that maintains the render loop, state, drawing functions, event handling, etc.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct PixEngine {
    settings: RendererSettings,
    frames: VecDeque<Instant>,
    last_frame_time: Instant,
    frame_timer: Duration,
}

impl PixEngine {
    /// Constructs a default [PixEngineBuilder] which can build a `PixEngine` instance.
    pub fn builder() -> PixEngineBuilder {
        PixEngineBuilder::default()
    }

    /// Starts the `PixEngine` application and begins executing the frame loop.
    pub fn run<A>(&mut self, app: &mut A) -> PixResult<()>
    where
        A: AppState,
    {
        #[cfg(not(target_arch = "wasm32"))]
        {
            fs::create_dir_all(&self.settings.asset_dir)?;
            match ASSETS.extract(&self.settings.asset_dir) {
                Err(e) if e.kind() != io::ErrorKind::AlreadyExists => return Err(e.into()),
                _ => (),
            }
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
                let time_since_last = now - self.last_frame_time;

                // Target frame rate
                let target_delta_time = state
                    .env
                    .target_frame_rate
                    .map(|rate| 1000.0 / rate)
                    .unwrap_or(0.0);

                if time_since_last.as_millis() as f64 >= target_delta_time {
                    state.env.delta_time = time_since_last.as_secs_f64();
                    self.last_frame_time = now;

                    if state.settings.running || state.settings.run_count > 0 {
                        app.on_update(&mut state)?;
                        if state.settings.run_count > 0 {
                            state.settings.run_count -= 1;
                        }
                        state.renderer.present();
                    }
                }

                if state.settings.running && state.settings.show_frame_rate {
                    let a_second_ago = now - ONE_SECOND;
                    while self.frames.front().map_or(false, |&t| t < a_second_ago) {
                        self.frames.pop_front();
                    }
                    self.frames.push_back(now);

                    self.frame_timer += time_since_last;
                    if self.frame_timer >= ONE_SECOND {
                        self.frame_timer -= ONE_SECOND;
                        state.env.frame_rate = self.frames.len();
                        let title = format!("{} - FPS: {}", state.title, state.env.frame_rate);
                        state.renderer.set_title(&title)?;
                    }
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
    #[inline]
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
                    WindowEvent::Resized(width, height)
                    | WindowEvent::SizeChanged(width, height) => {
                        app.on_window_resized(state, width, height)?
                    }
                    _ => (),
                },
                Event::KeyDown {
                    key: Some(key),
                    keymod,
                    repeat,
                } => {
                    state.keys.press(key);
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
                    state.keys.release(&key);
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
                Event::MouseMotion { x, y, xrel, yrel } => {
                    state.pmouse.pos = state.mouse.pos;
                    state.mouse.pos = point!(x, y);
                    if state.mouse.is_pressed() {
                        app.on_mouse_dragged(state)?;
                    }
                    app.on_mouse_motion(state, x, y, xrel, yrel)?;
                }
                Event::MouseDown { button, .. } => {
                    state.mouse.press(button);
                    app.on_mouse_pressed(state, button)?;
                }
                Event::MouseUp { button, .. } => {
                    if state.mouse.is_down(button) {
                        let now = Instant::now();
                        if let Some(&clicked) = state.mouse.last_clicked(&button) {
                            if now - clicked < Duration::from_millis(500) {
                                app.on_mouse_dbl_clicked(state, button)?;
                            }
                        }
                        state.mouse.click(button, now);
                        app.on_mouse_clicked(state, button)?;
                    }
                    state.mouse.release(&button);
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
