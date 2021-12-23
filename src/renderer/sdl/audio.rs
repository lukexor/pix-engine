use super::Renderer;
use crate::audio::{AudioRenderer, AudioStatus};
use log::warn;
use sdl2::audio::AudioStatus as SdlAudioStatus;

// ~1.5 minutes of audio @ 48,000 HZ.
const MAX_QUEUE_SIZE: u32 = 1 << 22;

impl AudioRenderer for Renderer {
    /// Add audio samples to the audio buffer queue.
    #[inline]
    fn enqueue_audio(&mut self, samples: &[f32]) {
        if self.audio_device.size() <= MAX_QUEUE_SIZE {
            self.audio_device.queue(samples);
        } else {
            warn!("Reached max audio queue size: {}. Did you forget to call `PixState::resume_audio`?", MAX_QUEUE_SIZE);
        }
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
