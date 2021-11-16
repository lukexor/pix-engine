//! [PixEngine] functions.
//!
//! This is the core module of the `pix-engine` crate and is responsible for building and running
//! any application using it.
//!
//! [PixEngineBuilder] allows you to customize various engine features and, once built, can
//! [run][PixEngine::run] your application which must implement [AppState::on_update].
//!
//!
//!
//! # Example
//!
//! ```rust no_run
//! use pix_engine::prelude::*;
//!
//! struct MyApp;
//!
//! impl AppState for MyApp {
//!     fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!         // Update every frame
//!         Ok(())
//!     }
//! }
//!
//! fn main() -> PixResult<()> {
//!     let mut engine = PixEngine::builder()
//!       .with_dimensions(800, 600)
//!       .with_title("MyApp")
//!       .build()?;
//!     let mut app = MyApp;
//!     engine.run(&mut app)
//! }
//! ```

use crate::{prelude::*, renderer::*};
use std::time::{Duration, Instant};

#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

/// Builds a [PixEngine] instance by providing several configration functions.
///
/// # Example
///
/// ```no_run
/// # use pix_engine::prelude::*;
/// # struct MyApp;
/// # impl AppState for MyApp {
/// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
/// # }
/// fn main() -> PixResult<()> {
///     let mut engine = PixEngine::builder()
///         .with_title("My App")
///         .position(10, 10)
///         .resizable()
///         .with_frame_rate()
///         .icon("myapp.png")
///         .build()?;
///     let mut app = MyApp;
///     engine.run(&mut app)
/// }
/// ```
#[must_use]
#[derive(Default, Debug)]
pub struct PixEngineBuilder {
    settings: RendererSettings,
    theme: Theme,
}

impl PixEngineBuilder {
    /// Constructs a `PixEngineBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a window title.
    pub fn with_title<S>(&mut self, title: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.settings.title = title.into();
        self
    }

    /// Set font for text rendering.
    pub fn with_font(&mut self, font: Font, size: u32) -> &mut Self {
        self.theme.fonts.body = font;
        self.theme.font_sizes.body = size;
        self
    }

    /// Set theme for UI rendering.
    pub fn with_theme(&mut self, theme: Theme) -> &mut Self {
        self.theme = theme;
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

    /// Position the window at the given `(x, y)` coordinates of the display.
    pub fn position(&mut self, x: i32, y: i32) -> &mut Self {
        self.settings.x = Position::Positioned(x);
        self.settings.y = Position::Positioned(y);
        self
    }

    /// Position the window in the center of the display. This is the default.
    pub fn position_centered(&mut self) -> &mut Self {
        self.settings.x = Position::Centered;
        self.settings.y = Position::Centered;
        self
    }

    /// Set window dimensions.
    pub fn with_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.settings.width = width;
        self.settings.height = height;
        self
    }

    /// Scales the window.
    pub fn scale(&mut self, x: f32, y: f32) -> &mut Self {
        self.settings.scale_x = x;
        self.settings.scale_y = y;
        self
    }

    /// Set audio sample rate.
    pub fn audio_sample_rate(&mut self, sample_rate: i32) -> &mut Self {
        self.settings.audio_sample_rate = sample_rate;
        self
    }

    /// Start window in fullscreen mode.
    pub fn fullscreen(&mut self) -> &mut Self {
        self.settings.fullscreen = true;
        self
    }

    /// Enable VSync.
    pub fn vsync_enabled(&mut self) -> &mut Self {
        self.settings.vsync = true;
        self
    }

    /// Allow window resizing.
    pub fn resizable(&mut self) -> &mut Self {
        self.settings.resizable = true;
        self
    }

    /// Removes the window decoration.
    pub fn borderless(&mut self) -> &mut Self {
        self.settings.borderless = true;
        self
    }

    /// Enables high-DPI on displays that support it.
    pub fn allow_highdpi(&mut self) -> &mut Self {
        self.settings.allow_highdpi = true;
        self
    }

    /// Starts engine with window hidden.
    pub fn hidden(&mut self) -> &mut Self {
        self.settings.hidden = true;
        self
    }

    /// Enable average frame rate (FPS) in title.
    pub fn with_frame_rate(&mut self) -> &mut Self {
        self.settings.show_frame_rate = true;
        self
    }

    /// Set a target frame rate to render at, controls how often
    /// [on_update](crate::prelude::AppState::on_update) is called.
    pub fn target_frame_rate(&mut self, rate: usize) -> &mut Self {
        self.settings.target_frame_rate = Some(rate);
        self
    }

    /// Set a custom texture cache size other than the default of `20`.
    /// Affects font family and image rendering caching operations.
    pub fn with_texture_cache(&mut self, size: usize) -> &mut Self {
        self.settings.texture_cache_size = size;
        self
    }

    /// Set a custom text cache size other than the default of `500`.
    /// Affects text rendering caching operations.
    pub fn with_text_cache(&mut self, size: usize) -> &mut Self {
        self.settings.text_cache_size = size;
        self
    }

    /// Convert [PixEngineBuilder] to a [PixEngine] instance.
    pub fn build(&self) -> PixResult<PixEngine> {
        Ok(PixEngine {
            state: PixState::new(self.settings.clone(), self.theme.clone())?,
        })
    }
}

/// The core engine that maintains the render loop, state, drawing functions, event handling, etc.
#[must_use]
#[derive(Debug)]
pub struct PixEngine {
    state: PixState,
}

impl PixEngine {
    /// Constructs a default [PixEngineBuilder] which can build a `PixEngine` instance.
    ///
    /// See [PixEngineBuilder] for examples.
    pub fn builder() -> PixEngineBuilder {
        PixEngineBuilder::default()
    }

    /// Starts the `PixEngine` application and begins executing the frame loop on a given
    /// application which must implement [AppState]. The only required method of which is
    /// [AppState::on_update].
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use pix_engine::prelude::*;
    /// # struct MyApp;
    /// # impl MyApp { fn new() -> Self { Self } }
    /// # impl AppState for MyApp {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// # }
    /// fn main() -> PixResult<()> {
    ///     let mut engine = PixEngine::builder().build()?;
    ///     let mut app = MyApp::new(); // MyApp implements `AppState`
    ///     engine.run(&mut app)
    /// }
    /// ```
    pub fn run<A>(&mut self, app: &mut A) -> PixResult<()>
    where
        A: AppState,
    {
        // Handle events before on_start to initialize window
        self.handle_events(app)?;

        self.state.clear()?;
        app.on_start(&mut self.state)?;
        if self.state.should_quit() {
            return Ok(());
        }
        self.state.present();

        // on_stop loop enables on_stop to prevent application close if necessary
        'on_stop: loop {
            // running loop continues until an event or on_update returns false or errors
            'running: loop {
                self.handle_events(app)?;
                if self.state.should_quit() {
                    break 'running;
                }

                let now = Instant::now();
                let time_since_last = now - self.state.last_frame_time();
                let target_delta_time = self.state.target_delta_time();
                if time_since_last >= target_delta_time {
                    self.state.set_delta_time(now, time_since_last);
                    if self.state.is_running() {
                        self.state.clear()?;
                        self.state.pre_update();
                        app.on_update(&mut self.state)?;
                        self.state.on_update()?;
                        self.state.post_update();
                        self.state.present();
                        self.state.increment_frame(now, time_since_last)?;
                    }
                }
            }

            app.on_stop(&mut self.state)?;
            if self.state.should_quit() {
                break 'on_stop;
            }
        }
        Ok(())
    }
}

impl PixEngine {
    /// Handle user and system events.
    #[inline]
    fn handle_events<A>(&mut self, app: &mut A) -> PixResult<()>
    where
        A: AppState,
    {
        let state = &mut self.state;
        while let Some(event) = state.renderer.poll_event() {
            app.on_event(state, &event)?;
            match event {
                Event::Quit { .. } | Event::AppTerminating { .. } => state.quit(),
                Event::Window {
                    window_id,
                    win_event,
                } => {
                    let id = window_id as WindowId;
                    app.on_window_event(state, id, win_event)?;
                    match win_event {
                        WindowEvent::FocusGained => state.focus_window(Some(id)),
                        WindowEvent::FocusLost => state.focus_window(None),
                        WindowEvent::Close => state.close_window(id)?,
                        _ => (),
                    }
                }
                Event::KeyDown {
                    key: Some(key),
                    keymod,
                    repeat,
                } => {
                    let evt = KeyEvent::new(key, keymod, true, repeat);
                    if !app.on_key_pressed(state, evt)? {
                        state.ui.keys.press(key, keymod);
                    }
                }
                Event::KeyUp {
                    key: Some(key),
                    keymod,
                    repeat,
                } => {
                    let evt = KeyEvent::new(key, keymod, false, repeat);
                    if !app.on_key_released(state, evt)? {
                        state.ui.keys.release(key, keymod);
                    }
                }
                Event::TextInput { text, .. } => {
                    if !app.on_key_typed(state, &text)? {
                        state.ui.keys.typed(text);
                    }
                }
                Event::MouseMotion { x, y, xrel, yrel } => {
                    let pos = point!(x, y);
                    let rel_pos = point!(xrel, yrel);
                    if state.ui.mouse.is_pressed() {
                        app.on_mouse_dragged(state, pos, rel_pos)?;
                    }
                    if !app.on_mouse_motion(state, pos, rel_pos)? {
                        state.on_mouse_motion(pos);
                    }
                }
                Event::MouseDown { button, x, y } => {
                    if !app.on_mouse_pressed(state, button, point!(x, y))? {
                        state.on_mouse_pressed(button);
                    }
                }
                Event::MouseUp { button, x, y } => {
                    if state.ui.mouse.is_down(button) {
                        let now = Instant::now();
                        if let Some(clicked) = state.ui.mouse.last_clicked(button) {
                            if now - *clicked < Duration::from_millis(500) {
                                app.on_mouse_dbl_clicked(state, button, point!(x, y))?;
                            }
                        }
                        if !app.on_mouse_clicked(state, button, point!(x, y))? {
                            state.on_mouse_click(button, now);
                        }
                    }
                    if !app.on_mouse_released(state, button, point!(x, y))? {
                        state.on_mouse_released(button);
                    }
                }
                Event::MouseWheel { x, y, .. } => {
                    if !app.on_mouse_wheel(state, point!(x, y))? {
                        state.on_mouse_wheel(x, y);
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }
}
