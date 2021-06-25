//! [`Point`] functions used for drawing.

use crate::prelude::{Draw, PixResult, PixState, Scalar, Vector};
use num_traits::{AsPrimitive, Float, Num};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    array::IntoIter,
    fmt,
    iter::{once, Chain, FromIterator, Once, Sum},
    ops::*,
};

/// A `Point`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Point<T = Scalar> {
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
        $crate::shape::point::Point::with_x($x)
    };
    ($x:expr, $y:expr$(,)?) => {
        $crate::shape::point::Point::with_xy($x, $y)
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
        Self::new(v.x, v.y, v.z)
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
    pub fn get(self) -> [T; 3]
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
    /// p1.set([1, 2, 4]);
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

    /// Returns an itereator over the `Point`s coordinates `[x, y, z]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p: Point<i32> = point!(1, 2, -4);
    /// let mut iterator = p.iter();
    ///
    /// assert_eq!(iterator.next(), Some(&1));
    /// assert_eq!(iterator.next(), Some(&2));
    /// assert_eq!(iterator.next(), Some(&-4));
    /// assert_eq!(iterator.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self)
    }

    /// Returns an itereator over the `Point` that allows modifying each value.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut p: Point<i32> = point!(1, 2, -4);
    /// for value in p.iter_mut() {
    ///     *value *= 2;
    /// }
    /// assert_eq!(p.get(), [2, 4, -8]);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut::new(self)
    }

    /// Converts [`Point<T>`] to [`Point<i16>`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p: Point<f32> = point!(f32::MAX, 2.0, 3.0);
    /// let p = p.as_i16();
    /// assert_eq!(p.get(), [i16::MAX, 2, 3]);
    /// ```
    pub fn as_i16(&self) -> Point<i16>
    where
        T: AsPrimitive<i16>,
    {
        Point::new(self.x.as_(), self.y.as_(), self.z.as_())
    }

    /// Converts [`Point<T>`] to [`Point<i32>`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p: Point<f32> = point!(f32::MAX, 2.0, 3.0);
    /// let p = p.as_i32();
    /// assert_eq!(p.get(), [i32::MAX, 2, 3]);
    /// ```
    pub fn as_i32(&self) -> Point<i32>
    where
        T: AsPrimitive<i32>,
    {
        Point::new(self.x.as_(), self.y.as_(), self.z.as_())
    }
}

impl<T: Num> Point<T> {
    /// Constructs a `Point<T>` with only an `x` magnitude.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = Point::with_x(2);
    /// assert_eq!(p.get(), [2, 0, 0]);
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
    /// assert_eq!(p.get(), [2, 3, 0]);
    /// ```
    pub fn with_xy(x: T, y: T) -> Self {
        Self::new(x, y, T::zero())
    }

    /// Constructs a `Point<T>` by shifting coordinates by given x, y, and z values.
    pub fn offset<U>(self, x: U, y: U, z: U) -> Self
    where
        T: Add<U, Output = T>,
    {
        Self::new(self.x + x, self.y + y, self.z + z)
    }

    /// Constructs a `Point<T>` by multiplying it by the given scale factor.
    pub fn scale<U>(self, s: U) -> Self
    where
        T: Mul<U, Output = T>,
        U: Num + Copy,
    {
        self * s
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
        Vector::new(self.x.as_(), self.y.as_(), self.z.as_())
    }

    /// Returns whether two points are approximately equal.
    pub fn approx_eq(&self, other: Point<T>, epsilon: T) -> bool
    where
        T: Float,
    {
        let xd = (self.x - other.x).abs();
        let yd = (self.y - other.y).abs();
        let zd = (self.z - other.z).abs();
        xd < epsilon && yd < epsilon && zd < epsilon
    }
}

impl<T> Draw for Point<T>
where
    Point<T>: Copy + Into<Point<Scalar>>,
{
    /// Draw point to the current [`PixState`] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.point(*self)
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

impl<T> ExactSizeIterator for Iter<'_, T> {}
impl<T> ExactSizeIterator for IterMut<'_, T> {}

impl<T> FromIterator<T> for Point<T>
where
    T: Num,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut xyz = [T::zero(), T::zero(), T::zero()];
        for (i, p) in iter.into_iter().enumerate() {
            xyz[i] = p;
        }
        let [x, y, z] = xyz;
        Self::new(x, y, z)
    }
}

impl<T> IntoIterator for Point<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item, 3>;

    /// Owned `Point<T>` iterator over `[x, y, z]`.
    ///
    /// This struct is created by the [`into_iter`](Point::into_iter) method on [`Point`]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p: Point<i32> = point!(1, 2, -4);
    /// let mut iterator = p.into_iter();
    ///
    /// assert_eq!(iterator.next(), Some(1));
    /// assert_eq!(iterator.next(), Some(2));
    /// assert_eq!(iterator.next(), Some(-4));
    /// assert_eq!(iterator.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new([self.x, self.y, self.z])
    }
}

/// Immutable `Point<T>` iterator over `[x, y, z]`.
///
/// This struct is created by the [`iter`](Point::iter) method on [`Point`]s.
///
/// # Example
///
/// ```
/// # use pix_engine::prelude::*;
/// let p: Point<i32> = point!(1, 2, -4);
/// let mut iterator = p.iter();
///
/// assert_eq!(iterator.next(), Some(&1));
/// assert_eq!(iterator.next(), Some(&2));
/// assert_eq!(iterator.next(), Some(&-4));
/// assert_eq!(iterator.next(), None);
/// ```
#[derive(Debug, Clone)]
pub struct Iter<'a, T = Scalar> {
    inner: [&'a T; 3],
    current: usize,
}

impl<'a, T> Iter<'a, T> {
    #[inline]
    fn new(p: &'a Point<T>) -> Self {
        Self {
            inner: [&p.x, &p.y, &p.z],
            current: 0,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current > 2 {
            return None;
        }
        let next = self.inner[self.current];
        self.current += 1;
        Some(next)
    }
}

impl<'a, T> IntoIterator for &'a Point<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

type ThreeChain<T> = Chain<Chain<Once<T>, Once<T>>, Once<T>>;

/// Mutable `Point<T>` iterator over `[x, y, z]`.
///
/// This struct is created by the [`iter_mut`](Point::iter_mut) method on [`Point`]s.
///
/// # Example
///
/// ```
/// # use pix_engine::prelude::*;
/// let mut p = point!(1, 2, -4);
/// for value in p.iter_mut() {
///     *value *= 2;
/// }
/// assert_eq!(p.get(), [2, 4, -8]);
/// ```
#[derive(Debug)]
pub struct IterMut<'a, T = Scalar> {
    inner: ThreeChain<&'a mut T>,
}

impl<'a, T> IterMut<'a, T> {
    #[inline]
    fn new(p: &'a mut Point<T>) -> Self {
        Self {
            inner: once(&mut p.x).chain(once(&mut p.y)).chain(once(&mut p.z)),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, T> IntoIterator for &'a mut Point<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

macro_rules! impl_ops {
    ($($target:ty),*) => {
        $(
            impl Mul<Point<$target>> for $target {
                type Output = Point<$target>;
                fn mul(self, p: Point<$target>) -> Self::Output {
                    Point::new(self * p.x, self * p.y, self * p.z)
                }
            }
        )*
    };
}

impl_ops!(i8, u8, i16, u16, i32, u32, i128, u128, isize, usize, f32, f64);

macro_rules! impl_from {
    ($from:ty => $to:ty) => {
        impl From<Point<$from>> for Point<$to> {
            fn from(p: Point<$from>) -> Self {
                Point::new(p.x.into(), p.y.into(), p.z.into())
            }
        }
    };
}

impl_from!(i8 => Scalar);
impl_from!(u8 => Scalar);
impl_from!(i16 => Scalar);
impl_from!(u16 => Scalar);
impl_from!(i32 => Scalar);
impl_from!(u32 => Scalar);
impl_from!(f32 => Scalar);

/// Converts `[U; 1]` to [`Point<T>`].
impl<T: Num, U: Into<T>> From<[U; 1]> for Point<T> {
    fn from([x]: [U; 1]) -> Self {
        Self::new(x.into(), T::zero(), T::zero())
    }
}

/// Converts `&[U; 1]` to [`Point<T>`].
impl<T: Num, U: Into<T> + Copy> From<&[U; 1]> for Point<T> {
    fn from(&[x]: &[U; 1]) -> Self {
        Self::new(x.into(), T::zero(), T::zero())
    }
}

/// Converts `[U; 2]` to [`Point<T>`].
impl<T: Num, U: Into<T>> From<[U; 2]> for Point<T> {
    fn from([x, y]: [U; 2]) -> Self {
        Self::new(x.into(), y.into(), T::zero())
    }
}

/// Converts `&[U; 2]` to [`Point<T>`].
impl<T: Num, U: Into<T> + Copy> From<&[U; 2]> for Point<T> {
    fn from(&[x, y]: &[U; 2]) -> Self {
        Self::new(x.into(), y.into(), T::zero())
    }
}

/// Converts `[U; 3]` to [`Point<T>`].
impl<T: Num, U: Into<T>> From<[U; 3]> for Point<T> {
    fn from([x, y, z]: [U; 3]) -> Self {
        Self::new(x.into(), y.into(), z.into())
    }
}

/// Converts `&[U; 3]` to [`Point<T>`].
impl<T: Num, U: Into<T> + Copy> From<&[U; 3]> for Point<T> {
    fn from(&[x, y, z]: &[U; 3]) -> Self {
        Self::new(x.into(), y.into(), z.into())
    }
}

/// Converts [`Point<U>`] to `[x]`.
impl<T: Num, U: Into<T>> From<Point<U>> for [T; 1] {
    fn from(p: Point<U>) -> Self {
        [p.x.into()]
    }
}

/// Converts [`&Point<U>`] to `[x]`.
impl<T: Num, U: Copy + Into<T>> From<&Point<U>> for [T; 1] {
    fn from(p: &Point<U>) -> Self {
        [p.x.into()]
    }
}

/// Converts [`Point<U>`] to `[x, y]`.
impl<T: Num, U: Into<T>> From<Point<U>> for [T; 2] {
    fn from(p: Point<U>) -> Self {
        [p.x.into(), p.y.into()]
    }
}

/// Converts [`&Point<U>`] to `[x, y]`.
impl<T: Num, U: Copy + Into<T>> From<&Point<U>> for [T; 2] {
    fn from(p: &Point<U>) -> Self {
        [p.x.into(), p.y.into()]
    }
}

/// Converts [`Point<U>`] to `[x, y, z]`.
impl<T: Num, U: Into<T>> From<Point<U>> for [T; 3] {
    fn from(p: Point<U>) -> Self {
        [p.x.into(), p.y.into(), p.z.into()]
    }
}

/// Converts [`&Point<U>`] to `[x, y, z]`.
impl<T: Num, U: Copy + Into<T>> From<&Point<U>> for [T; 3] {
    fn from(p: &Point<U>) -> Self {
        [p.x.into(), p.y.into(), p.z.into()]
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_ops {
        ($val:expr) => {
            // Mul<T> for Point
            let p = point!(2, -5, 0) * $val;
            assert_eq!(p.get(), [4, -10, 0]);

            // Mul<point> for T
            let p = $val * point!(2, -5, 0);
            assert_eq!(p.get(), [4, -10, 0]);

            // MulAssign<T> for point
            let mut p = point!(2, -5, 0);
            p *= $val;
            assert_eq!(p.get(), [4, -10, 0]);

            // Div<T> for point
            let p = point!(2, -6, 0) / $val;
            assert_eq!(p.get(), [1, -3, 0]);

            // DivAssign<T> for point
            let mut p = point!(2, -4, 0);
            p /= $val;
            assert_eq!(p.get(), [1, -2, 0]);
        };
    }

    #[test]
    fn test_ops() {
        // Add
        let p1 = point!(2, 5, 1);
        let p2 = point!(1, 5, -1);
        let p3 = p1 + p2;
        assert_eq!(p3.get(), [3, 10, 0]);

        // AddAssign
        let mut p1 = point!(2, 5, 1);
        let p2 = point!(1, 5, -1);
        p1 += p2;
        assert_eq!(p1.get(), [3, 10, 0]);

        // Sub
        let p1 = point!(2, 1, 2);
        let p2 = point!(1, 5, 3);
        let p3 = p1 - p2;
        assert_eq!(p3.get(), [1, -4, -1]);

        // SubAssign
        let mut p1 = point!(2, 1, 2);
        let p2 = point!(1, 5, 3);
        p1 -= p2;
        assert_eq!(p1.get(), [1, -4, -1]);

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
