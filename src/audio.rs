//! `Audio` functions.
//!
//! Enables queuing audio samples using the [`PixState`] instance in your application implementing
//! [`AppState`] by calling [`PixState::enqueue_audio`].
//!
//! # Example
//!
//! ```no_run
//! # use pix_engine::prelude::*;
//! # struct App;
//! # impl AppState for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     // Some random samples of audio
//!     let samples = [0.12, 0.23, 0.51];
//!     // Add samples to audio queue for playback
//!     s.enqueue_audio(&samples);
//!     Ok(())
//! }
//! # }
//! ```
//!
//! [`AppState`]: crate::prelude::AppState

use crate::prelude::PixState;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Playback status of the current audio device.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AudioStatus {
    /// Audio device is stopped.
    Stopped,
    /// Audio device is playing.
    Playing,
    /// Audio device is paused.
    Paused,
}

impl PixState {
    /// Add samples to the audio buffer queue.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     let samples = [0.12, 0.23, 0.51];
    ///     s.enqueue_audio(&samples);
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn enqueue_audio<S: AsRef<[f32]>>(&mut self, samples: S) {
        self.renderer.enqueue_audio(samples.as_ref());
    }

    /// Return the status of the current audio device.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
    ///     match event.key {
    ///         Key::Return => {
    ///             if s.audio_status() == AudioStatus::Paused {
    ///                 s.resume_audio();
    ///             }
    ///             Ok(true)
    ///         }
    ///         _ => Ok(false),
    ///     }
    /// }
    /// # }
    /// ```
    pub fn audio_status(&self) -> AudioStatus {
        self.renderer.audio_status()
    }

    /// Returns the sample rate for the current audio device.
    pub fn audio_sample_rate(&self) -> i32 {
        self.renderer.audio_sample_rate()
    }

    /// Resumes playback of the current audio device.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
    ///     match event.key {
    ///         Key::Return => {
    ///             s.resume_audio();
    ///             Ok(true)
    ///         }
    ///         _ => Ok(false),
    ///     }
    /// }
    /// # }
    /// ```
    pub fn resume_audio(&mut self) {
        self.renderer.resume_audio();
    }

    /// Pause playback of the current audio dewvice.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
    ///     match event.key {
    ///         Key::Return => {
    ///             s.pause_audio();
    ///             Ok(true)
    ///         }
    ///         _ => Ok(false),
    ///     }
    /// }
    /// # }
    /// ```
    pub fn pause_audio(&mut self) {
        self.renderer.pause_audio();
    }
}

/// Trait representing audio support.
pub(crate) trait AudioRenderer {
    /// Add audio samples to the audio buffer queue.
    fn enqueue_audio(&mut self, samples: &[f32]);

    /// Return the status of the current audio device.
    fn audio_status(&self) -> AudioStatus;

    /// Return the sample rate of the current audio device.
    fn audio_sample_rate(&self) -> i32;

    /// Resume playback of the current audio device.
    fn resume_audio(&mut self);

    /// Pause playback of the current audio device.
    fn pause_audio(&mut self);
}
