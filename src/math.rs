//! Math related types, constants and utility functions.

use num::Num;
use rand::{self, distributions::uniform::SampleUniform, Rng};

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

/// Returns a random number within a range.
///
/// # Examples
/// ```
/// use pix_engine::prelude::*;
///
/// let x = random(100); // x will range from (0..100]
/// let y = random((20, 50)); // x will range from (20..50]
/// ```
pub fn random<T: Num + SampleUniform, V: Into<Range<T>>>(val: V) -> T {
    let val = val.into();
    rand::thread_rng().gen_range(val.min, val.max)
}

/// A generic range type over anything that implements the `Num` trait.
pub struct Range<T: Num> {
    pub min: T,
    pub max: T,
}

impl<T: Num> From<T> for Range<T> {
    fn from(max: T) -> Self {
        Range {
            min: T::zero(),
            max,
        }
    }
}

impl<T: Num> From<(T, T)> for Range<T> {
    fn from((min, max): (T, T)) -> Self {
        Range { min, max }
    }
}

/// Interpolates values for a range of independent values based on depdendent values.
pub fn interpolate(ind_start: i32, dep_start: i32, ind_end: i32, dep_end: i32) -> Vec<f64> {
    if ind_start == ind_end {
        vec![dep_start as f64]
    } else {
        let mut values = Vec::new();
        let a = (dep_end - dep_start) as f64 / (ind_end - ind_start) as f64;
        let mut d = dep_start as f64;
        for i in ind_start..ind_end {
            values.push(d);
            d += a;
        }
        values
    }
}
