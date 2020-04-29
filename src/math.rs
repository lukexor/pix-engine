//! Math related types, constants and utility functions.

use num::{traits::real::Real, Num};
use rand::{self, distributions::uniform::SampleUniform, Rng};
use std::cmp;

pub use vector::Vector;
pub mod prelude {
    pub use super::{map, random, vector::Vector};
}

pub mod vector;

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
// TODO make generic
pub fn map(start1: i32, end1: i32, start2: i32, end2: i32) -> Vec<f64> {
    if start1 == end1 {
        vec![start2 as f64]
    } else {
        let mut values = Vec::new();
        let a = (end2 - start2) as f64 / (end1 - start1) as f64;
        let mut d = start2 as f64;
        for i in start1..end1 {
            values.push(d);
            d += a;
        }
        values
    }
}

/// Constraints a number value between a minimum and maximum value.
pub fn constrain<T: Ord>(val: T, min: T, max: T) -> T {
    cmp::max(min, cmp::min(val, max))
}

/// Constraints a floating point value between a minimum and maximum value.
pub fn constrainf<T: Real>(val: T, min: T, max: T) -> T {
    val.min(max).max(min)
}
