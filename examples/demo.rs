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
        self.primary_id = state.window_target().unwrap();
        self.second_id = state.create_window("Test", 200, 200)?;
        state.no_loop();
        Ok(true)
    }
    fn on_stop(&mut self, _state: &mut State) -> Result<bool> {
        Ok(true)
    }
    fn on_update(&mut self, state: &mut State) -> Result<bool> {
        state.set_bg_color((255, 0, 0));
        state.set_window_target(self.second_id)?;
        state.set_bg_color((255, 255, 0));
        state.revert_window_target();
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
