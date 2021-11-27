use super::Renderer;
use crate::audio::AudioRenderer;

impl AudioRenderer for Renderer {
    /// Add audio samples to the audio buffer queue.
    #[inline]
    fn enqueue_audio(&mut self, samples: &[f32]) {
        todo!()
    }
}
