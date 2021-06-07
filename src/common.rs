use crate::{image, renderer, state};
use std::{borrow::Cow, error, fmt, result};

/// The result type for [PixEngine](crate::prelude::PixEngine) operations.
pub type Result<T> = result::Result<T, Error>;

/// The error type for [PixEngine](crate::prelude::PixEngine) operations.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// An error from the underlying [Renderer](crate::prelude::Renderer).
    RendererError(renderer::Error),
    /// An error from [PixState](crate::prelude::PixState).
    StateError(state::Error),
    /// An error from [Image](crate::prelude::Image)
    ImageError(image::Error),
    /// Unknown errors.
    Other(Cow<'static, str>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            RendererError(err) => err.fmt(f),
            StateError(err) => err.fmt(f),
            ImageError(err) => err.fmt(f),
            Other(err) => write!(f, "unknown error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            RendererError(err) => err.source(),
            StateError(err) => err.source(),
            ImageError(err) => err.source(),
            _ => None,
        }
    }
}
