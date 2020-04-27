use crate::{image, renderer, state};
use std::{borrow::Cow, error, fmt, io};

pub use constants::*;

pub mod constants {
    //! Commonly used constants like `PI` and `TWO_PI`.
    pub use std::f64::consts::*;
    pub use std::f64::*;
    /// π/4. Clone of `FRAC_PI_4`.
    pub const QUARTER_PI: f64 = FRAC_PI_4;
    /// π/2. Clone of `FRAC_PI_2`.
    pub const HALF_PI: f64 = FRAC_PI_2;
    /// 2.0 * π
    pub const TWO_PI: f64 = 2.0 * PI;
    /// τ (or 2.0 * π). Clone of `TWO_PI`.
    pub const TAU: f64 = TWO_PI;
}

pub type Scalar = f64;

/// Result type for `PixEngine` Errors.
pub type Result<T> = std::result::Result<T, Error>;

/// Types of errors the `PixEngine` can return in a result.
#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    InvalidSetting {
        /// Invalid setting which caused the error.
        setting: Cow<'static, str>,
        /// Message why the setting is invalid.
        message: Cow<'static, str>,
    },
    ImageError(image::Error),
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
            ImageError(err) => write!(f, "image error: {}", &err),
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

impl From<image::Error> for Error {
    fn from(err: image::Error) -> Error {
        Error::ImageError(err)
    }
}
