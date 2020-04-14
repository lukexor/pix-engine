use pix_engine::{
    event::{PixEvent, WindowEvent},
    PixApp, PixEngine, Result, State,
};

struct Demo {
    window_id: u32,
}

impl Demo {
    fn new() -> Self {
        Self { window_id: 0 }
    }
}

impl PixApp for Demo {
    fn on_start(&mut self, state: &mut State) -> Result<bool> {
        self.window_id = state.create_window("Test", 200, 200)?;
        Ok(true)
    }
    fn on_stop(&mut self, _state: &mut State) -> Result<bool> {
        Ok(true)
    }
    fn on_update(&mut self, state: &mut State) -> Result<bool> {
        for event in state.poll_iter() {
            match event {
                PixEvent::Window {
                    window_id,
                    win_event: WindowEvent::Close,
                    ..
                } if window_id == self.window_id => {
                    self.window_id = 0;
                }
                _ => (),
            }
        }
        state.set_bg_color((255, 0, 0));
        if self.window_id > 0 {
            state.push_window_target(self.window_id)?;
            state.set_bg_color((0, 255, 0));
            let _ = state.pop_window_target();
        }
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
