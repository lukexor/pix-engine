//! Math related types, constants and utility functions.

use num_traits::{real::Real, Num, NumCast};
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
        $crate::math::random(0..$v);
    };
    ($s:expr, $e:expr) => {
        $crate::math::random($s..$e);
    };
}

/// Returns a random floating point number within a range.
///
/// # Examples
///
/// ```
/// use pix_engine::prelude::*;
///
/// let x = randomf!(100.0); // x will range from (0.0..100.0]
/// let y = randomf!(20.0, 50.0); // x will range from (20.0..50.0]
/// ```
#[macro_export]
macro_rules! randomf {
    ($v:expr) => {
        $crate::math::random(0.0..$v);
    };
    ($s:expr, $e:expr) => {
        $crate::math::random($s..$e);
    };
}

/// Remaps a number from one range to another
///
/// # Example
///
/// ```
/// use pix_engine::prelude::*;
///
/// let value = 25;
/// let m = map(value, 0, 100, 0, 800);
/// assert_eq!(m, Some(200));
///
/// let value = 50.0;
/// let m = map(value, 0.0, 100.0, 0.0, 1.0);
/// assert_eq!(m, Some(0.5));
/// ```
pub fn map<T>(value: T, start1: T, end1: T, start2: T, end2: T) -> Option<T>
where
    T: Copy + NumCast + PartialOrd + AddAssign,
{
    let value = <f64 as NumCast>::from(value)?;
    let start1 = <f64 as NumCast>::from(start1)?;
    let end1 = <f64 as NumCast>::from(end1)?;
    let start2 = <f64 as NumCast>::from(start2)?;
    let end2 = <f64 as NumCast>::from(end2)?;
    let new_val = (value - start1) / (end1 - start1) * (end2 - start2) + start2;
    let map = if start2 < end2 {
        constrainf(new_val, start2, end2)
    } else {
        constrainf(new_val, end2, start2)
    };
    T::from(map)
}

/// Linear interpolates between two values by a given amount.
///
/// # Examples
///
/// ```
/// use pix_engine::math::lerp;
///
/// let start = 0.0;
/// let end = 5.0;
/// let amount = 0.5;
/// let value = lerp(start, end, amount);
/// assert_eq!(value, 2.5);
/// ```
pub fn lerp<T>(start: T, end: T, amount: T) -> T
where
    T: Copy + Num + PartialOrd,
{
    (T::one() - amount) * start + amount * end
}

/// Linear interpolates values for a range of independent values based on depdendent values.
///
/// # Examples
///
/// ```
/// use pix_engine::math::lerp_map;
///
/// let x1 = 0;
/// let x2 = 5;
/// let y1 = 0;
/// let y2 = 10;
/// let values = lerp_map(x1, x2, y1, y2);
/// assert_eq!(values, vec![0, 2, 4, 6, 8, 10]);
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

/// Collision specific utility functions.
pub mod collision {
    /// Collision detection for basic circle shapes
    /// Detects whether a 2D point (x, y) lies inside circle located at (cx, cy) of radius r.
    pub fn inside_circle(x: i32, y: i32, cx: i32, cy: i32, r: u32) -> bool {
        ((x - cx).pow(2) + (y - cy).pow(2)) < r.pow(2) as i32
    }
}

#[allow(missing_docs)]
pub mod constants {
    pub const INFINITY: f64 = std::f64::INFINITY;
    pub const SQRT_2: f64 = std::f64::consts::SQRT_2;

    pub const HALF_PI: f64 = std::f64::consts::PI / 2.0;
    pub const PI: f64 = std::f64::consts::PI;
    pub const QUARTER_PI: f64 = std::f64::consts::PI / 4.0;
    pub const TAU: f64 = 2.0 * std::f64::consts::PI;
    pub const TWO_PI: f64 = TAU;
}
