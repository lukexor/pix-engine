//! # Getting Started
//!
//! `pix_engine` is a cross-platform graphics & UI library for simple games, visualizations,
//! digital art, and graphics applications written in Rust, supporting [SDL2][] and
//! [Web-Assembly][WASM]
//! renderers.
//!
//! The primary goal of this library is to be simple to setup and use for graphics or algorithm
//! exploration and is not meant to be as fully-featured as other, larger graphics libraries.
//!
//! It is intended to be more than just a toy library, however, and can be used to drive complex
//! applications. For example, the [`Tetanes`][] [NES][] emulator.
//!
//! [SDL2]: https://crates.io/crates/sdl2/
//! [WASM]: https://www.rust-lang.org/what/wasm
//! [`Tetanes`]: https://crates.io/crates/tetanes
//! [NES]: https://en.wikipedia.org/wiki/Nintendo_Entertainment_System
//!
//! ```no_run
//! # #![allow(unused_variables)]
//! use pix_engine::prelude::*;
//!
//! struct MyApp;
//!
//! impl AppState for MyApp {
//!     fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
//!         // Setup App state. PixState contains engine specific state and
//!         // utility functions for things like getting mouse coordinates,
//!         // drawing shapes, etc.
//!         Ok(())
//!     }
//!
//!     fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!         // Main render loop. Called roughly every 16ms.
//!         Ok(())
//!     }
//!
//!     fn on_stop(&mut self, s: &mut PixState) -> PixResult<()> {
//!         // Teardown any state or resources before exiting.
//!         Ok(())
//!     }
//! }
//!
//! fn main() -> PixResult<()> {
//!     let mut engine = PixEngine::builder()
//!       .with_dimensions(800, 600)
//!       .with_title("MyApp")
//!       .position_centered()
//!       .build();
//!     let mut app = MyApp;
//!     engine.run(&mut app)
//! }
//! ```

#![deny(missing_docs, missing_doc_code_examples)]
#![warn(
    unused,
    deprecated_in_future,
    unreachable_pub,
    unused_crate_dependencies,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    future_incompatible,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    variant_size_differences,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links
)]
#![doc(
    html_root_url = "https://docs.rs/pix-engine/0.3.5",
    html_favicon_url = "",
    html_logo_url = "",
    test(attr(deny(warnings)))
)]

#[cfg(target_arch = "wasm32")]
use getrandom as _; // Used to set "js" feature for rand
#[cfg(not(target_arch = "wasm32"))]
use include_dir::{include_dir, Dir};

/// Temporary directory to store libray assets.
#[cfg(not(target_arch = "wasm32"))]
pub const ASSET_DIR: &str = "/tmp/pix-engine";
// Bundles static binary assets with crate
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const ASSETS: Dir<'_> = include_dir!("./assets");

pub mod app_state;
#[macro_use]
pub mod color;
pub mod audio;
pub mod draw;
pub mod engine;
pub mod event;
pub mod image;
pub mod lighting;
#[macro_use]
pub mod math;
pub mod renderer;
#[macro_use]
pub mod shape;
pub mod state;
#[macro_use]
pub mod vector;
pub mod texture;
pub mod window;

mod common;
mod utils;

/// Exports most commonly used types, traits, and functions.
pub mod prelude {
    use super::*;
    pub use app_state::AppState;
    pub use color::{constants::*, Color, ColorError, ColorMode};
    pub use common::{Error as PixError, Result as PixResult};
    pub use draw::Draw;
    pub use engine::PixEngine;
    pub use event::{Axis, Button, Event, Key, KeyEvent, KeyMod, Mouse, WindowEvent};
    pub use image::{Image, PixelFormat};
    pub use lighting::{Light, LightSource};
    pub use math::{constants, map, Scalar};
    pub use shape::{Circle, Ellipse, Line, Point, Rect, Shape, Sphere, Triangle};
    pub use sphere;
    pub use state::{
        settings::{AngleMode, ArcMode, BlendMode, DrawMode, FontStyle},
        PixState,
    };
    pub use texture::TextureId;
    pub use vector::Vector;
    pub use window::{Position, WindowBuilder, WindowId};
    pub use {circle, ellipse, rect, square};
    pub use {noise, random, vector};
    // Color macros
    pub use {color, hsb, hsl, rgb};
    // Shape macros
    pub use point;
    // Math macros
}
