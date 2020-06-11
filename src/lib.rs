#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

//! # Pix-Engine
//!
//! ## Summary
//!
//! A simple, cross-platform graphics/UI engine framework with a minimal interface.
//!
//! TODO
//!
//! ## Usage
//!
//! TODO
//!

use include_dir::{include_dir, Dir};

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
pub mod state;
#[macro_use]
pub mod vector;

mod common;

pub use prelude::{PixEngine, PixError, PixResult, State, Stateful};

/// Re-exports most commonly used structs, traits, and functions.
pub mod prelude {
    use super::*;
    pub use color::{constants::*, Color, Hsv, Rgb};
    pub use common::{Error as PixError, Result as PixResult};
    pub use draw::{DrawMode::*, Rect};
    pub use engine::PixEngine;
    pub use event::*;
    pub use image::Image;
    pub use math::{collision, constants::*, constrain, constrainf, map};
    pub use state::{State, Stateful};
    pub use vector::Vector;
    pub use {hsv, random, randomf, rgb, vector};
}
