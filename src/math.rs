use rand::{self, Rng};

pub use vector::Vector;

mod vector;

/// The mode used to interpret angle parameters in draw functions. The default is Radians.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AngleMode {
    Radians,
    Degrees,
}

impl Default for AngleMode {
    fn default() -> Self {
        Self::Radians
    }
}

pub fn random<V: Into<i32>>(val: V) -> i32 {
    let val = val.into();
    rand::thread_rng().gen_range(0, val)
}
