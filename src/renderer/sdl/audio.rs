use super::Renderer;
use crate::audio::{AudioRenderer, AudioStatus};
use sdl2::audio::AudioStatus as SdlAudioStatus;

impl AudioRenderer for Renderer {
    /// Add audio samples to the audio buffer queue.
    #[inline]
    fn enqueue_audio(&mut self, samples: &[f32]) {
        // Don't let queue overflow
        let sample_rate = self.audio_device.spec().freq as u32;
        while self.audio_device.size() > sample_rate {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        self.audio_device.queue(samples);
    }

    /// Clear audio samples from the audio buffer queue.
    #[inline]
    fn clear_audio(&mut self) {
        self.audio_device.clear();
    }

    /// Return the status of the current audio device.
    #[inline]
    fn audio_status(&self) -> AudioStatus {
        self.audio_device.status().into()
    }

    /// Return the sample rate of the current audio device.
    fn audio_sample_rate(&self) -> i32 {
        self.audio_device.spec().freq
    }

    /// Resume playback of the current audio device.
    #[inline]
    fn resume_audio(&mut self) {
        self.audio_device.resume();
    }

    /// Pause playback of the current audio device.
    #[inline]
    fn pause_audio(&mut self) {
        self.audio_device.pause();
    }
}

impl From<SdlAudioStatus> for AudioStatus {
    fn from(status: SdlAudioStatus) -> Self {
        match status {
            SdlAudioStatus::Stopped => Self::Stopped,
            SdlAudioStatus::Playing => Self::Playing,
            SdlAudioStatus::Paused => Self::Paused,
        }
    }
}
