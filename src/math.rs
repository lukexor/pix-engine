//! Math related types, constants and utility functions.

use crate::vector::Vector;
use lazy_static::lazy_static;
use num_traits::{Num, NumCast};
use rand::{self, distributions::uniform::SampleUniform, Rng};
use std::ops::{AddAssign, Range};

/// Default type for math calculations
pub type Scalar = f64;

const PERLIN_YWRAPB: usize = 4;
const PERLIN_YWRAP: usize = 1 << PERLIN_YWRAPB;
const PERLIN_ZWRAPB: usize = 8;
const PERLIN_ZWRAP: usize = 1 << PERLIN_ZWRAPB;
const PERLIN_SIZE: usize = 4095;

lazy_static! {
    static ref PERLIN: Vec<f64> = {
        let mut perlin = Vec::with_capacity(PERLIN_SIZE + 1);
        for _ in 0..PERLIN_SIZE + 1 {
            perlin.push(random(1.0));
        }
        perlin
    };
}

fn scaled_cosine(i: f64) -> f64 {
    0.5 * (1.0 - (i * constants::PI).cos())
}

/// Returns a random number within a range.
pub fn random_rng<T, V>(val: V) -> T
where
    T: Num + SampleUniform + PartialOrd,
    V: Into<Range<T>>,
{
    let val = val.into();
    rand::thread_rng().gen_range(val)
}

/// Returns a random number between zero and the given value.
pub fn random<T>(val: T) -> T
where
    T: Num + SampleUniform + PartialOrd,
{
    if val > T::zero() {
        random_rng(T::zero()..val)
    } else {
        random_rng(val..T::zero())
    }
}

/// Returns the Perlin noise value at specified coordinates.
#[allow(clippy::many_single_char_names)]
pub fn noise<V>(v: V) -> f64
where
    V: Into<Vector>,
{
    let v = v.into();

    let x = v.x.abs();
    let y = v.y.abs();
    let z = v.z.abs();

    let mut xi = x.floor() as usize;
    let mut yi = y.floor() as usize;
    let mut zi = z.floor() as usize;

    let mut xf = x - xi as f64;
    let mut yf = y - yi as f64;
    let mut zf = z - zi as f64;
    let (mut rxf, mut ryf);

    let mut noise_result = 0.0;
    let mut ampl = 0.5;

    let (mut n1, mut n2, mut n3);

    // TODO: Make a state setting
    let perlin_octaves = 4; // default to medium smooth
    let perlin_amp_falloff = 0.5; // 50% reduction/octave
    for _ in 0..perlin_octaves {
        let mut of = xi + (yi << PERLIN_YWRAPB) + (zi << PERLIN_ZWRAPB);

        rxf = scaled_cosine(xf);
        ryf = scaled_cosine(yf);

        n1 = PERLIN[of & PERLIN_SIZE];
        n1 += rxf * (PERLIN[(of + 1) & PERLIN_SIZE] - n1);
        n2 = PERLIN[(of + PERLIN_YWRAP) & PERLIN_SIZE];
        n2 += rxf * (PERLIN[(of + PERLIN_YWRAP + 1) & PERLIN_SIZE] - n2);
        n1 += ryf * (n2 - n1);

        of += PERLIN_ZWRAP;
        n2 = PERLIN[of & PERLIN_SIZE];
        n2 += rxf * (PERLIN[(of + 1) & PERLIN_SIZE] - n2);
        n3 = PERLIN[(of + PERLIN_YWRAP) & PERLIN_SIZE];
        n3 += rxf * (PERLIN[(of + PERLIN_YWRAP + 1) & PERLIN_SIZE] - n3);
        n2 += ryf * (n3 - n2);

        n1 += scaled_cosine(zf) * (n2 - n1);

        noise_result += n1 * ampl;
        ampl *= perlin_amp_falloff;
        xi <<= 1;
        xf *= 2.0;
        yi <<= 1;
        yf *= 2.0;
        zi <<= 1;
        zf *= 2.0;

        if xf >= 1.0 {
            xi += 1;
            xf -= 1.0;
        }
        if yf >= 1.0 {
            yi += 1;
            yf -= 1.0;
        }
        if zf >= 1.0 {
            zi += 1;
            zf -= 1.0;
        }
    }
    noise_result
}

/// Returns a random number within a range.
///
/// # Examples
///
/// ```
/// use pix_engine::prelude::*;
///
/// let x = random!(); // x will range from (0.0..1.0]
/// assert!(x >= 0.0 && x < 1.0);
///
/// let x = random!(100); // x will range from (0..100]
/// assert!(x >= 0 && x < 100);
/// let y = random!(20, 50); // x will range from (20..50]
/// assert!(y >= 20 && y < 50);
///
/// let x = random!(100.0); // x will range from (0.0..100.0]
/// assert!(x >= 0.0 && x < 100.0);
/// let y = random!(20.0, 50.0); // x will range from (20.0..50.0]
/// assert!(y >= 20.0 && y < 50.0);
/// ```
#[macro_export]
macro_rules! random {
    () => {
        $crate::math::random(1.0)
    };
    ($v:expr) => {
        $crate::math::random($v)
    };
    ($s:expr, $e:expr$(,)?) => {{
        // TODO: move this into a function
        let s = $s;
        let e = $e;
        if s > e {
            $crate::math::random_rng(e..s)
        } else {
            $crate::math::random_rng(s..e)
        }
    }};
}

/// Returns the Perlin noise value at specified coordinates.
///
/// # Examples
///
/// TODO
#[macro_export]
macro_rules! noise {
    ($s:expr, $x:expr$(,)?) => {
        $crate::math::noise($s, $x, 0.0, 0.0)
    };
    ($s:expr, $x:expr, $y:expr$(,)?) => {
        $crate::math::noise($s, $x, $y, 0.0)
    };
    ($s:expr, $x:expr, $y:expr, $z:expr$(,)?) => {
        $crate::math::noise($s, $x, $y, $z)
    };
}

/// Remaps a number from one range to another.
///
/// Map range defaults to 0.0...Scalar::MAX in the event casting to Scalar fails.
/// NaN will result in the max mapped value.
///
/// # Example
///
/// ```
/// use pix_engine::prelude::*;
///
/// let value = 25;
/// let m = map(value, 0, 100, 0, 800);
/// assert_eq!(m, 200);
///
/// let value = 50.0;
/// let m = map(value, 0.0, 100.0, 0.0, 1.0);
/// assert_eq!(m, 0.5);
///
/// let value = Scalar::NAN;
/// let m = map(value, 0.0, 100.0, 0.0, 1.0);
/// assert!(m.is_nan());
///
/// let value = Scalar::INFINITY;
/// let m = map(value, 0.0, 100.0, 0.0, 1.0);
/// assert_eq!(m, 1.0);
///
/// let value = Scalar::NEG_INFINITY;
/// let m = map(value, 0.0, 100.0, 0.0, 1.0);
/// assert_eq!(m, 0.0);
/// ```
pub fn map<T>(value: T, start1: T, end1: T, start2: T, end2: T) -> T
where
    T: Copy + NumCast + PartialOrd + AddAssign,
{
    let default = end1;
    let start1: Scalar = NumCast::from(start1).unwrap_or(0.0);
    let end1: Scalar = NumCast::from(end1).unwrap_or(Scalar::MAX);
    let start2: Scalar = NumCast::from(start2).unwrap_or(0.0);
    let end2: Scalar = NumCast::from(end2).unwrap_or(Scalar::MAX);
    let value: Scalar = NumCast::from(value).unwrap_or(start1);
    let new_val = (value - start1) / (end1 - start1) * (end2 - start2) + start2;
    let mapped_val = if start2 < end2 {
        new_val.clamp(start2, end2)
    } else {
        new_val.clamp(end2, start2)
    };
    T::from(mapped_val).unwrap_or(default)
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
    T: Copy + Num + NumCast + PartialOrd + AddAssign,
{
    if start1 == end1 {
        vec![start2]
    } else {
        let size: usize = NumCast::from(end1 - start1).unwrap_or(4);
        let mut values = Vec::with_capacity(size);
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

/// Math constants
pub mod constants {
    pub use std::f64::consts::*;
    pub use std::f64::INFINITY;
    pub use std::f64::NAN;

    /// PI / 2
    pub const HALF_PI: f64 = std::f64::consts::FRAC_PI_2;
    /// PI / 4
    pub const QUARTER_PI: f64 = std::f64::consts::FRAC_PI_4;
    /// 2 PI
    pub const TWO_PI: f64 = TAU;
}
