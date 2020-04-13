use std::{borrow::Cow, error, fmt, io};

/// Result type for PixEngineError
pub type PixEngineResult<T> = std::result::Result<T, PixEngineError>;

/// Types of errors the PixEngine can return in a result.
#[derive(Debug)]
pub enum PixEngineError {
    IoError(io::Error),
    InvalidSetting {
        /// Invalid setting which caused the error
        setting: Cow<'static, str>,
        /// Message why the setting is invalid
        message: Cow<'static, str>,
    },
    Renderer(Cow<'static, str>),
    StateError(Cow<'static, str>),
    Other(Cow<'static, str>),
}

impl fmt::Display for PixEngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PixEngineError::*;
        match self {
            IoError(err) => write!(f, "{}", err),
            InvalidSetting { setting, message } => {
                write!(f, "invalid setting `{}`: {}", &setting, &message)
            }
            Renderer(desc) => write!(f, "renderer error: {}", &desc),
            StateError(desc) => write!(f, "state error: {}", &desc),
            Other(desc) => write!(f, "{}", &desc),
        }
    }
}

impl error::Error for PixEngineError {
    fn cause(&self) -> Option<&(dyn error::Error + 'static)> {
        use PixEngineError::*;
        match self {
            IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for PixEngineError {
    fn from(err: io::Error) -> PixEngineError {
        PixEngineError::IoError(err)
    }
}

impl From<String> for PixEngineError {
    fn from(err: String) -> PixEngineError {
        PixEngineError::Other(err.into())
    }
}

impl From<PixEngineError> for io::Error {
    fn from(err: PixEngineError) -> io::Error {
        match err {
            PixEngineError::IoError(err) => err,
            err => io::Error::new(io::ErrorKind::Other, err.to_string()),
        }
    }
}
