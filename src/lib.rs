#![doc = include_str!("../README.md")]
#![warn(
    anonymous_parameters,
    bare_trait_objects,
    deprecated_in_future,
    ellipsis_inclusive_range_patterns,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    rustdoc::bare_urls,
    rustdoc::broken_intra_doc_links,
    rustdoc::invalid_html_tags,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::private_intra_doc_links,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused,
    variant_size_differences
)]
#![doc(
    html_root_url = "https://docs.rs/pix-engine/latest",
    html_favicon_url = "",
    html_logo_url = ""
)]

#[macro_use]
pub mod color;
pub mod appstate;
pub mod draw;
pub mod engine;
pub mod error;
#[macro_use]
pub mod shape;
pub mod audio;
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
#[cfg(feature = "serde")]
pub mod serialize;
pub mod transform;

mod utils;

/// Exports most commonly used types, traits, and functions.
pub mod prelude {
    use super::*;

    pub use appstate::AppState;
    pub use audio::AudioStatus;
    pub use color::{Color, Mode as ColorMode};
    pub use draw::Draw;
    pub use engine::PixEngine;
    pub use error::{Error as PixError, Result as PixResult};
    pub use event::{
        Axis, ControllerButton, ControllerEvent, ControllerId, ControllerUpdate, Event, HatState,
        Key, KeyEvent, KeyMod, Mouse, WindowEvent,
    };
    pub use graphics::lighting::{Light, LightSource};
    pub use gui::theme::{self, ColorType, Font, Theme};
    pub use image::{Image, PixelFormat};
    pub use math::{map, random_rng, Float, Num, Scalar};
    pub use shape::{Contains, Ellipse, Intersects, Line, Point, PointI2, Quad, Rect, Sphere, Tri};
    pub use state::{
        settings::{
            AngleMode, ArcMode, BlendMode, DrawMode, EllipseMode, FontStyle, ImageMode, RectMode,
        },
        PixState,
    };
    pub use texture::TextureId;
    pub use transform::Flipped;
    pub use vector::Vector;
    pub use window::{Cursor, Position, SystemCursor, WindowBuilder, WindowId};

    // Shape macros
    pub use {circle, ellipse, line_, point, quad, rect, sphere, square, tri};
    // Math macros
    pub use {noise, random, vector};
    // Color macros
    pub use {color, hsb, hsl, rgb};
}
