use pix_engine::{shape::Rect, PixApp, PixEngine, Result, State};

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
        state.set_fill((255, 255, 0));
        state.set_show_frame_rate(true);
        Ok(true)
    }
    fn on_stop(&mut self, _state: &mut State) -> Result<bool> {
        Ok(true)
    }
    fn on_update(&mut self, state: &mut State) -> Result<bool> {
        state.set_background((255, 0, 0));
        state.draw_rect(Rect::new(100, 100, 200, 200))?;
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
