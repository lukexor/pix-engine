use pix_engine::prelude::*;

struct WindowDemo {
    window_ids: Vec<WindowId>,
}

impl WindowDemo {
    fn new() -> Self {
        Self { window_ids: vec![] }
    }
}

impl AppState for WindowDemo {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        self.window_ids.push(s.window_id());
        self.window_ids.push(
            s.window()
                .with_dimensions(200, 200)
                .with_title("Window 2")
                .position_centered()
                .build()?,
        );
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(YELLOW)?;
        Ok(())
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(800, 600)
        .with_title("Window 1")
        .position(10, 10)
        .build();
    let mut app = WindowDemo::new();
    engine.run(&mut app)
}
