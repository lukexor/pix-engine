//! Audio functions.
//!
//! # Example
//!
//! ```no_run
//! # use pix_engine::prelude::*;
//! # let mut s = PixState::default();
//! # struct App;
//! # impl AppState for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     // Some random sample of audio as an array of f32
//!     let samples = [0.12, 0.23, 0.51];
//!     // Add samples to audio queue for playback
//!     s.enqueue_audio(&samples);
//!     Ok(())
//! }
//! # }
//! # Ok::<(), PixError>(())
//! ```

use crate::{renderer::Rendering, state::PixState};

impl PixState {
    /// Add audio samples to the audio buffer queue.
    pub fn enqueue_audio(&mut self, samples: &[f32]) {
        self.renderer.enqueue_audio(samples);
    }
}
