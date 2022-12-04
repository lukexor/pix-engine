use pix_engine::prelude::*;
use std::{env, sync::mpsc, time::Duration};

const RECORDING_LENGTH_SECONDS: usize = 3;

struct Recording {
    record_buffer: Vec<f32>,
    pos: usize,
    tx: mpsc::Sender<Vec<f32>>,
    done: bool,
}

// Append the input of the callback to the record_buffer.
// When the record_buffer is full, send it to the main thread via done_sender.
impl AudioCallback for Recording {
    type Channel = f32;

    fn callback(&mut self, input: &mut [Self::Channel]) {
        if self.done {
            return;
        }

        for x in input {
            self.record_buffer[self.pos] = *x;
            self.pos += 1;
            if self.pos >= self.record_buffer.len() {
                self.done = true;
                self.tx
                    .send(self.record_buffer.clone())
                    .expect("could not send record buffer");
                break;
            }
        }
    }
}

struct Playback {
    data: Vec<f32>,
    pos: usize,
}

impl AudioCallback for Playback {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        for x in out.iter_mut() {
            *x = *self.data.get(self.pos).unwrap_or(&0.0);
            self.pos += 1;
        }
    }
}

struct CaptureReplayDemo {
    recording: bool,
}

impl CaptureReplayDemo {
    fn new() -> Self {
        Self { recording: false }
    }

    fn record(&self, desired_spec: &AudioSpecDesired, s: &mut PixState) -> PixResult<Vec<f32>> {
        log::info!("Recording {:} seconds...", RECORDING_LENGTH_SECONDS);

        let (tx, rx) = mpsc::channel();

        let capture_device = s.open_capture(None, desired_spec, |spec| {
            log::info!("Capture Spec: {:?}", spec);
            Recording {
                record_buffer: vec![
                    0.0;
                    spec.freq as usize
                        * RECORDING_LENGTH_SECONDS
                        * spec.channels as usize
                ],
                pos: 0,
                tx,
                done: false,
            }
        })?;

        log::info!("Audio Driver: {:?}", capture_device.driver());
        capture_device.resume();

        // Wait for recording to finish
        let recorded_samples = rx.recv()?;
        capture_device.pause();

        log::info!(
            "Finished recording with {} samples.",
            recorded_samples.len()
        );

        // Device is automatically closed when dropped
        Ok(recorded_samples)
    }
}

impl PixEngine for CaptureReplayDemo {
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.clear()?;

        if self.recording {
            let cursor_pos = s.cursor_pos();
            s.text(format!(
                "Recording for {RECORDING_LENGTH_SECONDS} seconds..."
            ))?;
            s.present();

            let desired_spec = AudioSpecDesired {
                freq: None,     // default device frequency
                channels: None, // default device channels
                samples: None,  // default sample size
            };

            let recorded_samples = self.record(&desired_spec, s)?;
            self.recording = false;

            s.clear()?;
            s.set_cursor_pos(cursor_pos);
            s.text("Playing recording...")?;
            s.present();

            let playback_device = s.open_playback(None, &desired_spec, |spec| {
                log::info!("Playback Spec: {:?}", spec);
                Playback {
                    data: recorded_samples,
                    pos: 0,
                }
            })?;

            // Start playback
            playback_device.resume();

            // Delay dropping until audio finishes
            std::thread::sleep(Duration::from_secs(RECORDING_LENGTH_SECONDS as u64));

            // Device is automatically closed when dropped
        } else {
            s.text(format!(
                "Press <Space> to start recording for {RECORDING_LENGTH_SECONDS} seconds."
            ))?;
        }

        Ok(())
    }

    fn on_key_pressed(&mut self, _s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
        if let Key::Space = event.key {
            self.recording = true;
            return Ok(true);
        }
        Ok(false)
    }
}

fn main() -> PixResult<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let mut engine = Engine::builder()
        .dimensions(800, 600)
        .title("Audio Capture & Replay")
        .build()?;
    let mut app = CaptureReplayDemo::new();
    engine.run(&mut app)
}
