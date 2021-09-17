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
        self.textures
            .push(s.create_texture(WIDTH / 2, HEIGHT, None)?);
        self.textures
            .push(s.create_texture(WIDTH / 2, HEIGHT, None)?);
        s.with_texture(&mut self.textures[0], |s: &mut PixState| -> PixResult<()> {
            s.background(GRAY)?;
            s.fill(GREEN);
            s.no_stroke();
            s.rect([10, 10, 100, 100])?;
            Ok(())
        })?;
        s.with_texture(&mut self.textures[1], |s: &mut PixState| -> PixResult<()> {
            s.background(YELLOW)?;
            s.fill(BLUE);
            s.text([20, 100], "Hello")?;
            Ok(())
        })?;
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.texture(
            &self.textures[0],
            None,
            rect!(0, 0, WIDTH as i32 / 2, HEIGHT as i32),
        )?;
        s.texture(
            &self.textures[1],
            None,
            rect!(WIDTH as i32 / 2, 0, WIDTH as i32 / 2, HEIGHT as i32),
        )?;
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
