use pix_engine::prelude::*;

struct MyApp;

impl AppState for MyApp {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(220)?;
        s.no_cursor();
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        // Main render loop. Called as often as possible, or based on `target frame rate`.
        if s.mouse_pressed() {
            s.fill(0);
        } else {
            s.fill(255);
        }
        let m = s.mouse_pos();
        s.triangle([
            point![m.x(), m.y()],
            point![m.x() + 100, m.y()],
            point![m.x() + 30, m.y() - 100],
        ])?;
        Ok(())
    }

    fn on_stop(&mut self, _s: &mut PixState) -> PixResult<()> {
        // Teardown any state or resources before exiting.
        Ok(())
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(800, 600)
        .with_title("MyApp")
        .position_centered()
        .build();
    let mut app = MyApp;
    engine.run(&mut app)
}
