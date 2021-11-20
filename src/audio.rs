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

/// Trait representing audio support.
pub(crate) trait AudioRenderer {
    /// Add audio samples to the audio buffer queue.
    fn enqueue_audio(&mut self, samples: &[f32]);
}

impl PixState {
    /// Add samples to the audio buffer queue.
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
}
