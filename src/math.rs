//! Math functions and constants.

use lazy_static::lazy_static;
use num_traits::{Num, NumCast};
use rand::{self, distributions::uniform::SampleUniform, Rng};
use std::ops::{AddAssign, Range};
use vector::Vector;

#[macro_use]
pub mod vector;

/// Default primitive type used for objects and shapes.
pub type Primitive = i32;

/// Default scalar type used for math operations.
pub type Scalar = f64;

/// Default math constants.
pub mod constants {
    pub use std::f64::consts::*;
}

/// Default number trait used for objects and shapes.
pub trait Number: Num + Copy + PartialOrd {}

impl<T> Number for T where T: Num + Copy + PartialOrd {}

const PERLIN_YWRAPB: usize = 4;
const PERLIN_YWRAP: usize = 1 << PERLIN_YWRAPB;
const PERLIN_ZWRAPB: usize = 8;
const PERLIN_ZWRAP: usize = 1 << PERLIN_ZWRAPB;
const PERLIN_SIZE: usize = 4095;

lazy_static! {
    static ref PERLIN: Vec<Scalar> = {
        let mut perlin = Vec::with_capacity(PERLIN_SIZE + 1);
        for _ in 0..PERLIN_SIZE + 1 {
            perlin.push(random(1.0));
        }
        perlin
    };
}

/// Returns a random number within a range.
///
/// # Examples
///
/// ```
/// use pix_engine::math::random_rng;
///
/// let x = random_rng(0.0..1.0); // x will range from (0.0..1.0]
/// assert!(x >= 0.0 && x < 1.0);
///
/// let x = random_rng(20..50); // x will range from (20..50]
/// assert!(x >= 20 && x < 50);
pub fn random_rng<T, R>(val: R) -> T
where
    T: SampleUniform + PartialOrd,
    R: Into<Range<T>>,
{
    let val = val.into();
    rand::thread_rng().gen_range(val)
}

/// Returns a random number between `0` and a given `value`.
///
/// # Examples
///
/// ```
/// use pix_engine::math::random;
///
/// let x = random(100); // x will range from (0..100]
/// assert!(x >= 0 && x < 100);
///
/// let x = random(100.0); // x will range from (0.0..100.0]
/// assert!(x >= 0.0 && x < 100.0);
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

/// Returns the [Perlin noise](https://en.wikipedia.org/wiki/Perlin_noise) value at specified coordinates.
///
/// # Examples
///
/// ```
/// use pix_engine::math::noise;
///
/// let n = noise([5.0]);
/// assert!(n >= 0.0 && n < 1.0);
///
/// let n = noise([2.0, 1.5]);
/// assert!(n >= 0.0 && n < 1.0);
///
/// let n = noise([2.0, 1.5, 3.0]);
/// assert!(n >= 0.0 && n < 1.0);
/// ```
#[allow(clippy::many_single_char_names)]
pub fn noise<V>(v: V) -> Scalar
where
    V: Into<Vector<Scalar>>,
{
    let v = v.into();

    let x = v.x().abs();
    let y = v.y().abs();
    let z = v.z().abs();

    let mut xi: usize = x.trunc() as usize;
    let mut yi: usize = y.trunc() as usize;
    let mut zi: usize = z.trunc() as usize;

    let mut xf = x.fract();
    let mut yf = y.fract();
    let mut zf = z.fract();
    let (mut rxf, mut ryf);

    let mut noise_result = 0.0;
    let mut ampl = 0.5;

    let (mut n1, mut n2, mut n3);

    let scaled_cosine = |i: Scalar| 0.5 * (1.0 - (i - constants::PI).cos());

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
/// # use pix_engine::prelude::*;
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
        $crate::math::random_rng($s..$e)
    }};
}

/// Returns the [Perlin noise](https://en.wikipedia.org/wiki/Perlin_noise) value at specified
/// coordinates.
///
/// # Examples
///
/// ```
/// # use pix_engine::prelude::*;
/// let n = noise!(5.0);
/// assert!(n >= 0.0 && n < 1.0);
///
/// let n = noise!(2.0, 1.5);
/// assert!(n >= 0.0 && n < 1.0);
///
/// let n = noise!(2.0, 1.5, 3.0);
/// assert!(n >= 0.0 && n < 1.0);
/// ```
#[macro_export]
macro_rules! noise {
    ($x:expr$(,)?) => {
        $crate::math::noise([$x, 0.0, 0.0])
    };
    ($x:expr, $y:expr$(,)?) => {
        $crate::math::noise([$x, $y, 0.0])
    };
    ($x:expr, $y:expr, $z:expr$(,)?) => {
        $crate::math::noise([$x, $y, $z])
    };
}

/// Remaps a number from one range to another.
///
/// Map range defaults to `0.0..=Scalar::MAX` in the event casting to [Scalar] fails.
/// NaN will result in the max mapped value.
///
/// # Example
///
/// ```
/// # use pix_engine::prelude::*;
/// let value = 25;
/// let m = map(value, 0, 100, 0, 800);
/// assert_eq!(m, 200);
///
/// let value = 50.0;
/// let m = map(value, 0.0, 100.0, 0.0, 1.0);
/// assert_eq!(m, 0.5);
///
/// let value = f64::NAN;
/// let m = map(value, 0.0, 100.0, 0.0, 1.0);
/// assert!(m.is_nan());
///
/// let value = f64::INFINITY;
/// let m = map(value, 0.0, 100.0, 0.0, 1.0);
/// assert_eq!(m, 1.0);
///
/// let value = f64::NEG_INFINITY;
/// let m = map(value, 0.0, 100.0, 0.0, 1.0);
/// assert_eq!(m, 0.0);
/// ```
pub fn map<T>(value: T, start1: T, end1: T, start2: T, end2: T) -> T
where
    T: NumCast + Into<Scalar> + PartialOrd + Copy,
{
    let default = end2;
    let start1 = start1.into();
    let end1 = end1.into();
    let start2 = start2.into();
    let end2 = end2.into();
    let value = value.into();
    let new_val = (value - start1) / (end1 - start1) * (end2 - start2) + start2;
    NumCast::from(new_val.clamp(start2, end2)).unwrap_or(default)
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
    T: Num + Copy + PartialOrd,
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
    T: Num + NumCast + Copy + PartialOrd + AddAssign,
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
