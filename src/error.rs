//! Errors that this crate can return.

use crate::prelude::*;
use std::{ffi::OsString, io};
use thiserror::Error;

/// The result type for [PixEngine] operations.
///
/// [PixEngine]: crate::prelude::PixEngine
pub type Result<T> = anyhow::Result<T, anyhow::Error>;

/// The error type for [PixEngine] operations.
///
/// [PixEngine]: crate::prelude::PixEngine
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum Error {
    /// Invalid screen [Position].
    #[error("invalid screen position: `({0:?}, {1:?})`")]
    InvalidPosition(Position, Position),
    /// Invalid Texture ID.
    #[error("invalid texture id `{0}`")]
    InvalidTexture(TextureId),
    /// Invalid Window ID.
    #[error("invalid window id `{0}`")]
    InvalidWindow(WindowId),
    /// Hexadecimal [Color] string parsing error.
    #[error("hexadecimal color string parsing error")]
    ParseColorError,
    /// Invalid [Color] slice.
    #[error("invalid color slice")]
    InvalidColorSlice,
    /// Invalid [Image].
    #[error(
        "invalid image {{ width: {width}, height: {height}, size: {size}, format: {format:?} }}"
    )]
    InvalidImage {
        /// `Image` width.
        width: u32,
        /// `Image` height.
        height: u32,
        /// Size in bytes.
        size: usize,
        /// `Image` format.
        format: PixelFormat,
    },
    /// Unsupported [Image] format.
    #[error("unsupported image format {{ bit_depth: {bit_depth:?}, color_type: {color_type:?} }}")]
    UnsupportedImageFormat {
        /// `Image` [png::BitDepth].
        bit_depth: png::BitDepth,
        /// `Image` [png::ColorType].
        color_type: png::ColorType,
    },
    /// Unsupported file type.
    #[error("unsupported file type with extension `{0:?}`")]
    UnsupportedFileType(Option<OsString>),
    /// Graphics renderer error.
    #[error("renderer error: {0}")]
    Renderer(String),
    /// I/O errors.
    #[error(transparent)]
    Io(#[from] io::Error),
    /// Other, unspecified errors.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
