use pix_engine::prelude::*;

struct HelloWorld;

impl PixEngine for HelloWorld {
    // Set up any state or resources before starting main event loop.
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(220);
        Ok(())
    }

    // Main render loop. Called as often as possible, or based on `target_frame_rate`.
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.clear()?;
        s.text("Hello world!")?;
        Ok(())
    }

    // Teardown any state or resources before exiting.
    fn on_stop(&mut self, _s: &mut PixState) -> PixResult<()> {
        Ok(())
    }
}

fn main() -> PixResult<()> {
    let mut app = HelloWorld;
    let mut engine = Engine::builder()
        .dimensions(800, 600)
        .title("Hello World")
        .build()?;
    engine.run(&mut app)
}
