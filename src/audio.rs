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
//!             sample_rate: Some(44_100), // 44,100 HZ
//!             channels: Some(1),         // mono audio
//!             buffer_size: None,         // device default
//!         };
//!         let mut device = s.open_playback(None, &desired_spec, |spec| {
//!             SquareWave {
//!                 phase_inc: 440.0 / spec.sample_rate as f32,
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
//!             sample_rate: None, // device default
//!             channels: None,    // device default
//!             buffer_size: None, // device default
//!         };
//!
//!         let (tx, rx) = mpsc::channel();
//!         let capture_device = s.open_capture(None, &desired_spec, |spec| {
//!             Recording {
//!                 record_buffer: vec![
//!                     0.0;
//!                     spec.sample_rate as usize
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

pub(crate) mod backend;

pub use backend::AudioDevice;

use backend::Direction;

/// Restricts [`AudioFormatNum`] to the sample types the engine converts.
mod sealed {
    /// Marks a type as an engine-provided sample format.
    pub trait Sealed {}
}

/// Sample types an [`AudioCallback`] can work in.
///
/// Sealed, because every implementation has to agree with the conversions the backend performs.
/// Implemented for `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `f32` and `f64`.
pub trait AudioFormatNum: sealed::Sealed + Copy + Send + 'static {
    /// The [`AudioFormat`] naming this sample type.
    const FORMAT: AudioFormat;

    /// Converts this sample to `f32`, the type the backend converts through.
    fn to_f32(self) -> f32;

    /// Converts an `f32` sample to this type.
    fn from_f32(sample: f32) -> Self;
}

/// Implements [`AudioFormatNum`] by deferring the conversion to `cpal`, which scales between
/// integer and float ranges rather than casting.
macro_rules! impl_audio_format_num {
    ($($ty:ty => $format:ident),+ $(,)?) => {
        $(
            impl sealed::Sealed for $ty {}

            impl AudioFormatNum for $ty {
                const FORMAT: AudioFormat = AudioFormat::$format;

                #[inline]
                fn to_f32(self) -> f32 {
                    cpal::Sample::to_sample::<f32>(self)
                }

                #[inline]
                fn from_f32(sample: f32) -> Self {
                    cpal::Sample::from_sample(sample)
                }
            }
        )+
    };
}

impl_audio_format_num! {
    i8 => I8,
    u8 => U8,
    i16 => I16,
    u16 => U16,
    i32 => I32,
    u32 => U32,
    f32 => F32,
    f64 => F64,
}

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

/// Sample format an audio device reads or writes.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[must_use]
pub enum AudioFormat {
    /// Signed 8-bit samples.
    I8,
    /// Unsigned 8-bit samples.
    U8,
    /// Signed 16-bit samples.
    I16,
    /// Unsigned 16-bit samples.
    U16,
    /// Signed 32-bit samples.
    I32,
    /// Unsigned 32-bit samples.
    U32,
    /// Signed 64-bit samples.
    I64,
    /// Unsigned 64-bit samples.
    U64,
    /// 32-bit floating point samples.
    #[default]
    F32,
    /// 64-bit floating point samples.
    F64,
}

#[doc(hidden)]
impl From<cpal::SampleFormat> for AudioFormat {
    fn from(format: cpal::SampleFormat) -> Self {
        match format {
            cpal::SampleFormat::I8 => Self::I8,
            cpal::SampleFormat::U8 => Self::U8,
            cpal::SampleFormat::I16 => Self::I16,
            cpal::SampleFormat::U16 => Self::U16,
            cpal::SampleFormat::I32 => Self::I32,
            cpal::SampleFormat::U32 => Self::U32,
            cpal::SampleFormat::I64 => Self::I64,
            cpal::SampleFormat::U64 => Self::U64,
            cpal::SampleFormat::F64 => Self::F64,
            _ => Self::F32,
        }
    }
}

/// Playback status of an audio device.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[must_use]
pub enum AudioStatus {
    /// Audio device is stopped.
    #[default]
    Stopped,
    /// Audio device is playing.
    Playing,
    /// Audio device is paused.
    Paused,
}

/// Audio device configuration to request when opening a device.
///
/// `None` accepts whatever the device offers.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[must_use]
pub struct AudioSpecDesired {
    /// Samples per second per channel, in Hz.
    pub sample_rate: Option<u32>,
    /// Number of channels. 1 for mono, 2 for stereo.
    pub channels: Option<u16>,
    /// Buffer size in frames. Smaller buffers lower latency and raise the risk of an underrun.
    pub buffer_size: Option<u32>,
}

/// Audio device configuration an opened device is running with.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[must_use]
pub struct AudioSpec {
    /// Samples per second per channel, in Hz.
    pub sample_rate: u32,
    /// Sample format the device reads or writes.
    pub format: AudioFormat,
    /// Number of channels. 1 for mono, 2 for stereo.
    pub channels: u16,
    /// Buffer size in frames.
    pub buffer_size: u32,
}

impl Default for AudioSpec {
    fn default() -> Self {
        Self {
            sample_rate: 44_100,
            format: AudioFormat::default(),
            channels: 1,
            buffer_size: 512,
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

impl<CB: AudioCallback> AudioDeviceDriver for AudioDevice<CB> {
    #[inline]
    fn status(&self) -> AudioStatus {
        Self::status(self)
    }

    #[inline]
    fn driver(&self) -> &'static str {
        Self::driver(self)
    }

    #[inline]
    fn spec(&self) -> AudioSpec {
        Self::spec(self)
    }

    #[inline]
    fn resume(&self) {
        Self::resume(self);
    }

    #[inline]
    fn pause(&self) {
        Self::pause(self);
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
        self.audio.enqueue(samples.as_ref())
    }

    /// Clear audio samples from the current audio buffer queue.
    #[inline]
    pub fn clear_audio(&mut self) {
        self.audio.clear();
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
        self.audio.status()
    }

    /// Return the current driver of this audio callback device.
    #[inline]
    #[must_use]
    pub fn audio_driver(&self) -> &'static str {
        self.audio.driver()
    }

    /// Returns the sample rate for the current audio queue device.
    #[inline]
    #[must_use]
    pub fn audio_sample_rate(&self) -> u32 {
        self.audio.spec().sample_rate
    }

    /// Returns how many samples are waiting to play in the audio queue.
    #[inline]
    #[must_use]
    pub fn audio_queued_samples(&self) -> usize {
        self.audio.queued_samples()
    }

    /// Returns the audio device buffer size in samples, across all channels.
    ///
    /// This is how much the device asks for at a time. Comparing it against
    /// [`PixState::audio_queued_samples`] is how an application paces its own output.
    #[inline]
    #[must_use]
    pub fn audio_buffer_size(&self) -> usize {
        self.audio.buffer_samples()
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
        self.audio.resume();
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
        self.audio.pause();
    }

    /// Opens and returns an audio callback device for playback.
    ///
    /// The audio device starts out `paused`. Call [resume](`AudioDeviceDriver::resume`) to start
    /// playback and [pause](`AudioDeviceDriver::pause`) to stop playback.
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
    ///             sample_rate: Some(44_100), // 44,100 HZ
    ///             channels: Some(1),         // mono audio
    ///             buffer_size: None,         // device default
    ///         };
    ///         let mut device = s.open_playback(None, &desired_spec, |spec| {
    ///             SquareWave {
    ///                 phase_inc: 440.0 / spec.sample_rate as f32,
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
        CB: AudioCallback + 'static,
        F: FnOnce(AudioSpec) -> CB,
        D: Into<Option<&'a str>>,
    {
        backend::open_device(device.into(), desired_spec, Direction::Output, get_callback)
    }

    /// Opens and returns an audio capture device for recording.
    ///
    /// The audio device starts out `paused`. Call [resume](`AudioDeviceDriver::resume`) to start
    /// recording and [pause](`AudioDeviceDriver::pause`) to stop recording.
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
    ///             sample_rate: None, // device default
    ///             channels: None,    // device default
    ///             buffer_size: None, // device default
    ///         };
    ///
    ///         let (tx, rx) = mpsc::channel();
    ///         let capture_device = s.open_capture(None, &desired_spec, |spec| {
    ///             Recording {
    ///                 record_buffer: vec![
    ///                     0.0;
    ///                     spec.sample_rate as usize
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
        CB: AudioCallback + 'static,
        F: FnOnce(AudioSpec) -> CB,
        D: Into<Option<&'a str>>,
    {
        backend::open_device(device.into(), desired_spec, Direction::Input, get_callback)
    }
}
