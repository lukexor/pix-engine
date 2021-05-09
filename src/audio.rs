//! Handles audio related functionality.

use crate::state::PixState;

impl PixState {
    /// Add audio samples to the audio buffer queue.
    pub fn enqueue_audio(&mut self, _samples: &[f32]) {
        todo!("enqueue audio");
    }
}
