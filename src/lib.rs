#![deny(rust_2018_idioms, missing_copy_implementations)]

use std::{error, fmt};

pub mod event;
pub mod image;
pub mod pixel;

mod audio;
mod driver;
mod engine;
mod state;

pub use engine::PixEngine;
pub use image::Image;
pub use state::{draw, transform, AlphaMode, State, StateData, WindowId};

pub type PixEngineResult<T> = std::result::Result<T, PixEngineErr>;

pub struct PixEngineErr {
    description: String,
}

impl PixEngineErr {
    pub fn new<D: ToString>(desc: D) -> Self {
        Self {
            description: desc.to_string(),
        }
    }
}

impl fmt::Display for PixEngineErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl fmt::Debug for PixEngineErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ err: {}, file: {}, line: {} }}",
            self.description,
            file!(),
            line!(),
        )
    }
}

impl error::Error for PixEngineErr {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
impl From<std::io::Error> for PixEngineErr {
    fn from(err: std::io::Error) -> Self {
        Self::new(err)
    }
}
impl From<String> for PixEngineErr {
    fn from(err: String) -> Self {
        Self::new(err)
    }
}
