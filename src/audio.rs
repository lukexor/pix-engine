//! Handles audio related functionality.

use crate::{renderer::Rendering, state::PixState};

// /// Audio
// pub struct Audio {
//     pub samples: VecDeque<f32>,
// }

impl PixState {
    /// Add audio samples to the audio buffer queue.
    pub fn enqueue_audio(&mut self, samples: &[f32]) {
        self.renderer.enqueue_audio(samples);
    }
}
