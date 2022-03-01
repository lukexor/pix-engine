use pix_engine::prelude::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

struct Textures {
    textures: Vec<TextureId>,
}

impl Textures {
    fn new() -> Self {
        Self { textures: vec![] }
    }
}

impl AppState for Textures {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        // One texture for each quadrant of the screen
        for i in 0..4 {
            let texture_id = s.create_texture(WIDTH / 2, HEIGHT / 2, None)?;

            // Draw to each texture separately
            s.with_texture(texture_id, |s: &mut PixState| -> PixResult<()> {
                let (w, h) = s.dimensions()?;
                let center = point!(w / 2, h / 2);
                s.background(Color::random());

                let color = Color::random();
                s.fill(color);
                s.stroke(None);
                s.rect([10, 10, w as i32 - 20, h as i32 - 20])?;

                s.fill(color.inverted());
                s.rect_mode(RectMode::Center);
                s.set_cursor_pos(center.as_::<i32>());
                s.text(format!("Quadrant {}", i))?;
                Ok(())
            })?;

            self.textures.push(texture_id);
        }

        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        for (i, texture_id) in self.textures.iter_mut().enumerate() {
            let w = WIDTH as i32 / 2;
            let h = HEIGHT as i32 / 2;
            let pos = match i {
                0 => point!(0, 0),
                1 => point!(w, 0),
                2 => point!(0, h),
                3 => point!(w, h),
                _ => unreachable!("invalid texture index"),
            };
            s.texture(*texture_id, None, rect!(pos.x(), pos.y(), w, h))?;
        }
        Ok(())
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("Textures")
        .with_frame_rate()
        .build()?;
    let mut app = Textures::new();
    engine.run(&mut app)
}
