//! A 2D/3D shape type representing a triangle used for drawing.
//!
//! # Examples
//!
//! You can create a [Triangle] using [Triangle::new]:
//!
//! ```
//! # use pix_engine::prelude::*;
//! // 2D
//! let tri = Triangle::new(10, 20, 30, 10, 20, 25);
//!
//! let p1 = point![10, 20];
//! let p2 = point![30, 10];
//! let p3 = point![20, 25];
//! let tri: Triangle<i32> = Triangle::with_points(p1, p2, p3);
//! ```

use crate::prelude::*;
use num_traits::{AsPrimitive, Float};
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
    /// Constructs a `Triangle<T>` with the given [Point]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let tri: Triangle<i32> = Triangle::with_points([10, 20], [30, 10], [20, 25]);
    /// assert_eq!(tri.p1.values(), [10, 20, 0]);
    /// assert_eq!(tri.p2.values(), [30, 10, 0]);
    /// assert_eq!(tri.p3.values(), [20, 25, 0]);
    /// ```
    pub fn with_points<P: Into<Point<T>>>(p1: P, p2: P, p3: P) -> Self {
        Self {
            p1: p1.into(),
            p2: p2.into(),
            p3: p3.into(),
        }
    }
}

impl<T: Number> Triangle<T> {
    /// Constructs a `Triangle<T>` with the given set of `(x, y)` coordinates.
    ///
    /// ```
    /// use pix_engine::prelude::*;
    /// let tri = Triangle::new(10, 20, 30, 10, 20, 25);
    /// assert_eq!(tri.p1.values(), [10, 20, 0]);
    /// assert_eq!(tri.p2.values(), [30, 10, 0]);
    /// assert_eq!(tri.p3.values(), [20, 25, 0]);
    /// ```
    pub fn new(x1: T, y1: T, x2: T, y2: T, x3: T, y3: T) -> Self {
        Self {
            p1: point!(x1, y1),
            p2: point!(x2, y2),
            p3: point!(x3, y3),
        }
    }

    /// Convert `Triangle<T>` to another primitive type using the `as` operator.
    #[inline]
    pub fn as_<U>(self) -> Triangle<U>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy,
    {
        Triangle::with_points(self.p1.as_(), self.p2.as_(), self.p3.as_())
    }

    /// Returns `Triangle` coordinates as `[x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let tri= Triangle::new(10, 20, 30, 10, 20, 25);
    /// assert_eq!(tri.values(), [10, 20, 0, 30, 10, 0, 20, 25, 0]);
    /// ```
    pub fn values(&self) -> [T; 9] {
        let [x1, y1, z1] = self.p1.values();
        let [x2, y2, z2] = self.p2.values();
        let [x3, y3, z3] = self.p3.values();
        [x1, y1, z1, x2, y2, z2, x3, y3, z3]
    }

    /// Returns `Triangle` as a [Vec].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let tri= Triangle::new(10, 20, 30, 10, 20, 25);
    /// assert_eq!(tri.to_vec(), vec![10, 20, 0, 30, 10, 0, 20, 25, 0]);
    /// ```
    pub fn to_vec(self) -> Vec<T> {
        let [x1, y1, z1] = self.p1.values();
        let [x2, y2, z2] = self.p2.values();
        let [x3, y3, z3] = self.p3.values();
        vec![x1, y1, z1, x2, y2, z2, x3, y3, z3]
    }
}

impl<T: Float> Triangle<T> {
    /// Returns `Triangle` with values rounded to the nearest integer number. Round half-way cases
    /// away from `0.0`.
    pub fn round(&self) -> Self {
        Self::with_points(self.p1.round(), self.p2.round(), self.p3.round())
    }
}

impl<T> Draw for Triangle<T>
where
    T: Number,
    Self: Into<Triangle>,
{
    /// Draw `Triangle` to the current [PixState] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.triangle(*self)
    }
}

impl<T: Number> From<&mut Triangle<T>> for Triangle<T> {
    fn from(tri: &mut Triangle<T>) -> Self {
        tri.clone()
    }
}

impl<T: Number> From<&Triangle<T>> for Triangle<T> {
    fn from(tri: &Triangle<T>) -> Self {
        *tri
    }
}

/// Convert [Triangle] to `[x1, y1, x2, y2, x3, y3]`.
impl<T: Number> From<Triangle<T>> for [T; 6] {
    fn from(tri: Triangle<T>) -> Self {
        let [x1, y1, _] = tri.p1.values();
        let [x2, y2, _] = tri.p2.values();
        let [x3, y3, _] = tri.p3.values();
        [x1, y1, x2, y2, x3, y3]
    }
}

/// Convert &[Triangle] to `[x1, y1, x2, y2, x3, y3]`.
impl<T: Number> From<&Triangle<T>> for [T; 6] {
    fn from(tri: &Triangle<T>) -> Self {
        let [x1, y1, _] = tri.p1.values();
        let [x2, y2, _] = tri.p2.values();
        let [x3, y3, _] = tri.p3.values();
        [x1, y1, x2, y2, x3, y3]
    }
}

/// Convert `[x1, y1, x2, y2, x3, y3]` to [Triangle].
impl<T: Number, U: Into<T>> From<[U; 6]> for Triangle<T> {
    fn from([x1, y1, x2, y2, x3, y3]: [U; 6]) -> Self {
        Self::new(
            x1.into(),
            y1.into(),
            x2.into(),
            y2.into(),
            x3.into(),
            y3.into(),
        )
    }
}

/// Convert `&[x1, y1, x2, y2, x3, y3]` to [Triangle].
impl<T: Number, U: Copy + Into<T>> From<&[U; 6]> for Triangle<T> {
    fn from(&[x1, y1, x2, y2, x3, y3]: &[U; 6]) -> Self {
        Self::new(
            x1.into(),
            y1.into(),
            x2.into(),
            y2.into(),
            x3.into(),
            y3.into(),
        )
    }
}

/// Convert `[Point<U>; 3]` to [Triangle].
impl<T, U> From<[Point<U>; 3]> for Triangle<T>
where
    T: Number,
    Point<U>: Into<Point<T>>,
{
    fn from([p1, p2, p3]: [Point<U>; 3]) -> Self {
        Self::with_points(p1, p2, p3)
    }
}

/// Convert [Triangle] to `[Point<U>; 3]`.
impl<T, U> From<Triangle<U>> for [Point<T>; 3]
where
    T: Number,
    U: Copy,
    Point<U>: Into<Point<T>>,
{
    fn from(tri: Triangle<U>) -> Self {
        [tri.p1.into(), tri.p2.into(), tri.p3.into()]
    }
}

/// Convert &[Triangle] to `[Point<U>; 3]`.
impl<T, U> From<&Triangle<U>> for [Point<T>; 3]
where
    T: Number,
    U: Copy,
    Point<U>: Into<Point<T>>,
{
    fn from(tri: &Triangle<U>) -> Self {
        [tri.p1.into(), tri.p2.into(), tri.p3.into()]
    }
}

/// Convert `[Vector<U>; 3]` to [Triangle].
impl<T, U> From<[Vector<U>; 3]> for Triangle<T>
where
    T: Number,
    Vector<U>: Into<Point<T>>,
{
    fn from([v1, v2, v3]: [Vector<U>; 3]) -> Self {
        Self::with_points(v1, v2, v3)
    }
}

/// Convert [Triangle] to `[Vector<U>; 3]`.
impl<T, U> From<Triangle<U>> for [Vector<T>; 3]
where
    T: Number,
    Point<U>: Into<Vector<T>>,
{
    fn from(tri: Triangle<U>) -> Self {
        [tri.p1.into(), tri.p2.into(), tri.p3.into()]
    }
}

/// Convert &[Triangle] to `[Vector<U>; 3]`.
impl<T, U> From<&Triangle<U>> for [Vector<T>; 3]
where
    T: Number,
    U: Copy,
    Point<U>: Into<Vector<T>>,
{
    fn from(tri: &Triangle<U>) -> Self {
        [tri.p1.into(), tri.p2.into(), tri.p3.into()]
    }
}
