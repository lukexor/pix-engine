use pix_engine::{math::map, prelude::*};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const SIZE: u32 = 4;

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
        let w = WIDTH as f64;
        let h = HEIGHT as f64;
        let size = SIZE as f64;
        for x in 0..WIDTH / SIZE {
            for y in 0..HEIGHT / SIZE {
                let x = (SIZE * x) as f64;
                let y = (SIZE * y) as f64;
                let s = map(x, 0.0, w, 0.0, 100.0);
                let v = map(y, 0.0, h, 0.0, 100.0);
                state.fill(hsb!(self.h, s, v));
                state.rect(rect!(x, y, size, size).as_::<i32>())?;
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
        s.stroke(None);
        s.fill(Color::WHITE);
        s.text("Hue: ")?;
        s.same_line(None);
        s.text(self.h.to_string())?;
        s.text("Left/Right/Up/Down: Change Hue")?;
        s.text("<Escape>: Return to demo.")?;
        Ok(())
    }

    fn on_key_pressed(&mut self, _s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
        match event.key {
            Key::Escape => self.auto = true,
            Key::Up => self.modify_hue(2.0, false),
            Key::Down => self.modify_hue(-2.0, false),
            Key::Left => self.modify_hue(-10.0, false),
            Key::Right => self.modify_hue(10.0, false),
            _ => (),
        }
        Ok(false)
    }
}

pub fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("Colors")
        .with_frame_rate()
        .build()?;
    let mut app = Colors::new();
    engine.run(&mut app)
}
