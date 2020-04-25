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
        s.no_loop();
        Ok(true)
    }
    fn on_update(&mut self, s: &mut State) -> Result<bool> {
        s.background(0);
        s.fill(GREEN);
        s.stroke(RED);
        let mx = s.width() as i32 / 2;
        let mh = s.height() as i32 / 2;
        let top = 10;
        let bot = s.height() as i32 - 10;
        s.triangle(mx, top, mx - 200, bot, mx + 300, bot - 100)?;
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
