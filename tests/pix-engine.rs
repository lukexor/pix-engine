//! Integration tests that run a real [Engine].
//!
//! ```no_run
//! PIX_ENGINE_TESTS=1 cargo test --features serde --test pix-engine
//! ```
//!
//! These run without the test harness, because a `winit` event loop has to be built on the main
//! thread and the harness runs every test on one it spawned. The loop can only be built once per
//! process, so every scenario shares this one binary and runs in turn. `PIX_ENGINE_TESTS` gates
//! them, so an ordinary `cargo test` does not open a window.

use pix_engine::prelude::*;
use std::env;

/// Environment variable that opts in to running these tests.
const ENABLE_VAR: &str = "PIX_ENGINE_TESTS";

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

/// Quitting from `on_start` exits the game loop before it runs.
fn engine_start() -> PixResult<()> {
    let mut app = App::new();
    app.quit_on_start = true;
    create_engine()?.run(&mut app)?;
    assert_eq!(app.start_count, 1, "on_start was called");
    assert_eq!(app.update_count, 0, "on_update was not called");
    assert_eq!(app.stop_count, 1, "on_stop was called");
    Ok(())
}

/// Quitting from `on_update` exits the game loop but still runs `on_stop`.
fn engine_update() -> PixResult<()> {
    let mut app = App::new();
    app.quit_on_update = true;
    create_engine()?.run(&mut app)?;
    assert_eq!(app.start_count, 1, "on_start was called");
    assert_eq!(app.update_count, 1, "on_update was called");
    assert_eq!(app.stop_count, 1, "on_stop was called");
    Ok(())
}

/// Aborting the quit from `on_stop` resumes the game loop.
fn engine_stop() -> PixResult<()> {
    let mut app = App::new();
    app.quit_on_update = true;
    app.abort_quit_on_stop = true;
    create_engine()?.run(&mut app)?;
    assert_eq!(app.start_count, 1, "on_start was called");
    // Accounts for the initial run, plus 1 more for on_stop being cancelled
    assert_eq!(app.update_count, 2, "on_update was called");
    assert_eq!(app.stop_count, 2, "on_stop was called");
    Ok(())
}

fn main() -> PixResult<()> {
    if env::var(ENABLE_VAR).is_err() {
        println!("engine tests skipped. Set {ENABLE_VAR}=1 to run them.");
        return Ok(());
    }
    for (name, test) in [
        ("engine_start", engine_start as fn() -> PixResult<()>),
        ("engine_update", engine_update),
        ("engine_stop", engine_stop),
    ] {
        test()?;
        println!("test {name} ... ok");
    }
    Ok(())
}
