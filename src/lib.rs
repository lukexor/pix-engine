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

#[macro_use]
pub mod color;
#[macro_use]
pub mod math;

/// Re-exports most commonly used structs, traits, and functions.
pub mod prelude {
    use super::*;
    pub use color::{constants::*, Hsv, Rgb};
    pub use hsv;
    pub use math::*;
    pub use random;
    pub use rgb;
}
