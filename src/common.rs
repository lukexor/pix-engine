use crate::{renderer, state};
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
    /// Unknown errors.
    Other(Cow<'static, str>),
}

impl Error {
    /// Creates a renderer error from anything that implements Display.
    pub fn renderer<E: std::fmt::Display>(err: E) -> Self {
        Self::Other(Cow::from(err.to_string()))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            InvalidSetting => write!(f, "invalid PixEngine setting"),
            RendererError(e) => e.fmt(f),
            StateError(e) => e.fmt(f),
            Other(e) => write!(f, "unknown error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Other(Cow::from(err))
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(err: std::num::TryFromIntError) -> Error {
        Error::Other(Cow::from(err.to_string()))
    }
}
