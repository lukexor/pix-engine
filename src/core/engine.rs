//! [PixEngine] functions.

use crate::{prelude::*, renderer::*};
use std::time::{Duration, Instant};

#[cfg(not(target_arch = "wasm32"))]
use crate::ASSETS;
#[cfg(not(target_arch = "wasm32"))]
use std::{fs, path::PathBuf};

/// Builds a [PixEngine] instance by providing several configration functions.
#[must_use]
#[derive(Default, Debug)]
pub struct PixEngineBuilder {
    settings: RendererSettings,
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
    pub fn with_font<F>(&mut self, font: F, size: u32) -> &mut Self
    where
        F: Into<Font>,
    {
        self.settings.theme.fonts.body = font.into();
        self.settings.theme.font_sizes.body = size;
        self
    }

    /// Set theme for UI rendering.
    pub fn with_theme(&mut self, theme: Theme) -> &mut Self {
        self.settings.theme = theme;
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

    /// Set a custom texture cache size other than the default of 20.
    /// Affects font family and image rendering caching operations.
    pub fn with_texture_cache(&mut self, size: usize) -> &mut Self {
        self.settings.texture_cache_size = size;
        self
    }

    /// Set a custom text cache size other than the default of 500.
    /// Affects text rendering caching operations.
    pub fn with_text_cache(&mut self, size: usize) -> &mut Self {
        self.settings.text_cache_size = size;
        self
    }

    /// Convert [PixEngineBuilder] to a [PixEngine] instance.
    pub fn build(&self) -> PixEngine {
        PixEngine {
            settings: self.settings.clone(),
        }
    }
}

/// The core engine that maintains the render loop, state, drawing functions, event handling, etc.
#[must_use]
#[derive(Debug)]
pub struct PixEngine {
    settings: RendererSettings,
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
            if self.settings.asset_dir.exists() {
                fs::remove_dir_all(&self.settings.asset_dir)?;
            }
            fs::create_dir_all(&self.settings.asset_dir)?;
            ASSETS.extract(&self.settings.asset_dir)?;
        }

        let renderer = Renderer::new(self.settings.clone())?;
        let mut state = PixState::new(renderer, self.settings.theme.clone());
        state.show_frame_rate(self.settings.show_frame_rate);
        if let Some(frame_rate) = self.settings.target_frame_rate {
            state.set_frame_rate(frame_rate);
        }

        // Handle events before on_start to initialize window
        self.handle_events(&mut state, app)?;
        app.on_start(&mut state)?;
        if state.should_quit() {
            return Ok(());
        }

        // on_stop loop enables on_stop to prevent application close if necessary
        'on_stop: loop {
            // running loop continues until an event or on_update returns false or errors
            'running: loop {
                self.handle_events(&mut state, app)?;
                if state.should_quit() {
                    break 'running;
                }

                let now = Instant::now();
                let time_since_last = now - state.last_frame_time();
                let target_delta_time = state.target_delta_time();
                if time_since_last >= target_delta_time {
                    state.set_delta_time(now, time_since_last);
                    if state.is_running() {
                        state.clear()?;
                        state.pre_update();
                        app.on_update(&mut state)?;
                        state.on_update()?;
                        state.post_update();
                        state.present();
                        state.increment_frame(now, time_since_last)?;
                    }
                }
            }

            app.on_stop(&mut state)?;
            if state.should_quit() {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    fs::remove_dir_all(&self.settings.asset_dir)?;
                }
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
                } => {
                    let id = window_id as WindowId;
                    match win_event {
                        WindowEvent::FocusGained => state.focus_window(Some(id)),
                        WindowEvent::FocusLost => state.focus_window(None),
                        WindowEvent::Close => state.close_window(id)?,
                        _ => (),
                    }
                    app.on_window_event(state, id, win_event)?;
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
                Event::TextInput { ref text, .. } => {
                    if !app.on_key_typed(state, text)? {
                        state.ui.keys.typed(text.clone());
                    }
                }
                Event::MouseMotion { x, y, xrel, yrel } => {
                    state.ui.set_mouse_pos([x, y]);
                    if state.ui.mouse.is_pressed() {
                        app.on_mouse_dragged(state)?;
                    }
                    app.on_mouse_motion(state, state.mouse_pos(), xrel, yrel)?;
                }
                Event::MouseDown { button, x, y } => {
                    if !app.on_mouse_pressed(state, button, point!(x, y))? {
                        state.ui.mouse.press(button);
                    }
                }
                Event::MouseUp { button, x, y } => {
                    if state.ui.mouse.is_down(button) {
                        let now = Instant::now();
                        if let Some(&clicked) = state.ui.mouse.last_clicked(button) {
                            if now - clicked < Duration::from_millis(500) {
                                app.on_mouse_dbl_clicked(state, button, point!(x, y))?;
                            }
                        }
                        if !app.on_mouse_clicked(state, button, point!(x, y))? {
                            state.ui.mouse.click(button, now);
                        }
                    }
                    if !app.on_mouse_released(state, button, point!(x, y))? {
                        state.ui.mouse.release(button);
                    }
                }
                Event::MouseWheel { x, y, .. } => {
                    if !app.on_mouse_wheel(state, point!(x, y))? {
                        state.ui.mouse.wheel(x, y);
                    }
                }
                _ => (),
            }
            app.on_event(state, event)?;
        }
        Ok(())
    }
}

impl Default for PixEngine {
    fn default() -> Self {
        PixEngine::builder().build()
    }
}
