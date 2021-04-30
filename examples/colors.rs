use pix_engine::{math::map, prelude::*};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
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
    auto: bool,
}

impl Colors {
    fn new() -> Self {
        Self { h: 0.0, auto: true }
    }

    fn draw_gradient(&mut self, state: &mut State) -> PixResult<bool> {
        for x in (0..WIDTH / SIZE).into_iter() {
            for y in (0..HEIGHT / SIZE).into_iter() {
                let s = map((SIZE * x) as f32, 0.0, WIDTH as f32, 0.0, 1.0);
                let v = map((SIZE * y) as f32, 0.0, HEIGHT as f32, 0.0, 1.0);
                state.fill(hsv!(self.h, s, v));
                state.rect((SIZE * x) as i32, (SIZE * y) as i32, SIZE, SIZE)?;
            }
        }
        state.fill(WHITE);
        state.text(
            &format!("Press arrow keys to change Hue: {}", self.h),
            20,
            100,
        )?;
        state.text("Press Escape to return to demo mode.", 20, 132)?;
        Ok(true)
    }

    fn modify_hue(&mut self, change: f32, auto: bool) {
        self.auto = auto;
        self.h = (self.h + change) % 360.0;
        if self.h < 0.0 {
            self.h = 360.0 + self.h;
        }
    }
}

impl Stateful for Colors {
    fn on_start(&mut self, s: &mut State) -> PixResult<bool> {
        s.show_frame_rate(true);
        Ok(true)
    }

    fn on_update(&mut self, s: &mut State) -> PixResult<bool> {
        if self.auto && s.frame_count() % 4 == 0 {
            self.modify_hue(1.0, true);
        }
        self.draw_gradient(s)?;
        Ok(true)
    }

    fn on_key_pressed(&mut self, _s: &mut State, key: Keycode) {
        match key {
            Keycode::Escape => self.auto = true,
            Keycode::Up => self.modify_hue(2.0, false),
            Keycode::Down => self.modify_hue(-2.0, false),
            Keycode::Left => self.modify_hue(-10.0, false),
            Keycode::Right => self.modify_hue(10.0, false),
            _ => (),
        }
    }
}
