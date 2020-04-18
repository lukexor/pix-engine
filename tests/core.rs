//! Integration tests must be run on the main thread using `cargo test -- --test-threads=1`.

use pix_engine::prelude::*;

struct App {}

impl PixApp for App {}

fn create_engine() -> Result<PixEngine<App>> {
    let app = App {};
    PixEngine::create("Blank App", app, 800, 600).build()
}

#[test]
#[ignore]
fn new_engine_test() {
    let eng = create_engine();
    assert!(eng.is_ok(), "engine err: {:?}", eng.err());
}
