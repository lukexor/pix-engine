#![allow(unused_variables)]
// #![warn(missing_docs)]

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
//!         state.fill((255, 255, 0));
//!         state.show_frame_rate(true);
//!         Ok(true)
//!     }
//!     fn on_update(&mut self, state: &mut State) -> Result<bool> {
//!         // Called every frame (as often as possible by default) but can be changed with
//!         // `State::set_target_frame_rate()`.
//!         state.background((255, 0, 0));
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

pub(crate) use crate::{common::constants, state::State};

/// Common set of exports for using the `PixEngine`.
pub mod prelude {
    pub use crate::{
        color::*,
        common::{Result, *},
        core::*,
        event::*,
        image::*,
        math::*,
        shape::*,
        state::*,
    };
}
