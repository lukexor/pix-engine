use crate::{renderer, state};
use std::{borrow::Cow, error, fmt, io};

/// Result type for PixEngine Errors.
pub type Result<T> = std::result::Result<T, Error>;

/// Types of errors the PixEngine can return in a result.
#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    InvalidSetting {
        /// Invalid setting which caused the error
        setting: Cow<'static, str>,
        /// Message why the setting is invalid
        message: Cow<'static, str>,
    },
    RendererError(renderer::Error),
    StateError(state::Error),
    Other(Cow<'static, str>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            IoError(err) => write!(f, "io error: {}", err),
            InvalidSetting { setting, message } => {
                write!(f, "invalid setting `{}`: {}", &setting, &message)
            }
            RendererError(err) => write!(f, "renderer error: {}", &err),
            StateError(err) => write!(f, "state error: {}", &err),
            Other(desc) => write!(f, "{}", &desc),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        match err {
            Error::IoError(err) => err,
            err => io::Error::new(io::ErrorKind::Other, err),
        }
    }
}

impl From<renderer::Error> for Error {
    fn from(err: renderer::Error) -> Error {
        Error::RendererError(err)
    }
}

impl From<state::Error> for Error {
    fn from(err: state::Error) -> Error {
        Error::StateError(err)
    }
}
