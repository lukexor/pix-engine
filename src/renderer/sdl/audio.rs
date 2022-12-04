//! SDL Audio

use super::Renderer;
use crate::{
    audio::{AudioDeviceDriver, AudioDriver},
    error::{Error, Result},
    prelude::*,
};
use anyhow::anyhow;
use log::warn;
use sdl2::audio::{
    AudioCallback as SdlAudioCallback, AudioDevice as SdlAudioDevice,
    AudioFormat as SdlAudioFormat, AudioSpec as SdlAudioSpec,
    AudioSpecDesired as SdlAudioSpecDesired, AudioStatus as SdlAudioStatus,
};
use std::fmt;

pub use sdl2::audio::AudioFormatNum;

// ~1.5 minutes of audio @ 48,000 HZ.
const WARN_QUEUE_SIZE: u32 = 1 << 22;
// ~11.5  minutes of audio @ 48,000 HZ.
const MAX_QUEUE_SIZE: u32 = 1 << 25;

/// Audio callback or playback device that can be paused and resumed.
pub struct AudioDevice<CB: AudioCallback>(SdlAudioDevice<UserCallback<CB>>);

impl<CB: AudioCallback> AudioDeviceDriver for AudioDevice<CB> {
    /// Return the status of this audio callback device.
    #[inline]
    fn status(&self) -> AudioStatus {
        self.0.status().into()
    }

    /// Return the current driver of this audio callback device.
    #[inline]
    #[must_use]
    fn driver(&self) -> &'static str {
        self.0.subsystem().current_audio_driver()
    }

    /// Returns the [`AudioSpec`] for this audio callback device.
    #[inline]
    fn spec(&self) -> AudioSpec {
        self.0.spec().into()
    }

    /// Resumes playback of this audio callback device.
    #[inline]
    fn resume(&self) {
        self.0.resume();
    }

    /// Pause playback of this audio callback device.
    #[inline]
    fn pause(&self) {
        self.0.pause();
    }
}

impl<CB: AudioCallback> fmt::Debug for AudioDevice<CB> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioDevice")
            .field("status", &self.status())
            .field("driver", &self.driver())
            .field("spec", &self.spec())
            .finish()
    }
}

/// A wrapper around `AudioCallback`, which we can implement `SdlAudioCallback` for.
pub(crate) struct UserCallback<CB>(CB);

impl<CB: AudioCallback> UserCallback<CB> {
    fn new(callback: CB) -> Self {
        Self(callback)
    }
}

impl<CB: AudioCallback> AudioDevice<CB> {
    /// Creates a new `AudioDevice` from a renderer-specific device.
    pub(crate) fn new(device: SdlAudioDevice<UserCallback<CB>>) -> Self {
        Self(device)
    }
}

impl<CB: AudioCallback> SdlAudioCallback for UserCallback<CB> {
    type Channel = CB::Channel;
    fn callback(&mut self, out: &mut [Self::Channel]) {
        self.0.callback(out);
    }
}

impl<CB: AudioCallback> From<SdlAudioDevice<UserCallback<CB>>> for AudioDevice<CB> {
    /// Convert [`SdlAudioDevice<UseCallback<CB>>`] to [`AudioDevice`].
    fn from(device: SdlAudioDevice<UserCallback<CB>>) -> Self {
        Self::new(device)
    }
}

impl AudioDriver for Renderer {
    /// Add audio samples to the audio buffer queue.
    #[inline]
    fn enqueue_audio(&mut self, samples: &[f32]) -> Result<()> {
        let size = self.audio_device.size();
        if size <= MAX_QUEUE_SIZE {
            if size >= WARN_QUEUE_SIZE {
                warn!("Audio queue size is increasing: {}. Did you forget to call `PixState::resume_audio`? Audio Device Status: {:?}", size, self.audio_device.status());
            }
            self.audio_device
                .queue_audio(samples)
                .map_err(Error::Renderer)?;
            Ok(())
        } else {
            Err(anyhow!("Reached max audio queue size: {}. Did you forget to call `PixState::resume_audio`? Audio Device Status: {:?}", MAX_QUEUE_SIZE, self.audio_device.status()))
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

    /// Returns the queued buffer size (in bytes) of the current audio queue device.
    fn audio_queued_size(&self) -> u32 {
        self.audio_device.size()
    }

    /// Returns the buffer size (in bytes) of the current audio queue device.
    fn audio_size(&self) -> u32 {
        self.audio_device.spec().size
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
    ) -> Result<AudioDevice<CB>>
    where
        CB: AudioCallback,
        F: FnOnce(AudioSpec) -> CB,
        D: Into<Option<&'a str>>,
    {
        Ok(self
            .context
            .audio()
            .map_err(Error::Renderer)?
            .open_playback(device, &desired_spec.into(), |spec| {
                UserCallback::new(get_callback(spec.into()))
            })
            .map_err(Error::Renderer)?
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
    ) -> Result<AudioDevice<CB>>
    where
        CB: AudioCallback,
        F: FnOnce(AudioSpec) -> CB,
        D: Into<Option<&'a str>>,
    {
        Ok(self
            .context
            .audio()
            .map_err(Error::Renderer)?
            .open_capture(device, &desired_spec.into(), |spec| {
                UserCallback::new(get_callback(spec.into()))
            })
            .map_err(Error::Renderer)?
            .into())
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
impl From<SdlAudioFormat> for AudioFormat {
    /// Convert [`SdlAudioFormat`] to [`AudioFormat`].
    fn from(format: SdlAudioFormat) -> Self {
        match format {
            SdlAudioFormat::U8 => Self::U8,
            SdlAudioFormat::S8 => Self::S8,
            SdlAudioFormat::U16LSB => Self::U16LSB,
            SdlAudioFormat::U16MSB => Self::U16MSB,
            SdlAudioFormat::S16LSB => Self::S16LSB,
            SdlAudioFormat::S16MSB => Self::S16MSB,
            SdlAudioFormat::S32LSB => Self::S32LSB,
            SdlAudioFormat::S32MSB => Self::S32MSB,
            SdlAudioFormat::F32LSB => Self::F32LSB,
            SdlAudioFormat::F32MSB => Self::F32MSB,
        }
    }
}

#[doc(hidden)]
impl From<SdlAudioSpec> for AudioSpec {
    /// Convert [`SdlAudioSpec`] to [`AudioSpec`].
    fn from(spec: SdlAudioSpec) -> Self {
        Self {
            freq: spec.freq,
            format: spec.format.into(),
            channels: spec.channels,
            samples: spec.samples,
            size: spec.size,
        }
    }
}

#[doc(hidden)]
impl From<&SdlAudioSpec> for AudioSpec {
    /// Convert [`SdlAudioSpec`] to [`AudioSpec`].
    fn from(spec: &SdlAudioSpec) -> Self {
        Self {
            freq: spec.freq,
            format: spec.format.into(),
            channels: spec.channels,
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
