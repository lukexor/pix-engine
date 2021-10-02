use pix_engine::prelude::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

struct Textures {
    textures: Vec<Texture>,
}

impl Textures {
    fn new() -> Self {
        Self { textures: vec![] }
    }
}

impl AppState for Textures {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(BLACK)?;

        // One texture for each quadrant of the screen
        for i in 0..4 {
            let mut texture = s.create_texture(WIDTH / 2, HEIGHT / 2, None)?;

            // Draw to each texture separately
            let (w, h) = texture.dimensions();
            let center = texture.center();
            s.with_texture(&mut texture, |s: &mut PixState| -> PixResult<()> {
                s.background(Color::random())?;

                s.fill(Color::random());
                s.no_stroke();
                s.rect([10, 10, w as i32 - 20, h as i32 - 20])?;

                s.fill(Color::random());
                s.rect_mode(RectMode::Center);
                s.text(center, &format!("Quadrant {}", i))?;
                Ok(())
            })?;

            self.textures.push(texture);
        }

        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        for (i, texture) in self.textures.iter_mut().enumerate() {
            let w = texture.width() as i32;
            let h = texture.height() as i32;
            let pos = match i {
                0 => point!(0, 0),
                1 => point!(w, 0),
                2 => point!(0, h),
                3 => point!(w, h),
                _ => unreachable!(),
            };
            s.texture(texture, None, rect!(pos.x(), pos.y(), w, h))?;
        }
        Ok(())
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("Textures")
        .position_centered()
        .with_frame_rate()
        .build();
    let mut app = Textures::new();
    engine.run(&mut app)
}
