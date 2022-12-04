//! Integration tests using [Engine] must be run on the main thread.
//!
//! ```no_run
//! cargo test engine -- --test-threads=1 --ignored
//! ```
//!
//! This is due to `SDL2` context needing to be on the main thread.

use pix_engine::prelude::*;

#[derive(Default, Debug)]
struct App {
    quit_on_start: bool,
    quit_on_update: bool,
    abort_quit_on_stop: bool,
    start_count: u32,
    update_count: u32,
    stop_count: u32,
}

impl App {
    fn new() -> Self {
        App::default()
    }
}

impl PixEngine for App {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        self.start_count += 1;
        if self.quit_on_start {
            s.quit();
        }
        Ok(())
    }
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        self.update_count += 1;
        if self.quit_on_update || self.update_count > 2 {
            s.quit();
        }
        Ok(())
    }
    fn on_stop(&mut self, s: &mut PixState) -> PixResult<()> {
        self.stop_count += 1;
        if self.abort_quit_on_stop {
            self.abort_quit_on_stop = false;
            self.quit_on_update = true;
            s.abort_quit();
        }
        Ok(())
    }
}

fn create_engine() -> PixResult<Engine> {
    Engine::builder()
        .title("pix-engine integration test")
        .position_centered()
        .hidden()
        .build()
}

#[test]
#[ignore = "engine can only be tested in the main thread. --test-threads=1"]
fn single_thread_engine_start() -> PixResult<()> {
    let mut eng = create_engine()?;
    // Quitting from on_start should exit the game loop early
    let mut app = App::new();
    app.quit_on_start = true;
    eng.run(&mut app)?;
    assert_eq!(app.start_count, 1, "on_start was called");
    assert_eq!(app.update_count, 0, "on_update was not called");
    assert_eq!(app.stop_count, 1, "on_stop was called");
    Ok(())
}

#[test]
#[ignore = "engine can only be tested in the main thread. --test-threads=1"]
fn single_thread_engine_update() -> PixResult<()> {
    let mut eng = create_engine()?;
    // Quitting from on_update should exit but still run on_stop
    let mut app = App::new();
    app.quit_on_update = true;
    eng.run(&mut app)?;
    assert_eq!(app.start_count, 1, "on_start was called");
    assert_eq!(app.update_count, 1, "on_update was called");
    assert_eq!(app.stop_count, 1, "on_stop was called");
    Ok(())
}

#[test]
#[ignore = "engine can only be tested in the main thread. --test-threads=1"]
fn single_thread_engine_stop() -> PixResult<()> {
    let mut eng = create_engine()?;
    // Aborting quit from on_stop should resume game loop
    let mut app = App::new();
    app.quit_on_update = true;
    app.abort_quit_on_stop = true;
    eng.run(&mut app)?;
    assert_eq!(app.start_count, 1, "on_start was called");
    // Accounts for the initial run, plus 1 more for on_stop being cancelled
    assert_eq!(app.update_count, 2, "on_update was called");
    assert_eq!(app.stop_count, 2, "on_stop was called");
    Ok(())
}
