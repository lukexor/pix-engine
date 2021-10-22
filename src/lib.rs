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

/// Exports most commonly used 2D types, traits, and functions.
pub mod prelude {
    use super::*;

    pub use self::core::{
        appstate::AppState,
        color::{constants::*, Color, ColorMode, Error as ColorError},
        common::{Error as PixError, Result as PixResult},
        draw::Draw,
        engine::PixEngine,
        shape::{
            Contains, Ellipse, Intersects, Line, LineF2, LineI2, Point, PointF2, PointI2, Quad,
            QuadF2, QuadI2, Rect, Tri, TriF2, TriI2,
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
    pub use gui::{fonts::*, Font, FontSrc, Theme, ThemeBuilder};
    pub use image::{Image, PixelFormat};
    pub use math::{
        constants::*,
        map, random_rng,
        vector::{Vector, VectorF2, VectorI2},
        Float, Num, Scalar,
    };
    #[cfg(not(target_arch = "wasm32"))]
    pub use transform::Flipped;
    // Shape macros
    pub use {circle, ellipse, line_, point, quad, rect, square, tri};
    // Math macros
    pub use {noise, random, vector};
    // Color macros
    pub use {color, hsb, hsl, rgb};
}

/// Exports most commonly used 3D types, traits, and functions.
pub mod prelude_3d {
    use super::*;

    pub use self::core::shape::{
        LineF3, LineI3, PointF3, PointI3, QuadF3, QuadI3, Sphere, TriF3, TriI3,
    };
    pub use graphics::lighting::{Light, LightF3, LightSource};
    pub use math::vector::{VectorF3, VectorI3};
    pub use prelude::*;
    pub use sphere;
    // Shape macros
    pub use {circle, ellipse, line_, point, quad, rect, square, tri};
    // Math macros
    pub use {noise, random, vector};
    // Color macros
    pub use {color, hsb, hsl, rgb};
}
