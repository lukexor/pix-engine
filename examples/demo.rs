use pix_engine::prelude::*;

struct Demo {
    img: Image,
}

impl Demo {
    fn new() -> Self {
        Self {
            img: Image::new(0, 0),
        }
    }
}

impl PixApp for Demo {
    fn on_start(&mut self, s: &mut State) -> Result<bool> {
        s.fill((255, 255, 0));
        s.show_frame_rate(true);
        let p = "castlevania_iii_draculas_curse.png";
        self.img = Image::load(&p)?;
        Ok(true)
    }
    fn on_update(&mut self, s: &mut State) -> Result<bool> {
        s.background((255, 0, 0));
        s.draw_rect(Rect::new(100, 100, 200, 200))?;
        // self.img.load_pixels();
        // self.img.update_pixels();
        Ok(true)
    }
    fn on_stop(&mut self, _s: &mut State) -> Result<bool> {
        Ok(true)
    }
}

fn main() {
    let demo = Demo::new();
    PixEngine::create("Demo Example", demo, 800, 600)
        .build()
        .expect("engine")
        .run()
        .expect("ran");
}
