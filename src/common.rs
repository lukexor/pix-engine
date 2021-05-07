use crate::{image, renderer, state};
use std::borrow::Cow;

/// Result wrapper for `PixEngine` Errors.
pub type Result<T> = std::result::Result<T, Error>;

/// Types of errors the `PixEngine` can return in a `Result`.
#[derive(Debug)]
pub enum Error {
    /// Indicates an invalid `PixEngineBuilder` setting.
    InvalidSetting,
    /// An error from the underlying `Renderer`.
    RendererError(renderer::Error),
    /// An error from the `Stateful` application.
    StateError(state::Error),
    /// An error from 'Image'
    ImageError(image::Error),
    /// Unknown errors.
    Other(Cow<'static, str>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            InvalidSetting => write!(f, "invalid PixEngine setting"), // TODO add setting to this
            RendererError(e) => e.fmt(f),
            StateError(e) => e.fmt(f),
            ImageError(e) => e.fmt(f),
            Other(e) => write!(f, "unknown error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
