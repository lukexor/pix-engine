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
