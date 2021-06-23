//! [`Triangle`] type used for drawing.

use crate::{
    prelude::{Draw, Line, PixResult, PixState, Point, Shape},
    vector::Vector,
};
use num_traits::Num;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A `Triangle` with three [`Point<T>`]s.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Triangle<T> {
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
}

impl<T: Num + PartialOrd> Shape<T> for Triangle<T> {
    type Item = Triangle<T>;
    type DrawType = i16;

    /// Returns whether this triangle contains a given [`Point<T>`].
    fn contains_point(&self, p: impl Into<Point<T>>) -> bool {
        todo!()
    }

    /// Returns whether this triangle completely contains another triangle.
    fn contains(&self, other: impl Into<Self::Item>) -> bool {
        todo!()
    }

    /// Returns whether this triangle intersects with a line.
    fn intersects_line(&self, line: impl Into<Line<f64>>) -> Option<(Point<f64>, Point<f64>)> {
        todo!()
    }

    /// Returns whether this triangle intersects with another triangle.
    fn intersects(&self, other: impl Into<Self::Item>) -> bool {
        todo!()
    }
}

impl<T> Draw for Triangle<T> {
    /// Draw triangle to the current [`PixState`] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.triangle(self)
    }
}

/// Convert `[x1, y1, x2, y2, x3, y3]` to [`Triangle<T>`].
impl<T: Num, U: Into<T>> From<[U; 6]> for Triangle<T> {
    fn from([x1, y1, x2, y2, x3, y3]: [U; 6]) -> Self {
        Self::new([x1, y1], [x2, y2], [x3, y3])
    }
}

/// Convert [`Triangle<T>`] to `[x1, y1, x2, y2, x3, y3]`.
impl<T, U: Into<T>> From<Triangle<U>> for [T; 6] {
    fn from(tri: Triangle<U>) -> Self {
        let [x1, y1, _] = tri.p1.into();
        let [x2, y2, _] = tri.p2.into();
        let [x3, y3, _] = tri.p3.into();
        (x1, y1, x2, y2, x3, y3)
    }
}

/// Convert [`&Triangle<T>`] to `[x1, y1, x2, y2, x3, y3]`.
impl<T, U: Into<T>> From<&Triangle<U>> for [T; 6] {
    fn from(tri: &Triangle<U>) -> Self {
        let [x1, y1] = tri.p1.into();
        let [x2, y2] = tri.p2.into();
        let [x3, y3] = tri.p3.into();
        (x1, y1, x2, y2, x3, y3)
    }
}

/// Convert ([`Point<U>`], [`Point<U>`], [`Point<U>`]) to [`Triangle<T>`].
impl<T, U: Into<T>> From<(Point<U>, Point<U>, Point<U>)> for Triangle<T>
where
    Point<U>: Into<Point<T>>,
{
    fn from((p1, p2, p3): (Point<U>, Point<U>, Point<U>)) -> Self {
        Self::new(p1, p2, p3)
    }
}

/// Convert [`Triangle<U>`] to ([`Point<T>`], [`Point<T>`], [`Point<T>`]).
impl<T, U: Into<T>> From<Triangle<U>> for (Point<T>, Point<T>, Point<T>)
where
    Point<U>: Into<Point<T>>,
{
    fn from(tri: Triangle<U>) -> Self {
        (tri.p1.into(), tri.p2.into(), tri.p3.into())
    }
}

/// Convert [`&Triangle<U>`] to ([`Point<T>`], [`Point<T>`], [`Point<T>`]).
impl<T, U: Into<T>> From<&Triangle<U>> for (Point<T>, Point<T>, Point<T>)
where
    Point<U>: Into<Point<T>>,
{
    fn from(tri: &Triangle<U>) -> Self {
        (tri.p1.into(), tri.p2.into(), tri.p3.into())
    }
}

/// Convert ([`Vector<U>`], [`Vector<U>`], [`Vector<U>`]) to [`Triangle<T>`].
impl<T, U: Into<T>> From<(Vector<U>, Vector<U>, Vector<U>)> for Triangle<T>
where
    Vector<U>: Into<Point<T>>,
{
    fn from((v1, v2, v3): (Vector<U>, Vector<U>, Vector<U>)) -> Self {
        Self::new(v1, v2, v3)
    }
}

/// Convert [`Triangle<U>`] to ([`Vector<T>`], [`Vector<T>`], [`Vector<T>`]).
impl<T, U: Into<T>> From<Triangle<U>> for (Vector<T>, Vector<T>, Vector<T>)
where
    Point<U>: Into<Vector<T>>,
{
    fn from(tri: Triangle<U>) -> Self {
        (tri.p1.into(), tri.p2.into(), tri.p3.into())
    }
}

/// Convert [`&Triangle<U>`] to ([`Vector<T>`], [`Vector<T>`], [`Vector<T>`]).
impl<T, U: Into<T>> From<&Triangle<U>> for (Vector<T>, Vector<T>, Vector<T>)
where
    Point<U>: Into<Vector<T>>,
{
    fn from(tri: &Triangle<U>) -> Self {
        (tri.p1.into(), tri.p2.into(), tri.p3.into())
    }
}
