//! Common crate functions and error types.

use crate::{
    core::{state, window},
    image, renderer,
};
use std::{borrow::Cow, error, fmt, io, result};

/// The result type for [PixEngine] operations.
///
/// [PixEngine]: crate::prelude::PixEngine
pub type Result<T> = result::Result<T, Error>;

/// The error type for [PixEngine] operations.
///
/// [PixEngine]: crate::prelude::PixEngine
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// An error from the underlying Renderer.
    RendererError(renderer::Error),
    /// An error from window operations.
    WindowError(window::Error),
    /// An error from [PixState](crate::prelude::PixState).
    StateError(state::Error),
    /// An error from [Image](crate::prelude::Image)
    ImageError(image::Error),
    /// An error from invalid type conversions.
    Conversion(Cow<'static, str>),
    /// I/O errors.
    IoError(io::Error),
    /// Unknown errors.
    Other(Cow<'static, str>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            Conversion(err) => write!(f, "conversion error: {}", err),
            Other(err) => write!(f, "unknown error: {}", err),
            _ => self.fmt(f),
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<&'static str> for Error {
    fn from(err: &'static str) -> Self {
        Error::Other(err.into())
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(err: std::num::TryFromIntError) -> Self {
        Error::Conversion(err.to_string().into())
    }
}
