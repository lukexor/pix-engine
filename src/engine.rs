//! [`PixEngine`] functions.
//!
//! This is the core module of the `pix-engine` crate and is responsible for building and running
//! any application using it.
//!
//! [`PixEngineBuilder`] allows you to customize various engine features and, once built, can
//! [run][`PixEngine::run`] your application which must implement [`AppState::on_update`].
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
//!     fn on_update(&mut self, s: &mut PixState) -> Result<()> {
//!         // Update every frame
//!         Ok(())
//!     }
//! }
//!
//! fn main() -> Result<()> {
//!     let mut engine = PixEngine::builder()
//!       .with_dimensions(800, 600)
//!       .with_title("MyApp")
//!       .build()?;
//!     let mut app = MyApp;
//!     engine.run(&mut app)
//! }
//! ```

use crate::{image::Icon, prelude::*, renderer::RendererSettings};
use log::{debug, error, info};
use std::{
    thread,
    time::{Duration, Instant},
};

/// Builds a [`PixEngine`] instance by providing several configration functions.
///
/// # Example
///
/// ```no_run
/// # use pix_engine::prelude::*;
/// # struct MyApp;
/// # impl AppState for MyApp {
/// # fn on_update(&mut self, s: &mut PixState) -> Result<()> { Ok(()) }
/// # }
/// fn main() -> Result<()> {
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
#[derive(Debug)]
pub struct PixEngineBuilder {
    settings: RendererSettings,
    theme: Theme,
    joystick_deadzone: i32,
}

impl Default for PixEngineBuilder {
    fn default() -> Self {
        Self {
            settings: RendererSettings::default(),
            theme: Theme::default(),
            joystick_deadzone: 8000,
        }
    }
}

impl PixEngineBuilder {
    /// Constructs a `PixEngineBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a window title.
    pub fn with_title<S>(mut self, title: S) -> Self
    where
        S: Into<String>,
    {
        self.settings.title = title.into();
        self
    }

    /// Set font for text rendering.
    pub fn with_font(mut self, font: Font) -> Self {
        self.theme.fonts.body = font;
        self
    }

    /// Set font size for text rendering.
    pub fn with_font_size(mut self, size: u32) -> Self {
        self.theme.font_size = size;
        self
    }

    /// Set theme for UI rendering.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Set a window icon.
    pub fn icon<I>(mut self, icon: I) -> Self
    where
        I: Into<Icon>,
    {
        self.settings.icon = Some(icon.into());
        self
    }

    /// Position the window at the given `(x, y)` coordinates of the display.
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.settings.x = Position::Positioned(x);
        self.settings.y = Position::Positioned(y);
        self
    }

    /// Position the window in the center of the display. This is the default.
    pub fn position_centered(mut self) -> Self {
        self.settings.x = Position::Centered;
        self.settings.y = Position::Centered;
        self
    }

    /// Set window dimensions.
    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.settings.width = width;
        self.settings.height = height;
        self
    }

    /// Set the rendering scale of the current canvas. Drawing coordinates are scaled by x/y
    /// factors before being drawn to the canvas.
    pub fn scale(mut self, x: f32, y: f32) -> Self {
        self.settings.scale_x = x;
        self.settings.scale_y = y;
        self
    }

    /// Set audio sample rate in Hz (samples per second). Defaults to device fallback sample rate.
    pub fn audio_sample_rate(mut self, sample_rate: i32) -> Self {
        self.settings.audio_sample_rate = Some(sample_rate);
        self
    }

    /// Set number of audio channels (1 for Mono, 2 for Stereo, etc). Defaults to device fallback
    /// number of channels.
    pub fn audio_channels(mut self, channels: u8) -> Self {
        self.settings.audio_channels = Some(channels);
        self
    }

    /// Set audio buffer size in samples. Defaults to device fallback sample size.
    pub fn audio_buffer_size(mut self, buffer_size: u16) -> Self {
        self.settings.audio_buffer_size = Some(buffer_size);
        self
    }

    /// Start window in fullscreen mode.
    pub fn fullscreen(mut self) -> Self {
        self.settings.fullscreen = true;
        self
    }

    /// Set the window to synchronize frame rate to the screens refresh rate ([`VSync`]).
    ///
    /// [`VSync`]: https://en.wikipedia.org/wiki/Screen_tearing#Vertical_synchronization
    pub fn vsync_enabled(mut self) -> Self {
        self.settings.vsync = true;
        self
    }

    /// Allow window resizing.
    pub fn resizable(mut self) -> Self {
        self.settings.resizable = true;
        self
    }

    /// Removes the window decoration.
    pub fn borderless(mut self) -> Self {
        self.settings.borderless = true;
        self
    }

    /// Alter the joystick axis deadzone.
    pub fn with_deadzone(mut self, value: i32) -> Self {
        self.joystick_deadzone = value;
        self
    }

    /// Enables high-DPI on displays that support it.
    pub fn allow_highdpi(mut self) -> Self {
        self.settings.allow_highdpi = true;
        self
    }

    /// Starts engine with window hidden.
    pub fn hidden(mut self) -> Self {
        self.settings.hidden = true;
        self
    }

    /// Enable average frame rate (FPS) in title.
    pub fn with_frame_rate(mut self) -> Self {
        self.settings.show_frame_rate = true;
        self
    }

    /// Set a target frame rate to render at, controls how often
    /// [`AppState::on_update`] is called.
    pub fn target_frame_rate(mut self, rate: usize) -> Self {
        self.settings.target_frame_rate = Some(rate);
        self
    }

    /// Set a custom texture cache size other than the default of `20`.
    /// Affects font family and image rendering caching operations.
    pub fn with_texture_cache(mut self, size: usize) -> Self {
        self.settings.texture_cache_size = size;
        self
    }

    /// Set a custom text cache size other than the default of `500`.
    /// Affects text rendering caching operations.
    pub fn with_text_cache(mut self, size: usize) -> Self {
        self.settings.text_cache_size = size;
        self
    }

    /// Convert [PixEngineBuilder] to a [`PixEngine`] instance.
    ///
    /// # Errors
    ///
    /// If the engine fails to create a new renderer, then an error is returned.
    ///
    /// Possible errors include the title containing a `nul` character, the position or dimensions
    /// being invalid values or overlowing and an internal renderer error such as running out of
    /// memory or a software driver issue.
    pub fn build(self) -> Result<PixEngine> {
        Ok(PixEngine {
            state: PixState::new(self.settings.clone(), self.theme.clone())?,
            joystick_deadzone: self.joystick_deadzone,
        })
    }
}

/// The core engine that maintains the render loop, state, drawing functions, event handling, etc.
#[must_use]
#[derive(Debug)]
pub struct PixEngine {
    state: PixState,
    joystick_deadzone: i32,
}

impl PixEngine {
    /// Constructs a default [PixEngineBuilder] which can build a `PixEngine` instance.
    ///
    /// See [PixEngineBuilder] for examples.
    pub fn builder() -> PixEngineBuilder {
        PixEngineBuilder::default()
    }

    /// Starts the `PixEngine` application and begins executing the frame loop on a given
    /// application which must implement [`AppState`]. The only required method of which is
    /// [`AppState::on_update`].
    ///
    /// # Errors
    ///
    /// Any error in the entire library can propagate here and terminate the program. See the
    /// [error](crate::error) module for details. Also see [`AppState::on_stop`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use pix_engine::prelude::*;
    /// # struct MyApp;
    /// # impl MyApp { fn new() -> Self { Self } }
    /// # impl AppState for MyApp {
    /// # fn on_update(&mut self, s: &mut PixState) -> Result<()> { Ok(()) }
    /// # }
    /// fn main() -> Result<()> {
    ///     let mut engine = PixEngine::builder().build()?;
    ///     let mut app = MyApp::new(); // MyApp implements `AppState`
    ///     engine.run(&mut app)
    /// }
    /// ```
    pub fn run<A>(&mut self, app: &mut A) -> Result<()>
    where
        A: AppState,
    {
        info!("Starting `PixEngine`...");

        // Handle events before on_start to initialize window
        self.handle_events(app)?;

        debug!("Starting with `AppState::on_start`");
        self.state.clear()?;
        let on_start = app.on_start(&mut self.state);
        if on_start.is_err() || self.state.should_quit() {
            debug!("Quitting during startup with `AppState::on_stop`");
            if let Err(ref err) = on_start {
                error!("Error: {}", err);
            }
            return app.on_stop(&mut self.state).and(on_start);
        }
        self.state.present();

        // on_stop loop enables on_stop to prevent application close if necessary
        'on_stop: loop {
            debug!("Starting `AppState::on_update` loop.");
            // running loop continues until an event or on_update returns false or errors
            let result = 'running: loop {
                let start_time = Instant::now();
                let time_since_last = start_time - self.state.last_frame_time();

                self.handle_events(app)?;
                if self.state.should_quit() {
                    break 'running Ok(());
                }

                if self.state.is_running() {
                    self.state.pre_update();
                    let on_update = app.on_update(&mut self.state);
                    if on_update.is_err() {
                        self.state.quit();
                        break 'running on_update;
                    }
                    self.state.on_update()?;
                    self.state.post_update();
                    self.state.present();
                    self.state.set_delta_time(start_time, time_since_last);
                    self.state.increment_frame(time_since_last)?;
                }

                if !self.state.vsync_enabled() {
                    if let Some(target_delta_time) = self.state.target_delta_time() {
                        let time_to_next_frame = start_time + target_delta_time;
                        let now = Instant::now();
                        if time_to_next_frame > now {
                            thread::sleep(time_to_next_frame - now);
                        }
                    }
                }
            };

            debug!("Quitting with `AppState::on_stop`");
            let on_stop = app.on_stop(&mut self.state);
            if self.state.should_quit() {
                info!("Quitting `PixEngine`...");
                break 'on_stop on_stop.and(result);
            }
        }
    }
}

impl PixEngine {
    /// Handle user and system events.
    #[inline]
    fn handle_events<A>(&mut self, app: &mut A) -> Result<()>
    where
        A: AppState,
    {
        let state = &mut self.state;
        while let Some(event) = state.poll_event() {
            if let Event::ControllerAxisMotion { .. }
            | Event::JoyAxisMotion { .. }
            | Event::MouseMotion { .. }
            | Event::MouseWheel { .. }
            | Event::KeyDown { repeat: true, .. }
            | Event::KeyUp { repeat: true, .. } = event
            {
                // Ignore noisy events
            } else {
                debug!("Polling event {:?}", event);
            }
            app.on_event(state, &event)?;
            match event {
                Event::Quit { .. } | Event::AppTerminating { .. } => state.quit(),
                Event::Window {
                    window_id,
                    win_event,
                } => {
                    let window_id = WindowId(window_id);
                    match win_event {
                        WindowEvent::FocusGained => state.focus_window(Some(window_id)),
                        WindowEvent::FocusLost => state.focus_window(None),
                        WindowEvent::Close => state.close_window(window_id)?,
                        _ => (),
                    }
                    app.on_window_event(state, window_id, win_event)?;
                }
                Event::KeyDown {
                    key: Some(key),
                    keymod,
                    repeat,
                } => {
                    let evt = KeyEvent::new(key, keymod, repeat);
                    if !app.on_key_pressed(state, evt)? {
                        state.ui.keys.press(key, keymod);
                    }
                }
                Event::KeyUp {
                    key: Some(key),
                    keymod,
                    repeat,
                } => {
                    let evt = KeyEvent::new(key, keymod, repeat);
                    if !app.on_key_released(state, evt)? {
                        state.ui.keys.release(key, keymod);
                    }
                }
                Event::ControllerDown {
                    controller_id,
                    button,
                } => {
                    let evt = ControllerEvent::new(controller_id, button);
                    app.on_controller_pressed(state, evt)?;
                }
                Event::ControllerUp {
                    controller_id,
                    button,
                } => {
                    let evt = ControllerEvent::new(controller_id, button);
                    app.on_controller_released(state, evt)?;
                }
                Event::ControllerAxisMotion {
                    controller_id,
                    axis,
                    value,
                } => {
                    let value = i32::from(value);
                    let value =
                        if (-self.joystick_deadzone..self.joystick_deadzone).contains(&value) {
                            0
                        } else {
                            value
                        };
                    let id = ControllerId(controller_id);
                    app.on_controller_axis_motion(state, id, axis, value)?;
                }
                Event::ControllerAdded { controller_id } => {
                    let id = ControllerId(controller_id);
                    if !app.on_controller_update(state, id, ControllerUpdate::Added)? {
                        state.open_controller(id)?;
                    }
                }
                Event::JoyDeviceAdded { joy_id } => {
                    let id = ControllerId(joy_id);
                    if !app.on_controller_update(state, id, ControllerUpdate::Added)? {
                        state.open_controller(id)?;
                    }
                }
                Event::ControllerRemoved { controller_id } => {
                    let id = ControllerId(controller_id);
                    if !app.on_controller_update(state, id, ControllerUpdate::Removed)? {
                        state.close_controller(id);
                    }
                }
                Event::JoyDeviceRemoved { joy_id } => {
                    let id = ControllerId(joy_id);
                    if !app.on_controller_update(state, id, ControllerUpdate::Removed)? {
                        state.open_controller(id)?;
                    }
                }
                Event::ControllerRemapped { controller_id } => {
                    let id = ControllerId(controller_id);
                    app.on_controller_update(state, id, ControllerUpdate::Remapped)?;
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
                            if now - *clicked < Duration::from_millis(500)
                                && !app.on_mouse_dbl_clicked(state, button, point!(x, y))?
                            {
                                state.on_mouse_dbl_click(button, now);
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
