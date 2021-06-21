//! [`Point`] functions used for drawing.

use crate::vector::Vector;
use num_traits::{AsPrimitive, Num};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt, iter::Sum, ops::*};

/// A `Point`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Point<T> {
    /// X-coord
    pub x: T,
    /// Y-coord
    pub y: T,
    /// Z-coord
    pub z: T,
}

/// # Constructs a [`Point<T>`].
///
/// ```
/// use pix_engine::prelude::*;
///
/// let p = point!(1, 2, 0);
/// assert_eq!(p.get(), [1, 2, 0]);
/// ```
#[macro_export]
macro_rules! point {
    () => {
        $crate::shape::point::Point::default()
    };
    ($x:expr) => {
        $crate::shape::point::Point::new_x($x)
    };
    ($x:expr, $y:expr$(,)?) => {
        $crate::shape::point::Point::new_xy($x, $y)
    };
    ($x:expr, $y:expr, $z:expr$(,)?) => {
        $crate::shape::point::Point::new($x, $y, $z)
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
    /// assert_eq!(p.get(), [2, 3, 1]);
    /// ```
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Constructs a `Point<T>` with only an `x` magnitude.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = Point::new_x(2);
    /// assert_eq!(p.get(), [2, 0, 0]);
    /// ```
    pub fn new_x(x: T) -> Self
    where
        T: Num,
    {
        Self {
            x,
            y: T::zero(),
            z: T::zero(),
        }
    }

    /// Constructs a `Point<T>` with only `x` and `y magnitudes.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = Point::new_xy(2, 3);
    /// assert_eq!(p.get(), [2, 3, 0]);
    /// ```
    pub fn new_xy(x: T, y: T) -> Self
    where
        T: Num,
    {
        Self { x, y, z: T::zero() }
    }

    /// Constructs a `Point<T>` from a [`Vector<T>`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v: Vector<f64> = vector!(1.0, 2.0);
    /// let p = Point::from_vector(v);
    /// assert_eq!(p.get(), [1.0, 2.0, 0.0]);
    /// ```
    pub fn from_vector(v: impl Into<Vector<T>>) -> Self {
        let v = v.into();
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }

    /// Copy the current `Point`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p1 = point!(1, 0, 1);
    /// let mut p2 = p1.copy();
    /// p2.x = 2;
    /// assert_eq!(p1.get(), [1, 0, 1]);
    /// assert_eq!(p2.get(), [2, 0, 1]);
    /// ```
    pub fn copy(&self) -> Self
    where
        T: Copy,
    {
        *self
    }

    /// Returns `Point` coordinates as `[x, y, z]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = point!(2, 1, 3);
    /// assert_eq!(p.get(), [2, 1, 3]);
    /// ```
    pub fn get(&self) -> [T; 3]
    where
        T: Copy,
    {
        [self.x, self.y, self.z]
    }

    /// Set `Point` coordinates from any type that implements [`Into<Point<T>>`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut p1 = Point::new(2, 1, 3);
    /// assert_eq!(p1.get(), [2, 1, 3]);
    /// p1.set((1, 2, 4));
    /// assert_eq!(p1.get(), [1, 2, 4]);
    ///
    /// let p2 = Point::new(-2, 5, 1);
    /// p1.set(p2);
    /// assert_eq!(p1.get(), [-2, 5, 1]);
    /// ```
    pub fn set(&mut self, p: impl Into<Point<T>>) {
        let p = p.into();
        self.x = p.x;
        self.y = p.y;
        self.z = p.z;
    }

    /// Converts [`Point<T>`] to [`Vector<U>`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = point!(1, 2, 3);
    /// let v: Vector<f64> = p.as_vector();
    /// assert_eq!(v.get(), [1.0, 2.0, 3.0]);
    /// ```
    pub fn as_vector<U>(&self) -> Vector<U>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy,
    {
        Vector {
            x: self.x.as_(),
            y: self.y.as_(),
            z: self.z.as_(),
        }
    }
}

impl<T> Index<usize> for Point<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds: the len is 3 but the index is {}", idx),
        }
    }
}

impl<T> IndexMut<usize> for Point<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds: the len is 3 but the index is {}", idx),
        }
    }
}

impl<T> Add for Point<T>
where
    T: Num,
{
    type Output = Self;
    fn add(self, p: Point<T>) -> Self::Output {
        Point::new(self.x + p.x, self.y + p.y, self.z + p.z)
    }
}

impl<T, U> Add<U> for Point<T>
where
    T: Num + Add<U, Output = T>,
    U: Num + Copy,
{
    type Output = Self;
    fn add(self, val: U) -> Self::Output {
        Point::new(self.x + val, self.y + val, self.z + val)
    }
}

impl<T> AddAssign for Point<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, p: Point<T>) {
        self.x += p.x;
        self.y += p.y;
        self.z += p.z;
    }
}

impl<T, U> AddAssign<U> for Point<T>
where
    T: AddAssign<U>,
    U: Num + Copy,
{
    fn add_assign(&mut self, val: U) {
        self.x += val;
        self.y += val;
        self.z += val;
    }
}

impl<T> Sub for Point<T>
where
    T: Num,
{
    type Output = Self;
    fn sub(self, p: Point<T>) -> Self::Output {
        Point::new(self.x - p.x, self.y - p.y, self.z - p.z)
    }
}

impl<T, U> Sub<U> for Point<T>
where
    T: Num + Sub<U, Output = T>,
    U: Num + Copy,
{
    type Output = Self;
    fn sub(self, val: U) -> Self::Output {
        Point::new(self.x - val, self.y - val, self.z - val)
    }
}

impl<T> SubAssign for Point<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, p: Point<T>) {
        self.x -= p.x;
        self.y -= p.y;
        self.z -= p.z;
    }
}

impl<T, U> SubAssign<U> for Point<T>
where
    T: SubAssign<U>,
    U: Num + Copy,
{
    fn sub_assign(&mut self, val: U) {
        self.x -= val;
        self.y -= val;
        self.z -= val;
    }
}

impl<T> Neg for Point<T>
where
    T: Num + Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y, -self.z)
    }
}

impl<T, U> Mul<U> for Point<T>
where
    T: Num + Mul<U, Output = T>,
    U: Num + Copy,
{
    type Output = Self;
    fn mul(self, s: U) -> Self::Output {
        Point::new(self.x * s, self.y * s, self.z * s)
    }
}

impl<T, U> MulAssign<U> for Point<T>
where
    T: MulAssign<U>,
    U: Num + Copy,
{
    fn mul_assign(&mut self, s: U) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }
}

impl<T, U> Div<U> for Point<T>
where
    T: Num + Div<U, Output = T>,
    U: Num + Copy,
{
    type Output = Self;
    fn div(self, s: U) -> Self::Output {
        Point::new(self.x / s, self.y / s, self.z / s)
    }
}

impl<T, U> DivAssign<U> for Point<T>
where
    T: Num + DivAssign<U>,
    U: Num + Copy,
{
    fn div_assign(&mut self, s: U) {
        self.x /= s;
        self.y /= s;
        self.z /= s;
    }
}

impl<T, U> Rem<U> for Point<T>
where
    T: Num + Rem<U, Output = T>,
    U: Num + Copy,
{
    type Output = Self;
    fn rem(self, s: U) -> Self::Output {
        Point::new(self.x % s, self.y % s, self.z % s)
    }
}

impl<T, U> RemAssign<U> for Point<T>
where
    T: Num + RemAssign<U>,
    U: Num + Copy,
{
    fn rem_assign(&mut self, s: U) {
        self.x %= s;
        self.y %= s;
        self.z %= s;
    }
}

impl<T> Sum for Point<T>
where
    Self: Add<Output = Self>,
    T: Num + Add,
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
    Self: Add<Output = Self>,
    T: Num + Add + Copy,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        let p = Point::new(T::zero(), T::zero(), T::zero());
        iter.fold(p, |a, b| a + *b)
    }
}

macro_rules! impl_mul {
    ($target:ty, $zero:expr) => {
        impl Mul<Point<$target>> for $target {
            type Output = Point<$target>;
            fn mul(self, p: Point<$target>) -> Self::Output {
                Point::new(self * p.x, self * p.x, self * p.z)
            }
        }
    };
}

impl_mul!(i8, 0);
impl_mul!(u8, 0);
impl_mul!(i16, 0);
impl_mul!(u16, 0);
impl_mul!(i32, 0);
impl_mul!(u32, 0);
impl_mul!(i64, 0);
impl_mul!(u64, 0);
impl_mul!(i128, 0);
impl_mul!(u128, 0);
impl_mul!(isize, 0);
impl_mul!(usize, 0);
impl_mul!(f32, 0.0);
impl_mul!(f64, 0.0);

/// Converts `T` to [`Point<T>`].
impl<T> From<T> for Point<T>
where
    T: Num + Copy,
{
    fn from(v: T) -> Self {
        Self { x: v, y: v, z: v }
    }
}

/// Converts `(T, T)` to [`Point<T>`].
impl<T> From<(T, T)> for Point<T>
where
    T: Num,
{
    fn from((x, y): (T, T)) -> Self {
        Self { x, y, z: T::zero() }
    }
}

/// Converts `(T, T, T)` to [`Point<T>`].
impl<T> From<(T, T, T)> for Point<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self { x, y, z }
    }
}

/// Converts [`Point<T>`] to `(x, y)`.
impl<T> From<Point<T>> for (T, T) {
    fn from(p: Point<T>) -> Self {
        (p.x, p.y)
    }
}

/// Converts [`Point<T>`] to `(x, y, z)`.
impl<T> From<Point<T>> for (T, T, T) {
    fn from(p: Point<T>) -> Self {
        (p.x, p.y, p.z)
    }
}

/// Converts `[T]` to [`Point<T>`].
impl<T> From<[T; 1]> for Point<T>
where
    T: Num,
{
    fn from([x]: [T; 1]) -> Self {
        Self {
            x,
            y: T::zero(),
            z: T::zero(),
        }
    }
}

/// Converts `[T, T]` to [`Point<T>`].
impl<T> From<[T; 2]> for Point<T>
where
    T: Num,
{
    fn from([x, y]: [T; 2]) -> Self {
        Self { x, y, z: T::zero() }
    }
}

/// Converts `[T, T, T]` to [`Point<T>`].
impl<T> From<[T; 3]> for Point<T> {
    fn from([x, y, z]: [T; 3]) -> Self {
        Self { x, y, z }
    }
}

/// Converts [`Point<T>`] to `[x, y]`.
impl<T> From<Point<T>> for [T; 2] {
    fn from(v: Point<T>) -> Self {
        [v.x, v.y]
    }
}

/// Converts [`Point<T>`] to `[x, y, z]`.
impl<T> From<Point<T>> for [T; 3] {
    fn from(v: Point<T>) -> Self {
        [v.x, v.y, v.z]
    }
}

/// Display [`Point<T>`] as "(x, y, z)".
impl<T> fmt::Display for Point<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
