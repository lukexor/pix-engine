use pix_engine::prelude::*;

struct Demo {}

impl Demo {
    fn new() -> Self {
        Self {}
    }
}

impl State for Demo {
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
    let demo = Demo::new();
    PixEngine::create("Demo Example", demo, 800, 600)
        .build()
        .expect("engine")
        .run()
        .expect("ran");
}
