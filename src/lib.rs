#![warn(missing_docs, unused)]
#![deny(
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    variant_size_differences,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links
)]
#![doc(html_favicon_url = "")]
#![doc(html_logo_url = "")]
#![doc(html_playground_url = "")]

//! # Getting Started
//!
//! A cross-platform graphics/UI engine framework for simple games, visualizations, and graphics
//! demos.
//!
//! The goal of this library is to be simpler to setup and use for graphics or algorithm
//! exploration than larger graphics libraries.
//!
//! This is more than just a toy project, however, and can be used to drive powerful
//! applications. The primary use of this project is in the
//! [Tetanes](https://crates.io/crates/tetanes) NES emulator project.
//!
//! ```no_run
//! use pix_engine::prelude::*;
//!
//! struct App;
//!
//! impl App {
//!     fn new() -> Self {
//!         Self
//!     }
//! }
//!
//! impl AppState for App {
//!     fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
//!         // Setup App state. State contains engine specific state and
//!         // functions like mouse coordinates, draw functions, etc.
//!         Ok(())
//!     }
//!     fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!         // Update App state roughly every 16ms.
//!         Ok(())
//!     }
//!     fn on_stop(&mut self, s: &mut PixState) -> PixResult<()> {
//!         // Teardown any state or resources.
//!         Ok(())
//!     }
//! }
//!
//! pub fn main() {
//!     let width = 800;
//!     let height = 600;
//!     let mut engine = PixEngine::create(width, height)
//!       .with_title("App Title")
//!       .position_centered()
//!       .vsync_enabled()
//!       .build()
//!       .expect("valid engine");
//!     let mut app = App::new();
//!     engine.run(&mut app).expect("engine run");
//! }
//! ```

use include_dir::{include_dir, Dir};

// Bundles static binary assets with crate
const _STATIC_DIR: Dir<'_> = include_dir!("./static");

#[macro_use]
pub mod color;
#[macro_use]
pub mod math;
pub mod audio;
pub mod draw;
pub mod engine;
pub mod event;
pub mod image;
pub mod renderer;
#[macro_use]
pub mod shape;
pub mod state;
#[macro_use]
pub mod vector;

mod common;
mod utils;

pub use prelude::{AppState, PixEngine, PixError, PixResult, PixState};

/// Re-exports most commonly used structs, traits, and functions.
#[macro_use]
pub mod prelude {
    use super::*;
    pub use color::{constants::*, Color, Hsv, Rgb};
    pub use common::{Error as PixError, Result as PixResult};
    pub use draw::TextureId;
    pub use engine::PixEngine;
    pub use event::*;
    pub use image::{Error as ImageError, Image, PixelFormat, Result as ImageResult};
    pub use math::{constants::*, map, Scalar};
    pub use renderer::{Error as RendererError, Position, Renderer, Result as RendererResult};
    pub use shape::*;
    pub use state::{
        environment::WindowId,
        settings::{ArcMode, BlendMode, DrawMode},
        AppState, Error as StateError, PixState, Result as StateResult,
    };
    pub use vector::Vector;
    // Color macros
    pub use {hsv, rgb};
    // Math macros
    pub use {noise, random, vector};
    // Shape macros
    pub use {circle, ellipse, point, rect, square};
}
