//! Environment methods for the [`PixEngine`].
//!
//! Methods for reading and setting various engine environment values.
//!
//! Provided [`PixState`] methods:
//!
//! - [`PixState::focused`]: Whether the current window target has focus.
//! - [`PixState::delta_time`]: Time elapsed since last frame in milliseconds.
//! - [`PixState::elapsed`]: Time elapsed since application start in milliseconds.
//! - [`PixState::frame_count`]: Total number of frames since application start.
//! - [`PixState::redraw`]: Run render loop 1 time, calling [`AppState::on_update`].
//! - [`PixState::run_times`]: Run render loop N times, calling [`AppState::on_update`].
//! - [`PixState::avg_frame_rate`]: Average frames per second rendered.
//! - [`PixState::quit`]: Trigger application quit.
//! - [`PixState::abort_quit`]: Abort application quit.

use crate::{
    prelude::*,
    renderer::{Rendering, WindowRenderer},
};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

const ONE_SECOND: Duration = Duration::from_secs(1);

/// Environment values for [`PixState`]
#[derive(Debug, Clone)]
pub(crate) struct Environment {
    focused_window: Option<WindowId>,
    delta_time: Scalar,
    start: Instant,
    frame_rate: usize,
    frame_count: usize,
    run_count: usize,
    quit: bool,
    frames: VecDeque<Instant>,
    last_frame_time: Instant,
    frame_timer: Duration,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            focused_window: None,
            delta_time: 0.0,
            start: Instant::now(),
            frame_rate: 0,
            frame_count: 0,
            run_count: 0,
            quit: false,
            frames: VecDeque::with_capacity(128),
            last_frame_time: Instant::now(),
            frame_timer: Duration::from_secs(1),
        }
    }
}

impl PixState {
    /// Returns whether the current window target has focus.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
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
    pub fn focused(&self) -> bool {
        matches!(self.env.focused_window, Some(id) if id == self.renderer.window_id())
    }

    /// The time elapsed since last frame in milliseconds.
    ///
    /// Value can not exceed [`Scalar::MAX`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { position: Scalar, velocity: Scalar };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     // Update position based on frame timestep
    ///     self.position = self.velocity * s.delta_time();
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn delta_time(&self) -> Scalar {
        let delta = self.env.delta_time * 1000.0;
        if delta.is_infinite() {
            Scalar::MAX
        } else {
            delta
        }
    }

    /// The time elapsed since application start in milliseconds.
    ///
    /// Value can not exceed [`Scalar::MAX`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     // Draw a blinking box, indepdendent of frame rate
    ///     if s.elapsed() as usize >> 9 & 1 > 0 {
    ///         s.rect([0, 0, 10, 10])?;
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn elapsed(&self) -> Scalar {
        #[cfg(target_pointer_width = "32")]
        let elapsed = self.env.start.elapsed().as_secs_f32();
        #[cfg(target_pointer_width = "64")]
        let elapsed = self.env.start.elapsed().as_secs_f64();
        let elapsed = elapsed * 1000.0;
        if elapsed.is_infinite() {
            Scalar::MAX
        } else {
            elapsed
        }
    }

    /// The total number of frames rendered since application start.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
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

    /// Run the render loop 1 time by calling [`AppState::on_update`].
    ///
    /// This can be used to only redraw in response to user actions such as
    /// [`AppState::on_mouse_pressed`] or [`AppState::on_key_pressed`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.no_run(); // Disable render loop
    ///     Ok(())
    /// }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
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

    /// Run the render loop N times by calling [`AppState::on_update`].
    ///
    /// This can be used to only redraw in response to user actions such as
    /// [`AppState::on_mouse_pressed`] or [`AppState::on_key_pressed`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.no_run(); // Disable render loop
    ///     Ok(())
    /// }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
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
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.text(format!("FPS: {}", s.avg_frame_rate()))?;
    ///     Ok(())
    /// # }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub const fn avg_frame_rate(&self) -> usize {
        self.env.frame_rate
    }

    /// Trigger application quit.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
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

    /// Abort application quit and resume render loop by calling [`AppState::on_update`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { has_unsaved_changes: bool, prompt_save_dialog: bool }
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_stop(&mut self, s: &mut PixState) -> PixResult<()> {
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
        #[cfg(target_pointer_width = "32")]
        let delta = time_since_last.as_secs_f32();
        #[cfg(target_pointer_width = "64")]
        let delta = time_since_last.as_secs_f64();
        self.env.delta_time = delta;
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
    pub(crate) fn increment_frame(
        &mut self,
        now: Instant,
        time_since_last: Duration,
    ) -> PixResult<()> {
        let s = &self.settings;
        let mut env = &mut self.env;
        if env.run_count > 0 {
            env.run_count -= 1;
        }
        env.frame_count += 1;

        if s.running && s.show_frame_rate {
            let a_second_ago = now - ONE_SECOND;
            while env.frames.front().map_or(false, |&t| t < a_second_ago) {
                env.frames.pop_front();
            }
            env.frames.push_back(now);

            env.frame_timer += time_since_last;
            if env.frame_timer >= ONE_SECOND {
                env.frame_timer -= ONE_SECOND;
                env.frame_rate = env.frames.len();
                self.renderer.set_fps(env.frame_rate)?;
            }
        }
        Ok(())
    }

    /// Present all renderer changes since last frame.
    #[inline]
    pub(crate) fn present(&mut self) {
        self.renderer.present();
    }

    /// Focus a given window.
    pub(crate) fn focus_window(&mut self, id: Option<WindowId>) {
        self.env.focused_window = id;
    }
}
