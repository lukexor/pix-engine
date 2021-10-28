//! Environment information for the [PixEngine].
//!
//! [PixEngine]: crate::prelude::PixEngine

use crate::{prelude::*, renderer::*};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

const ONE_SECOND: Duration = Duration::from_secs(1);

/// Environment values for [PixState]
#[derive(Debug, Clone)]
pub(crate) struct Environment {
    focused: bool,
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
            focused: false,
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
    /// Whether the application has focus or not.
    pub fn focused(&self) -> bool {
        matches!(self.env.focused_window, Some(id) if id == self.renderer.window_id())
    }

    /// The time elapsed since last frame in seconds.
    #[inline]
    pub fn delta_time(&self) -> Scalar {
        self.env.delta_time
    }

    /// The time elapsed since application start in milliseconds.
    #[inline]
    pub fn elapsed(&self) -> Scalar {
        #[cfg(target_pointer_width = "32")]
        let elapsed = self.env.start.elapsed().as_secs_f32();
        #[cfg(target_pointer_width = "64")]
        let elapsed = self.env.start.elapsed().as_secs_f64();
        1000.0 * elapsed
    }

    /// The total number of frames rendered since application start.
    #[inline]
    pub fn frame_count(&self) -> usize {
        self.env.frame_count
    }

    /// Run the render loop 1 time by calling [AppState::on_update].
    ///
    /// This can be used to only redraw in response to user actions such as
    /// [AppState::on_mouse_pressed] or [AppState::on_key_pressed].
    ///
    /// [AppState::on_update]: crate::prelude::AppState::on_update
    /// [AppState::on_mouse_pressed]: crate::prelude::AppState::on_mouse_pressed
    /// [AppState::on_key_pressed]: crate::prelude::AppState::on_key_pressed
    pub fn redraw(&mut self) {
        self.env.run_count = 1;
    }

    /// Run the render loop N times by calling [AppState::on_update].
    ///
    /// This can be used to only redraw in response to user actions such as
    /// [AppState::on_mouse_pressed] or [AppState::on_key_pressed].
    ///
    /// [AppState::on_update]: crate::prelude::AppState::on_update
    /// [AppState::on_mouse_pressed]: crate::prelude::AppState::on_mouse_pressed
    /// [AppState::on_key_pressed]: crate::prelude::AppState::on_key_pressed
    pub fn run_times(&mut self, n: usize) {
        self.env.run_count = n;
    }

    /// The average frames per second rendered.
    #[inline]
    pub fn frame_rate(&self) -> usize {
        self.env.frame_rate
    }

    /// Trigger exiting of the game loop.
    #[inline]
    pub fn quit(&mut self) {
        self.env.quit = true;
    }

    /// Abort exiting of the game loop.
    #[inline]
    pub fn abort_quit(&mut self) {
        self.env.quit = false;
    }
}

impl PixState {
    #[inline]
    pub(crate) fn last_frame_time(&self) -> Instant {
        self.env.last_frame_time
    }

    #[inline]
    pub(crate) fn set_delta_time(&mut self, now: Instant, time_since_last: Duration) {
        #[cfg(target_pointer_width = "32")]
        let delta = time_since_last.as_secs_f32();
        #[cfg(target_pointer_width = "64")]
        let delta = time_since_last.as_secs_f64();
        self.env.delta_time = delta;
        self.env.last_frame_time = now;
    }

    #[inline]
    pub(crate) fn is_running(&self) -> bool {
        self.settings.running || self.env.run_count > 0
    }

    #[inline]
    pub(crate) fn should_quit(&self) -> bool {
        self.env.quit
    }

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

    #[inline]
    pub(crate) fn present(&mut self) {
        self.renderer.present();
    }

    pub(crate) fn focus_window(&mut self, id: Option<WindowId>) {
        self.env.focused_window = id;
    }
}
