use super::Renderer;
use crate::audio::Audio;

impl Audio for Renderer {
    /// Add audio samples to the audio buffer queue.
    fn enqueue_audio(&mut self, samples: &[f32]) {
        // Don't let queue overflow
        let sample_rate = self.audio_device.spec().freq as u32;
        while self.audio_device.size() > sample_rate {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        self.audio_device.queue(samples);
    }
}
