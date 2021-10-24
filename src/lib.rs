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

#[cfg(not(target_arch = "wasm32"))]
use include_dir::{include_dir, Dir};

/// Bundles static binary assets with crate.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const ASSETS: Dir<'_> = include_dir!("./assets");

#[macro_use]
pub mod core;

pub mod audio;
pub mod event;
pub mod graphics;
pub mod image;
#[macro_use]
pub mod math;
pub mod gui;
pub mod renderer;
pub mod transform;

mod utils;

/// Exports most commonly used types, traits, and functions.
pub mod prelude {
    use super::*;

    pub use self::core::{
        appstate::AppState,
        color::{constants::*, Color, ColorMode, Error as ColorError},
        common::{Error as PixError, Result as PixResult},
        draw::Draw,
        engine::PixEngine,
        shape::{
            Contains, Ellipse, Intersects, Line, LineF2, LineF3, LineI2, LineI3, Point, PointF2,
            PointF3, PointI2, PointI3, Quad, QuadF2, QuadF3, QuadI2, QuadI3, Rect, Sphere, Tri,
            TriF2, TriF3, TriI2, TriI3,
        },
        state::{
            settings::{
                AngleMode, ArcMode, BlendMode, DrawMode, EllipseMode, FontStyle, ImageMode,
                RectMode,
            },
            PixState,
        },
        texture::TextureId,
        window::{Cursor, Position, SystemCursor, WindowBuilder, WindowId},
    };
    pub use event::{Axis, ControllerButton, Event, Key, KeyEvent, KeyMod, Mouse, WindowEvent};
    pub use graphics::lighting::{Light, LightF3, LightSource};
    pub use gui::{fonts::*, Font, FontSrc, Theme, ThemeBuilder};
    pub use image::{Image, PixelFormat};
    pub use math::{
        constants::*,
        map, random_rng,
        vector::{Vector, VectorF2, VectorF3, VectorI2, VectorI3},
        Float, Num, Scalar,
    };
    #[cfg(not(target_arch = "wasm32"))]
    pub use transform::Flipped;

    // Shape macros
    pub use {circle, ellipse, line_, point, quad, rect, sphere, square, tri};
    // Math macros
    pub use {noise, random, vector};
    // Color macros
    pub use {color, hsb, hsl, rgb};
}
