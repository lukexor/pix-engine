use pix_engine::{math::map, prelude::*};

const WIDTH: Primitive = 800;
const HEIGHT: Primitive = 600;
const SIZE: Primitive = 4;

struct Colors {
    h: f64,
    auto: bool,
}

impl Colors {
    fn new() -> Self {
        Self { h: 0.0, auto: true }
    }

    #[allow(clippy::many_single_char_names)]
    fn draw_gradient(&mut self, state: &mut PixState) -> PixResult<()> {
        let w = WIDTH as Scalar;
        let h = HEIGHT as Scalar;
        let size = SIZE as Scalar;
        for x in 0..WIDTH / SIZE {
            for y in 0..HEIGHT / SIZE {
                let x = (SIZE * x) as Scalar;
                let y = (SIZE * y) as Scalar;
                let s = map(x, 0.0, w, 0.0, 100.0);
                let v = map(y, 0.0, h, 0.0, 100.0);
                state.fill(hsb!(self.h, s, v));
                state.rect(rect!(x, y, size, size))?;
            }
        }
        Ok(())
    }

    fn modify_hue(&mut self, change: f64, auto: bool) {
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
        s.no_stroke();
        s.fill(WHITE);
        s.text(
            [20, 100],
            &format!("Press arrow keys to change Hue: {}", self.h),
        )?;
        s.text([20, 132], "Press Escape to return to demo mode.")?;
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
        .with_title("Colors")
        .with_frame_rate()
        .position_centered()
        .vsync_enabled()
        .build();
    let mut app = Colors::new();
    engine.run(&mut app)
}
