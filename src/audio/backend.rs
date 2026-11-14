//! cpal audio backend.
//!
//! Two things live here. [`AudioQueue`] backs [`PixState::enqueue_audio`], feeding samples to an
//! output stream the engine owns. [`AudioDevice`] backs [`PixState::open_playback`] and
//! [`PixState::open_capture`], handing a stream to a user [`AudioCallback`].
//!
//! Devices rarely accept the sample type an application wants to work in, so every conversion goes
//! through `f32`. That adds one multiply per sample and keeps [`AudioFormatNum`] free of the
//! conversion bounds a direct type-to-type mapping would leak into every signature.

use crate::{
    audio::{AudioCallback, AudioFormat, AudioFormatNum, AudioSpec, AudioSpecDesired, AudioStatus},
    error::Result,
};
use anyhow::{anyhow, Context};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, Device, FromSample, Host, SampleFormat, SizedSample, Stream, StreamConfig,
    SupportedBufferSize, SupportedStreamConfig,
};
use log::{debug, error, warn};
use std::{
    cell::Cell,
    collections::VecDeque,
    fmt,
    marker::PhantomData,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, PoisonError, TryLockError,
    },
};

/// Queue depth at which a warning is logged, in samples.
///
/// Roughly 1.5 minutes at 48 kHz. A queue this deep almost always means samples are being pushed
/// without the stream running.
const WARN_QUEUE_SAMPLES: usize = 1 << 22;

/// Queue depth at which [`AudioQueue::enqueue`] reports an error, in samples.
///
/// Roughly 11 minutes at 48 kHz. The queue grows on demand up to this point rather than reserving
/// the space, because most applications push a buffer at a time and never approach it.
const MAX_QUEUE_SAMPLES: usize = 1 << 23;

/// Frames of silence played before the queue is trusted to keep up.
///
/// The first callbacks arrive before an application has queued anything. Padding them with silence
/// rather than reporting starvation avoids a burst of garbage at startup.
const PRIME_FRAMES: usize = 2;

/// Per-sample gain applied when the queue runs dry, easing to silence instead of stepping to it.
const UNDERRUN_DECAY: f32 = 0.99;

/// Buffer size in frames requested when an application does not name one.
const DEFAULT_BUFFER_FRAMES: u32 = 512;

/// Samples waiting to play, shared between the application and the stream callback.
type SampleQueue = Arc<Mutex<VecDeque<f32>>>;

/// Opens the default output device and streams whatever is pushed into it.
pub(crate) struct AudioQueue {
    /// The running output stream. Dropping it closes the device.
    stream: Option<Stream>,
    /// Samples shared with the stream callback.
    samples: SampleQueue,
    /// Set while playback is paused, making the callback emit silence.
    paused: Arc<AtomicBool>,
    /// Configuration the device opened with.
    spec: AudioSpec,
    /// Name of the cpal host backing the device.
    driver: &'static str,
    /// Playback status as last requested.
    status: AudioStatus,
}

impl AudioQueue {
    /// Returns a queue playing to the default output device.
    ///
    /// A machine with no usable audio device still runs everything else, so a failure here is
    /// logged and playback is disabled instead of propagating.
    pub(crate) fn new(desired: &AudioSpecDesired) -> Self {
        match Self::open(desired) {
            Ok(queue) => queue,
            Err(err) => {
                warn!("audio output is disabled: {err:#}");
                Self::disabled()
            }
        }
    }

    /// Opens the default output device.
    fn open(desired: &AudioSpecDesired) -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow!("no default output device"))?;
        let supported = negotiate(&device, desired, Direction::Output)?;
        let spec = spec_from(&supported, desired);
        report_mismatch(&spec, desired, Direction::Output);
        let config = config_from(&spec);

        let samples: SampleQueue = Arc::new(Mutex::new(VecDeque::new()));
        let paused = Arc::new(AtomicBool::new(true));

        let stream = build_output(
            &device,
            &config,
            supported.sample_format(),
            QueueSource {
                samples: Arc::clone(&samples),
                paused: Arc::clone(&paused),
                primed: false,
                prime_target: None,
                held: 0.0,
            },
        )?;
        stream.play().context("failed to start audio output")?;

        Ok(Self {
            stream: Some(stream),
            samples,
            paused,
            spec,
            driver: host.id().name(),
            status: AudioStatus::Paused,
        })
    }

    /// Returns a queue with no device behind it, which accepts samples and drops them.
    fn disabled() -> Self {
        Self {
            stream: None,
            samples: Arc::new(Mutex::new(VecDeque::new())),
            paused: Arc::new(AtomicBool::new(true)),
            spec: AudioSpec::default(),
            driver: "none",
            status: AudioStatus::Stopped,
        }
    }

    /// Adds samples to the end of the queue.
    ///
    /// # Errors
    ///
    /// Returns an error once the queue passes [`MAX_QUEUE_SAMPLES`], which means samples are
    /// arriving faster than the device drains them.
    pub(crate) fn enqueue(&mut self, samples: &[f32]) -> Result<()> {
        if self.stream.is_none() {
            return Ok(());
        }
        let mut queued = self.lock();
        if queued.len() >= MAX_QUEUE_SAMPLES {
            let depth = queued.len();
            drop(queued);
            return Err(anyhow!(
                "audio queue reached {depth} samples, status {:?}. Was `PixState::resume_audio` called?",
                self.status
            ));
        }
        if queued.len() >= WARN_QUEUE_SAMPLES {
            warn!(
                "audio queue is {} samples deep, status {:?}. Was `PixState::resume_audio` called?",
                queued.len(),
                self.status
            );
        }
        queued.extend(samples);
        Ok(())
    }

    /// Drops every queued sample.
    pub(crate) fn clear(&mut self) {
        self.lock().clear();
    }

    /// Returns the number of samples waiting to play.
    pub(crate) fn queued_samples(&self) -> usize {
        self.lock().len()
    }

    /// Returns the device buffer size in samples, across all channels.
    ///
    /// The device asks for this much at a time. An application pacing its own output compares it
    /// against [`AudioQueue::queued_samples`].
    pub(crate) const fn buffer_samples(&self) -> usize {
        self.spec.buffer_size as usize * self.spec.channels as usize
    }

    /// Locks the queue, recovering from a callback thread that panicked while holding it.
    fn lock(&self) -> std::sync::MutexGuard<'_, VecDeque<f32>> {
        self.samples.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Returns the configuration the device opened with.
    pub(crate) const fn spec(&self) -> AudioSpec {
        self.spec
    }

    /// Returns the name of the cpal host backing the device.
    pub(crate) const fn driver(&self) -> &'static str {
        self.driver
    }

    /// Returns the playback status as last requested.
    pub(crate) const fn status(&self) -> AudioStatus {
        self.status
    }

    /// Starts playing queued samples.
    pub(crate) fn resume(&mut self) {
        if self.stream.is_some() {
            self.paused.store(false, Ordering::Release);
            self.status = AudioStatus::Playing;
        }
    }

    /// Stops playing without discarding queued samples.
    pub(crate) fn pause(&mut self) {
        if self.stream.is_some() {
            self.paused.store(true, Ordering::Release);
            self.status = AudioStatus::Paused;
        }
    }
}

impl fmt::Debug for AudioQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioQueue")
            .field("driver", &self.driver)
            .field("spec", &self.spec)
            .field("status", &self.status)
            .field("queued_samples", &self.queued_samples())
            .finish()
    }
}

/// Reads the queue on behalf of the stream callback.
struct QueueSource {
    /// Samples shared with [`AudioQueue`].
    samples: SampleQueue,
    /// Set while playback is paused.
    paused: Arc<AtomicBool>,
    /// Whether enough samples have accumulated to start playing.
    primed: bool,
    /// Samples that must accumulate before playback starts.
    ///
    /// Learned from the first callback. The requested buffer size is a request, and a device
    /// that rounds it leaves the target either too small to gate anything or seconds long.
    prime_target: Option<usize>,
    /// Last sample emitted, decayed across an underrun.
    held: f32,
}

impl QueueSource {
    /// Fills `out` with a fading tail, used when the queue cannot supply samples.
    fn decay_from(&mut self, out: &mut [f32]) {
        for sample in out {
            self.held *= UNDERRUN_DECAY;
            *sample = self.held;
        }
    }
}

impl OutputSource for QueueSource {
    fn fill(&mut self, out: &mut [f32]) {
        if self.paused.load(Ordering::Acquire) {
            out.fill(0.0);
            self.held = 0.0;
            return;
        }

        // Never block the device thread. A lock held by the application for the length of one
        // callback is far better spent decaying the previous sample than waiting.
        let mut queued = match self.samples.try_lock() {
            Ok(queued) => queued,
            Err(TryLockError::WouldBlock) => {
                // Written out rather than calling `decay_from`, which would borrow all of `self`
                // while the lock attempt still borrows one field of it.
                let mut held = self.held;
                for sample in out {
                    held *= UNDERRUN_DECAY;
                    *sample = held;
                }
                self.held = held;
                return;
            }
            Err(TryLockError::Poisoned(err)) => err.into_inner(),
        };

        let prime_target = *self.prime_target.get_or_insert(PRIME_FRAMES * out.len());
        if !self.primed {
            if queued.len() < prime_target {
                out.fill(0.0);
                return;
            }
            self.primed = true;
        }

        let filled = out.len().min(queued.len());
        for (target, sample) in out.iter_mut().zip(queued.drain(..filled)) {
            *target = sample;
        }
        drop(queued);
        if filled == out.len() {
            // `last` rather than an index, because a device may hand over an empty buffer.
            self.held = out.last().copied().unwrap_or(self.held);
            return;
        }
        // The queue ran dry. Decaying the last sample toward zero is quieter than a hard step to
        // silence, which reads as a click.
        if filled > 0 {
            self.held = out[filled - 1];
        }
        self.decay_from(&mut out[filled..]);
        if filled == 0 {
            // Nothing at all arrived, so wait for a full buffer again rather than stuttering.
            self.primed = false;
        }
    }
}

/// Wraps a capture callback so a paused device delivers nothing.
struct GatedCapture<CB> {
    /// Set while the device is paused.
    paused: Arc<AtomicBool>,
    /// The application's callback.
    callback: CB,
}

impl<CB: AudioCallback> AudioCallback for GatedCapture<CB> {
    type Channel = CB::Channel;

    fn callback(&mut self, buffer: &mut [Self::Channel]) {
        if self.paused.load(Ordering::Acquire) {
            return;
        }
        self.callback.callback(buffer);
    }
}

/// Adapts a user [`AudioCallback`] to the `f32` output path.
struct CallbackSource<CB: AudioCallback> {
    /// Set while the device is paused.
    paused: Arc<AtomicBool>,
    /// The application's callback.
    callback: CB,
    /// Reused buffer handed to the callback, sized to the device buffer.
    scratch: Vec<CB::Channel>,
}

impl<CB: AudioCallback + 'static> OutputSource for CallbackSource<CB> {
    fn fill(&mut self, out: &mut [f32]) {
        if self.paused.load(Ordering::Acquire) {
            out.fill(0.0);
            return;
        }
        self.scratch.clear();
        self.scratch.resize(out.len(), CB::Channel::from_f32(0.0));
        self.callback.callback(&mut self.scratch);
        for (target, sample) in out.iter_mut().zip(self.scratch.iter()) {
            *target = sample.to_f32();
        }
    }
}

/// Audio callback or playback device that can be paused and resumed.
///
/// Dropping it closes the device.
pub struct AudioDevice<CB: AudioCallback> {
    /// The running stream. Dropping it closes the device.
    stream: Stream,
    /// Configuration the device opened with.
    spec: AudioSpec,
    /// Name of the cpal host backing the device.
    driver: &'static str,
    /// Playback status as last requested.
    status: Cell<AudioStatus>,
    /// Set while the device is paused.
    ///
    /// `Stream::pause` reports `UnsupportedOperation` on the ALSA paths most Linux systems use,
    /// so the callback is gated here as well and silence is produced either way.
    paused: Arc<AtomicBool>,
    /// Ties the device to the callback type it was opened with.
    callback: PhantomData<fn() -> CB>,
}

impl<CB: AudioCallback> AudioDevice<CB> {
    /// Returns the configuration the device opened with.
    pub(crate) const fn spec(&self) -> AudioSpec {
        self.spec
    }

    /// Returns the name of the cpal host backing the device.
    pub(crate) const fn driver(&self) -> &'static str {
        self.driver
    }

    /// Returns the playback status as last requested.
    pub(crate) fn status(&self) -> AudioStatus {
        self.status.get()
    }

    /// Starts the device.
    pub(crate) fn resume(&self) {
        self.paused.store(false, Ordering::Release);
        if let Err(err) = self.stream.play() {
            error!("failed to start audio device: {err}");
        }
        self.status.set(AudioStatus::Playing);
    }

    /// Stops the device.
    ///
    /// The gate takes effect whether or not the device supports pausing its stream.
    pub(crate) fn pause(&self) {
        self.paused.store(true, Ordering::Release);
        if let Err(err) = self.stream.pause() {
            debug!("device does not support pausing its stream, gating instead: {err}");
        }
        self.status.set(AudioStatus::Paused);
    }
}

impl<CB: AudioCallback> fmt::Debug for AudioDevice<CB> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioDevice")
            .field("driver", &self.driver)
            .field("spec", &self.spec)
            .field("status", &self.status.get())
            .finish()
    }
}

/// Whether a device is being opened to play or to record.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Direction {
    /// Playback.
    Output,
    /// Capture.
    Input,
}

/// Opens a device and hands it to a user callback.
///
/// # Errors
///
/// Returns an error if no matching device exists or the device rejects every configuration.
pub(crate) fn open_device<CB, F>(
    name: Option<&str>,
    desired: &AudioSpecDesired,
    direction: Direction,
    get_callback: F,
) -> Result<AudioDevice<CB>>
where
    CB: AudioCallback + 'static,
    F: FnOnce(AudioSpec) -> CB,
{
    let host = cpal::default_host();
    let device = find_device(&host, name, direction)?;
    let supported = negotiate(&device, desired, direction)?;
    let spec = spec_from(&supported, desired);
    report_mismatch(&spec, desired, direction);
    let config = config_from(&spec);
    let callback = get_callback(spec);
    let paused = Arc::new(AtomicBool::new(true));

    let stream = match direction {
        Direction::Output => build_output(
            &device,
            &config,
            supported.sample_format(),
            CallbackSource {
                paused: Arc::clone(&paused),
                callback,
                scratch: Vec::new(),
            },
        )?,
        Direction::Input => build_input(
            &device,
            &config,
            supported.sample_format(),
            GatedCapture {
                paused: Arc::clone(&paused),
                callback,
            },
        )?,
    };

    Ok(AudioDevice {
        stream,
        spec,
        driver: host.id().name(),
        status: Cell::new(AudioStatus::Paused),
        paused,
        callback: PhantomData,
    })
}

/// Finds a device by name, or the default when no name is given.
fn find_device(host: &Host, name: Option<&str>, direction: Direction) -> Result<Device> {
    let Some(name) = name else {
        let default = match direction {
            Direction::Output => host.default_output_device(),
            Direction::Input => host.default_input_device(),
        }
        .ok_or_else(|| anyhow!("no default {direction:?} audio device"))?;

        if direction == Direction::Output || opens(&default, direction) {
            return Ok(default);
        }

        // Under PipeWire the ALSA default capture device advertises a configuration and then
        // refuses the stream, so the enumerated devices are tried before giving up. The device is
        // opened once to check, because a failed open cannot be retried after the application's
        // callback has been moved into the stream.
        warn!("default capture device could not be opened, trying the remaining devices");
        return host
            .input_devices()
            .context("failed to list capture devices")?
            .find(|device| opens(device, direction))
            .ok_or_else(|| anyhow!("no capture device could be opened"));
    };

    let devices = match direction {
        Direction::Output => host.output_devices().context("failed to list devices")?,
        Direction::Input => host.input_devices().context("failed to list devices")?,
    };
    devices
        .into_iter()
        .find(|device| device.to_string() == name)
        .ok_or_else(|| anyhow!("no {direction:?} audio device named {name:?}"))
}

/// Reports whether a device accepts a stream at its default configuration.
///
/// Opens and immediately drops one, because advertising a configuration is no guarantee the
/// device will take it.
fn opens(device: &Device, direction: Direction) -> bool {
    let Ok(config) = (match direction {
        Direction::Output => device.default_output_config(),
        Direction::Input => device.default_input_config(),
    }) else {
        return false;
    };
    let stream_config = StreamConfig {
        channels: config.channels(),
        sample_rate: config.sample_rate(),
        buffer_size: BufferSize::Default,
    };
    // The sample type selects the hardware format on ALSA, so the probe dispatches the same way
    // the real build does. Probing as `f32` rejects a device that only takes `i16`.
    let format = config.sample_format();
    match direction {
        Direction::Output => build_output(device, &stream_config, format, Silence).is_ok(),
        Direction::Input => build_input(device, &stream_config, format, Discard).is_ok(),
    }
}

/// An [`AudioCallback`] that ignores what it is given, used to test whether a device opens.
struct Discard;

impl AudioCallback for Discard {
    type Channel = f32;

    fn callback(&mut self, _buffer: &mut [Self::Channel]) {}
}

/// An [`OutputSource`] that emits silence, used to test whether a device opens.
struct Silence;

impl OutputSource for Silence {
    fn fill(&mut self, out: &mut [f32]) {
        out.fill(0.0);
    }
}

/// Picks a supported configuration matching the requested sample rate and channel count.
///
/// Falls back to the device default when nothing matches, because a device that cannot give an
/// application exactly what it asked for still beats no audio.
fn negotiate(
    device: &Device,
    desired: &AudioSpecDesired,
    direction: Direction,
) -> Result<SupportedStreamConfig> {
    let default = match direction {
        Direction::Output => device.default_output_config(),
        Direction::Input => device.default_input_config(),
    };
    if let Ok(default) = &default {
        let rate_ok = desired
            .sample_rate
            .is_none_or(|rate| rate == default.sample_rate());
        let channels_ok = desired
            .channels
            .is_none_or(|channels| channels == default.channels());
        if rate_ok && channels_ok {
            return Ok(*default);
        }
    }

    let mut candidates: Vec<_> = match direction {
        Direction::Output => device
            .supported_output_configs()
            .map(Iterator::collect)
            .unwrap_or_default(),
        Direction::Input => device
            .supported_input_configs()
            .map(Iterator::collect)
            .unwrap_or_default(),
    };
    // Prefer f32, which needs no conversion on the way out of a callback.
    candidates.sort_by_key(|config| u8::from(config.sample_format() != SampleFormat::F32));

    for config in candidates {
        if desired
            .channels
            .is_some_and(|want| want != config.channels())
        {
            continue;
        }
        match desired.sample_rate {
            Some(rate) => {
                if let Some(config) = config.try_with_sample_rate(rate) {
                    return Ok(config);
                }
            }
            None => {
                // Only the channel count differed, so the rate stays as close to the device
                // default as the range allows. Taking the maximum opens a 192kHz stream on
                // hardware that reports one, quadrupling the callback rate for no gain.
                let rate = default.as_ref().map_or_else(
                    |_| config.max_sample_rate(),
                    |default| {
                        default
                            .sample_rate()
                            .clamp(config.min_sample_rate(), config.max_sample_rate())
                    },
                );
                return Ok(config.with_sample_rate(rate));
            }
        }
    }

    default.context("device reported no usable audio configuration")
}

/// Logs when a device could not supply what the application asked for.
///
/// Nothing resamples or remixes between the application and the device, so a mismatch plays back
/// at the wrong pitch or across the wrong channels. Reporting it is the difference between a
/// puzzling noise and a one-line explanation.
fn report_mismatch(spec: &AudioSpec, desired: &AudioSpecDesired, direction: Direction) {
    if let Some(rate) = desired.sample_rate {
        if rate != spec.sample_rate {
            warn!(
                "{direction:?} device runs at {} Hz, not the requested {rate} Hz. Samples are not resampled.",
                spec.sample_rate
            );
        }
    }
    if let Some(channels) = desired.channels {
        if channels != spec.channels {
            warn!(
                "{direction:?} device runs {} channels, not the requested {channels}. Samples are not remixed.",
                spec.channels
            );
        }
    }
}

/// Builds the [`AudioSpec`] an opened device runs with.
///
/// The buffer size is clamped into the range the device published. A device is still free to
/// round what it is given, so treat the value as the size asked for.
fn spec_from(supported: &SupportedStreamConfig, desired: &AudioSpecDesired) -> AudioSpec {
    let requested = desired.buffer_size.unwrap_or(DEFAULT_BUFFER_FRAMES);
    let buffer_size = match supported.buffer_size() {
        SupportedBufferSize::Range { min, max } => requested.clamp(*min, *max),
        SupportedBufferSize::Unknown => requested,
    };
    AudioSpec {
        sample_rate: supported.sample_rate(),
        format: AudioFormat::from(supported.sample_format()),
        channels: supported.channels(),
        buffer_size,
    }
}

/// Builds the cpal stream configuration for a spec.
fn config_from(spec: &AudioSpec) -> StreamConfig {
    StreamConfig {
        channels: spec.channels,
        sample_rate: spec.sample_rate,
        buffer_size: BufferSize::Fixed(spec.buffer_size),
    }
}

/// Logs a stream error reported by the device thread.
fn on_stream_error(err: cpal::Error) {
    error!("audio stream error: {err}");
}

/// Calls `$build` with the concrete sample type the device reported.
///
/// The sample type has to be a compile-time choice, so each arm is its own instantiation.
macro_rules! dispatch_format {
    ($format:expr, $build:ident, $($arg:expr),+ $(,)?) => {
        match $format {
            SampleFormat::I8 => $build::<i8, _>($($arg),+),
            SampleFormat::U8 => $build::<u8, _>($($arg),+),
            SampleFormat::I16 => $build::<i16, _>($($arg),+),
            SampleFormat::U16 => $build::<u16, _>($($arg),+),
            SampleFormat::I32 => $build::<i32, _>($($arg),+),
            SampleFormat::U32 => $build::<u32, _>($($arg),+),
            SampleFormat::I64 => $build::<i64, _>($($arg),+),
            SampleFormat::U64 => $build::<u64, _>($($arg),+),
            SampleFormat::F32 => $build::<f32, _>($($arg),+),
            SampleFormat::F64 => $build::<f64, _>($($arg),+),
            format => Err(anyhow!("unsupported audio sample format {format:?}").into()),
        }
    };
}

/// Builds an output stream driving `source`.
fn build_output<S: OutputSource>(
    device: &Device,
    config: &StreamConfig,
    format: SampleFormat,
    source: S,
) -> Result<Stream> {
    dispatch_format!(format, build_output_as, device, config, source)
}

/// Builds an output stream for one concrete device sample type.
fn build_output_as<T, S>(device: &Device, config: &StreamConfig, mut source: S) -> Result<Stream>
where
    T: SizedSample + FromSample<f32>,
    S: OutputSource,
{
    let mut scratch = Vec::new();
    device
        .build_output_stream(
            *config,
            move |out: &mut [T], _| {
                scratch.clear();
                scratch.resize(out.len(), 0.0);
                source.fill(&mut scratch);
                for (target, sample) in out.iter_mut().zip(scratch.iter()) {
                    *target = T::from_sample(*sample);
                }
            },
            on_stream_error,
            None,
        )
        .context("failed to open audio output stream")
}

/// Builds an input stream feeding `callback`.
fn build_input<CB: AudioCallback + 'static>(
    device: &Device,
    config: &StreamConfig,
    format: SampleFormat,
    callback: CB,
) -> Result<Stream> {
    dispatch_format!(format, build_input_as, device, config, callback)
}

/// Builds an input stream for one concrete device sample type.
fn build_input_as<T, CB>(device: &Device, config: &StreamConfig, mut callback: CB) -> Result<Stream>
where
    T: SizedSample,
    f32: FromSample<T>,
    CB: AudioCallback + 'static,
{
    let mut scratch: Vec<CB::Channel> = Vec::new();
    device
        .build_input_stream(
            *config,
            move |input: &[T], _| {
                scratch.clear();
                scratch.extend(
                    input
                        .iter()
                        .map(|sample| CB::Channel::from_f32(sample.to_sample::<f32>())),
                );
                callback.callback(&mut scratch);
            },
            on_stream_error,
            None,
        )
        .context("failed to open audio input stream")
}

/// Produces `f32` samples for an output stream.
///
/// Implemented by the queue and by the user-callback adapter, so both drive one stream builder.
pub(crate) trait OutputSource: Send + 'static {
    /// Fills `out` with the next samples.
    fn fill(&mut self, out: &mut [f32]);
}
