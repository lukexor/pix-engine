#![doc = include_str!("../README.md")]
#![deny(
    missing_docs,
    rustdoc::missing_doc_code_examples,
    rustdoc::invalid_html_tags
)]
#![warn(
    unused,
    deprecated_in_future,
    unreachable_pub,
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
    html_logo_url = ""
)]

#[macro_use]
pub mod color;
pub mod appstate;
pub mod draw;
pub mod engine;
#[macro_use]
pub mod shape;
pub mod audio;
pub mod common;
pub mod event;
pub mod graphics;
pub mod image;
pub mod ops;
pub mod state;
pub mod texture;
pub mod window;
#[macro_use]
pub mod math;
#[macro_use]
pub mod vector;
pub mod gui;
pub mod renderer;
pub mod transform;

mod utils;

/// Exports most commonly used types, traits, and functions.
pub mod prelude {
    use super::*;

    pub use appstate::AppState;
    pub use color::{constants::*, Color, ColorMode, Error as ColorError};
    pub use common::{Error as PixError, Result as PixResult};
    pub use draw::Draw;
    pub use engine::PixEngine;
    pub use event::{Axis, ControllerButton, Event, Key, KeyEvent, KeyMod, Mouse, WindowEvent};
    pub use graphics::lighting::{Light, LightF3, LightSource};
    pub use gui::theme::{fonts::*, Font, FontSrc, Theme, ThemeBuilder};
    pub use image::{Image, PixelFormat};
    pub use math::{constants::*, map, random_rng, Float, Num, Scalar};
    pub use shape::{
        Contains, Ellipse, Intersects, Line, LineF2, LineF3, LineI2, LineI3, Point, PointF2,
        PointF3, PointI2, PointI3, Quad, QuadF2, QuadF3, QuadI2, QuadI3, Rect, Sphere, Tri, TriF2,
        TriF3, TriI2, TriI3,
    };
    pub use state::{
        settings::{
            AngleMode, ArcMode, BlendMode, DrawMode, EllipseMode, FontStyle, ImageMode, RectMode,
        },
        PixState,
    };
    pub use texture::TextureId;
    #[cfg(not(target_arch = "wasm32"))]
    pub use transform::Flipped;
    pub use vector::{Vector, VectorF2, VectorF3, VectorI2, VectorI3};
    pub use window::{Cursor, Position, SystemCursor, WindowBuilder, WindowId};

    // Shape macros
    pub use {circle, ellipse, line_, point, quad, rect, sphere, square, tri};
    // Math macros
    pub use {noise, random, vector};
    // Color macros
    pub use {color, hsb, hsl, rgb};
}
