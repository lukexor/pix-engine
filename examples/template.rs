use pix_engine::prelude::*;

struct MyApp;

impl AppState for MyApp {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(220);
        s.cursor(None)?;
        s.stroke(Color::BLACK);
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.clear()?;

        // Main render loop. Called as often as possible, or based on `target frame rate`.
        if s.mouse_pressed() {
            s.fill(color!(0));
        } else {
            s.fill(color!(255));
        }
        let m = s.mouse_pos();
        s.circle([m.x(), m.y(), 45])?;
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
        .build()?;
    let mut app = MyApp;
    engine.run(&mut app)
}
