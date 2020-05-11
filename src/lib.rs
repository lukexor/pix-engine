// #![deny(warnings, missing_docs)]

//! # Getting Started
//!
//! ```no_run
//! use pix_engine::prelude::*;
//! use app::App;
//!
//! pub fn main() -> () {
//!     let app = App::new();
//!     let (width, height) = (800, 600);
//!     PixEngine::create("App Title", app, width, height)
//!         .build()
//!         .expect("engine built")
//!         .run()
//!         .expect("engine ran");
//! }
//!
//! mod app {
//!     use pix_engine::prelude::*;
//!
//!     pub struct App {
//!         // App data
//!     }
//!
//!     impl App {
//!         pub fn new() -> Self {
//!             Self {}
//!         }
//!
//!         // App methods
//!     }
//!
//!     impl State for App {
//!         fn on_start(&mut self, state: &mut StateData) -> Result<bool> {
//!             // Setup application state/resources. Called once upon start when
//!             // `PixEngine::run()` is called.
//!             state.fill([255, 255, 0]);
//!             state.show_frame_rate(true);
//!             Ok(true)
//!         }
//!         fn on_update(&mut self, state: &mut StateData) -> Result<bool> {
//!             // Called every frame (as often as possible by default) but can be changed. e.g.
//!             // `state.set_target_frame_rate(30)`.
//!             state.background("darkgray");
//!             state.rect((100, 100), (200, 200))?;
//!             Ok(true)
//!         }
//!         fn on_stop(&mut self, _state: &mut StateData) -> Result<bool> {
//!             // Tear down application state/clean up resources. Called once upon an exit/close
//!             // event.
//!             Ok(true)
//!         }
//!     }
//! }
//! ```

pub mod color;
pub mod event;
pub mod gui;
pub mod image;
pub mod math;
pub mod shape;
pub mod transform;
pub mod typography;

mod common;
mod core;
mod renderer;
mod state_data;
mod time;

pub use crate::{common::constants, state_data::StateData};
/// Common set of exports for using the `PixEngine`.
pub mod prelude {
    pub use crate::{
        color::prelude::*, common::prelude::*, core::prelude::*, event::prelude::*,
        image::prelude::*, math::prelude::*, shape::prelude::*, state_data::prelude::*,
    };
}
