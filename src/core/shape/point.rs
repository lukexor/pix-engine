//! A 1D/2D/3D shape type representing a point used for drawing.
//!
//! # Examples
//!
//! You can create a [Point] using [Point::new]:
//!
//! ```
//! # use pix_engine::prelude::*;
//! let p = Point::new(10, 20, 15);
//! ```
//! ...or by using the [point!] macro:
//!
//! ```
//! # use pix_engine::prelude::*;
//! let p: Point<i32> = point!(); // origin point at (0, 0, 0)
//!
//! let p = point!(5); // 1D point on the x-axis
//!
//! let p = point!(5, 10); // 2D point in the x/y-plane
//!
//! let p = point!(5, 10, 7); // 3D point
//! ```

use crate::prelude::*;
use num_traits::{AsPrimitive, Float, Signed};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    fmt,
    iter::{FromIterator, Sum},
    ops::*,
};

/// A `Point` in 2D or 3D space.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Point<T = Scalar>([T; 3]);

/// # Constructs a `Point<T>` with `(x, y, z)`.
///
/// ```
/// # use pix_engine::prelude::*;
/// let p: Point<i32> = point!();
/// assert_eq!(p.values(), [0, 0, 0]);
///
/// let p = point!(1);
/// assert_eq!(p.values(), [1, 0, 0]);
///
/// let p = point!(1, 2);
/// assert_eq!(p.values(), [1, 2, 0]);
///
/// let p = point!(1, -2, 1);
/// assert_eq!(p.values(), [1, -2, 1]);
/// ```
#[macro_export]
macro_rules! point {
    () => {
        $crate::prelude::Point::default()
    };
    ($x:expr) => {
        $crate::prelude::Point::with_x($x)
    };
    ($x:expr, $y:expr$(,)?) => {
        $crate::prelude::Point::with_xy($x, $y)
    };
    ($x:expr, $y:expr, $z:expr$(,)?) => {
        $crate::prelude::Point::new($x, $y, $z)
    };
}

impl<T> Point<T> {
    /// Constructs a `Point<T>`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = Point::new(2, 3, 1);
    /// assert_eq!(p.values(), [2, 3, 1]);
    /// ```
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self([x, y, z])
    }
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut p = Point::new(2, 1, 3);
    /// assert_eq!(p.values(), [2, 1, 3]);
    /// p.set([1, 2, 4]);
    /// assert_eq!(p.values(), [1, 2, 4]);
    /// ```
    pub fn set(&mut self, [x, y, z]: [T; 3]) {
        self.0 = [x, y, z];
    }
}

impl<T: Number> Point<T> {
    /// Constructs a `Point<T>` from a [Vector].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v  = vector!(1.0, 2.0);
    /// let p = Point::from_vector(v);
    /// assert_eq!(p.values(), [1.0, 2.0, 0.0]);
    /// ```
    pub fn from_vector(v: Vector<T>) -> Self {
        Self::new(v.x(), v.y(), v.z())
    }

    /// Returns the `x-coordinate`.
    #[inline(always)]
    pub fn x(&self) -> T {
        self.0[0]
    }

    /// Sets the `x-coordinate`.
    #[inline(always)]
    pub fn set_x(&mut self, x: T) {
        self.0[0] = x;
    }

    /// Returns the `y-coordinate`.
    #[inline(always)]
    pub fn y(&self) -> T {
        self.0[1]
    }

    /// Sets the `y-coordinate`.
    #[inline(always)]
    pub fn set_y(&mut self, y: T) {
        self.0[1] = y;
    }

    /// Returns the `z-coordinate`.
    #[inline(always)]
    pub fn z(&self) -> T {
        self.0[2]
    }

    /// Sets the `z-magnitude`.
    #[inline(always)]
    pub fn set_z(&mut self, z: T) {
        self.0[2] = z;
    }

    /// Returns `Point` coordinates as `[x, y, z]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = point!(2, 1, 3);
    /// assert_eq!(p.values(), [2, 1, 3]);
    /// ```
    pub fn values(&self) -> [T; 3] {
        [self.x(), self.y(), self.z()]
    }

    /// Tries to convert `Point` coordinates as `[x, y, z]` from `T` to `U` of `T` implements
    /// `TryInto<U>`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p: Point<i32> = point!(2, 1, 3);
    /// let values: [i16; 3] = p.try_into_values()?;
    /// assert_eq!(values, [2i16, 1, 3]);
    /// # Ok::<(), PixError>(())
    /// ```
    pub fn try_into_values<U>(&self) -> PixResult<[U; 3]>
    where
        T: TryInto<U>,
        PixError: From<<T as TryInto<U>>::Error>,
    {
        Ok([
            self.x().try_into()?,
            self.y().try_into()?,
            self.z().try_into()?,
        ])
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
    pub fn to_vec(self) -> Vec<T> {
        vec![self.x(), self.y(), self.z()]
    }

    /// Constructs a `Point<T>` with only an `x` magnitude.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = Point::with_x(2);
    /// assert_eq!(p.values(), [2, 0, 0]);
    /// ```
    pub fn with_x(x: T) -> Self {
        Self::new(x, T::zero(), T::zero())
    }

    /// Constructs a `Point<T>` with only `x` and `y magnitudes.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = Point::with_xy(2, 3);
    /// assert_eq!(p.values(), [2, 3, 0]);
    /// ```
    pub fn with_xy(x: T, y: T) -> Self {
        Self::new(x, y, T::zero())
    }

    /// Convert `Point<T>` to another primitive type using the `as` operator.
    #[inline]
    pub fn as_<U>(self) -> Point<U>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy,
    {
        Point::new(self.x().as_(), self.y().as_(), self.z().as_())
    }

    /// Constructs a `Point<T>` by shifting coordinates by given x, y, and z values.
    pub fn offset<U>(self, x: U, y: U, z: U) -> Self
    where
        T: Add<U, Output = T>,
    {
        Self::new(self.x() + x, self.y() + y, self.z() + z)
    }

    /// Constructs a `Point<T>` by multip.y()ing it by the given scale factor.
    pub fn scale<U>(self, s: U) -> Self
    where
        T: Mul<U, Output = T>,
        U: Number,
    {
        self * s
    }

    /// Wraps `Point` around the given width, height, and size (radius).
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut p = point!(200.0, 300.0);
    /// p.wrap_2d(150.0, 400.0, 10.0);
    /// assert_eq!(p.values(), [-10.0, 300.0, 0.0]);
    ///
    /// let mut p = point!(-100.0, 300.0);
    /// p.wrap_2d(150.0, 400.0, 10.0);
    /// assert_eq!(p.values(), [160.0, 300.0, 0.0]);
    /// ```
    pub fn wrap_2d(&mut self, width: T, height: T, size: T)
    where
        T: Signed,
    {
        if self.x() > width + size {
            self.set_x(-size);
        } else if self.x() < -size {
            self.set_x(width + size);
        }
        if self.y() > height + size {
            self.set_y(-size);
        } else if self.y() < -size {
            self.set_y(height + size);
        }
    }

    /// Wraps `Point` around the given width, height, depth, and size (radius).
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut p = point!(200.0, 300.0, 250.0);
    /// p.wrap_3d(150.0, 400.0, 200.0, 10.0);
    /// assert_eq!(p.values(), [-10.0, 300.0, -10.0]);
    ///
    /// let mut p = point!(-100.0, 300.0, 250.0);
    /// p.wrap_3d(150.0, 400.0, 200.0, 10.0);
    /// assert_eq!(p.values(), [160.0, 300.0, -10.0]);
    /// ```
    pub fn wrap_3d(&mut self, width: T, height: T, depth: T, size: T)
    where
        T: Signed,
    {
        if self.x() > width + size {
            self.set_x(-size);
        } else if self.x() < -size {
            self.set_x(width + size);
        }
        if self.y() > height + size {
            self.set_y(-size);
        } else if self.y() < -size {
            self.set_y(height + size);
        }
        if self.z() > depth + size {
            self.set_z(-size);
        } else if self.z() < -size {
            self.set_z(depth + size);
        }
    }
}

impl<T: Float> Point<T> {
    /// Returns `Point` with values rounded to the nearest integer number. Round half-way cases
    /// away from `0.0`.
    pub fn round(&self) -> Self {
        Self::new(self.x().round(), self.y().round(), self.z().round())
    }

    /// Returns whether two `Point`s are approximately equal.
    pub fn approx_eq(&self, other: Point<T>, epsilon: T) -> bool {
        let xd = (self.x() - other.x()).abs();
        let yd = (self.y() - other.y()).abs();
        let zd = (self.z() - other.z()).abs();
        xd < epsilon && yd < epsilon && zd < epsilon
    }
}

impl<T> Draw for Point<T>
where
    T: Number,
    Self: Into<Point>,
{
    /// Draw point to the current [p.x()State] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.point(*self)
    }
}

impl<T: Number> From<&mut Point<T>> for Point<T> {
    fn from(p: &mut Point<T>) -> Self {
        *p
    }
}

impl<T: Number> From<&Point<T>> for Point<T> {
    fn from(p: &Point<T>) -> Self {
        *p
    }
}

impl<T: Number> FromIterator<T> for Point<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter();
        Self([
            iter.next().unwrap_or_else(T::zero),
            iter.next().unwrap_or_else(T::zero),
            iter.next().unwrap_or_else(T::zero),
        ])
    }
}

impl<T> Deref for Point<T> {
    type Target = [T; 3];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Point<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Number> Index<usize> for Point<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl<T: Number> IndexMut<usize> for Point<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

/// [Point] + [Point] yields a [Vector].
impl<T: Number> Add for Point<T> {
    type Output = Vector<T>;
    fn add(self, p: Point<T>) -> Self::Output {
        Self::Output::new(self.x() + p.x(), self.y() + p.y(), self.z() + p.z())
    }
}

/// [Point] + [Vector] yields a [Point].
impl<T: Number> Add<Vector<T>> for Point<T> {
    type Output = Point<T>;
    fn add(self, v: Vector<T>) -> Self::Output {
        Self::Output::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}

/// [Vector] + [Point] yields a [Point].
impl<T: Number> Add<Point<T>> for Vector<T> {
    type Output = Point<T>;
    fn add(self, p: Point<T>) -> Self::Output {
        Self::Output::new(self.x() + p.x(), self.y() + p.y(), self.z() + p.z())
    }
}

/// [Point] + U.
impl<T, U> Add<U> for Point<T>
where
    T: Number + Add<U, Output = T>,
    U: Number,
{
    type Output = Self;
    fn add(self, val: U) -> Self::Output {
        Self::Output::new(self.x() + val, self.y() + val, self.z() + val)
    }
}

/// [Point] += [Point].
impl<T> AddAssign for Point<T>
where
    T: Number + Add<Output = T>,
{
    fn add_assign(&mut self, p: Point<T>) {
        self.set_x(self.x() + p.x());
        self.set_y(self.y() + p.y());
        self.set_z(self.z() + p.z());
    }
}

/// [Point] += [Vector].
impl<T> AddAssign<Vector<T>> for Point<T>
where
    T: Number + Add<Output = T>,
{
    fn add_assign(&mut self, v: Vector<T>) {
        self.set_x(self.x() + v.x());
        self.set_y(self.y() + v.y());
        self.set_z(self.z() + v.z());
    }
}

/// [Point] += U.
impl<T, U> AddAssign<U> for Point<T>
where
    T: Number + Add<U, Output = T>,
    U: Number,
{
    fn add_assign(&mut self, val: U) {
        self.set_x(self.x() + val);
        self.set_y(self.y() + val);
        self.set_z(self.z() + val);
    }
}

/// [Point] - [Point] yields a [Vector].
impl<T: Number> Sub for Point<T> {
    type Output = Vector<T>;
    fn sub(self, p: Point<T>) -> Self::Output {
        Self::Output::new(self.x() - p.x(), self.y() - p.y(), self.z() - p.z())
    }
}

/// [Point] - [Vector] yields a [Point].
impl<T: Number> Sub<Vector<T>> for Point<T> {
    type Output = Point<T>;
    fn sub(self, v: Vector<T>) -> Self::Output {
        Self::Output::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}

/// [Point] - U.
impl<T, U> Sub<U> for Point<T>
where
    T: Number + Sub<U, Output = T>,
    U: Number,
{
    type Output = Self;
    fn sub(self, val: U) -> Self::Output {
        Self::Output::new(self.x() - val, self.y() - val, self.z() - val)
    }
}

/// [Point] -= [Point].
impl<T> SubAssign for Point<T>
where
    T: Number + Sub<Output = T>,
{
    fn sub_assign(&mut self, p: Point<T>) {
        self.set_x(self.x() - p.x());
        self.set_y(self.y() - p.y());
        self.set_z(self.z() - p.z());
    }
}

/// [Point] -= [Vector].
impl<T> SubAssign<Vector<T>> for Point<T>
where
    T: Number + Sub<Output = T>,
{
    fn sub_assign(&mut self, v: Vector<T>) {
        self.set_x(self.x() - v.x());
        self.set_y(self.y() - v.y());
        self.set_z(self.z() - v.z());
    }
}

/// [Point] -= U.
impl<T, U> SubAssign<U> for Point<T>
where
    T: Number + Sub<U, Output = T>,
    U: Number,
{
    fn sub_assign(&mut self, val: U) {
        self.set_x(self.x() - val);
        self.set_y(self.y() - val);
        self.set_z(self.z() - val);
    }
}

/// ![Point].
impl<T: Number + Neg<Output = T>> Neg for Point<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::Output::new(-self.x(), -self.y(), -self.z())
    }
}

/// [Point] * U.
impl<T, U> Mul<U> for Point<T>
where
    T: Number + Mul<U, Output = T>,
    U: Number,
{
    type Output = Self;
    fn mul(self, s: U) -> Self::Output {
        Self::Output::new(self.x() * s, self.y() * s, self.z() * s)
    }
}

/// [Point] *= U.
impl<T, U> MulAssign<U> for Point<T>
where
    T: Number + Mul<U, Output = T>,
    U: Number,
{
    fn mul_assign(&mut self, s: U) {
        self.set_x(self.x() * s);
        self.set_y(self.y() * s);
        self.set_z(self.z() * s);
    }
}

/// [Point] / U.
impl<T, U> Div<U> for Point<T>
where
    T: Number + Div<U, Output = T>,
    U: Number,
{
    type Output = Self;
    fn div(self, s: U) -> Self::Output {
        Self::Output::new(self.x() / s, self.y() / s, self.z() / s)
    }
}

/// [Point] /= U.
impl<T, U> DivAssign<U> for Point<T>
where
    T: Number + Div<U, Output = T>,
    U: Number,
{
    fn div_assign(&mut self, s: U) {
        self.set_x(self.x() / s);
        self.set_y(self.y() / s);
        self.set_z(self.z() / s);
    }
}

// Required because of orphan rules.
macro_rules! impl_primitive_mul {
    ($($target:ty),*) => {
        $(
            /// T * [Point].
            impl Mul<Point<$target>> for $target {
                type Output = Point<$target>;
                fn mul(self, p: Point<$target>) -> Self::Output {
                    Self::Output::new(self * p.x(), self * p.y(), self * p.z())
                }
            }
        )*
    };
}

impl_primitive_mul!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);

impl<T> Sum for Point<T>
where
    T: Number,
    Self: Add<Output = Self>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let p = Point::new(T::zero(), T::zero(), T::zero());
        iter.fold(p, |a, b| a + b)
    }
}

impl<'a, T: 'a> Sum<&'a Point<T>> for Point<T>
where
    T: Number,
    Self: Add<Output = Self>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        let p = Point::new(T::zero(), T::zero(), T::zero());
        iter.fold(p, |a, b| a + *b)
    }
}

macro_rules! impl_from {
    ($from:ty => $($to:ty),*) => {
        $(
            impl From<Point<$from>> for Point<$to> {
                fn from(p: Point<$from>) -> Self {
                    Self::new(p.x().into(), p.y().into(), p.z().into())
                }
            }
        )*
    };
}

impl_from!(i8 => i16, i32, i64, isize, f32, f64);
impl_from!(u8 => i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);
impl_from!(i16 => i32, i64, isize, f32, f64);
impl_from!(u16 => i32, u32, i64, u64, usize, f32, f64);
impl_from!(i32 => i64, f64);
impl_from!(u32 => i64, u64, f64);
impl_from!(f32 => f64);

macro_rules! impl_try_from {
    ($from:ty => $($to:ty),*) => {
        $(
            impl TryFrom<Point<$from>> for Point<$to> {
                type Error = std::num::TryFromIntError;
                fn try_from(p: Point<$from>) -> Result<Self, Self::Error> {
                    Ok(Self::new(p.x().try_into()?, p.y().try_into()?, p.z().try_into()?))
                }
            }
        )*
    };
}

impl_try_from!(i8 => u8, u16, u32, u64, usize);
impl_try_from!(u8 => i8);
impl_try_from!(i16 => i8, u8, u16, u32, u64, usize);
impl_try_from!(u16 => i8, u8, i16, isize);
impl_try_from!(i32 => i8, u8, i16, u32, u64, isize, usize);
impl_try_from!(u32 => i8, u8, i16, i32, isize, usize);
impl_try_from!(i64 => i8, u8, i16, i32, u32, u64, isize, usize);
impl_try_from!(u64 => i8, u8, i16, i32, u32, i64, isize, usize);
impl_try_from!(isize => i8, u8, i16, u16, i32, u32, i64, u64, usize);
impl_try_from!(usize => i8, u8, i16, u16, i32, u32, i64, u64, isize);

/// Convert [Point] to `[x]`.
impl<T: Number> From<Point<T>> for [T; 1] {
    fn from(p: Point<T>) -> Self {
        [p.x()]
    }
}
/// Convert &[Point] to `[x]`.
impl<T: Number> From<&Point<T>> for [T; 1] {
    fn from(p: &Point<T>) -> Self {
        [p.x()]
    }
}

/// Convert [Point] to `[x, y]`.
impl<T: Number> From<Point<T>> for [T; 2] {
    fn from(p: Point<T>) -> Self {
        [p.x(), p.y()]
    }
}
/// Convert &[Point] to `[x, y]`.
impl<T: Number> From<&Point<T>> for [T; 2] {
    fn from(p: &Point<T>) -> Self {
        [p.x(), p.y()]
    }
}

/// Convert [Point] to `[x, y, z]`.
impl<T: Number> From<Point<T>> for [T; 3] {
    fn from(p: Point<T>) -> Self {
        [p.x(), p.y(), p.z()]
    }
}
/// Convert &[Point] to `[x, y, z]`.
impl<T: Number> From<&Point<T>> for [T; 3] {
    fn from(p: &Point<T>) -> Self {
        [p.x(), p.y(), p.z()]
    }
}

/// Convert `[U; 1]` to [Point].
impl<T: Number, U: Into<T>> From<[U; 1]> for Point<T> {
    fn from([x]: [U; 1]) -> Self {
        Self::new(x.into(), T::zero(), T::zero())
    }
}
/// Convert `&[U; 1]` to [Point].
impl<T: Number, U: Copy + Into<T>> From<&[U; 1]> for Point<T> {
    fn from(&[x]: &[U; 1]) -> Self {
        Self::new(x.into(), T::zero(), T::zero())
    }
}

/// Convert `[U; 2]` to [Point].
impl<T: Number, U: Into<T>> From<[U; 2]> for Point<T> {
    fn from([x, y]: [U; 2]) -> Self {
        Self::new(x.into(), y.into(), T::zero())
    }
}
/// Convert `&[U; 2]` to [Point].
impl<T: Number, U: Copy + Into<T>> From<&[U; 2]> for Point<T> {
    fn from(&[x, y]: &[U; 2]) -> Self {
        Self::new(x.into(), y.into(), T::zero())
    }
}

/// Convert `[U; 3]` to [Point].
impl<T: Number, U: Into<T>> From<[U; 3]> for Point<T> {
    fn from([x, y, z]: [U; 3]) -> Self {
        Self::new(x.into(), y.into(), z.into())
    }
}
/// Convert `&[U; 3]` to [Point].
impl<T: Number, U: Copy + Into<T>> From<&[U; 3]> for Point<T> {
    fn from(&[x, y, z]: &[U; 3]) -> Self {
        Self::new(x.into(), y.into(), z.into())
    }
}

/// Display [Point] as "(x, y, z)".
impl<T> fmt::Display for Point<T>
where
    T: Number + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
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
        let _: Point<u8> = [50u8].into();
        let _: Point<i8> = [50i8].into();
        let _: Point<u16> = [50u16].into();
        let _: Point<i16> = [50i16].into();
        let _: Point<u32> = [50u32].into();
        let _: Point<i32> = [50i32].into();
        let _: Point<f32> = [50.0f32].into();
        let _: Point<f64> = [50.0f64].into();

        let _: Point<u8> = [50u8, 100].into();
        let _: Point<i8> = [50i8, 100].into();
        let _: Point<u16> = [50u16, 100].into();
        let _: Point<i16> = [50i16, 100].into();
        let _: Point<u32> = [50u32, 100].into();
        let _: Point<i32> = [50i32, 100].into();
        let _: Point<f32> = [50.0f32, 100.0].into();
        let _: Point<f64> = [50.0f64, 100.0].into();

        let _: Point<u8> = [50u8, 100, 55].into();
        let _: Point<i8> = [50i8, 100, 55].into();
        let _: Point<u16> = [50u16, 100, 55].into();
        let _: Point<i16> = [50i16, 100, 55].into();
        let _: Point<u32> = [50u32, 100, 55].into();
        let _: Point<i32> = [50i32, 100, 55].into();
        let _: Point<f32> = [50.0f32, 100.0, 55.0].into();
        let _: Point<f64> = [50.0f64, 100.0, 55.0].into();
    }
}
