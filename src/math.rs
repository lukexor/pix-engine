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
///
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

/// Linear interpolates values for a range of independent values based on depdendent values.
///
/// # Examples
///
/// ```
/// use pix_engine::prelude::*;
///
/// let x1 = 0.0;
/// let x2 = 4.0;
/// let y1 = 0.0;
/// let y2 = 14.0;
/// let values = lerp_map(x1, x2, y1, y2);
/// assert_eq!(values, vec![0.0, 3.5, 7.0, 10.5, 14.0]);
/// ```
pub fn lerp_map<T>(start1: T, end1: T, start2: T, end2: T) -> Vec<T>
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
        while i <= end1 {
            values.push(d);
            d += a;
            i += T::one();
        }
        values
    }
}

/// Constraints an integer value between a minimum and maximum value.
///
/// # Examples
///
/// ```
/// use pix_engine::prelude::*;
///
/// let v = 15;
/// assert_eq!(constrain(v, 0, 10), 10);
///
/// let v = -5;
/// assert_eq!(constrain(v, 0, 10), 0);
/// ```
pub fn constrain<T: Ord>(val: T, min: T, max: T) -> T {
    cmp::max(min, cmp::min(val, max))
}

/// Constraints a floating point value between a minimum and maximum value.
///
/// # Examples
///
/// ```
/// use pix_engine::prelude::*;
///
/// let v = 1.5;
/// assert_eq!(constrainf(v, 0.0, 1.0), 1.0);
///
/// let v = -2.0;
/// assert_eq!(constrainf(v, 0.0, 1.0), 0.0);
/// ```
pub fn constrainf<T: Real>(val: T, min: T, max: T) -> T {
    val.min(max).max(min)
}
