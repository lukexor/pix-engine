//! Environment methods for the [`Engine`].
//!
//! Methods for reading and setting various engine environment values.
//!
//! Provided [`PixState`] methods:
//!
//! - [`PixState::focused`]: Whether the current window target has focus.
//! - [`PixState::delta_time`]: [Duration] elapsed since last frame.
//! - [`PixState::elapsed`]: [Duration] elapsed since application start.
//! - [`PixState::frame_count`]: Total number of frames since application start.
//! - [`PixState::redraw`]: Run render loop 1 time, calling [`PixEngine::on_update`].
//! - [`PixState::run_times`]: Run render loop N times, calling [`PixEngine::on_update`].
//! - [`PixState::avg_frame_rate`]: Average frames per second rendered.
//! - [`PixState::quit`]: Trigger application quit.
//! - [`PixState::abort_quit`]: Abort application quit.
//! - [`PixState::day`]: Return the current day between 1-31.
//! - [`PixState::month`]: Return the current month between 1-12.
//! - [`PixState::year`]: Return the current year as an integer.
//! - [`PixState::hour`]: Return the current hour between 0-23.
//! - [`PixState::minute`]: Return the current minute between 0-59.
//! - [`PixState::second`]: Return the current second between 0-59.

use crate::{
    prelude::*,
    renderer::{Rendering, WindowRenderer},
};
use chrono::prelude::*;
use std::time::{Duration, Instant};

const ONE_SECOND: Duration = Duration::from_secs(1);

/// Environment values for [`PixState`]
#[derive(Debug, Clone)]
pub(crate) struct Environment {
    focused_window: Option<WindowId>,
    delta_time: Duration,
    start: Instant,
    frame_rate: f32,
    frame_count: usize,
    run_count: usize,
    quit: bool,
    last_frame_time: Instant,
    frame_timer: Duration,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            focused_window: None,
            delta_time: Duration::default(),
            start: Instant::now(),
            frame_rate: 0.0,
            frame_count: 0,
            run_count: 0,
            quit: false,
            last_frame_time: Instant::now(),
            frame_timer: Duration::default(),
        }
    }
}

impl PixState {
    /// Present all renderer changes since last frame.
    #[inline]
    pub fn present(&mut self) {
        self.renderer.present();
    }

    /// Returns whether any active window has focus. To check focus of a specific window, see
    /// [`PixState::focused_window`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     if s.focused() {
    ///         // Update screen only when focused
    ///         s.rect([0, 0, 100, 100])?;
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub const fn focused(&self) -> bool {
        self.env.focused_window.is_some()
    }

    /// Returns whether a given window has focus. To check focus of a any window, see
    /// [`PixState::focused`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     if s.focused_window(s.window_id()) {
    ///         // Update screen only when focused
    ///         s.rect([0, 0, 100, 100])?;
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn focused_window(&self, window_id: WindowId) -> bool {
        matches!(self.env.focused_window, Some(id) if id == window_id)
    }

    /// The [Duration] elapsed since last frame.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { position: f64, velocity: f64 };
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     // Update position based on frame timestep
    ///     self.position = self.velocity * s.delta_time().as_secs_f64();
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub const fn delta_time(&self) -> Duration {
        self.env.delta_time
    }

    /// The [Duration[ elapsed since application start.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     // Draw a blinking box, indepdendent of frame rate
    ///     if s.elapsed().as_millis() >> 9 & 1 > 0 {
    ///         s.rect([0, 0, 10, 10])?;
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.env.start.elapsed()
    }

    /// The total number of frames rendered since application start.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     // Create a strobe effect, dependent on frame rate
    ///     if s.frame_count() % 5 == 0 {
    ///         s.rect([0, 0, 10, 10])?;
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub const fn frame_count(&self) -> usize {
        self.env.frame_count
    }

    /// Run the render loop 1 time by calling [`PixEngine::on_update`].
    ///
    /// This can be used to only redraw in response to user actions such as
    /// [`PixEngine::on_mouse_pressed`] or [`PixEngine::on_key_pressed`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> Result<()> { Ok(()) }
    /// fn on_start(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.run(false); // Disable render loop
    ///     Ok(())
    /// }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> Result<bool> {
    ///     // Step one frame draw at a time on each space press
    ///     // Useful for debugging frame-by-frame
    ///     if let Key::Space = event.key {
    ///         s.redraw();
    ///         return Ok(true);
    ///     }
    ///     Ok(false)
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn redraw(&mut self) {
        self.env.run_count = 1;
    }

    /// Run the render loop N times by calling [`PixEngine::on_update`].
    ///
    /// This can be used to only redraw in response to user actions such as
    /// [`PixEngine::on_mouse_pressed`] or [`PixEngine::on_key_pressed`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> Result<()> { Ok(()) }
    /// fn on_start(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.run(false); // Disable render loop
    ///     Ok(())
    /// }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> Result<bool> {
    ///     // Step one frame draw at a time on each space press
    ///     // Useful for debugging by multiple frames at a time
    ///     if let Key::Space = event.key {
    ///         s.run_times(4);
    ///         return Ok(true);
    ///     }
    ///     Ok(false)
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn run_times(&mut self, n: usize) {
        self.env.run_count = n;
    }

    /// The average frames per second rendered.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.text(format!("FPS: {}", s.avg_frame_rate()))?;
    ///     Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub const fn avg_frame_rate(&self) -> f32 {
        self.env.frame_rate
    }

    /// Trigger application quit.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     if s.button("Quit?")? {
    ///         s.quit();
    ///     }
    ///     Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn quit(&mut self) {
        self.env.quit = true;
    }

    /// Abort application quit and resume render loop by calling [`PixEngine::on_update`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { has_unsaved_changes: bool, prompt_save_dialog: bool }
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> Result<()> { Ok(()) }
    /// fn on_stop(&mut self, s: &mut PixState) -> Result<()> {
    ///     if self.has_unsaved_changes {
    ///         self.prompt_save_dialog = true;
    ///         s.abort_quit();
    ///     }
    ///     Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    pub fn abort_quit(&mut self) {
        self.env.quit = false;
    }

    /// Return the current day between 1-31.
    #[inline]
    #[must_use]
    pub fn day() -> u32 {
        Local::now().day()
    }

    /// Return the current month between 1-12.
    #[inline]
    #[must_use]
    pub fn month() -> u32 {
        Local::now().month()
    }

    /// Return the current year as an integer.
    #[inline]
    #[must_use]
    pub fn year() -> i32 {
        Local::now().year()
    }

    /// Return the current hour between 0-23.
    #[inline]
    #[must_use]
    pub fn hour() -> u32 {
        Local::now().hour()
    }

    /// Return the current minute between 0-59.
    #[inline]
    #[must_use]
    pub fn minute() -> u32 {
        Local::now().minute()
    }

    /// Return the current second between 0-59.
    #[inline]
    #[must_use]
    pub fn second() -> u32 {
        Local::now().second()
    }
}

impl PixState {
    /// Return the instant the last frame was rendered at.
    #[inline]
    pub(crate) const fn last_frame_time(&self) -> Instant {
        self.env.last_frame_time
    }

    /// Set the delta time since last frame.
    #[inline]
    pub(crate) fn set_delta_time(&mut self, now: Instant, time_since_last: Duration) {
        self.env.delta_time = time_since_last;
        self.env.last_frame_time = now;
    }

    /// Whether the current render loop should be running or not.
    #[inline]
    pub(crate) const fn is_running(&self) -> bool {
        self.settings.running || self.env.run_count > 0
    }

    /// Whether the render loop should quit and terminate the application.
    #[inline]
    pub(crate) const fn should_quit(&self) -> bool {
        self.env.quit
    }

    /// Increment the internal frame counter. If the `show_frame_rate` option is set, update the
    /// title at most once every second.
    #[inline]
    pub(crate) fn increment_frame(&mut self, time_since_last: Duration) -> Result<()> {
        let s = &self.settings;
        let mut env = &mut self.env;

        if env.run_count > 0 {
            env.run_count -= 1;
        }
        env.frame_count += 1;

        if s.running && s.show_frame_rate {
            env.frame_timer += time_since_last;
            if env.frame_timer >= ONE_SECOND {
                env.frame_rate = env.frame_count as f32 / env.frame_timer.as_secs_f32();
                env.frame_timer -= ONE_SECOND;
                env.frame_count = 0;
                self.renderer.set_fps(env.frame_rate)?;
            }
        }

        Ok(())
    }

    /// Focus a given window.
    pub(crate) fn focus_window(&mut self, id: Option<WindowId>) {
        self.env.focused_window = id;
    }
}
