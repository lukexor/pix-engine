//! Fixed-frame timing harness, enabled by setting `PIX_BENCH_FRAMES`.
//!
//! Runs the application for a set number of frames, reports the frame-time distribution, and
//! exits. Living in the engine rather than in each example keeps one measurement method across
//! renderer backends, so numbers taken before and after a backend change describe the same thing.
//!
//! The application keeps its own frame-rate pacing. Each sample is taken before the pacing sleep,
//! so it covers the work a frame did and not the interval it was scheduled at. Removing the
//! pacing instead would shrink [`PixState::delta_time`] to near zero, and an application that
//! drives motion from it then renders a still scene, measuring the wrong thing.
//!
//! ```sh
//! PIX_BENCH_FRAMES=600 cargo run --release --example matrix
//! ```

use log::warn;
use std::{
    env,
    time::{Duration, Instant},
};

/// Environment variable holding the number of frames to measure.
const FRAMES_VAR: &str = "PIX_BENCH_FRAMES";

/// Frames discarded before timing starts.
///
/// Covers the first-draw texture uploads that populate every cache and the CPU and GPU clock
/// ramp. Including them puts a multi-millisecond outlier in every percentile.
const WARMUP_FRAMES: usize = 60;

/// Wall time that must pass before timing starts, alongside [`WARMUP_FRAMES`].
///
/// Applications commonly hold off real work for the first second or two, whether for an intro or
/// to let state settle. An application with no target frame rate draws sixty frames in a few
/// milliseconds, landing every one of them inside that window.
const WARMUP_TIME: Duration = Duration::from_secs(2);

/// Collects per-frame durations for a fixed number of frames.
pub(crate) struct Bench {
    /// Frames left to discard before recording starts.
    remaining_warmup: usize,
    /// Recorded frame durations, one per measured frame.
    frames: Vec<Duration>,
    /// Number of frames to measure.
    target: usize,
    /// When the run loop began, used for the first-draw measurement.
    start: Instant,
    /// Time from the start of the run loop until the first frame finished.
    first_frame: Option<Duration>,
}

impl Bench {
    /// Reads the frame count from `PIX_BENCH_FRAMES`.
    ///
    /// Returns `None` when the variable is unset, which is the case for every ordinary run.
    pub(crate) fn from_env() -> Option<Self> {
        let raw = env::var(FRAMES_VAR).ok()?;
        match raw.parse::<usize>() {
            Ok(target) if target > 0 => Some(Self {
                remaining_warmup: WARMUP_FRAMES,
                frames: Vec::with_capacity(target),
                target,
                start: Instant::now(),
                first_frame: None,
            }),
            _ => {
                warn!("{FRAMES_VAR} must be a positive integer, got {raw:?}");
                None
            }
        }
    }

    /// Records one frame, returning `true` once the target frame count is reached.
    pub(crate) fn record(&mut self, frame: Duration) -> bool {
        if self.first_frame.is_none() {
            self.first_frame = Some(self.start.elapsed());
        }
        if self.remaining_warmup > 0 {
            self.remaining_warmup -= 1;
            return false;
        }
        if self.start.elapsed() < WARMUP_TIME {
            return false;
        }
        self.frames.push(frame);
        self.frames.len() >= self.target
    }

    /// Prints the frame-time distribution.
    ///
    /// Percentiles rather than a mean alone, because sleep granularity, cache misses and texture
    /// uploads all show up as isolated spikes that a mean absorbs.
    pub(crate) fn report(&self, vsync: bool) {
        if self.frames.is_empty() {
            warn!("no frames were measured");
            return;
        }

        let mut sorted = self.frames.clone();
        sorted.sort_unstable();
        let count = sorted.len();
        let total: Duration = sorted.iter().sum();
        let mean = total / u32::try_from(count).unwrap_or(u32::MAX);

        println!("--- pix-engine bench ---");
        println!(
            "frames     {count} measured, {WARMUP_FRAMES}+ discarded over the first {}s",
            WARMUP_TIME.as_secs()
        );
        println!("busy       {:.3} s summed frame time", total.as_secs_f64());
        println!(
            "mean       {:.3} ms ({:.1} fps)",
            millis(mean),
            1.0 / mean.as_secs_f64()
        );
        println!("p50        {:.3} ms", millis(percentile(&sorted, 0.50)));
        println!("p99        {:.3} ms", millis(percentile(&sorted, 0.99)));
        println!("max        {:.3} ms", millis(sorted[count - 1]));
        if let Some(first_frame) = self.first_frame {
            // Measured from the start of the run loop. Window and renderer creation happen in
            // `Engine::builder().build()`, before this point, and are not included.
            println!("first draw {:.1} ms after loop start", millis(first_frame));
        }
        if let Some(peak) = peak_rss() {
            println!("peak rss   {peak}");
        }
        if vsync {
            println!("NOTE       vsync is enabled, so frame times include the display wait");
        }
    }
}

/// Returns a duration in fractional milliseconds.
fn millis(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

/// Returns the value at `fraction` through an ascending slice.
fn percentile(sorted: &[Duration], fraction: f64) -> Duration {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let index = (sorted.len() as f64 * fraction) as usize;
    sorted[index.min(sorted.len() - 1)]
}

/// Returns the process high-water resident set size as reported by the kernel.
#[cfg(target_os = "linux")]
fn peak_rss() -> Option<String> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    let line = status.lines().find(|line| line.starts_with("VmHWM:"))?;
    Some(line.trim_start_matches("VmHWM:").trim().to_owned())
}

/// Returns the process high-water resident set size, where the platform reports one.
#[cfg(not(target_os = "linux"))]
fn peak_rss() -> Option<String> {
    None
}
