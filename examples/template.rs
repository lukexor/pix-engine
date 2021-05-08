use pix_engine::prelude::*;

const TITLE: &str = "Example App";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct App {}

impl App {
    fn new() -> Self {
        Self {}
    }
}

impl Stateful for App {
    fn on_start(&mut self, s: &mut State) -> PixResult<()> {
        s.show_frame_rate(true);
        Ok(())
    }

    fn on_update(&mut self, _s: &mut State) -> PixResult<()> {
        Ok(())
    }

    fn on_stop(&mut self, _s: &mut State) -> PixResult<()> {
        Ok(())
    }
}

pub fn main() {
    let mut engine = PixEngine::create(WIDTH, HEIGHT)
        .with_title(TITLE)
        .position_centered()
        .build()
        .expect("valid engine");

    let mut app = App::new();

    engine.run(&mut app).expect("ran successfully");
}
