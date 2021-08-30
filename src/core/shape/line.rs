//! A 2D/3D shape type representing a line used for drawing.
//!
//! # Examples
//!
//! You can create a [Line] using [Line::new]:
//!
//! ```
//! # use pix_engine::prelude::*;
//! // 2D
//! let line: Line<i32> = Line::new([10, 20], [30, 10]);
//!
//! let p1 = point![10, 20];
//! let p2 = point![30, 10];
//! let line: Line<i32> = Line::new(p1, p2);
//!
//! // 3D
//! let line: Line<i32> = Line::new([10, 20, 5], [30, 10, 5]);
//! ```

use crate::prelude::*;
use num_traits::{AsPrimitive, Float};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A `Line` with a starting [Point] and ending [Point<T>].
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Line<T = Scalar> {
    /// Start of line.
    pub start: Point<T>,
    /// End of line.
    pub end: Point<T>,
}

impl<T> Line<T> {
    /// Constructs a `Line` from `start` to `end` [Point]s.
    pub fn new<P>(start: P, end: P) -> Self
    where
        P: Into<Point<T>>,
    {
        Self {
            start: start.into(),
            end: end.into(),
        }
    }

    /// Convert `Line` to another primitive type using the `as` operator.
    #[inline]
    pub fn as_<U>(self) -> Line<U>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy,
    {
        Line::new(self.start.as_(), self.end.as_())
    }
}

impl<T: Number> Line<T> {
    /// Returns `Line` coordinates as `[x1, y1, z1, x2, y2, z2]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p1 = point!(5, 10);
    /// let p2 = point!(100, 100);
    /// let l: Line<i32> = Line::new(p1, p2);
    /// assert_eq!(l.values(), [5, 10, 0, 100, 100, 0]);
    /// ```
    pub fn values(&self) -> [T; 6] {
        let [x1, y1, z1] = self.start.values();
        let [x2, y2, z2] = self.end.values();
        [x1, y1, z1, x2, y2, z2]
    }

    /// Returns `Line` as a [Vec].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p1 = point!(5, 10);
    /// let p2 = point!(100, 100);
    /// let l: Line<i32> = Line::new(p1, p2);
    /// assert_eq!(l.to_vec(), vec![5, 10, 0, 100, 100, 0]);
    /// ```
    pub fn to_vec(self) -> Vec<T> {
        let [x1, y1, z1] = self.start.values();
        let [x2, y2, z2] = self.end.values();
        vec![x1, y1, z1, x2, y2, z2]
    }
}

impl<T: Float> Line<T> {
    /// Returns `Line` with values rounded to the nearest integer number. Round half-way cases
    /// away from `0.0`.
    pub fn round(&self) -> Self {
        Self::new(self.start.round(), self.end.round())
    }
}

impl<T: Number> Shape<T> for Line<T> {
    type Item = Line<T>;

    /// Returns whether this line intersects with another line.
    #[allow(clippy::many_single_char_names)]
    fn intersects_line<L>(&self, other: L) -> Option<(Point<T>, T)>
    where
        T: Float,
        L: Into<Line<T>>,
    {
        let other = other.into();
        let [x1, y1, x2, y2] = [self.start.x, self.start.y, self.end.x, self.end.y];
        let [x3, y3, x4, y4] = [other.start.x, other.start.y, other.end.x, other.end.y];
        let d = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if d == T::zero() {
            return None;
        }
        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / d;
        let u = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / d;
        if (T::zero()..).contains(&t) && (T::zero()..=T::one()).contains(&u) {
            let x = x1 + t * (x2 - x1);
            let y = y1 + t * (y2 - y1);
            Some((point!(x, y), t))
        } else {
            None
        }
    }
}

impl<T> Draw for Line<T>
where
    T: Number,
    Self: Into<Line>,
{
    /// Draw line to the current [PixState] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.line(*self)
    }
}

impl<T: Number> From<&mut Line<T>> for Line<T> {
    fn from(line: &mut Line<T>) -> Self {
        line.clone()
    }
}

impl<T: Number> From<&Line<T>> for Line<T> {
    fn from(line: &Line<T>) -> Self {
        *line
    }
}

/// Convert [Line] to `[x1, y1, x2, y2]`.
impl<T: Number> From<Line<T>> for [T; 4] {
    fn from(line: Line<T>) -> Self {
        let [x1, y1, _] = line.start.values();
        let [x2, y2, _] = line.end.values();
        [x1, y1, x2, y2]
    }
}

/// Convert &[Line] to `[x1, y1, x2, y2]`.
impl<T: Number> From<&Line<T>> for [T; 4] {
    fn from(line: &Line<T>) -> Self {
        let [x1, y1, _] = line.start.values();
        let [x2, y2, _] = line.end.values();
        [x1, y1, x2, y2]
    }
}

/// Convert `[x1, y1, x2, y2]` to [Line].
impl<T: Number, U: Into<T>> From<[U; 4]> for Line<T> {
    fn from([x1, y1, x2, y2]: [U; 4]) -> Self {
        Self::new([x1, y1], [x2, y2])
    }
}

/// Convert `&[x1, y1, x2, y2]` to [Line].
impl<T: Number, U: Copy + Into<T>> From<&[U; 4]> for Line<T> {
    fn from(&[x1, y1, x2, y2]: &[U; 4]) -> Self {
        Self::new([x1, y1], [x2, y2])
    }
}

/// Convert [Line] to `[x1, y1, z1, x2, y2, z2]`.
impl<T: Number> From<Line<T>> for [T; 6] {
    fn from(line: Line<T>) -> Self {
        line.values()
    }
}

/// Convert &[Line] to `[x1, y1, z1, x2, y2, z2]`.
impl<T: Number> From<&Line<T>> for [T; 6] {
    fn from(line: &Line<T>) -> Self {
        line.values()
    }
}

/// Convert `[x1, y1, z1, x2, y2, z2]` to [Line].
impl<T: Number, U: Into<T>> From<[U; 6]> for Line<T> {
    fn from([x1, y1, z1, x2, y2, z2]: [U; 6]) -> Self {
        Self::new([x1, y1, z1], [x2, y2, z2])
    }
}

/// Convert `&[x1, y1, z1, x2, y2, z2]` to [Line].
impl<T: Number, U: Copy + Into<T>> From<&[U; 6]> for Line<T> {
    fn from(&[x1, y1, z1, x2, y2, z2]: &[U; 6]) -> Self {
        Self::new([x1, y1, z1], [x2, y2, z2])
    }
}

/// Convert `[Point<U>; 2]` to [Line].
impl<T, U> From<[Point<U>; 2]> for Line<T>
where
    T: Number,
    Point<U>: Into<Point<T>>,
{
    fn from([p1, p2]: [Point<U>; 2]) -> Self {
        Self::new(p1, p2)
    }
}

/// Convert `&[Point<U>; 2]` to [Line].
impl<T, U> From<&[Point<U>; 2]> for Line<T>
where
    T: Number,
    U: Copy,
    Point<U>: Into<Point<T>>,
{
    fn from(&[p1, p2]: &[Point<U>; 2]) -> Self {
        Self::new(p1, p2)
    }
}

/// Convert [Line] to `[Point<T>; 2]`.
impl<T, U> From<Line<U>> for [Point<T>; 2]
where
    T: Number,
    Point<U>: Into<Point<T>>,
{
    fn from(line: Line<U>) -> Self {
        [line.start.into(), line.end.into()]
    }
}

/// Convert &[Line] to `[Point<T>; 2]`.
impl<T, U> From<&Line<U>> for [Point<T>; 2]
where
    T: Number,
    U: Copy,
    Point<U>: Into<Point<T>>,
{
    fn from(line: &Line<U>) -> Self {
        [line.start.into(), line.end.into()]
    }
}

/// Convert `[Vector<U>; 2]` to [Line].
impl<T, U> From<[Vector<U>; 2]> for Line<T>
where
    T: Number,
    Vector<U>: Into<Point<T>>,
{
    fn from([v1, v2]: [Vector<U>; 2]) -> Self {
        Self::new(v1, v2)
    }
}

/// Convert `&[Vector<U>; 2]` to [Line].
impl<T, U> From<&[Vector<U>; 2]> for Line<T>
where
    T: Number,
    U: Copy,
    Vector<U>: Into<Point<T>>,
{
    fn from(&[v1, v2]: &[Vector<U>; 2]) -> Self {
        Self::new(v1, v2)
    }
}

/// Convert [Line] to `[Vector<T>; 2]`.
impl<T, U> From<Line<U>> for [Vector<T>; 2]
where
    T: Number,
    Point<U>: Into<Vector<T>>,
{
    fn from(line: Line<U>) -> Self {
        [line.start.into(), line.end.into()]
    }
}

/// Convert &[Line] to `[Vector<T>; 2]`.
impl<T, U> From<&Line<U>> for [Vector<T>; 2]
where
    T: Number,
    U: Copy,
    Point<U>: Into<Vector<T>>,
{
    fn from(line: &Line<U>) -> Self {
        [line.start.into(), line.end.into()]
    }
}
