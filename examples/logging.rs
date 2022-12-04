use pix_engine::prelude::*;
use std::env;

struct LoggingDemo;

impl PixEngine for LoggingDemo {
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.clear()?;
        s.text("Press any key and check the console for log events...")?;
        Ok(())
    }

    fn on_key_pressed(&mut self, _s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
        log::info!("Key Press Event: {:?}", event);
        Ok(false)
    }
}

fn main() -> PixResult<()> {
    // Default log level to "info".
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    // Initialize logger.
    pretty_env_logger::init();

    // Build and start application as normal.
    let mut engine = Engine::builder()
        .dimensions(800, 600)
        .title("Logging Demo")
        .build()?;
    let mut app = LoggingDemo;
    engine.run(&mut app)
}
