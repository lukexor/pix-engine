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
//! use pix_engine::prelude_3d::*;
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
use num_traits::{AsPrimitive, Float, Signed};
// #[cfg(feature = "serde")]
// use serde::{Deserialize, Serialize};
use std::{fmt, iter::Sum, ops::*};

/// A `Point` in N-dimensional space.
///
/// Please see the [module-level documentation] for examples.
///
/// [module-level documentation]: crate::core::shape::point
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Point<T, const N: usize>([T; N]);

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

/// # Constructs a `Point` with N coordinates.
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
    T: Copy,
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
    /// use pix_engine::prelude_3d::*;
    /// let mut p: PointI3 = Point::new([2, 1, 3]);
    /// assert_eq!(p.values(), [2, 1, 3]);
    /// p.set_values([1, 2, 4]);
    /// assert_eq!(p.values(), [1, 2, 4]);
    /// ```
    #[inline]
    pub fn set_values(&mut self, coords: [T; N]) {
        self.0 = coords;
    }

    /// Sets the `x-coordinate`.
    #[inline]
    pub fn set_x(&mut self, x: T) {
        if !self.0.is_empty() {
            self.0[0] = x;
        }
    }

    /// Sets the `y-coordinate`.
    #[inline]
    pub fn set_y(&mut self, y: T) {
        if self.0.len() > 1 {
            self.0[1] = y;
        }
    }

    /// Sets the `z-magnitude`.
    #[inline]
    pub fn set_z(&mut self, z: T) {
        if self.0.len() > 2 {
            self.0[2] = z;
        }
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

    /// Convert `Point<T, N>` to `Point<U, N>` using the `as` operator.
    #[inline]
    pub fn as_<U>(self) -> Point<U, N>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy + Default,
    {
        let mut coords = [U::default(); N];
        for i in 0..N {
            coords[i] = self[i].as_();
        }
        Point::new(coords)
    }
}

impl<T, const N: usize> Point<T, N>
where
    T: Copy + Default,
{
    /// Returns the `x-coordinate`.
    #[inline]
    pub fn x(&self) -> T {
        match self.0.get(0) {
            Some(z) => *z,
            None => T::default(),
        }
    }

    /// Returns the `y-coordinate`.
    #[inline]
    pub fn y(&self) -> T {
        match self.0.get(1) {
            Some(z) => *z,
            None => T::default(),
        }
    }

    /// Returns the `z-coordinate`.
    #[inline]
    pub fn z(&self) -> T {
        match self.0.get(0) {
            Some(z) => *z,
            None => T::default(),
        }
    }
}

impl<T, const N: usize> Point<T, N>
where
    T: Num,
{
    /// Constructs a `Point` by shifting coordinates by given amount.
    pub fn offset<U>(mut self, offsets: [U; N]) -> Self
    where
        T: Add<U, Output = T>,
        U: Copy,
    {
        for i in 0..N {
            self[i] = self[i] + offsets[i]
        }
        self
    }

    /// Constructs a `Point` by multiplying it by the given scale factor.
    pub fn scale<U>(self, s: U) -> Self
    where
        T: Mul<U, Output = T>,
        U: Num,
    {
        self * s
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
    T: Copy,
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

impl<T, const N: usize> Deref for Point<T, N> {
    type Target = [T; N];
    /// Deref `Point` to `&[T; N]`.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for Point<T, N> {
    /// Deref `Point` to `&mut [T; N]`.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const N: usize> Index<usize> for Point<T, N>
where
    T: Copy,
{
    type Output = T;
    /// Return `&T` by indexing `Point` with `usize`.
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl<T, const N: usize> IndexMut<usize> for Point<T, N>
where
    T: Copy,
{
    /// Return `&mut T` by indexing `Point` with `usize`.
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl<T, const N: usize> From<&Point<T, N>> for Point<T, N>
where
    T: Copy,
{
    /// Convert `&Point` to `Point`.
    fn from(p: &Point<T, N>) -> Self {
        *p
    }
}

impl<T, const N: usize> From<&mut Point<T, N>> for Point<T, N>
where
    T: Copy,
{
    /// Convert `&mut Point` to `Point`.
    fn from(p: &mut Point<T, N>) -> Self {
        *p
    }
}

// Operations

impl<T, U, const N: usize> Add<U> for Point<T, N>
where
    T: Num + Add<U, Output = T>,
    U: Num,
{
    type Output = Self;
    /// [Point] + U.
    fn add(mut self, val: U) -> Self::Output {
        for i in 0..N {
            self[i] = self[i] + val;
        }
        self
    }
}

impl<T, const N: usize> AddAssign for Point<T, N>
where
    T: Num,
{
    /// [Point] += [Point].
    fn add_assign(&mut self, p: Point<T, N>) {
        for i in 0..N {
            self[i] += p[i];
        }
    }
}

impl<T, U, const N: usize> AddAssign<U> for Point<T, N>
where
    T: Num + AddAssign<U>,
    U: Num,
{
    /// [Point] += U.
    fn add_assign(&mut self, val: U) {
        for i in 0..N {
            self[i] += val;
        }
    }
}

impl<T, U, const N: usize> Sub<U> for Point<T, N>
where
    T: Num + Sub<U, Output = T>,
    U: Num,
{
    type Output = Self;
    /// [Point] - U.
    fn sub(mut self, val: U) -> Self::Output {
        for i in 0..N {
            self[i] = self[i] - val;
        }
        self
    }
}

impl<T, const N: usize> SubAssign for Point<T, N>
where
    T: Num,
{
    /// [Point] -= [Point].
    fn sub_assign(&mut self, p: Point<T, N>) {
        for i in 0..N {
            self[i] -= p[i];
        }
    }
}

impl<T, U, const N: usize> SubAssign<U> for Point<T, N>
where
    T: Num + SubAssign<U>,
    U: Num,
{
    /// [Point] -= U.
    fn sub_assign(&mut self, val: U) {
        for i in 0..N {
            self[i] -= val;
        }
    }
}

impl<T, const N: usize> Neg for Point<T, N>
where
    T: Num + Neg<Output = T>,
{
    type Output = Self;
    /// ![Point].
    fn neg(mut self) -> Self::Output {
        for i in 0..N {
            self[i] = self[i].neg();
        }
        self
    }
}

impl<T, U, const N: usize> Mul<U> for Point<T, N>
where
    T: Num + Mul<U, Output = T>,
    U: Num,
{
    type Output = Self;
    /// [Point] * U.
    fn mul(mut self, s: U) -> Self::Output {
        for i in 0..N {
            self[i] = self[i] * s;
        }
        self
    }
}

impl<T, U, const N: usize> MulAssign<U> for Point<T, N>
where
    T: Num + MulAssign<U>,
    U: Num,
{
    /// [Point] *= U.
    fn mul_assign(&mut self, s: U) {
        for i in 0..N {
            self[i] *= s;
        }
    }
}

impl<T, U, const N: usize> Div<U> for Point<T, N>
where
    T: Num + Div<U, Output = T>,
    U: Num,
{
    type Output = Self;
    /// [Point] / U.
    fn div(mut self, s: U) -> Self::Output {
        for i in 0..N {
            self[i] = self[i] / s;
        }
        self
    }
}

impl<T, U, const N: usize> DivAssign<U> for Point<T, N>
where
    T: Num + DivAssign<U>,
    U: Num,
{
    /// [Point] /= U.
    fn div_assign(&mut self, s: U) {
        for i in 0..N {
            self[i] /= s;
        }
    }
}

impl<T, const N: usize> Sum for Point<T, N>
where
    T: Default + Copy + Add,
    Self: Add<Output = Self>,
{
    /// Sum a list of `Point`s.
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let p = Point::origin();
        iter.fold(p, |a, b| a + b)
    }
}

impl<'a, T, const N: usize> Sum<&'a Point<T, N>> for Point<T, N>
where
    T: Default + Copy + Add,
    Self: Add<Output = Self>,
{
    /// Sum a list of `&Point`s.
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        let p = Point::origin();
        iter.fold(p, |a, b| a + *b)
    }
}

// Required because of orphan rules.
macro_rules! impl_primitive_mul {
    ($($target:ty),*) => {
        $(
            impl<const N: usize> Mul<Point<$target, N>> for $target {
                type Output = Point<$target, N>;
                /// T * [Point].
                fn mul(self, mut p: Point<$target, N>) -> Self::Output {
                    for i in 0..N {
                        p[i] *= self;
                    }
                    p
                }
            }
        )*
    };
}

impl_primitive_mul!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);

macro_rules! impl_from_as {
    ($($from:ty),* => $to:ty, $zero:expr) => {
        $(
            impl<const N: usize> From<Point<$from, N>> for Point<$to, N> {
                /// Convert `Point<U, N>` to `Point<T, N>`.
                fn from(p: Point<$from, N>) -> Self {
                    let mut coords = [$zero; N];
                    for i in 0..N {
                        coords[i] = p[i] as $to;
                    }
                    Self::new(coords)
                }
            }

            impl<const N: usize> From<[$from; N]> for Point<$to, N> {
                /// Convert `[T; N]` to `Point`.
                fn from(arr: [$from; N]) -> Self {
                    let mut coords = [$zero; N];
                    for i in 0..N {
                        coords[i] = arr[i] as $to;
                    }
                    Self::new(coords)
                }
            }

            impl<const N: usize> From<&[$from; N]> for Point<$to, N> {
                /// Convert `&[T; N]` to `Point`.
                fn from(&arr: &[$from; N]) -> Self {
                    let mut coords = [$zero; N];
                    for i in 0..N {
                        coords[i] = arr[i] as $to;
                    }
                    Self::new(coords)
                }
            }
        )*
    };
}

impl_from_as!(i8, u8, i16, u16, u32, i64, u64, isize, usize, f32, f64 => i32, 0);
impl_from_as!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32 => f64, 0.0);

impl<T, const N: usize> From<[T; N]> for Point<T, N> {
    /// Convert `[T; N]` to `Point`.
    fn from(arr: [T; N]) -> Self {
        Self::new(arr)
    }
}

impl<T, const N: usize> From<&[T; N]> for Point<T, N>
where
    T: Copy,
{
    /// Convert `&[T; N]` to `Point`.
    fn from(&arr: &[T; N]) -> Self {
        Self::new(arr)
    }
}

impl<T, const N: usize> From<Point<T, N>> for [T; N]
where
    T: Copy + Default,
{
    /// Convert [Point] to `[T; N]`.
    fn from(p: Point<T, N>) -> Self {
        p.values()
    }
}

impl<T, const N: usize> From<&Point<T, N>> for [T; N]
where
    T: Copy + Default,
{
    /// Convert &[Point] to `[T; N]`.
    fn from(p: &Point<T, N>) -> Self {
        p.values()
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_ops {
        ($val:expr) => {
            // Mul<T> for Point
            let p = point!(2, -5, 0) * $val;
            assert_eq!(p.values(), [4, -10, 0]);

            // Mul<point> for T
            let p = $val * point!(2, -5, 0);
            assert_eq!(p.values(), [4, -10, 0]);

            // MulAssign<T> for point
            let mut p = point!(2, -5, 0);
            p *= $val;
            assert_eq!(p.values(), [4, -10, 0]);

            // Div<T> for point
            let p = point!(2, -6, 0) / $val;
            assert_eq!(p.values(), [1, -3, 0]);

            // DivAssign<T> for point
            let mut p = point!(2, -4, 0);
            p /= $val;
            assert_eq!(p.values(), [1, -2, 0]);
        };
    }

    #[test]
    fn test_ops() {
        // Add
        let p1 = point!(2, 5, 1);
        let p2 = point!(1, 5, -1);
        let p3 = p1 + p2;
        assert_eq!(p3.values(), [3, 10, 0]);

        // AddAssign
        let mut p1 = point!(2, 5, 1);
        let p2 = point!(1, 5, -1);
        p1 += p2;
        assert_eq!(p1.values(), [3, 10, 0]);

        // Sub
        let p1 = point!(2, 1, 2);
        let p2 = point!(1, 5, 3);
        let p3 = p1 - p2;
        assert_eq!(p3.values(), [1, -4, -1]);

        // SubAssign
        let mut p1 = point!(2, 1, 2);
        let p2 = point!(1, 5, 3);
        p1 -= p2;
        assert_eq!(p1.values(), [1, -4, -1]);

        test_ops!(2i32);
        test_ops!(2i32);
    }

    #[test]
    fn test_slice_conversions() {
        let _: Point<u8, 1> = [50u8].into();
        let _: Point<i8, 1> = [50i8].into();
        let _: Point<u16, 1> = [50u16].into();
        let _: Point<i16, 1> = [50i16].into();
        let _: Point<u32, 1> = [50u32].into();
        let _: Point<i32, 1> = [50i32].into();
        let _: Point<f32, 1> = [50.0f32].into();
        let _: Point<f64, 1> = [50.0f64].into();

        let _: Point<u8, 2> = [50u8, 100].into();
        let _: Point<i8, 2> = [50i8, 100].into();
        let _: Point<u16, 2> = [50u16, 100].into();
        let _: Point<i16, 2> = [50i16, 100].into();
        let _: Point<u32, 2> = [50u32, 100].into();
        let _: Point<i32, 2> = [50i32, 100].into();
        let _: Point<f32, 2> = [50.0f32, 100.0].into();
        let _: Point<f64, 2> = [50.0f64, 100.0].into();

        let _: Point<u8, 3> = [50u8, 100, 55].into();
        let _: Point<i8, 3> = [50i8, 100, 55].into();
        let _: Point<u16, 3> = [50u16, 100, 55].into();
        let _: Point<i16, 3> = [50i16, 100, 55].into();
        let _: Point<u32, 3> = [50u32, 100, 55].into();
        let _: Point<i32, 3> = [50i32, 100, 55].into();
        let _: Point<f32, 3> = [50.0f32, 100.0, 55.0].into();
        let _: Point<f64, 3> = [50.0f64, 100.0, 55.0].into();
    }
}
