use pix_engine::{PixApp, PixEngine, Result, State};

struct Demo {
    primary_id: u32,
    second_id: u32,
}

impl Demo {
    fn new() -> Self {
        Self {
            primary_id: 0,
            second_id: 0,
        }
    }
}

impl PixApp for Demo {
    fn on_start(&mut self, state: &mut State) -> Result<bool> {
        self.primary_id = state.window_target();
        // self.second_id = state.create_window("Test", 200, 200)?;
        Ok(true)
    }
    fn on_stop(&mut self, _state: &mut State) -> Result<bool> {
        Ok(true)
    }
    fn on_update(&mut self, state: &mut State) -> Result<bool> {
        let mut image = state.create_image(2, 2);
        image.set_pixel(1, 1, (255, 0, 0));
        println!("(1, 1): {}", image.pixel(1, 1));
        image.load_pixels();
        for p in image.pixels_mut() {
            p.set_g(255);
            println!("Updating green: {}", p);
        }
        image.update_pixels();
        for p in image.bytes() {
            println!("{}", p);
        }
        println!("{}", image.pixel(0, 1));
        state.no_loop();
        // state.set_bg_color((255, 0, 0));
        // state.set_title("Main");

        // println!("Looping");
        // state.set_window_target(self.second_id)?;
        // state.set_bg_color((0, 255, 0));
        // state.set_title("Second");
        // state.set_window_target(None)?;

        // state.no_loop();
        Ok(true)
    }
}

fn main() {
    let demo = Demo::new();
    PixEngine::new("Demo Example", demo, 800, 600)
        .expect("engine")
        .run()
        .expect("ran");
}
