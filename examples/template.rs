use pix_engine::prelude::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct App {}

impl App {
    fn new() -> Self {
        Self {}
    }
}

impl Stateful for App {
    fn on_start(&mut self, s: &mut State) -> PixResult<bool> {
        s.show_frame_rate(true);
        Ok(true)
    }
    fn on_update(&mut self, _s: &mut State) -> PixResult<bool> {
        Ok(true)
    }
    fn on_stop(&mut self, _s: &mut State) -> PixResult<bool> {
        Ok(true)
    }
}

pub fn main() {
    let mut engine = PixEngine::create("Window Title", WIDTH, HEIGHT)
        .position_centered()
        .vsync_enabled()
        .build()
        .expect("valid engine");
    let mut app = App::new();
    engine.run(&mut app).expect("ran successfully");
}
