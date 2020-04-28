use pix_engine::prelude::*;

struct App {}

impl App {
    fn new() -> Self {
        Self {}
    }
}

impl State for App {
    fn on_start(&mut self, _s: &mut StateData) -> Result<bool> {
        Ok(true)
    }
    fn on_update(&mut self, _s: &mut StateData) -> Result<bool> {
        Ok(true)
    }
    fn on_stop(&mut self, _s: &mut StateData) -> Result<bool> {
        Ok(true)
    }
}

fn main() {
    let app = App::new();
    PixEngine::create("Blank App", app, 800, 600)
        .build()
        .expect("engine")
        .run()
        .expect("ran");
}
