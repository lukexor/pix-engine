//! Handles audio related functionality.

use crate::{renderer::Rendering, state::PixState};

impl PixState {
    /// Add audio samples to the audio buffer queue.
    pub fn enqueue_audio(&mut self, samples: &[f32]) {
        self.renderer.enqueue_audio(samples);
    }
}
