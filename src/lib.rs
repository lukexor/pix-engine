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
//! Creating an application is as simple as implementing the only required trait of [AppState] for
//! your application: [AppState::on_update] which gets executed as often as possible. Within that
//! function you'll have access to a mutable [PixState] object which provides several methods for
//! changing settings, responding to events, and drawing to the screen.
//!
//! [SDL2]: https://crates.io/crates/sdl2/
//! [WASM]: https://www.rust-lang.org/what/wasm
//! [`Tetanes`]: https://crates.io/crates/tetanes
//! [NES]: https://en.wikipedia.org/wiki/Nintendo_Entertainment_System
//! [AppState]: crate::prelude::AppState
//! [AppState::on_update]: crate::prelude::AppState::on_update
//! [PixState]: crate::prelude::PixState
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
//!         s.background(220);
//!         s.circle([10, 10, 100])?;
//!         Ok(())
//!     }
//!
//!     fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!         // Main render loop. Called as often as possible, or based on `target frame rate`.
//!         if s.mouse_pressed() {
//!             s.fill(0);
//!         } else {
//!             s.fill(255);
//!         }
//!         let m = s.mouse_pos();
//!         s.circle([m.x, m.y, 80])?;
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

/// Bundles static binary assets with crate.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const ASSETS: Dir<'_> = include_dir!("./assets");

#[macro_use]
pub mod color;
pub mod audio;
#[macro_use]
pub mod core;
pub mod event;
pub mod graphics;
pub mod image;
#[macro_use]
pub mod math;
pub mod renderer;

mod utils;

/// Exports most commonly used types, traits, and functions.
pub mod prelude {
    pub use self::core::{
        appstate::AppState,
        common::{Error as PixError, Result as PixResult},
        draw::{Draw, DrawPrimitive},
        engine::PixEngine,
        shape::{Circle, Ellipse, Line, Point, Quad, Rect, Shape, Sphere, Triangle},
        state::{
            settings::{AngleMode, ArcMode, BlendMode, DrawMode, FontStyle},
            PixState,
        },
        texture::TextureId,
        window::{Position, WindowBuilder, WindowId},
    };
    use super::*;
    pub use color::{constants::*, Color, ColorError, ColorMode};
    pub use event::{Axis, Button, Event, Key, KeyEvent, KeyMod, Mouse, WindowEvent};
    pub use graphics::lighting::{Light, LightSource};
    pub use image::{Image, PixelFormat};
    pub use math::{constants::*, map, random_rng, vector::Vector, Number, Primitive, Scalar};
    #[cfg(not(target_arch = "wasm32"))]
    pub use renderer::DEFAULT_ASSET_DIR;
    pub use sphere;
    // Shape macros
    pub use {circle, ellipse, point, rect, square};
    // Math macros
    pub use {noise, random, vector};
    // Color macros
    pub use {color, hsb, hsl, rgb};
}
