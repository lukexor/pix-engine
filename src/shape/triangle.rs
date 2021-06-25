//! [`Triangle`] type used for drawing.

use crate::{
    prelude::{Draw, Line, PixResult, PixState, Point, Scalar, Shape, ShapeNum},
    vector::Vector,
};
use num_traits::{AsPrimitive, Num};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A `Triangle` with three [`Point<T>`]s.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Triangle<T = Scalar> {
    /// Point 1
    pub p1: Point<T>,
    /// Point 2
    pub p2: Point<T>,
    /// Point 3
    pub p3: Point<T>,
}

impl<T> Triangle<T> {
    /// Constructs a [`Triangle<T>`].
    pub fn new<P>(p1: P, p2: P, p3: P) -> Self
    where
        P: Into<Point<T>>,
    {
        Self {
            p1: p1.into(),
            p2: p2.into(),
            p3: p3.into(),
        }
    }

    /// Converts [`Triangle<T>`] to [`Triangle<i16>`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let t: Triangle<f32> = Triangle::new([f32::MAX, 2.0], [3.0, f32::MIN], [1.0, 5.0]);
    /// let t = t.as_i16();
    /// assert_eq!(t.p1.get(), [i16::MAX, 2, 0]);
    /// assert_eq!(t.p2.get(), [3, i16::MIN, 0]);
    /// assert_eq!(t.p3.get(), [1, 5, 0]);
    /// ```
    pub fn as_i16(&self) -> Triangle<i16>
    where
        T: AsPrimitive<i16>,
    {
        Triangle::new(self.p1.as_i16(), self.p2.as_i16(), self.p3.as_i16())
    }
}

impl<T: ShapeNum> Shape<T> for Triangle<T> {
    type Item = Triangle<T>;

    /// Returns whether this triangle contains a given [`Point<T>`].
    fn contains_point(&self, _p: impl Into<Point<T>>) -> bool {
        todo!()
    }

    /// Returns whether this triangle completely contains another triangle.
    fn contains(&self, _other: impl Into<Self::Item>) -> bool {
        todo!()
    }

    /// Returns the closest intersection point with a given line and distance along the line or
    /// `None` if there is no intersection.
    fn intersects_line(&self, _line: impl Into<Line<Scalar>>) -> Option<(Point<Scalar>, Scalar)> {
        todo!()
    }

    /// Returns whether this triangle intersects with another triangle.
    fn intersects(&self, _other: impl Into<Self::Item>) -> bool {
        todo!()
    }
}

impl<T> Draw for Triangle<T>
where
    Triangle<T>: Copy + Into<Triangle<Scalar>>,
{
    /// Draw triangle to the current [`PixState`] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.triangle(*self)
    }
}

/// Convert `[x1, y1, x2, y2, x3, y3]` to [`Triangle<T>`].
impl<T: Num, U: Into<T>> From<[U; 6]> for Triangle<T> {
    fn from([x1, y1, x2, y2, x3, y3]: [U; 6]) -> Self {
        Self::new([x1, y1], [x2, y2], [x3, y3])
    }
}

/// Convert [`Triangle<T>`] to `[x1, y1, x2, y2, x3, y3]`.
impl<T: Num, U: Into<T>> From<Triangle<U>> for [T; 6] {
    fn from(tri: Triangle<U>) -> Self {
        let [x1, y1]: [T; 2] = tri.p1.into();
        let [x2, y2]: [T; 2] = tri.p2.into();
        let [x3, y3]: [T; 2] = tri.p3.into();
        [x1, y1, x2, y2, x3, y3]
    }
}

/// Convert [`&Triangle<T>`] to `[x1, y1, x2, y2, x3, y3]`.
impl<T: Num, U: Copy + Into<T>> From<&Triangle<U>> for [T; 6] {
    fn from(tri: &Triangle<U>) -> Self {
        let [x1, y1]: [T; 2] = tri.p1.into();
        let [x2, y2]: [T; 2] = tri.p2.into();
        let [x3, y3]: [T; 2] = tri.p3.into();
        [x1, y1, x2, y2, x3, y3]
    }
}

/// Convert `[Point<U>; 3]` to [`Triangle<T>`].
impl<T, U: Into<T>> From<[Point<U>; 3]> for Triangle<T>
where
    Point<U>: Into<Point<T>>,
{
    fn from([p1, p2, p3]: [Point<U>; 3]) -> Self {
        Self::new(p1, p2, p3)
    }
}

/// Convert [`Triangle<U>`] to `[Point<U>; 3]`.
impl<T, U: Into<T>> From<Triangle<U>> for [Point<T>; 3]
where
    Point<U>: Into<Point<T>>,
{
    fn from(tri: Triangle<U>) -> Self {
        [tri.p1.into(), tri.p2.into(), tri.p3.into()]
    }
}

/// Convert [`&Triangle<U>`] to `[Point<U>; 3]`.
impl<T, U: Copy + Into<T>> From<&Triangle<U>> for [Point<T>; 3]
where
    Point<U>: Into<Point<T>>,
{
    fn from(tri: &Triangle<U>) -> Self {
        [tri.p1.into(), tri.p2.into(), tri.p3.into()]
    }
}

/// Convert `[Vector<U>; 3]` to [`Triangle<T>`].
impl<T, U: Into<T>> From<[Vector<U>; 3]> for Triangle<T>
where
    Vector<U>: Into<Point<T>>,
{
    fn from([v1, v2, v3]: [Vector<U>; 3]) -> Self {
        Self::new(v1, v2, v3)
    }
}

/// Convert [`Triangle<U>`] to `[Vector<U>; 3]`.
impl<T, U: Into<T>> From<Triangle<U>> for [Vector<T>; 3]
where
    Point<U>: Into<Vector<T>>,
{
    fn from(tri: Triangle<U>) -> Self {
        [tri.p1.into(), tri.p2.into(), tri.p3.into()]
    }
}

/// Convert [`&Triangle<U>`] to `[Vector<U>; 3]`.
impl<T, U: Copy + Into<T>> From<&Triangle<U>> for [Vector<T>; 3]
where
    Point<U>: Into<Vector<T>>,
{
    fn from(tri: &Triangle<U>) -> Self {
        [tri.p1.into(), tri.p2.into(), tri.p3.into()]
    }
}
