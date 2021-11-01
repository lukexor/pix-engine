//! A N-dimensional shape type representing geometric points used for drawing.
//!
//! # Examples
//!
//! You can create a [Point] using [Point::new]:
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! let p: PointI2 = Point::new([10, 20]);
//! ```
//! ...or by using the [point!] macro:
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! let p: PointI2 = point!(); // origin point at (0, 0, 0)
//!
//! let p = point!(5); // 1D point on the x-axis
//!
//! let p = point!(5, 10); // 2D point in the x/y-plane
//!
//! let p = point!(5, 10, 7); // 3D point
//! ```

use crate::prelude::*;
#[cfg(feature = "serde")]
use crate::serialize::arrays;
use num_traits::Signed;
#[cfg(feature = "serde")]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fmt, ops::*};

/// A `Point` in N-dimensional space.
///
/// Please see the [module-level documentation] for examples.
///
/// [module-level documentation]: crate::shape::point
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound = "T: Serialize + DeserializeOwned"))]
pub struct Point<T, const N: usize>(
    #[cfg_attr(feature = "serde", serde(with = "arrays"))] pub(crate) [T; N],
);

/// A 1D `Point` represented by integers.
pub type PointI1 = Point<i32, 1>;

/// A 2D `Point` represented by integers.
pub type PointI2 = Point<i32, 2>;

/// A 3D `Point` represented by integers.
pub type PointI3 = Point<i32, 3>;

/// A 1D `Point` represented by integers.
pub type PointF1 = Point<Scalar, 1>;

/// A 2D `Point` represented by floating point numbers.
pub type PointF2 = Point<Scalar, 2>;

/// A 3D `Point` represented by floating point numbers.
pub type PointF3 = Point<Scalar, 3>;

/// Constructs a [Point] with N coordinates.
///
/// ```
/// # use pix_engine::prelude::*;
/// let p: PointI2 = point!();
/// assert_eq!(p.values(), [0, 0]);
///
/// let p = point!(1);
/// assert_eq!(p.values(), [1]);
///
/// let p = point!(1, 2);
/// assert_eq!(p.values(), [1, 2]);
///
/// let p = point!(1, -2, 1);
/// assert_eq!(p.values(), [1, -2, 1]);
/// ```
#[macro_export]
macro_rules! point {
    () => {
        $crate::prelude::Point::origin()
    };
    ($x:expr) => {
        $crate::prelude::Point::new([$x])
    };
    ($x:expr, $y:expr$(,)?) => {
        $crate::prelude::Point::new([$x, $y])
    };
    ($x:expr, $y:expr, $z:expr$(,)?) => {
        $crate::prelude::Point::new([$x, $y, $z])
    };
}

impl<T, const N: usize> Point<T, N> {
    /// Constructs a `Point` from `[T; N]` coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = Point::new([1]);
    /// assert_eq!(p.values(), [1]);
    ///
    /// let p = Point::new([1, 2]);
    /// assert_eq!(p.values(), [1, 2]);
    ///
    /// let p = Point::new([1, -2, 1]);
    /// assert_eq!(p.values(), [1, -2, 1]);
    /// ```
    #[inline]
    pub const fn new(coords: [T; N]) -> Self {
        Self(coords)
    }

    /// Constructs a `Point` at the origin.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p: PointI2 = Point::origin();
    /// assert_eq!(p.values(), [0, 0]);
    /// ```
    #[inline]
    pub fn origin() -> Self
    where
        T: Default + Copy,
    {
        Self::new([T::default(); N])
    }
}

impl<T, const N: usize> Point<T, N>
where
    T: Copy + Default,
{
    /// Constructs a `Point` from a [Vector].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = vector!(1.0, 2.0);
    /// let p = Point::from_vector(v);
    /// assert_eq!(p.values(), [1.0, 2.0]);
    /// ```
    pub fn from_vector(v: Vector<T, N>) -> Self {
        Self::new(v.values())
    }

    /// Returns `Point` coordinates as `[T; N]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = point!(2, 1, 3);
    /// assert_eq!(p.values(), [2, 1, 3]);
    /// ```
    #[inline]
    pub fn values(&self) -> [T; N] {
        self.0
    }

    /// Set `Point` coordinates from `[T; N]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::prelude::*;
    /// let mut p: PointI3 = Point::new([2, 1, 3]);
    /// assert_eq!(p.values(), [2, 1, 3]);
    /// p.set_values([1, 2, 4]);
    /// assert_eq!(p.values(), [1, 2, 4]);
    /// ```
    #[inline]
    pub fn set_values(&mut self, coords: [T; N]) {
        self.0 = coords;
    }

    /// Returns the `x-coordinate`.
    ///
    /// # Panics
    ///
    /// If `Point` has zero dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = point!(1, 2);
    /// assert_eq!(p.x(), 1);
    /// ```
    #[inline]
    pub fn x(&self) -> T {
        *self.0.get(0).expect("greater than 0 dimensions")
    }

    /// Sets the `x-coordinate`.
    ///
    /// # Panics
    ///
    /// If `Vector` has zero dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut p = point!(1, 2);
    /// p.set_x(3);
    /// assert_eq!(p.values(), [3, 2]);
    /// ```
    #[inline]
    pub fn set_x(&mut self, x: T) {
        self.0[0] = x;
    }

    /// Returns the `y-coordinate`.
    ///
    /// # Panics
    ///
    /// If `Vector` has less than 2 dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = point!(1, 2);
    /// assert_eq!(p.y(), 2);
    /// ```
    #[inline]
    pub fn y(&self) -> T {
        *self.0.get(1).expect("greater than 1 dimension")
    }

    /// Sets the `y-coordinate`.
    ///
    /// # Panics
    ///
    /// If `Vector` has less than 2 dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut p = point!(1, 2);
    /// p.set_y(3);
    /// assert_eq!(p.values(), [1, 3]);
    /// ```
    #[inline]
    pub fn set_y(&mut self, y: T) {
        self.0[1] = y;
    }

    /// Returns the `z-coordinate`.
    ///
    /// # Panics
    ///
    /// If `Vector` has less than 3 dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = point!(1, 2, 2);
    /// assert_eq!(p.z(), 2);
    /// ```
    #[inline]
    pub fn z(&self) -> T {
        *self.0.get(2).expect("greater than 2 dimensions")
    }

    /// Sets the `z-magnitude`.
    ///
    /// # Panics
    ///
    /// If `Vector` has less than 3 dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut p = point!(1, 2, 1);
    /// p.set_z(3);
    /// assert_eq!(p.values(), [1, 2, 3]);
    /// ```
    #[inline]
    pub fn set_z(&mut self, z: T) {
        self.0[2] = z;
    }

    /// Returns `Point` as a [Vec].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = point!(1, 1, 0);
    /// assert_eq!(p.to_vec(), vec![1, 1, 0]);
    /// ```
    #[inline]
    pub fn to_vec(self) -> Vec<T> {
        self.0.to_vec()
    }
}

impl<T, const N: usize> Point<T, N>
where
    T: Num,
{
    /// Offsets a `Point` by shifting coordinates by given amount.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut p: PointI3 = point!(2, 3, 1);
    /// p.offset([2, -4]);
    /// assert_eq!(p.values(), [4, -1, 1]);
    /// ```
    #[inline]
    pub fn offset<P, const M: usize>(&mut self, offset: P)
    where
        P: Into<Point<T, M>>,
    {
        let offset = offset.into();
        assert!(N >= M);
        for i in 0..M {
            self[i] += offset[i]
        }
    }

    /// Offsets the `x-coordinate` of the point by a given amount.
    #[inline]
    pub fn offset_x(&mut self, offset: T) {
        self.0[0] += offset;
    }

    /// Offsets the `y-coordinate` of the point by a given amount.
    #[inline]
    pub fn offset_y(&mut self, offset: T) {
        self.0[1] += offset;
    }

    /// Offsets the `z-coordinate` of the point by a given amount.
    #[inline]
    pub fn offset_z(&mut self, offset: T) {
        self.0[2] += offset;
    }

    /// Constructs a `Point` by multiplying it by the given scale factor.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut p: PointI2 = point!(2, 3);
    /// p.scale(2);
    /// assert_eq!(p.values(), [4, 6]);
    /// ```
    pub fn scale<U>(&mut self, s: U)
    where
        T: MulAssign<U>,
        U: Num,
    {
        *self *= s;
    }

    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut p = point!(200.0, 300.0);
    /// p.wrap([150.0, 400.0], 10.0);
    /// assert_eq!(p.values(), [-10.0, 300.0]);
    ///
    /// let mut p = point!(-100.0, 300.0);
    /// p.wrap([150.0, 400.0], 10.0);
    /// assert_eq!(p.values(), [160.0, 300.0]);
    /// ```
    pub fn wrap(&mut self, wrap: [T; N], size: T)
    where
        T: Signed,
    {
        for i in 0..N {
            if self[i] > wrap[i] + size {
                self[i] = -size;
            } else if self[i] < -size {
                self[i] = wrap[i] + size;
            }
        }
    }
}

impl<T, const N: usize> Point<T, N>
where
    T: Num + Float,
{
    /// Returns the Euclidean distance between two `Point`s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p1: PointF3 = point!(1.0, 0.0, 0.0);
    /// let p2: PointF3 = point!(0.0, 1.0, 0.0);
    /// let dist = p1.dist(p2);
    /// let abs_difference: f64 = (dist - std::f64::consts::SQRT_2).abs();
    /// assert!(abs_difference <= 1e-4);
    /// ```
    pub fn dist<P: Into<Point<T, N>>>(&self, p: P) -> T {
        let p = p.into();
        (*self - p).mag()
    }

    /// Constructs a `Point` by linear interpolating between two `Point`s by a given amount
    /// between `0.0` and `1.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p1 = point!(1.0, 1.0, 0.0);
    /// let p2 = point!(3.0, 3.0, 0.0);
    /// let p3 = p1.lerp(p2, 0.5);
    /// assert_eq!(p3.values(), [2.0, 2.0, 0.0]);
    /// ```
    pub fn lerp<P: Into<Point<T, N>>>(&self, p: P, amt: T) -> Self {
        let p = p.into();
        let lerp = |start, stop, amt| amt * (stop - start) + start;
        let amt = num_traits::clamp(amt, T::zero(), T::one());
        let mut coords = [T::zero(); N];
        for i in 0..N {
            coords[i] = lerp(self[i], p[i], amt);
        }
        Self::new(coords)
    }

    /// Returns whether two `Point`s are approximately equal.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p1: PointF2 = point!(10.0, 20.0);
    /// let p2: PointF2 = point!(10.0001, 20.0);
    /// assert!(p1.approx_eq(p2, 1e-3));
    /// ```
    pub fn approx_eq(&self, other: Point<T, N>, epsilon: T) -> bool {
        let mut approx_eq = true;
        for i in 0..N {
            approx_eq &= (self[i] - other[i]).abs() < epsilon;
        }
        approx_eq
    }
}

impl<T, const N: usize> Draw for Point<T, N>
where
    Self: Into<PointI2>,
    T: Num,
{
    /// Draw point to the current [p.x()State] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.point(*self)
    }
}

impl<T, const N: usize> Default for Point<T, N>
where
    T: Default + Copy,
{
    /// Return default `Point` as origin.
    fn default() -> Self {
        Self::origin()
    }
}

impl<T, const N: usize> fmt::Display for Point<T, N>
where
    T: Copy + Default + fmt::Debug,
{
    /// Display [Point] as a string of coordinates.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.values())
    }
}
