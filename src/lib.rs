//! # Getting Started
//!
//! ```no_run
//! use pix_engine::prelude::*;
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
//! struct App {
//!     // App-specific data
//! }
//!
//! impl App {
//!     fn new() -> Self {
//!         Self {}
//!     }
//! }
//!
//! impl PixApp for App {
//!     fn on_start(&mut self, state: &mut State) -> Result<bool> {
//!         // Setup application state/resources. Called once upon start when `PixEngine::run()` is
//!         // called.
//!         state.set_fill((255, 255, 0));
//!         state.set_show_frame_rate(true);
//!         Ok(true)
//!     }
//!     fn on_update(&mut self, state: &mut State) -> Result<bool> {
//!         // Called every frame (as often as possible by default) but can be changed with
//!         // `State::set_target_frame_rate()`.
//!         state.set_background((255, 0, 0));
//!         state.draw_rect(Rect::new(100, 100, 200, 200))?;
//!         Ok(true)
//!     }
//!     fn on_stop(&mut self, _state: &mut State) -> Result<bool> {
//!         // Tear down application state/clean up resources. Called once upon an exit/close
//!         // event.
//!         Ok(true)
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
mod state;
mod time;

pub use crate::state::State;

pub mod prelude {
    pub use crate::{
        color::*,
        common::{Error, Result},
        core::*,
        math::*,
        shape::*,
        state::*,
    };
}

pub mod constants {
    pub use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};
    pub const QUARTER_PI: f32 = FRAC_PI_2;
    pub const HALF_PI: f32 = FRAC_PI_2;
    pub const TWO_PI: f32 = 2.0 * PI;
    pub const TAU: f32 = TWO_PI;
}
