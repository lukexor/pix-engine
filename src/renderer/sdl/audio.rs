use super::Renderer;
use crate::{
    audio::{AudioRenderer, DeviceCallback},
    prelude::*,
};
use anyhow::anyhow;
use log::warn;
use sdl2::audio::{
    AudioCallback as SdlAudioCallback, AudioDevice as SdlAudioDevice, AudioSpec as SdlAudioSpec,
    AudioSpecDesired as SdlAudioSpecDesired, AudioStatus as SdlAudioStatus,
};

// ~1.5 minutes of audio @ 48,000 HZ.
const WARN_QUEUE_SIZE: u32 = 1 << 22;
// ~11.5  minutes of audio @ 48,000 HZ.
const MAX_QUEUE_SIZE: u32 = 1 << 25;

/// A alias around a SDL audio device.
pub(crate) type RendererAudioDevice<CB> = SdlAudioDevice<CB>;

impl AudioRenderer for Renderer {
    /// Add audio samples to the audio buffer queue.
    #[inline]
    fn enqueue_audio(&mut self, samples: &[f32]) -> PixResult<()> {
        let size = self.audio_device.size();
        if size <= MAX_QUEUE_SIZE {
            if size >= WARN_QUEUE_SIZE {
                warn!("Audio queue size is increasing: {}. Did you forget to call `PixState::resume_audio`?", size);
            }
            self.audio_device
                .queue_audio(samples)
                .map_err(PixError::Renderer)?;
            Ok(())
        } else {
            Err(anyhow!("Reached max audio queue size: {}. Did you forget to call `PixState::resume_audio`?", MAX_QUEUE_SIZE))
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

    /// Return the driver of current audio queue device.
    fn audio_driver(&self) -> &'static str {
        self.audio_device.subsystem().current_audio_driver()
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

    /// Opens and returns an audio callback device for playback.
    #[allow(single_use_lifetimes)]
    #[inline]
    fn open_playback<'a, CB, F, D>(
        &self,
        device: D,
        desired_spec: &AudioSpecDesired,
        get_callback: F,
    ) -> PixResult<AudioDevice<CB>>
    where
        CB: AudioCallback,
        F: FnOnce(AudioSpec) -> CB,
        D: Into<Option<&'a str>>,
    {
        Ok(self
            .context
            .audio()
            .map_err(PixError::Renderer)?
            .open_playback(device, &desired_spec.into(), |spec| DeviceCallback {
                inner: get_callback(spec.into()),
            })
            .map_err(PixError::Renderer)?
            .into())
    }

    /// Opens and returns an audio capture device for recording.
    #[allow(single_use_lifetimes)]
    #[inline]
    fn open_capture<'a, CB, F, D>(
        &self,
        device: D,
        desired_spec: &AudioSpecDesired,
        get_callback: F,
    ) -> PixResult<AudioDevice<CB>>
    where
        CB: AudioCallback,
        F: FnOnce(AudioSpec) -> CB,
        D: Into<Option<&'a str>>,
    {
        Ok(self
            .context
            .audio()
            .map_err(PixError::Renderer)?
            .open_capture(device, &desired_spec.into(), |spec| DeviceCallback {
                inner: get_callback(spec.into()),
            })
            .map_err(PixError::Renderer)?
            .into())
    }
}

#[doc(hidden)]
impl<CB: AudioCallback> SdlAudioCallback for DeviceCallback<CB> {
    type Channel = f32;
    fn callback(&mut self, out: &mut [Self::Channel]) {
        self.inner.callback(out);
    }
}

#[doc(hidden)]
impl<CB: AudioCallback> From<SdlAudioDevice<DeviceCallback<CB>>> for AudioDevice<CB> {
    /// Convert [`<SdlAudioDevice<DeviceCallback<CB>>>`] to [`AudioDevice<CB>`].
    fn from(device: SdlAudioDevice<DeviceCallback<CB>>) -> Self {
        Self::new(device)
    }
}

#[doc(hidden)]
impl From<SdlAudioSpecDesired> for AudioSpecDesired {
    /// Convert [`SdlAudioSpecDesired`] to [`AudioSpecDesired`].
    fn from(spec: SdlAudioSpecDesired) -> Self {
        Self {
            freq: spec.freq,
            channels: spec.channels,
            samples: spec.samples,
        }
    }
}

#[doc(hidden)]
impl From<&AudioSpecDesired> for SdlAudioSpecDesired {
    /// Convert [`&AudioSpecDesired`] to [`SdlAudioSpecDesired`].
    fn from(spec: &AudioSpecDesired) -> Self {
        Self {
            freq: spec.freq,
            channels: spec.channels,
            samples: spec.samples,
        }
    }
}

#[doc(hidden)]
impl From<SdlAudioSpec> for AudioSpec {
    /// Convert [`SdlAudioSpec`] to [`AudioSpec`].
    fn from(spec: SdlAudioSpec) -> Self {
        Self {
            freq: spec.freq,
            channels: spec.channels,
            silence: spec.silence,
            samples: spec.samples,
            size: spec.size,
        }
    }
}

#[doc(hidden)]
impl From<SdlAudioStatus> for AudioStatus {
    /// Convert [`SdlAudioStatus`] to [`AudioStatus`].
    fn from(status: SdlAudioStatus) -> Self {
        match status {
            SdlAudioStatus::Stopped => Self::Stopped,
            SdlAudioStatus::Playing => Self::Playing,
            SdlAudioStatus::Paused => Self::Paused,
        }
    }
}
