use pix_engine::{math, prelude::*};
use rand::{thread_rng, Rng};

const FRAC_2_PI: f32 = math::FRAC_2_PI as f32;
const PI: f32 = math::PI as f32;
const TWO_PI: f32 = 2.0 * PI;

struct AudioDemo {
    volume: f32,
    frequency: f32,
    sample_rate: f32,
    sample_count: usize,
    samples: Vec<f32>,
}

impl AudioDemo {
    fn new() -> Self {
        Self {
            volume: 0.2,
            frequency: 440.0, // A4 note
            sample_rate: 48_000.0,
            sample_count: 4 * 48_000,
            samples: vec![],
        }
    }

    fn gen_square_wave(&self) -> Vec<f32> {
        let mut result = Vec::with_capacity(self.sample_count);
        for x in 0..self.sample_count {
            let s = (TWO_PI * self.frequency * x as f32 / self.sample_rate).sin();
            result.push(if s <= 0.0 { -self.volume } else { self.volume });
        }
        result
    }

    fn gen_sine_wave(&self) -> Vec<f32> {
        let mut result = Vec::with_capacity(self.sample_count);
        for x in 0..self.sample_count {
            result
                .push(self.volume * (TWO_PI * self.frequency * x as f32 / self.sample_rate).sin());
        }
        result
    }

    fn gen_triangle_wave(&self) -> Vec<f32> {
        let mut result = Vec::with_capacity(self.sample_count);
        for x in 0..self.sample_count {
            result.push(
                self.volume
                    * FRAC_2_PI
                    * (TWO_PI * self.frequency * x as f32 / self.sample_rate)
                        .sin()
                        .asin(),
            );
        }
        result
    }

    fn gen_saw_wave(&self) -> Vec<f32> {
        let mut result = Vec::with_capacity(self.sample_count);
        for x in 0..self.sample_count {
            result.push(
                self.volume
                    * -FRAC_2_PI
                    * (1.0 / (PI * self.frequency * x as f32 / self.sample_rate).tan()).atan(),
            );
        }
        result
    }

    fn gen_noise(&self) -> Vec<f32> {
        let mut rng = thread_rng();
        let mut result = Vec::with_capacity(self.sample_count);
        for _ in 0..self.sample_count {
            result.push(self.volume * (rng.gen_range(0.0..2.0) - 1.0));
        }
        result
    }
}

impl AppState for AudioDemo {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        self.sample_rate = s.audio_sample_rate() as f32;
        self.sample_count = 4 * self.sample_rate as usize;
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.heading("Audio Demo")?;
        s.spacing()?;

        s.next_width(200);
        s.slider("Volume", &mut self.volume, 0.0, 1.0)?;
        s.next_width(200);
        s.slider("Frequency", &mut self.frequency, 20.0, 1000.0)?;

        if s.button("Sine Wave")? {
            self.samples = self.gen_sine_wave();
            s.enqueue_audio(&self.samples);
        }
        if s.button("Square Wave")? {
            self.samples = self.gen_square_wave();
            s.enqueue_audio(&self.samples);
        }
        if s.button("Triangle Wave")? {
            self.samples = self.gen_triangle_wave();
            s.enqueue_audio(&self.samples);
        }
        if s.button("Saw Wave")? {
            self.samples = self.gen_saw_wave();
            s.enqueue_audio(&self.samples);
        }
        if s.button("Noise")? {
            self.samples = self.gen_noise();
            s.enqueue_audio(&self.samples);
        }

        if !self.samples.is_empty() {
            s.stroke(Color::RED);
            let mut py = self.volume as i32;
            for (x, sample_index) in (0..self.sample_count).enumerate() {
                let x = x as i32;
                if x > 800 {
                    break;
                }
                let y = (100.0 * self.samples[sample_index] + self.volume) as i32;
                if x != 0 && (y - py).abs() > self.volume as i32 / 2 {
                    s.line(line_!([10 + x, 400 + y], [10 + x, 400 + py]))?;
                }
                py = y;
                s.point([10 + x, 400 + y])?;
            }
            s.no_stroke();
        }

        Ok(())
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(1024, 768)
        .with_title("Audio Demo")
        .with_frame_rate()
        .build()?;
    let mut app = AudioDemo::new();
    engine.run(&mut app)
}
