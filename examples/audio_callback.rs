use pix_engine::prelude::*;
use std::{env, time::Duration};

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: i8,
}

struct CallbackDemo;

impl AudioCallback for SquareWave {
    type Channel = i8;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

impl AppState for CallbackDemo {
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.clear()?;

        let desired_spec = AudioSpecDesired {
            freq: Some(44_100), // 44,100 HZ
            channels: Some(1),  // mono audio
            samples: None,      // default sample size
        };
        let device = s.open_playback(None, &desired_spec, |spec| {
            log::info!("Playback Spec: {:?}", spec);
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 40,
            }
        })?;

        // Start playback
        log::info!("Audio Driver: {:?}", device.driver());
        device.resume();

        // Play for 2 seconds then quit.
        std::thread::sleep(Duration::from_millis(2000));
        s.quit();

        // Device is automatically closed when dropped
        Ok(())
    }
}

fn main() -> PixResult<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let mut engine = PixEngine::builder()
        .with_dimensions(800, 600)
        .with_title("Audio Callback")
        .build()?;
    let mut app = CallbackDemo;
    engine.run(&mut app)
}
