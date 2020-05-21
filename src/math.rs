//! Math related types, constants and utility functions.

use num_traits::{real::Real, Num};
use rand::{self, distributions::uniform::SampleUniform, Rng};
use std::{
    cmp,
    ops::{AddAssign, Range},
};

/// Returns a random number within a range.
pub fn random<T: Num + SampleUniform, V: Into<Range<T>>>(val: V) -> T {
    let val = val.into();
    rand::thread_rng().gen_range(val.start, val.end)
}

/// Returns a random number within a range.
///
/// # Examples
/// ```
/// use pix_engine::prelude::*;
///
/// let x = random!(100); // x will range from (0..100]
/// let y = random!(20, 50); // x will range from (20..50]
/// ```
#[macro_export]
macro_rules! random {
    ($v:expr) => {
        $crate::prelude::random(0..$v);
    };
    ($s:expr, $e:expr) => {
        $crate::prelude::random($s..$e);
    };
}

/// Interpolates values for a range of independent values based on depdendent values.
pub fn map<T>(start1: T, end1: T, start2: T, end2: T) -> Vec<T>
where
    T: Copy + Num + PartialOrd + AddAssign,
{
    if start1 == end1 {
        vec![start2]
    } else {
        let mut values = Vec::new();
        let a = (end2 - start2) / (end1 - start1);
        let mut d = start2;
        let mut i = start1;
        while i < end1 {
            values.push(d);
            d += a;
            i += T::one();
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
