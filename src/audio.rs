//! `Audio` functions.
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
//! # Ok::<(), PixError>(())
//! ```

use crate::state::PixState;

/// Trait representing audio support.
pub(crate) trait Audio {
    /// Add audio samples to the audio buffer queue.
    fn enqueue_audio(&mut self, samples: &[f32]);
}

impl PixState {
    /// Add samples to the audio buffer queue.
    #[inline]
    pub fn enqueue_audio(&mut self, samples: &[f32]) {
        self.renderer.enqueue_audio(samples);
    }
}
