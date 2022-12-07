//! Trait and types for allowing [`Engine`] to play and capture audio.
//!
//! There are several methods for playing audio in your application:
//!
//! - Queuing pre-recorded or generated audio samples by calling [`PixState::enqueue_audio`].
//! - Having [`Engine`] request pre-recorded or generated audio samples by implementing the
//!   [`AudioCallback`] trait on a type and calling [`PixState::open_playback`].
//! - Loading and playing a `.wav` or `.mp3` file. (Coming soon!).
//!
//! You can also record audio from a capture device using [`PixState::open_capture`].
//!
//! [`Engine`]: crate::engine::Engine
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
//! impl PixEngine for MyApp {
//!     fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
//!         s.resume_audio();
//!         Ok(())
//!     }
//!
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
//!     type Channel = f32;
//!
//!     fn callback(&mut self, out: &mut [Self::Channel]) {
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
//! impl PixEngine for MyApp {
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
//!     type Channel = f32;
//!
//!     fn callback(&mut self, input: &mut [Self::Channel]) {
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
//! impl PixEngine for MyApp {
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
//! [`PixEngine`]: crate::prelude::PixEngine

use crate::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
pub use crate::renderer::sdl::{AudioDevice, AudioFormatNum};

#[cfg(target_arch = "wasm32")]
pub use crate::renderer::wasm::{AudioDevice, AudioFormatNum};

/// Trait for allowing [`Engine`] to request audio samples from your application.
///
/// Please see the [module-level documentation] for more examples.
///
/// [`Engine`]: crate::engine::Engine
/// [module-level documentation]: crate::audio
pub trait AudioCallback: Send
where
    Self::Channel: AudioFormatNum + 'static,
{
    /// The audio type format for channel samples.
    type Channel;

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
    ///     type Channel = f32;
    ///
    ///     fn callback(&mut self, out: &mut [Self::Channel]) {
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
    fn callback(&mut self, buffer: &mut [Self::Channel]);
}

/// Audio number and endianness format for the given audio device.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[must_use]
pub enum AudioFormat {
    /// Unsigned 8-bit samples
    U8,
    /// Signed 8-bit samples
    S8,
    /// Unsigned 16-bit samples, little-endian
    U16LSB,
    /// Unsigned 16-bit samples, big-endian
    U16MSB,
    /// Signed 16-bit samples, little-endian
    S16LSB,
    /// Signed 16-bit samples, big-endian
    S16MSB,
    /// Signed 32-bit samples, little-endian
    S32LSB,
    /// Signed 32-bit samples, big-endian
    S32MSB,
    /// 32-bit floating point samples, little-endian
    F32LSB,
    /// 32-bit floating point samples, big-endian
    F32MSB,
}

#[cfg(target_endian = "little")]
impl AudioFormat {
    /// Unsigned 16-bit samples, native endian
    #[inline]
    pub const fn u16_sys() -> AudioFormat {
        AudioFormat::U16LSB
    }
    /// Signed 16-bit samples, native endian
    #[inline]
    pub const fn s16_sys() -> AudioFormat {
        AudioFormat::S16LSB
    }
    /// Signed 32-bit samples, native endian
    #[inline]
    pub const fn s32_sys() -> AudioFormat {
        AudioFormat::S32LSB
    }
    /// 32-bit floating point samples, native endian
    #[inline]
    pub const fn f32_sys() -> AudioFormat {
        AudioFormat::F32LSB
    }
}

#[cfg(target_endian = "big")]
impl AudioFormat {
    /// Unsigned 16-bit samples, native endian
    #[inline]
    pub const fn u16_sys() -> AudioFormat {
        AudioFormat::U16MSB
    }
    /// Signed 16-bit samples, native endian
    #[inline]
    pub const fn s16_sys() -> AudioFormat {
        AudioFormat::S16MSB
    }
    /// Signed 32-bit samples, native endian
    #[inline]
    pub const fn s32_sys() -> AudioFormat {
        AudioFormat::S32MSB
    }
    /// 32-bit floating point samples, native endian
    #[inline]
    pub const fn f32_sys() -> AudioFormat {
        AudioFormat::F32MSB
    }
}

impl Default for AudioFormat {
    fn default() -> Self {
        Self::f32_sys()
    }
}

/// Playback status of an audio device.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[must_use]
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
#[must_use]
pub struct AudioSpecDesired {
    /// DSP frequency (samples per second) in Hz. Set to None for the device’s fallback frequency.
    pub freq: Option<i32>,
    /// Number of separate sound channels. Set to None for the device’s fallback number of channels.
    pub channels: Option<u8>,
    /// The audio buffer size in samples (power of 2). Set to None for the device’s fallback sample size.
    pub samples: Option<u16>,
}

/// Audio device specification.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[must_use]
pub struct AudioSpec {
    /// DSP frequency (samples per second) in Hz.
    pub freq: i32,
    /// `AudioFormat` for the generic sample type.
    pub format: AudioFormat,
    /// Number of separate sound channels.
    pub channels: u8,
    /// The audio buffer size in samples (power of 2).
    pub samples: u16,
    /// The audio buffer size in bytes.
    pub size: u32,
}

impl Default for AudioSpec {
    fn default() -> Self {
        Self {
            freq: 44_100,
            format: AudioFormat::default(),
            channels: 1,
            samples: 512,
            size: 2048,
        }
    }
}

/// Provides access to audio device driver properties and controlling playback.
pub trait AudioDeviceDriver {
    /// Return the status of this audio callback device.
    fn status(&self) -> AudioStatus;

    /// Return the current driver of this audio callback device.
    fn driver(&self) -> &'static str;

    /// Returns the [`AudioSpec`] for this audio callback device.
    fn spec(&self) -> AudioSpec;

    /// Resumes playback of this audio callback device.
    fn resume(&self);

    /// Pause playback of this audio callback device.
    fn pause(&self);
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
    /// # impl PixEngine for App {
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
    /// # impl PixEngine for App {
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

    /// Returns the queued buffer size of the current audio queue device.
    #[inline]
    #[must_use]
    pub fn audio_queued_size(&self) -> u32 {
        self.renderer.audio_queued_size()
    }

    /// Returns the buffer size of the current audio queue device.
    #[inline]
    #[must_use]
    pub fn audio_size(&self) -> u32 {
        self.renderer.audio_size()
    }

    /// Resumes playback of the current audio queue device.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
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
    /// # impl PixEngine for App {
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
    ///     type Channel = f32;
    ///
    ///     fn callback(&mut self, out: &mut [Self::Channel]) {
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
    /// impl PixEngine for MyApp {
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
    ///     type Channel = f32;
    ///
    ///     fn callback(&mut self, input: &mut [Self::Channel]) {
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
    /// impl PixEngine for MyApp {
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

/// Trait representing audio support.
pub(crate) trait AudioDriver {
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

    /// Returns the queued buffer size (in bytes) of the current audio queue device.
    fn audio_queued_size(&self) -> u32;

    /// Returns the buffer size (in bytes) of the current audio queue device.
    fn audio_size(&self) -> u32;

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
