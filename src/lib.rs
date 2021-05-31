#![warn(
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused,
    variant_size_differences
)]
// TODO: Fix intra-doc links

//! # Pix-Engine
//!
//! ## Summary
//!
//! A cross-platform graphics/UI engine framework for simple games, visualizations, and graphics
//! demos.
//!
//! The goal of this library is to be simpler to setup and use for graphics or algorithm
//! exploration than larger graphics libraries.
//!
//! This is more than just a toy project, however, and can be used to drive powerful
//! applications. The primary use of this project is in the [`Tetanes`] NES emulator project.
//!
//! ## Usage
//!
//! TODO
//!

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
// TODO: Vertex { x, y, z, u, v }

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
    pub use renderer::{Error as RendererError, Position, Result as RendererResult};
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
    pub use {random, vector};
    // Shape macros
    pub use {circle, ellipse, point, rect, square};
}
