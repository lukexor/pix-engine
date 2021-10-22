use pix_engine::prelude::*;
use std::{env, path::Path};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct ImageDemo(Image);

impl ImageDemo {
    fn new<P: AsRef<Path>>(png: P) -> PixResult<Self> {
        Ok(Self(Image::from_file(png)?))
    }
}

impl AppState for ImageDemo {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(CADET_BLUE)?;
        s.blend_mode(BlendMode::Blend);
        s.image_mode(ImageMode::Center);
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.image([s.width()? / 2, s.height()? / 2], &self.0)?;
        s.text("<Esc>: Disable Tint")?;
        s.text("<Return>: Random Tint")?;
        s.text("<Left>: Disable Blend Mode")?;
        s.text("<Right>: Alpha Blend Mode")?;
        s.text("<Up>: Additive Blend Mode")?;
        s.text("<Down>: Modulated Blend Mode")?;
        Ok(())
    }

    fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
        match event.key {
            Key::Escape => s.image_tint(None),
            Key::Return => s.image_tint(Color::random_alpha()),
            Key::Left => s.blend_mode(BlendMode::None),
            Key::Right => s.blend_mode(BlendMode::Blend),
            Key::Up => s.blend_mode(BlendMode::Add),
            Key::Down => s.blend_mode(BlendMode::Mod),
            _ => (),
        }
        Ok(false)
    }
}

fn main() -> PixResult<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        Err(PixError::from("Usage: cargo run /path/to/image.png"))
    } else {
        let mut engine = PixEngine::builder()
            .with_dimensions(WIDTH, HEIGHT)
            .with_title("Image Demo")
            .position_centered()
            .with_frame_rate()
            .build();
        let mut app = ImageDemo::new(&args[1])?;
        engine.run(&mut app)
    }
}
