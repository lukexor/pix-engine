//! [Triangle] type used for drawing.

use crate::{prelude::*, vector::Vector};
use num_traits::{AsPrimitive, Num};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A `Triangle` with three [Point]s.
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
    /// Constructs a `Triangle<T>`.
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

    /// Convert `Triangle<T>` to another primitive type using the `as` operator.
    #[inline]
    pub fn as_<U>(self) -> Triangle<U>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy,
    {
        Triangle::new(self.p1.as_(), self.p2.as_(), self.p3.as_())
    }
}

impl<T> Draw for Triangle<T>
where
    T: Copy,
    Self: Into<Triangle<Scalar>>,
{
    /// Draw triangle to the current [PixState] canvas.
    #[inline]
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.triangle(*self)
    }
}

/// Convert [Triangle] to `[x1, y1, x2, y2, x3, y3]`.
impl<T> From<Triangle<T>> for [T; 6] {
    fn from(tri: Triangle<T>) -> Self {
        let [x1, y1]: [T; 2] = tri.p1.into();
        let [x2, y2]: [T; 2] = tri.p2.into();
        let [x3, y3]: [T; 2] = tri.p3.into();
        [x1, y1, x2, y2, x3, y3]
    }
}

/// Convert &[Triangle] to `[x1, y1, x2, y2, x3, y3]`.
impl<T: Copy> From<&Triangle<T>> for [T; 6] {
    fn from(tri: &Triangle<T>) -> Self {
        let [x1, y1]: [T; 2] = tri.p1.into();
        let [x2, y2]: [T; 2] = tri.p2.into();
        let [x3, y3]: [T; 2] = tri.p3.into();
        [x1, y1, x2, y2, x3, y3]
    }
}

/// Convert `[x1, y1, x2, y2, x3, y3]` to [Triangle].
impl<T: Num, U: Into<T>> From<[U; 6]> for Triangle<T> {
    fn from([x1, y1, x2, y2, x3, y3]: [U; 6]) -> Self {
        Self::new([x1, y1], [x2, y2], [x3, y3])
    }
}

/// Convert `[Point<U>; 3]` to [Triangle].
impl<T, U: Into<T>> From<[Point<U>; 3]> for Triangle<T>
where
    Point<U>: Into<Point<T>>,
{
    fn from([p1, p2, p3]: [Point<U>; 3]) -> Self {
        Self::new(p1, p2, p3)
    }
}

/// Convert [Triangle] to `[Point<U>; 3]`.
impl<T, U: Into<T>> From<Triangle<U>> for [Point<T>; 3]
where
    Point<U>: Into<Point<T>>,
{
    fn from(tri: Triangle<U>) -> Self {
        [tri.p1.into(), tri.p2.into(), tri.p3.into()]
    }
}

/// Convert &[Triangle] to `[Point<U>; 3]`.
impl<T, U: Copy + Into<T>> From<&Triangle<U>> for [Point<T>; 3]
where
    Point<U>: Into<Point<T>>,
{
    fn from(tri: &Triangle<U>) -> Self {
        [tri.p1.into(), tri.p2.into(), tri.p3.into()]
    }
}

/// Convert `[Vector<U>; 3]` to [Triangle].
impl<T, U: Into<T>> From<[Vector<U>; 3]> for Triangle<T>
where
    Vector<U>: Into<Point<T>>,
{
    fn from([v1, v2, v3]: [Vector<U>; 3]) -> Self {
        Self::new(v1, v2, v3)
    }
}

/// Convert [Triangle] to `[Vector<U>; 3]`.
impl<T, U: Into<T>> From<Triangle<U>> for [Vector<T>; 3]
where
    Point<U>: Into<Vector<T>>,
{
    fn from(tri: Triangle<U>) -> Self {
        [tri.p1.into(), tri.p2.into(), tri.p3.into()]
    }
}

/// Convert &[Triangle] to `[Vector<U>; 3]`.
impl<T, U: Copy + Into<T>> From<&Triangle<U>> for [Vector<T>; 3]
where
    Point<U>: Into<Vector<T>>,
{
    fn from(tri: &Triangle<U>) -> Self {
        [tri.p1.into(), tri.p2.into(), tri.p3.into()]
    }
}
