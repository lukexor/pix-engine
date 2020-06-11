//! Integration tests using `PixEngine` must be run on the main thread.
//!
//! ```no_run
//! cargo test engine -- --test-threads=1 --ignored`
//! ```
//!
//! This is due to SDL2 context needing to be on the main thread.

use pix_engine::prelude::*;

#[derive(Default, Debug)]
struct App {
    start_return: bool,
    update_return: bool,
    stop_return: bool,
    start_count: u32,
    update_count: u32,
    stop_count: u32,
}

impl Stateful for App {
    fn on_start(&mut self, _s: &mut State) -> PixResult<bool> {
        self.start_count += 1;
        Ok(self.start_return)
    }
    fn on_update(&mut self, _s: &mut State) -> PixResult<bool> {
        // Ensures we don't loop forever
        if self.update_count >= 2 {
            Ok(false)
        } else {
            self.update_count += 1;
            Ok(self.update_return)
        }
    }
    fn on_stop(&mut self, _s: &mut State) -> PixResult<bool> {
        // Ensures we exit eventually
        if self.stop_count >= 2 {
            Ok(true)
        } else {
            self.stop_count += 1;
            Ok(self.stop_return)
        }
    }
}

fn create_engine() -> PixResult<PixEngine> {
    PixEngine::create("Test App", 800, 600)
        .position_centered()
        .vsync_enabled()
        .build()
}

#[test]
#[ignore]
fn test_engine_create() {
    // Nominal use case
    let eng = create_engine();
    assert!(eng.is_ok(), "should create new engine: {:?}", eng.err());
}

#[test]
#[ignore]
fn test_run_engine_start() {
    let mut eng = create_engine().unwrap();
    // Returning false from on_start should return early and not call any other methods
    let mut app = App::default();
    app.start_return = false;
    let _ = eng.run(&mut app);
    assert_eq!(app.start_count, 1, "on_start was called");
    assert_eq!(app.update_count, 0, "on_update was not called");
    assert_eq!(app.stop_count, 0, "on_stop was not called");
}

#[test]
#[ignore]
fn test_run_engine_update() {
    let mut eng = create_engine().unwrap();
    // Returning false from on_update should exit run and still call on_stop
    let mut app = App::default();
    app.start_return = true;
    app.update_return = false;
    app.stop_return = true;
    let _ = eng.run(&mut app);
    assert_eq!(app.start_count, 1, "on_start was called");
    assert_eq!(app.update_count, 1, "on_update was called");
    assert_eq!(app.stop_count, 1, "on_stop was called");
}

#[test]
#[ignore]
fn test_run_engine_stop() {
    let mut eng = create_engine().unwrap();
    // Returning false from on_stop should prevent exiting run until it returns true
    let mut app = App::default();
    app.start_return = true;
    app.update_return = true;
    app.stop_return = false;
    let _ = eng.run(&mut app);
    assert_eq!(app.start_count, 1, "on_start was called");
    // Accounts for the initial run, plus 1 more for on_stop being cancelled
    assert_eq!(app.update_count, 2, "on_update was called");
    assert_eq!(app.stop_count, 2, "on_stop was called");
}

#[test]
#[ignore]
fn test_engine_state_env() {
    let mut eng = create_engine().unwrap();
    let mut app = App::default();
    app.start_return = false;

    assert_eq!(eng.state().focused(), false, "not focused before run");
    let _ = eng.run(&mut app);
    assert_eq!(eng.state().focused(), true, "focused after run");
    assert_eq!(eng.state().width(), 800, "valid width");
    assert_eq!(eng.state().height(), 600, "valid heeight");
    assert_eq!(eng.state().fullscreen(), false, "start windowed");
    eng.state_mut().set_fullscreen(true);
    assert_eq!(eng.state().fullscreen(), true, "now fullscreen");
    eng.state_mut().set_fullscreen(false);
}
