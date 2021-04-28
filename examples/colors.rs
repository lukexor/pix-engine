use pix_engine::{math::map, prelude::*};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 1024;
const SIZE: u32 = 4;

pub fn main() {
    let mut engine = PixEngine::create("Colors", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .expect("valid engine");
    let mut app = Colors::new();
    engine.run(&mut app).expect("ran successfully");
}

struct Colors {
    h: f32,
}

impl Colors {
    pub fn new() -> Self {
        Self { h: 0.0 }
    }
}

impl Stateful for Colors {
    fn on_start(&mut self, s: &mut State) -> PixResult<bool> {
        s.show_frame_rate(true);
        let _ = hsv!(-0.5, 0.0, 0.0);
        Ok(true)
    }

    fn on_update(&mut self, state: &mut State) -> PixResult<bool> {
        for x in (0..WIDTH / SIZE).into_iter() {
            for y in (0..HEIGHT / SIZE).into_iter() {
                let s = map((SIZE * x) as f32, 0.0, WIDTH as f32, 0.0, 1.0).unwrap();
                let v = map((SIZE * y) as f32, 0.0, HEIGHT as f32, 0.0, 1.0).unwrap();
                state.fill(hsv!(self.h, s, v));
                state.rect((SIZE * x) as i32, (SIZE * y) as i32, SIZE, SIZE)?;
            }
        }
        state.fill(WHITE);
        state.text_size(32);
        state.text(
            &format!("Press Up/Down to change Hue: {}", self.h),
            WIDTH as i32 / 2 - 250,
            100,
        )?;
        Ok(true)
    }

    fn on_key_pressed(&mut self, _s: &mut State, key: Keycode) {
        match key {
            Keycode::Up => self.h = (self.h + 2.0) % 360.0,
            Keycode::Down => {
                if self.h > 2.0 {
                    self.h -= 2.0;
                } else {
                    self.h = 360.0;
                }
            }
            _ => (),
        }
    }
}
