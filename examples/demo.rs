use pix_engine::{PixApp, PixEngine, PixEngineResult, State};

struct Demo {
    window_id: u32,
}

impl Demo {
    fn new() -> Self {
        Self { window_id: 0 }
    }
}

impl PixApp for Demo {
    fn on_start(&mut self, state: &mut State) -> PixEngineResult<bool> {
        state.set_draw_color((255, 0, 0));
        self.window_id = state.create_window("Test", 200, 200)?;
        state.push_window_target(self.window_id)?;
        state.set_draw_color((0, 255, 0));
        let _ = state.pop_window_target();
        Ok(true)
    }
    fn on_stop(&mut self, _state: &mut State) -> PixEngineResult<bool> {
        Ok(true)
    }
    fn on_update(&mut self, _state: &mut State) -> PixEngineResult<bool> {
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
