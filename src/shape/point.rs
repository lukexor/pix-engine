//! 2D/3D Point type used for drawing.

use crate::vector::Vector;
use num::Num;
use num_traits::AsPrimitive;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt, iter::Sum, ops::*};

/// A Point.
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

/// # Create a [Point<T>].
///
/// ```
/// use pix_engine::prelude::*;
///
/// let p = point!(1, 2, 0);
/// assert_eq!(p.values(), [1, 2, 0]);
/// ```
#[macro_export]
macro_rules! point {
    () => {
        point!(0, 0, 0)
    };
    ($x:expr) => {
        point!($x, 0, 0)
    };
    ($x:expr, $y:expr$(,)?) => {
        point!($x, $y, 0)
    };
    ($x:expr, $y:expr, $z:expr$(,)?) => {
        $crate::shape::point::Point::new_3d($x, $y, $z)
    };
}

impl<T> Point<T> {
    /// Create 3D `Point`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = Point::new_3d(2, 3, 1);
    /// assert_eq!(p.get(), (2, 3, 1));
    /// ```
    pub const fn new_3d(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Set `Point` coordinates from (x, y, z).
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut p = Point::new_3d(2, 1, 3);
    /// assert_eq!(p.get(), (2, 1, 3));
    /// p.set((1, 2, 4));
    /// assert_eq!(p.get(), (1, 2, 4));
    /// ```
    pub fn set(&mut self, v: impl Into<Vector<T>>) {
        let v = v.into();
        self.x = v.x;
        self.y = v.y;
        self.z = v.z;
    }
}

impl<T> Point<T>
where
    T: Num + Copy,
{
    /// Create 2D `Point`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = Point::new_2d(2, 3);
    /// assert_eq!(p.get(), (2, 3, 0));
    /// ```
    pub fn new_2d(x: T, y: T) -> Self {
        Self { x, y, z: T::zero() }
    }

    /// Get `Point` coordinates as (x, y, z).
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = point!(2, 1, 3);
    /// assert_eq!(p.get(), (2, 1, 3));
    /// ```
    pub fn get(&self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }

    /// Get `Point` coordinates as [x, y, z].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = point!(1, 2, 0);
    /// assert_eq!(p.values(), [1, 2, 0]);
    /// ```
    pub fn values(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    /// Convert [Point<T>] to [Vector<U>].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = point!(1, 2, 3);
    /// let v: Vector<f64> = p.as_vector();
    /// assert_eq!(v.values(), [1.0, 2.0, 3.0]);
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
    T: Num + Add + Copy,
{
    type Output = Self;
    fn add(self, p: Point<T>) -> Self::Output {
        Point::new_3d(self.x + p.x, self.y + p.y, self.z + p.z)
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

impl<T, U> Add<U> for Point<T>
where
    T: Num + Add<U, Output = T> + Copy,
    U: Num + Copy,
{
    type Output = Self;
    fn add(self, val: U) -> Self::Output {
        Point::new_3d(self.x + val, self.y + val, self.z + val)
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
    T: Num + Sub + Copy,
{
    type Output = Self;
    fn sub(self, p: Point<T>) -> Self::Output {
        Point::new_3d(self.x - p.x, self.y - p.y, self.z - p.z)
    }
}

impl<T, U> Sub<U> for Point<T>
where
    T: Num + Sub<U, Output = T> + Copy,
    U: Num + Copy,
{
    type Output = Self;
    fn sub(self, val: U) -> Self::Output {
        Point::new_3d(self.x - val, self.y - val, self.z - val)
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
    T: Num + Neg<Output = T> + Copy,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Point::new_3d(-self.x, -self.y, -self.z)
    }
}

impl<T, U> Mul<U> for Point<T>
where
    T: Num + Mul<U, Output = T> + Copy,
    U: Num + Copy,
{
    type Output = Self;
    fn mul(self, s: U) -> Self::Output {
        Point::new_3d(self.x * s, self.y * s, self.z * s)
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
    T: Num + Div<U, Output = T> + Copy,
    U: Num + Copy,
{
    type Output = Self;
    fn div(self, s: U) -> Self::Output {
        Point::new_3d(self.x / s, self.y / s, self.z / s)
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

impl<T> Rem for Point<T>
where
    T: Num + Rem,
{
    type Output = Self;
    fn rem(mut self, p: Point<T>) -> Self::Output {
        if p.x != T::zero() {
            self.x = self.x % p.x;
        }
        if p.y != T::zero() {
            self.y = self.y % p.y;
        }
        if p.z != T::zero() {
            self.z = self.z % p.z;
        }
        self
    }
}

impl<T> RemAssign for Point<T>
where
    T: Num + RemAssign,
{
    fn rem_assign(&mut self, p: Point<T>) {
        if p.x != T::zero() {
            self.x %= p.x;
        }
        if p.y != T::zero() {
            self.y %= p.y;
        }
        if p.z != T::zero() {
            self.z %= p.z;
        }
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
        let p = Point {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        };
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
        let p = Point {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        };
        iter.fold(p, |a, b| a + *b)
    }
}

macro_rules! impl_op {
    ($target:ty, $zero:expr) => {
        impl Mul<Point<$target>> for $target {
            type Output = Point<$target>;
            fn mul(self, p: Point<$target>) -> Self::Output {
                Point::new_3d(self * p.x, self * p.x, self * p.z)
            }
        }

        impl Div<Point<$target>> for $target {
            type Output = Point<$target>;
            fn div(self, p: Point<$target>) -> Self::Output {
                if p.x == $zero || p.y == $zero || p.z == $zero {
                    panic!("divisor is zero");
                }
                Point::new_3d(self / p.x, self / p.x, self / p.z)
            }
        }
    };
}

impl_op!(i8, 0);
impl_op!(u8, 0);
impl_op!(i16, 0);
impl_op!(u16, 0);
impl_op!(i32, 0);
impl_op!(u32, 0);
impl_op!(i64, 0);
impl_op!(u64, 0);
impl_op!(i128, 0);
impl_op!(u128, 0);
impl_op!(isize, 0);
impl_op!(usize, 0);
impl_op!(f32, 0.0);
impl_op!(f64, 0.0);

/// Convert `T` to [Point<T>].
impl<T> From<T> for Point<T>
where
    T: Num + Copy,
{
    fn from(v: T) -> Self {
        Self { x: v, y: v, z: v }
    }
}

/// Convert `(T, T)` to [Point<T>].
impl<T> From<(T, T)> for Point<T>
where
    T: Num,
{
    fn from((x, y): (T, T)) -> Self {
        Self { x, y, z: T::zero() }
    }
}

/// Convert `(T, T, T)` to [Point<T>].
impl<T> From<(T, T, T)> for Point<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self { x, y, z }
    }
}

/// Convert [Point<T>] into a (x, y) tuple.
impl<T> From<Point<T>> for (T, T) {
    fn from(p: Point<T>) -> Self {
        (p.x, p.y)
    }
}

/// Convert [Point<T>] into a (x, y, z) tuple.
impl<T> From<Point<T>> for (T, T, T) {
    fn from(p: Point<T>) -> Self {
        (p.x, p.y, p.z)
    }
}

/// Display [Point<T>] as "(x, y, z)".
impl<T> fmt::Display for Point<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
