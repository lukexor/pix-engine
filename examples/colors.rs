use pix_engine::{math::map, prelude::*};

const TITLE: &str = "Colors";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const SIZE: u32 = 4;

struct Colors {
    h: f32,
    auto: bool,
}

impl Colors {
    fn new() -> Self {
        Self { h: 0.0, auto: true }
    }

    fn draw_gradient(&mut self, state: &mut PixState) -> PixResult<()> {
        for x in (0..WIDTH / SIZE).into_iter() {
            for y in (0..HEIGHT / SIZE).into_iter() {
                let s = map((SIZE * x) as f32, 0.0, WIDTH as f32, 0.0, 1.0);
                let v = map((SIZE * y) as f32, 0.0, HEIGHT as f32, 0.0, 1.0);
                state.fill(hsv!(self.h, s, v));
                state.rect(((SIZE * x), (SIZE * y), SIZE, SIZE))?;
            }
        }
        state.fill(WHITE);
        state.text(
            (20, 100),
            &format!("Press arrow keys to change Hue: {}", self.h),
        )?;
        state.text((20, 132), "Press Escape to return to demo mode.")?;
        Ok(())
    }

    fn modify_hue(&mut self, change: f32, auto: bool) {
        self.auto = auto;
        self.h = (self.h + change) % 360.0;
        if self.h < 0.0 {
            self.h += 360.0;
        }
    }
}

impl AppState for Colors {
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        if self.auto && s.frame_count() % 4 == 0 {
            self.modify_hue(1.0, true);
        }
        self.draw_gradient(s)?;
        Ok(())
    }

    fn on_key_pressed(&mut self, _s: &mut PixState, event: KeyEvent) -> PixResult<()> {
        match event.key {
            Key::Escape => self.auto = true,
            Key::Up => self.modify_hue(2.0, false),
            Key::Down => self.modify_hue(-2.0, false),
            Key::Left => self.modify_hue(-10.0, false),
            Key::Right => self.modify_hue(10.0, false),
            _ => (),
        }
        Ok(())
    }
}

pub fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title(TITLE)
        .with_frame_rate()
        .position_centered()
        .vsync_enabled()
        .build();
    let mut app = Colors::new();
    engine.run(&mut app)
}
