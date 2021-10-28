//! Common crate functions and error types.

use crate::{
    image,
    renderer::{Error as RendererError, WindowError},
    state,
};
use std::{borrow::Cow, error, fmt, io, result};

/// The result type for [PixEngine] operations.
///
/// [PixEngine]: crate::prelude::PixEngine
pub type Result<T> = result::Result<T, Error>;

/// 1
#[derive(Debug)]
pub struct MyType1<T, const N: usize>([T; N]);
/// 2
#[derive(Debug)]
pub struct MyType2<T>([T; 3]);
/// 3
#[derive(Debug)]
pub struct MyType3<T, const N: usize>([MyType1<T, N>; 4]);

/// The error type for [PixEngine] operations.
///
/// [PixEngine]: crate::prelude::PixEngine
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// An error from the underlying Renderer.
    RendererError(RendererError),
    /// An error from window operations.
    WindowError(WindowError),
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

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            RendererError(err) => err.source(),
            WindowError(err) => err.source(),
            StateError(err) => err.source(),
            ImageError(err) => err.source(),
            IoError(err) => err.source(),
            _ => None,
        }
    }
}

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
