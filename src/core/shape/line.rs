//! A shape type representing lines used for drawing.
//!
//! # Examples
//!
//! You can create a [Line] using [Line::new]:
//!
//! ```
//! use pix_engine::prelude_3d::*;
//!
//! // 2D
//! let line: LineI2 = Line::new([10, 20], [30, 10]);
//!
//! let p1 = point![10, 20];
//! let p2 = point![30, 10];
//! let line: LineI2 = Line::new(p1, p2);
//!
//! // 3D
//! let line: LineI3 = Line::new([10, 20, 5], [30, 10, 5]);
//! ```

use crate::prelude::*;
use num_traits::AsPrimitive;
// #[cfg(feature = "serde")]
// use serde::{Deserialize, Serialize};
use std::{
    array::IntoIter,
    ops::{Deref, DerefMut, Index, IndexMut},
};

/// A `Line` with start and end [Point]s.
///
/// Please see the [module-level documentation] for examples.
///
/// [module-level documentation]: crate::core::shape::line
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
// TODO: serde is not ready for const generics yet
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Line<T, const N: usize>([Point<T, N>; 2]);

/// A 2D `Line` represented by integers.
pub type LineI2 = Line<i32, 2>;

/// A 3D `Line` represented by integers.
pub type LineI3 = Line<i32, 3>;

/// A 2D `Line` represented by floating point numbers.
pub type LineF2 = Line<Scalar, 2>;

/// A 3D `Line` represented by floating point numbers.
pub type LineF3 = Line<Scalar, 3>;

/// # Constructs a `Line` with two points.
///
/// ```
/// # use pix_engine::prelude_3d::*;
///
/// let l: LineI2 = line_!([10, 20], [30, 10]);
/// assert_eq!(l.values(), [
///   point!(10, 20),
///   point!(30, 10),
/// ]);
///
/// let l: LineI3 = line_!([10, 20, 10], [30, 10, 40]);
/// assert_eq!(l.values(), [
///   point!(10, 20, 10),
///   point!(30, 10, 40),
/// ]);
/// ```
#[macro_export]
macro_rules! line_ {
    ($p1:expr, $p2:expr$(,)?) => {
        $crate::prelude::Line::new($p1, $p2)
    };
    ($x1:expr, $y1:expr, $x2:expr, $y2:expr$(,)?) => {
        $crate::prelude::Line::new([$x1, $y1], [$x2, $y2])
    };
    ($x1:expr, $y1:expr, $z1:expr, $x2:expr, $y2:expr, $z2:expr$(,)?) => {
        $crate::prelude::Line::new([$x1, $y1, $z2], [$x2, $y2, $z2])
    };
}

impl<T, const N: usize> Line<T, N> {
    /// Constructs a `Line` from `start` to `end` [Point]s.
    ///
    /// # Example
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// // 2D
    /// let line: LineI2 = Line::new([10, 20], [30, 10]);
    ///
    /// let p1 = point![10, 20];
    /// let p2 = point![30, 10];
    /// let line: LineI2 = Line::new(p1, p2);
    ///
    /// // 3D
    /// let line: LineI3 = Line::new([10, 20, 5], [30, 10, 5]);
    /// ```
    pub fn new<P>(start: P, end: P) -> Self
    where
        P: Into<Point<T, N>>,
    {
        Self([start.into(), end.into()])
    }
}

impl<T, const N: usize> Line<T, N>
where
    T: Copy + Default,
{
    /// Returns the starting point of the line.
    #[inline]
    pub fn start(&self) -> Point<T, N> {
        self.0[0]
    }

    /// Sets the starting point of the line.
    #[inline]
    pub fn set_start<P: Into<Point<T, N>>>(&mut self, start: P) {
        self.0[0] = start.into();
    }

    /// Returns the ending point of the line.
    #[inline]
    pub fn end(&self) -> Point<T, N> {
        self.0[1]
    }

    /// Sets the ending point of the line.
    #[inline]
    pub fn set_end<P: Into<Point<T, N>>>(&mut self, end: P) {
        self.0[1] = end.into();
    }

    /// Convert `Line<T, N>` to `Line<U, N>` using the `as` operator.
    #[inline]
    pub fn as_<U>(self) -> Line<U, N>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy + Default,
    {
        Line::new(self.start().as_(), self.end().as_())
    }

    /// Returns `Line` coordinates as `[x1, y1, z1, x2, y2, z2]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p1 = point!(5, 10);
    /// let p2 = point!(100, 100);
    /// let l: LineI2 = Line::new(p1, p2);
    /// assert_eq!(l.values(), [point!(5, 10), point!(100, 100)]);
    /// ```
    pub fn values(&self) -> [Point<T, N>; 2] {
        self.0
    }

    /// Returns `Line` as a [Vec].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p1 = point!(5, 10);
    /// let p2 = point!(100, 100);
    /// let l: LineI2 = Line::new(p1, p2);
    /// assert_eq!(l.to_vec(), vec![[5, 10], [100, 100]]);
    /// ```
    pub fn to_vec(self) -> Vec<Vec<T>> {
        let start = self.start().to_vec();
        let end = self.end().to_vec();
        vec![start, end]
    }
}

impl<T> Intersects for Line<T, 2>
where
    T: Num + AsPrimitive<Scalar>,
{
    type Type = T;
    type Shape = Line<Self::Type, 2>;

    /// Returns the closest intersection point with a given line and distance along the line or
    /// `None` if there is no intersection.
    #[allow(clippy::many_single_char_names)]
    fn intersects_line<L>(&self, other: L) -> Option<(Point<Scalar, 2>, Scalar)>
    where
        L: Into<Line<T, 2>>,
    {
        let other = other.into();
        let [x1, y1, x2, y2]: [T; 4] = self.into();
        let [x3, y3, x4, y4]: [T; 4] = other.into();
        let d = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if d == T::zero() {
            return None;
        }
        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / d;
        let u = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / d;
        if (T::zero()..).contains(&t) && (T::zero()..=T::one()).contains(&u) {
            let x = x1 + t * (x2 - x1);
            let y = y1 + t * (y2 - y1);
            Some((point!(x, y).as_(), t.as_()))
        } else {
            None
        }
    }

    /// Returns whether this line intersections with another line
    fn intersects_shape<O>(&self, other: O) -> bool
    where
        O: Into<Self::Shape>,
    {
        self.intersects_line(other).is_some()
    }
}

impl<T, const N: usize> Draw for Line<T, N>
where
    Self: Into<LineI2>,
    T: Copy,
{
    /// Draw `Line` to the current [PixState] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.line(*self)
    }
}

impl<T, const N: usize> Deref for Line<T, N> {
    type Target = [Point<T, N>; 2];
    /// Deref `Line` to `&[[Point<T>; N]; 2]`.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for Line<T, N> {
    /// Deref `Line` to `&mut [[Point<T>; N]; 2]`.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const N: usize> Index<usize> for Line<T, N>
where
    T: Copy,
{
    type Output = Point<T, N>;
    /// Return `&Point<T>` by indexing `Line` with `usize`.
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl<T, const N: usize> IndexMut<usize> for Line<T, N>
where
    T: Copy,
{
    /// Return `&mut Point<T>` by indexing `Line` with `usize`.
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl<T, const N: usize> From<&Line<T, N>> for Line<T, N>
where
    T: Copy,
{
    /// Convert `&Line` to `Line`.
    fn from(line: &Line<T, N>) -> Self {
        *line
    }
}

impl<T, const N: usize> From<&mut Line<T, N>> for Line<T, N>
where
    T: Copy,
{
    /// Convert `&mut Line` to `Line`.
    fn from(line: &mut Line<T, N>) -> Self {
        *line
    }
}

impl<T, const N: usize> IntoIterator for Line<T, N> {
    type Item = Point<T, N>;
    type IntoIter = IntoIter<Self::Item, 2>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.0)
    }
}

macro_rules! impl_from {
    ($($from:ty),* => $to:ty) => {
        $(
            /// Convert `Line<T, N>` to `Line<U, N>`.
            impl<const N: usize> From<Line<$from, N>> for Line<$to, N> {
                fn from(line: Line<$from, N>) -> Self {
                    let start: Point<$to, N> = line.start().into();
                    let end: Point<$to, N> = line.end().into();
                    Self::new(start, end)
                }
            }

            /// Convert `[[T; N]; 2]` to [`Line<T, N>`].
            impl<const N: usize> From<[[$from; N]; 2]> for Line<$to, N> {
                fn from([start, end]: [[$from; N]; 2]) -> Self {
                    Self::new(start, end)
                }
            }

            /// Convert `&[[T; N]; 2]` to [`Line<T, N>`].
            impl<const N: usize> From<&[[$from; N]; 2]> for Line<$to, N> {
                fn from([start, end]: &[[$from; N]; 2]) -> Self {
                    Self::new(start, end)
                }
            }
        )*
    };
}

impl_from!(i8, u8, i16, u16, u32, i64, u64, isize, usize, f32, f64 => i32);
impl_from!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32 => f64);

impl<T, U, const N: usize> From<[Point<U, N>; 2]> for Line<T, N>
where
    Point<U, N>: Into<Point<T, N>>,
{
    /// Convert `[Point<U, N>; 2]` to [`Line<T, N>`].
    fn from([start, end]: [Point<U, N>; 2]) -> Self {
        Self::new(start, end)
    }
}

impl<T, U, const N: usize> From<&[Point<U, N>; 2]> for Line<T, N>
where
    U: Copy,
    Point<U, N>: Into<Point<T, N>>,
{
    /// Convert `&[Point<U, N>; 2]` to [`Line<T, N>`].
    fn from(&[start, end]: &[Point<U, N>; 2]) -> Self {
        Self::new(start, end)
    }
}

impl<T, U, const N: usize> From<Line<U, N>> for [Point<T, N>; 2]
where
    U: Copy + Default,
    Point<U, N>: Into<Point<T, N>>,
{
    /// Convert [`Line<U, N>`] to `[Point<T, N>; 2]`.
    fn from(line: Line<U, N>) -> Self {
        [line.start().into(), line.end().into()]
    }
}

impl<T, U, const N: usize> From<&Line<U, N>> for [Point<T, N>; 2]
where
    Point<U, N>: Into<Point<T, N>>,
{
    /// Convert &[`Line<U, N>`] to `[Point<T, N>; 2]`.
    fn from(line: &Line<U, N>) -> Self {
        line.into()
    }
}

impl<T, U, const N: usize> From<Line<U, N>> for [T; 4]
where
    T: Copy + Default,
    U: Copy + Default,
    Point<U, N>: Into<Point<T, N>>,
{
    /// Convert [`Line<U, N>`] to `[T; 4]`.
    fn from(line: Line<U, N>) -> Self {
        let start = line.start().into();
        let end = line.end().into();
        [start.x(), start.y(), end.x(), end.y()]
    }
}

impl<T, U, const N: usize> From<&Line<U, N>> for [T; 4]
where
    T: Copy + Default,
    U: Copy + Default,
    Point<U, N>: Into<Point<T, N>>,
{
    /// Convert &[`Line<U, N>`] to `[T; 4]`.
    fn from(line: &Line<U, N>) -> Self {
        let start = line.start().into();
        let end = line.end().into();
        [start.x(), start.y(), end.x(), end.y()]
    }
}

impl<T, U, const N: usize> From<Line<U, N>> for [T; 6]
where
    T: Copy + Default,
    U: Copy + Default,
    Point<U, N>: Into<Point<T, N>>,
{
    /// Convert [`Line<U, N>`] to `[T; 4]`.
    fn from(line: Line<U, N>) -> Self {
        let start = line.start().into();
        let end = line.end().into();
        [start.x(), start.y(), start.z(), end.x(), end.y(), end.z()]
    }
}

impl<T, U, const N: usize> From<&Line<U, N>> for [T; 6]
where
    T: Copy + Default,
    U: Copy + Default,
    Point<U, N>: Into<Point<T, N>>,
{
    /// Convert &[`Line<U, N>`] to `[T; 4]`.
    fn from(line: &Line<U, N>) -> Self {
        let start = line.start().into();
        let end = line.end().into();
        [start.x(), start.y(), start.z(), end.x(), end.y(), end.z()]
    }
}
