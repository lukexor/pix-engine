use crate::{image::ImageError, renderer::RendererError, state::StateError};
use std::{borrow::Cow, error, fmt, result};

/// `PixEngine` Result.
pub type PixResult<T> = result::Result<T, PixError>;

/// `PixEngine` errors returned in `PixResult`.
#[non_exhaustive]
#[derive(Debug)]
pub enum PixError {
    /// An error from the underlying `Renderer`.
    RendererError(RendererError),
    /// An error from the `Stateful` application.
    StateError(StateError),
    /// An error from 'Image'
    ImageError(ImageError),
    /// Unknown errors.
    Other(Cow<'static, str>),
}

impl fmt::Display for PixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PixError::*;
        match self {
            RendererError(e) => e.fmt(f),
            StateError(e) => e.fmt(f),
            ImageError(e) => e.fmt(f),
            Other(e) => write!(f, "unknown error: {}", e),
        }
    }
}

impl error::Error for PixError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
