#![doc = include_str!("../README.md")]
#![warn(
    anonymous_parameters,
    bare_trait_objects,
    clippy::branches_sharing_code,
    clippy::map_unwrap_or,
    clippy::match_wildcard_for_single_variants,
    // clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::needless_for_each,
    clippy::redundant_closure_for_method_calls,
    clippy::semicolon_if_nothing_returned,
    clippy::unreadable_literal,
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

/// Exports most commonly used types, traits, and functions.
pub mod prelude {
    pub use super::appstate::AppState;
    pub use super::audio::{AudioCallback, AudioDevice, AudioSpec, AudioSpecDesired, AudioStatus};
    pub use super::color::{Color, Mode as ColorMode};
    pub use super::draw::Draw;
    pub use super::engine::PixEngine;
    pub use super::error::{Error as PixError, Result as PixResult};
    pub use super::event::{
        Axis, ControllerButton, ControllerEvent, ControllerId, ControllerUpdate, Event, HatState,
        Key, KeyEvent, KeyMod, Mouse, WindowEvent,
    };
    pub use super::graphics::lighting::{Light, LightSource};
    pub use super::gui::theme::{self, Builder as ThemeBuilder, ColorType, Font, Theme};
    pub use super::image::{Image, PixelFormat};
    pub use super::math::{map, random_rng, Float, Num};
    pub use super::shape::{Contains, Ellipse, Intersects, Line, Point, Quad, Rect, Sphere, Tri};
    pub use super::state::{
        settings::{
            AngleMode, ArcMode, BlendMode, DrawMode, EllipseMode, FontStyle, ImageMode, RectMode,
        },
        PixState,
    };
    pub use super::texture::TextureId;
    pub use super::transform::Flipped;
    pub use super::vector::Vector;
    pub use super::window::{Cursor, Position, SystemCursor, WindowBuilder, WindowId};

    // Shape macros
    pub use {circle, ellipse, line_, point, quad, rect, sphere, square, tri};
    // Math macros
    pub use {noise, random, vector};
    // Color macros
    pub use {color, hsb, hsl, rgb};
}
