//! A N-dimensional shape type representing geometric points used for drawing.
//!
//! # Examples
//!
//! You can create a [Point] using [`Point::new`]:
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! let p = Point::new([10, 20]);
//! ```
//! ...or by using the [point!] macro:
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! let p: Point<i32> = point!(); // origin point at (0, 0, 0)
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
use std::{fmt, ops::MulAssign};

/// A `Point` in N-dimensional space.
///
/// Please see the [module-level documentation] for examples.
///
/// [module-level documentation]: crate::shape::point
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[must_use]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound = "T: Serialize + DeserializeOwned"))]
pub struct Point<T = i32, const N: usize = 2>(
    #[cfg_attr(feature = "serde", serde(with = "arrays"))] pub(crate) [T; N],
);

/// Constructs a [Point] with N coordinates.
///
/// ```
/// # use pix_engine::prelude::*;
/// let p: Point<i32> = point!();
/// assert_eq!(p.coords(), [0, 0]);
///
/// let p = point!(1);
/// assert_eq!(p.coords(), [1]);
///
/// let p = point!(1, 2);
/// assert_eq!(p.coords(), [1, 2]);
///
/// let p = point!(1, -2, 1);
/// assert_eq!(p.coords(), [1, -2, 1]);
/// ```
#[macro_export]
macro_rules! point {
    () => {
        $crate::prelude::Point::origin()
    };
    ($x:expr) => {
        $crate::prelude::Point::from_x($x)
    };
    ($x:expr, $y:expr$(,)?) => {
        $crate::prelude::Point::from_xy($x, $y)
    };
    ($x:expr, $y:expr, $z:expr$(,)?) => {
        $crate::prelude::Point::from_xyz($x, $y, $z)
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
    /// assert_eq!(p.coords(), [1]);
    ///
    /// let p = Point::new([1, 2]);
    /// assert_eq!(p.coords(), [1, 2]);
    ///
    /// let p = Point::new([1, -2, 1]);
    /// assert_eq!(p.coords(), [1, -2, 1]);
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
    /// let p: Point<i32> = Point::origin();
    /// assert_eq!(p.coords(), [0, 0]);
    /// ```
    #[inline]
    pub fn origin() -> Self
    where
        T: Default,
    {
        Self::new([(); N].map(|_| T::default()))
    }
}

impl<T> Point<T, 1> {
    /// Constructs a `Point` from an individual x coordinate.
    #[inline]
    pub const fn from_x(x: T) -> Self {
        Self([x])
    }
}

impl<T> Point<T> {
    /// Constructs a `Point` from individual x/y coordinates.
    #[inline]
    pub const fn from_xy(x: T, y: T) -> Self {
        Self([x, y])
    }
}

impl<T> Point<T, 3> {
    /// Constructs a `Point` from individual x/y/z coordinates.
    #[inline]
    pub const fn from_xyz(x: T, y: T, z: T) -> Self {
        Self([x, y, z])
    }
}

impl<T: Copy, const N: usize> Point<T, N> {
    /// Constructs a `Point` from a [Vector].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = vector!(1.0, 2.0);
    /// let p = Point::from_vector(v);
    /// assert_eq!(p.coords(), [1.0, 2.0]);
    /// ```
    pub fn from_vector(v: Vector<T, N>) -> Self {
        Self::new(v.coords())
    }

    /// Returns `Point` coordinates as `[T; N]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = point!(2, 1, 3);
    /// assert_eq!(p.coords(), [2, 1, 3]);
    /// ```
    #[inline]
    pub fn coords(&self) -> [T; N] {
        self.0
    }

    /// Returns `Point` coordinates as a mutable slice `&mut [T; N]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut p = point!(2, 1, 3);
    /// for v in p.coords_mut() {
    ///     *v *= 2;
    /// }
    /// assert_eq!(p.coords(), [4, 2, 6]);
    /// ```
    #[inline]
    pub fn coords_mut(&mut self) -> &mut [T; N] {
        &mut self.0
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
        self.0[0]
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
    /// assert_eq!(p.coords(), [3, 2]);
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
        self.0[1]
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
    /// assert_eq!(p.coords(), [1, 3]);
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
        self.0[2]
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
    /// assert_eq!(p.coords(), [1, 2, 3]);
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

impl<T: Num, const N: usize> Point<T, N> {
    /// Offsets a `Point` by shifting coordinates by given amount.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut p = point!(2, 3, 1);
    /// p.offset([2, -4]);
    /// assert_eq!(p.coords(), [4, -1, 1]);
    /// ```
    #[inline]
    pub fn offset<P, const M: usize>(&mut self, offsets: P)
    where
        P: Into<Point<T, M>>,
    {
        let offsets = offsets.into();
        for (v, o) in self.iter_mut().zip(offsets) {
            *v += o;
        }
    }

    /// Offsets the `x-coordinate` of the point by a given amount.
    ///
    /// # Panics
    ///
    /// If `Point` has zero dimensions.
    #[inline]
    pub fn offset_x(&mut self, offset: T) {
        self.0[0] += offset;
    }

    /// Offsets the `y-coordinate` of the point by a given amount.
    ///
    /// # Panics
    ///
    /// If `Vector` has less than 2 dimensions.
    #[inline]
    pub fn offset_y(&mut self, offset: T) {
        self.0[1] += offset;
    }

    /// Offsets the `z-coordinate` of the point by a given amount.
    ///
    /// # Panics
    ///
    /// If `Vector` has less than 3 dimensions.
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
    /// let mut p = point!(2, 3);
    /// p.scale(2);
    /// assert_eq!(p.coords(), [4, 6]);
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
    /// assert_eq!(p.coords(), [-10.0, 300.0]);
    ///
    /// let mut p = point!(-100.0, 300.0);
    /// p.wrap([150.0, 400.0], 10.0);
    /// assert_eq!(p.coords(), [160.0, 300.0]);
    /// ```
    pub fn wrap(&mut self, wrap: [T; N], size: T)
    where
        T: Signed,
    {
        for (v, w) in self.iter_mut().zip(wrap) {
            let w = w + size;
            if *v > w {
                *v = -size;
            } else if *v < -size {
                *v = w;
            }
        }
    }
}

impl<T: Num + Float, const N: usize> Point<T, N> {
    /// Returns the Euclidean distance between two `Point`s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p1 = point!(1.0, 0.0, 0.0);
    /// let p2 = point!(0.0, 1.0, 0.0);
    /// let dist = p1.dist(p2);
    /// let abs_difference: f64 = (dist - std::f64::consts::SQRT_2).abs();
    /// assert!(abs_difference <= 1e-4);
    /// ```
    pub fn dist<P>(&self, p: P) -> T
    where
        P: Into<Point<T, N>>,
    {
        (*self - p.into()).mag()
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
    /// assert_eq!(p3.coords(), [2.0, 2.0, 0.0]);
    /// ```
    pub fn lerp<P>(&self, o: P, amt: T) -> Self
    where
        P: Into<Point<T, N>>,
    {
        let o = o.into();
        let lerp = |start, stop, amt| amt * (stop - start) + start;
        let amt = num_traits::clamp(amt, T::zero(), T::one());
        let mut coords = [T::zero(); N];
        for ((c, &v), o) in coords.iter_mut().zip(self.iter()).zip(o) {
            *c = lerp(v, o, amt);
        }
        Self::new(coords)
    }

    /// Returns whether two `Point`s are approximately equal.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p1 = point!(10.0, 20.0);
    /// let p2 = point!(10.0001, 20.0);
    /// assert!(p1.approx_eq(p2, 1e-3));
    /// ```
    pub fn approx_eq(&self, other: Point<T, N>, epsilon: T) -> bool {
        let mut approx_eq = true;
        for (&v, o) in self.iter().zip(other) {
            approx_eq &= (v - o).abs() < epsilon;
        }
        approx_eq
    }
}

impl Draw for Point<i32> {
    /// Draw point to the current [`PixState`] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.point(*self)
    }
}

impl<T: Default, const N: usize> Default for Point<T, N> {
    /// Return default `Point` as origin.
    fn default() -> Self {
        Self::origin()
    }
}

impl<T, const N: usize> fmt::Display for Point<T, N>
where
    [T; N]: fmt::Debug,
{
    /// Display [Point] as a string of coordinates.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
