use pix_engine::prelude::*;

const TITLE: &str = "MyApp";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct MyApp;

impl AppState for MyApp {
    fn on_start(&mut self, _s: &mut PixState) -> PixResult<()> {
        // Setup App state. PixState contains engine specific state and
        // utility functions for things like getting mouse coordinates,
        // drawing shapes, etc.
        Ok(())
    }

    fn on_update(&mut self, _s: &mut PixState) -> PixResult<()> {
        // Main render loop. Called roughly every 16ms.
        Ok(())
    }

    fn on_stop(&mut self, _s: &mut PixState) -> PixResult<()> {
        // Teardown any state or resources before exiting.
        Ok(())
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title(TITLE)
        .with_frame_rate()
        .position_centered()
        .build();
    let mut app = MyApp;
    engine.run(&mut app)
}
