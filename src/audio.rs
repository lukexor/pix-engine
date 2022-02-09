//! Trait and types for allowing [`PixEngine`] to play and capture audio.
//!
//! There are several methods for playing audio in your application:
//!
//! - Queuing pre-recorded or generated audio samples by calling [`PixState::enqueue_audio`].
//! - Having [`PixEngine`] request pre-recorded or generated audio samples by implementing the
//!   [`AudioCallback`] trait on a type and calling [`PixState::open_playback`].
//! - Loading and playing a `.wav` or `.mp3` file. (Coming soon!).
//!
//! You can also record audio from a capture device using [`PixState::open_capture`].
//!
//! [`PixEngine`]: crate::engine::PixEngine
//!
//! # Examples
//!
//! ## Audio Queue
//!
//! ```no_run
//! use pix_engine::{prelude::*, math::PI};
//!
//! struct MyApp;
//!
//! impl AppState for MyApp {
//!     fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!         // Some square wave samples of audio
//!         let volume = 0.2;
//!         let sample_rate = s.audio_sample_rate() as f32;
//!         let sample_count = 4 * sample_rate as usize;
//!         let frequency = 440.0; // A4 note
//!         let mut samples = Vec::with_capacity(sample_count);
//!         for x in 0..sample_count {
//!             let s = (2.0 * PI as f32 * frequency * x as f32 / sample_rate).sin();
//!             samples.push(if s <= 0.0 { -volume } else { volume });
//!         }
//!         // Add samples to audio queue for playback
//!         s.enqueue_audio(&samples)?;
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ## Audio Callback
//!
//! ```no_run
//! use pix_engine::prelude::*;
//! use std::time::Duration;
//!
//! struct SquareWave {
//!     phase_inc: f32,
//!     phase: f32,
//!     volume: f32,
//! }
//!
//! impl AudioCallback for SquareWave {
//!     fn callback(&mut self, out: &mut [f32]) {
//!         // Generate a square wave
//!         for x in out.iter_mut() {
//!             *x = if self.phase <= 0.5 {
//!                 self.volume
//!             } else {
//!                 -self.volume
//!             };
//!             self.phase = (self.phase + self.phase_inc) % 1.0;
//!         }
//!     }
//! }
//!
//! struct MyApp;
//!
//! impl AppState for MyApp {
//!     fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!         let desired_spec = AudioSpecDesired {
//!             freq: Some(44_100), // 44,100 HZ
//!             channels: Some(1),  // mono audio
//!             samples: None,      // default sample size
//!         };
//!         let mut device = s.open_playback(None, &desired_spec, |spec| {
//!             SquareWave {
//!                 phase_inc: 440.0 / spec.freq as f32,
//!                 phase: 0.0,
//!                 volume: 0.25,
//!             }
//!         })?;
//!
//!         // Start playback
//!         device.resume();
//!
//!         // Play for 2 seconds then quit.
//!         std::thread::sleep(Duration::from_millis(2000));
//!         s.quit();
//!
//!         // Device stops playback when dropped.
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ## Audio Capture
//!
//! For a more complete example, see `audio_capture_and_replay` in the `examples/` directory.
//!
//! ```no_run
//! use pix_engine::prelude::*;
//! use std::{sync::mpsc, time::Duration};
//!
//! struct Recording {
//!     record_buffer: Vec<f32>,
//!     pos: usize,
//!     tx: mpsc::Sender<Vec<f32>>,
//!     done: bool,
//! }
//!
//! impl AudioCallback for Recording {
//!     fn callback(&mut self, input: &mut [f32]) {
//!         if self.done {
//!             return;
//!         }
//!         for x in input {
//!            self.record_buffer[self.pos] = *x;
//!            self.pos += 1;
//!            if self.pos >= self.record_buffer.len() {
//!                self.done = true;
//!                self.tx
//!                    .send(self.record_buffer.clone())
//!                    .expect("could not send record buffer");
//!                break;
//!            }
//!         }
//!     }
//! }
//!
//! struct MyApp;
//!
//! const RECORDING_LENGTH_SECONDS: usize = 3;
//!
//! impl AppState for MyApp {
//!     fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!         let desired_spec = AudioSpecDesired {
//!             freq: None,     // default device frequency
//!             channels: None, // default device channels
//!             samples: None,  // default sample size
//!         };
//!
//!         let (tx, rx) = mpsc::channel();
//!         let capture_device = s.open_capture(None, &desired_spec, |spec| {
//!             Recording {
//!                 record_buffer: vec![
//!                     0.0;
//!                     spec.freq as usize
//!                         * RECORDING_LENGTH_SECONDS
//!                         * spec.channels as usize
//!                 ],
//!                 pos: 0,
//!                 tx,
//!                 done: false,
//!             }
//!         })?;
//!
//!         // Start playback
//!         capture_device.resume();
//!
//!         // Wait for recording to finish
//!         let recorded_samples = rx.recv()?;
//!         capture_device.pause();
//!
//!         // Handle recorded_samples
//!
//!         // Device stops playback when dropped.
//!         Ok(())
//!     }
//! }
//! ```
//!
//! [`AppState`]: crate::prelude::AppState

use crate::prelude::{PixResult, PixState};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

#[cfg(not(target_arch = "wasm32"))]
use crate::renderer::sdl::RendererAudioDevice;

/// Trait for allowing [`PixEngine`] to request audio samples from your application.
///
/// Please see the [module-level documentation] for more examples.
///
/// [`PixEngine`]: crate::engine::PixEngine
/// [module-level documentation]: crate::audio
pub trait AudioCallback: Send {
    /// Called when the audio playback device needs samples to play or the capture device has
    /// samples available. `buffer` is a pre-allocated buffer you can iterate over and update to
    /// provide audio samples, or consume to record audio samples.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::prelude::*;
    ///
    /// struct SquareWave {
    ///     phase_inc: f32,
    ///     phase: f32,
    ///     volume: f32,
    /// }
    ///
    /// impl AudioCallback for SquareWave {
    ///     fn callback(&mut self, out: &mut [f32]) {
    ///         // Generate a square wave
    ///         for x in out.iter_mut() {
    ///             *x = if self.phase <= 0.5 {
    ///                 self.volume
    ///             } else {
    ///                 -self.volume
    ///             };
    ///             self.phase = (self.phase + self.phase_inc) % 1.0;
    ///         }
    ///     }
    /// }
    /// ```
    fn callback(&mut self, buffer: &mut [f32]);
}

/// Playback status of an audio device.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AudioStatus {
    /// Audio device is stopped.
    Stopped,
    /// Audio device is playing.
    Playing,
    /// Audio device is paused.
    Paused,
}

impl Default for AudioStatus {
    fn default() -> Self {
        AudioStatus::Stopped
    }
}

/// Desired audio device specification.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AudioSpecDesired {
    /// DSP frequency (samples per second) in Hz. Set to None for the device’s fallback frequency.
    pub freq: Option<i32>,
    /// Number of separate sound channels. Set to None for the device’s fallback number of channels.
    pub channels: Option<u8>,
    /// The audio buffer size in samples (power of 2). Set to None for the device’s fallback sample size.
    pub samples: Option<u16>,
}

/// Audio device specification.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AudioSpec {
    /// DSP frequency (samples per second) in Hz.
    pub freq: i32,
    /// Number of separate sound channels.
    pub channels: u8,
    /// The audio buffer silence value.
    pub silence: u8,
    /// The audio buffer size in samples (power of 2).
    pub samples: u16,
    /// The audio buffer size in bytes.
    pub size: u32,
}

impl Default for AudioSpec {
    fn default() -> Self {
        Self {
            freq: 48_000,
            channels: 1,
            silence: 0,
            samples: 0,
            size: 0,
        }
    }
}

/// Audio callback or playback device that can be paused and resumed.
pub struct AudioDevice<CB: AudioCallback> {
    inner: RendererAudioDevice<DeviceCallback<CB>>,
}

impl<CB: AudioCallback> AudioDevice<CB> {
    /// Return the status of this audio callback device.
    #[inline]
    #[must_use]
    pub fn audio_status(&self) -> AudioStatus {
        self.inner.status().into()
    }

    /// Return the current driver of this audio callback device.
    #[inline]
    #[must_use]
    pub fn audio_driver(&self) -> &'static str {
        self.inner.subsystem().current_audio_driver()
    }

    /// Returns the sample rate for this audio callback device.
    #[inline]
    #[must_use]
    pub fn audio_sample_rate(&self) -> i32 {
        self.inner.spec().freq
    }

    /// Resumes playback of this audio callback device.
    #[inline]
    pub fn resume(&self) {
        self.inner.resume();
    }

    /// Pause playback of this audio callback device.
    #[inline]
    pub fn pause(&self) {
        self.inner.resume();
    }
}

impl<CB: AudioCallback> AudioDevice<CB> {
    /// Creates a new `AudioDevice` from a renderer-specific device.
    pub(crate) fn new(device: RendererAudioDevice<DeviceCallback<CB>>) -> Self {
        Self { inner: device }
    }
}

impl<CB: AudioCallback> fmt::Debug for AudioDevice<CB> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioDevice").finish()
    }
}

impl PixState {
    /// Add samples to the current audio buffer queue.
    ///
    /// # Errors
    ///
    /// If the audio device fails to queue samples, or if the audio buffer max size is reached,
    /// then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::{math::PI, prelude::*};
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     // Some square wave samples of audio
    ///     let volume = 0.2;
    ///     let sample_rate = s.audio_sample_rate() as f32;
    ///     let sample_count = 4 * sample_rate as usize;
    ///     let frequency = 440.0; // A4 note
    ///     let mut samples = Vec::with_capacity(sample_count);
    ///     for x in 0..sample_count {
    ///         let s = (2.0 * PI as f32 * frequency * x as f32 / sample_rate).sin();
    ///         samples.push(if s <= 0.0 { -volume } else { volume });
    ///     }
    ///     // Add samples to audio queue for playback
    ///     s.enqueue_audio(&samples)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn enqueue_audio<S: AsRef<[f32]>>(&mut self, samples: S) -> PixResult<()> {
        self.renderer.enqueue_audio(samples.as_ref())
    }

    /// Clear audio samples from the current audio buffer queue.
    #[inline]
    pub fn clear_audio(&mut self) {
        self.renderer.clear_audio();
    }

    /// Return the status of the current audio queue device.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
    ///     match event.key {
    ///         Key::Return => {
    ///             if s.audio_status() == AudioStatus::Paused {
    ///                 s.resume_audio();
    ///             }
    ///             Ok(true)
    ///         }
    ///         _ => Ok(false),
    ///     }
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn audio_status(&self) -> AudioStatus {
        self.renderer.audio_status()
    }

    /// Return the current driver of this audio callback device.
    #[inline]
    #[must_use]
    pub fn audio_driver(&self) -> &'static str {
        self.renderer.audio_driver()
    }

    /// Returns the sample rate for the current audio queue device.
    #[inline]
    #[must_use]
    pub fn audio_sample_rate(&self) -> i32 {
        self.renderer.audio_sample_rate()
    }

    /// Resumes playback of the current audio queue device.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
    ///     match event.key {
    ///         Key::Return => {
    ///             s.resume_audio();
    ///             Ok(true)
    ///         }
    ///         _ => Ok(false),
    ///     }
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn resume_audio(&mut self) {
        self.renderer.resume_audio();
    }

    /// Pause playback of the current audio queue device.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
    ///     match event.key {
    ///         Key::Return => {
    ///             s.pause_audio();
    ///             Ok(true)
    ///         }
    ///         _ => Ok(false),
    ///     }
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn pause_audio(&mut self) {
        self.renderer.pause_audio();
    }

    /// Opens and returns an audio callback device for playback.
    ///
    /// The audio device starts out `paused`. Call [resume](`AudioDevice::resume`) to start
    /// playback and [pause](`AudioDevice::pause`) to stop playback.
    ///
    /// # Errors
    ///
    /// If the renderer fails to open an audio device, then an error is returned.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use pix_engine::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct SquareWave {
    ///     phase_inc: f32,
    ///     phase: f32,
    ///     volume: f32,
    /// }
    ///
    /// impl AudioCallback for SquareWave {
    ///     fn callback(&mut self, out: &mut [f32]) {
    ///         // Generate a square wave
    ///         for x in out.iter_mut() {
    ///             *x = if self.phase <= 0.5 {
    ///                 self.volume
    ///             } else {
    ///                 -self.volume
    ///             };
    ///             self.phase = (self.phase + self.phase_inc) % 1.0;
    ///         }
    ///     }
    /// }
    ///
    /// struct MyApp;
    ///
    /// impl AppState for MyApp {
    ///     fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///         let desired_spec = AudioSpecDesired {
    ///             freq: Some(44_100), // 44,100 HZ
    ///             channels: Some(1),  // mono audio
    ///             samples: None,      // default sample size
    ///         };
    ///         let mut device = s.open_playback(None, &desired_spec, |spec| {
    ///             SquareWave {
    ///                 phase_inc: 440.0 / spec.freq as f32,
    ///                 phase: 0.0,
    ///                 volume: 0.25,
    ///             }
    ///         })?;
    ///
    ///         // Start playback
    ///         device.resume();
    ///
    ///         // Play for 2 seconds then quit.
    ///         std::thread::sleep(Duration::from_millis(2000));
    ///         s.quit();
    ///
    ///         // Device stops playback when dropped.
    ///         Ok(())
    ///     }
    /// }
    /// ```
    #[allow(single_use_lifetimes)]
    #[inline]
    pub fn open_playback<'a, CB, F, D>(
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
        self.renderer
            .open_playback(device, desired_spec, get_callback)
    }

    /// Opens and returns an audio capture device for recording.
    ///
    /// The audio device starts out `paused`. Call [resume](`AudioDevice::resume`) to start
    /// recording and [pause](`AudioDevice::pause`) to stop recording.
    ///
    /// # Errors
    ///
    /// If the renderer fails to open an audio device, then an error is returned.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use pix_engine::prelude::*;
    /// use std::{sync::mpsc, time::Duration};
    ///
    /// struct Recording {
    ///     record_buffer: Vec<f32>,
    ///     pos: usize,
    ///     tx: mpsc::Sender<Vec<f32>>,
    ///     done: bool,
    /// }
    ///
    /// impl AudioCallback for Recording {
    ///     fn callback(&mut self, input: &mut [f32]) {
    ///         if self.done {
    ///             return;
    ///         }
    ///         for x in input {
    ///            self.record_buffer[self.pos] = *x;
    ///            self.pos += 1;
    ///            if self.pos >= self.record_buffer.len() {
    ///                self.done = true;
    ///                self.tx
    ///                    .send(self.record_buffer.clone())
    ///                    .expect("could not send record buffer");
    ///                break;
    ///            }
    ///         }
    ///     }
    /// }
    ///
    /// struct MyApp;
    ///
    /// const RECORDING_LENGTH_SECONDS: usize = 3;
    ///
    /// impl AppState for MyApp {
    ///     fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///         let desired_spec = AudioSpecDesired {
    ///             freq: None,     // default device frequency
    ///             channels: None, // default device channels
    ///             samples: None,  // default sample size
    ///         };
    ///
    ///         let (tx, rx) = mpsc::channel();
    ///         let capture_device = s.open_capture(None, &desired_spec, |spec| {
    ///             Recording {
    ///                 record_buffer: vec![
    ///                     0.0;
    ///                     spec.freq as usize
    ///                         * RECORDING_LENGTH_SECONDS
    ///                         * spec.channels as usize
    ///                 ],
    ///                 pos: 0,
    ///                 tx,
    ///                 done: false,
    ///             }
    ///         })?;
    ///
    ///         // Start playback
    ///         capture_device.resume();
    ///
    ///         // Wait for recording to finish
    ///         let recorded_samples = rx.recv()?;
    ///         capture_device.pause();
    ///
    ///         // Handle recorded_samples
    ///
    ///         // Device stops playback when dropped.
    ///         Ok(())
    ///     }
    /// }
    /// ```
    #[allow(single_use_lifetimes)]
    #[inline]
    pub fn open_capture<'a, CB, F, D>(
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
        self.renderer
            .open_capture(device, desired_spec, get_callback)
    }
}

/// A newtype wrapper around `AudioCallback`.
pub(crate) struct DeviceCallback<CB: AudioCallback> {
    pub(crate) inner: CB,
}

/// Trait representing audio support.
pub(crate) trait AudioRenderer {
    /// Add audio samples to the current audio buffer queue.
    fn enqueue_audio(&mut self, samples: &[f32]) -> PixResult<()>;

    /// Clear audio samples from the current audio buffer queue.
    fn clear_audio(&mut self);

    /// Return the status of the current audio queue device.
    fn audio_status(&self) -> AudioStatus;

    /// Return the driver of current audio queue device.
    fn audio_driver(&self) -> &'static str;

    /// Return the sample rate of the current audio queue device.
    fn audio_sample_rate(&self) -> i32;

    /// Resume playback of the current audio queue device.
    fn resume_audio(&mut self);

    /// Pause playback of the current audio queue device.
    fn pause_audio(&mut self);

    /// Opens and returns an audio callback device for playback.
    #[allow(single_use_lifetimes)]
    fn open_playback<'a, CB, F, D>(
        &self,
        device: D,
        desired_spec: &AudioSpecDesired,
        get_callback: F,
    ) -> PixResult<AudioDevice<CB>>
    where
        CB: AudioCallback,
        F: FnOnce(AudioSpec) -> CB,
        D: Into<Option<&'a str>>;

    /// Opens and returns an audio capture device for recording.
    #[allow(single_use_lifetimes)]
    fn open_capture<'a, CB, F, D>(
        &self,
        device: D,
        desired_spec: &AudioSpecDesired,
        get_callback: F,
    ) -> PixResult<AudioDevice<CB>>
    where
        CB: AudioCallback,
        F: FnOnce(AudioSpec) -> CB,
        D: Into<Option<&'a str>>;
}
