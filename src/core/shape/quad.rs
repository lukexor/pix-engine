//! A 2D/3D shape type representing a quadrilateral used for drawing.
//!
//! `Quad` is similar to [Rect] but the angles between edges are not constrained to 90 degrees and
//! can also be used to represent a `Plane` in 3D space.
//!
//! # Examples
//!
//! You can create a [Quad] using [Quad::new]:
//!
//! ```
//! # use pix_engine::prelude::*;
//! let quad = Quad::new(10, 20, 30, 10, 20, 25, 15, 15);
//!
//! let quad: Quad<i32> = Quad::with_points(
//!     [10, 20],
//!     [30, 10],
//!     [20, 25],
//!     [15, 15]
//! );
//!
//! let plane: Quad<i32> = Quad::with_points(
//!     [10, 20, 10],
//!     [30, 10, 5],
//!     [20, 25, 20],
//!     [15, 15, 10],
//! );
//! ```

use crate::prelude::*;
use num_traits::{AsPrimitive, Float};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A `Quad` or quadrilateral, a four-sided polygon.
///
/// `Quad` is similar to [Rect] but the angles between edges are not constrained to 90 degrees.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Quad<T = Scalar> {
    /// Point 1
    pub p1: Point<T>,
    /// Point 2
    pub p2: Point<T>,
    /// Point 3
    pub p3: Point<T>,
    /// Point 4
    pub p4: Point<T>,
}

impl<T> Quad<T> {
    /// Constructs a `Quad<T>` with the given [Point]s.
    ///
    /// ```
    /// use pix_engine::prelude::*;
    /// let quad: Quad<i32> = Quad::with_points([10, 20], [30, 10], [20, 25], [15, 15]);
    /// assert_eq!(quad.p1.values(), [10, 20, 0]);
    /// assert_eq!(quad.p2.values(), [30, 10, 0]);
    /// assert_eq!(quad.p3.values(), [20, 25, 0]);
    /// assert_eq!(quad.p4.values(), [15, 15, 0]);
    /// ```
    pub fn with_points<P>(p1: P, p2: P, p3: P, p4: P) -> Self
    where
        P: Into<Point<T>>,
    {
        Self {
            p1: p1.into(),
            p2: p2.into(),
            p3: p3.into(),
            p4: p4.into(),
        }
    }

    /// Convert `Quad<T>` to another primitive type using the `as` operator.
    #[inline]
    pub fn as_<U>(self) -> Quad<U>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy,
    {
        Quad::with_points(self.p1.as_(), self.p2.as_(), self.p3.as_(), self.p4.as_())
    }
}

impl<T: Number> Quad<T> {
    /// Constructs a `Quad<T>` with the given [Point]s.
    ///
    /// ```
    /// use pix_engine::prelude::*;
    /// let quad: Quad<i32> = Quad::with_points([10, 20], [30, 10], [20, 25], [15, 15]);
    /// assert_eq!(quad.p1.values(), [10, 20, 0]);
    /// assert_eq!(quad.p2.values(), [30, 10, 0]);
    /// assert_eq!(quad.p3.values(), [20, 25, 0]);
    /// assert_eq!(quad.p4.values(), [15, 15, 0]);
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new(x1: T, y1: T, x2: T, y2: T, x3: T, y3: T, x4: T, y4: T) -> Self {
        Self {
            p1: point!(x1, y1),
            p2: point!(x2, y2),
            p3: point!(x3, y3),
            p4: point!(x4, y4),
        }
    }

    /// Returns `Quad` coordinates as `[x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let quad: Quad<i32> = Quad::with_points([10, 20], [30, 10], [20, 25], [15, 15]);
    /// assert_eq!(quad.values(), [10, 20, 0, 30, 10, 0, 20, 25, 0, 15, 15, 0]);
    /// ```
    pub fn values(&self) -> [T; 12] {
        let [x1, y1, z1] = self.p1.values();
        let [x2, y2, z2] = self.p2.values();
        let [x3, y3, z3] = self.p3.values();
        let [x4, y4, z4] = self.p4.values();
        [x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]
    }

    /// Returns `Quad` as a [Vec].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let quad: Quad<i32> = Quad::with_points([10, 20], [30, 10], [20, 25], [15, 15]);
    /// assert_eq!(quad.to_vec(), vec![10, 20, 0, 30, 10, 0, 20, 25, 0, 15, 15, 0]);
    /// ```
    pub fn to_vec(self) -> Vec<T> {
        let [x1, y1, z1] = self.p1.values();
        let [x2, y2, z2] = self.p2.values();
        let [x3, y3, z3] = self.p3.values();
        let [x4, y4, z4] = self.p4.values();
        vec![x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]
    }
}

impl<T: Float> Quad<T> {
    /// Returns `Quad` with values rounded to the nearest integer number. Round half-way cases
    /// away from `0.0`.
    pub fn round(&self) -> Self {
        Self::with_points(
            self.p1.round(),
            self.p2.round(),
            self.p3.round(),
            self.p4.round(),
        )
    }
}

impl<T> Draw for Quad<T>
where
    T: Number,
    Self: Into<Quad>,
{
    /// Draw `Quad` to the current [PixState] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.quad(*self)
    }
}

impl<T: Number> From<&mut Quad<T>> for Quad<T> {
    fn from(quad: &mut Quad<T>) -> Self {
        quad.clone()
    }
}

impl<T: Number> From<&Quad<T>> for Quad<T> {
    fn from(quad: &Quad<T>) -> Self {
        *quad
    }
}

/// Convert [Quad] to `[x1, y1, x2, y2, x3, y3, x4, y4]`.
impl<T: Number> From<Quad<T>> for [T; 8] {
    fn from(quad: Quad<T>) -> Self {
        let [x1, y1, _] = quad.p1.values();
        let [x2, y2, _] = quad.p2.values();
        let [x3, y3, _] = quad.p3.values();
        let [x4, y4, _] = quad.p4.values();
        [x1, y1, x2, y2, x3, y3, x4, y4]
    }
}

/// Convert &[Quad] to `[x1, y1, x2, y2, x3, y3, x4, y4]`.
impl<T: Number> From<&Quad<T>> for [T; 8] {
    fn from(quad: &Quad<T>) -> Self {
        let [x1, y1, _] = quad.p1.values();
        let [x2, y2, _] = quad.p2.values();
        let [x3, y3, _] = quad.p3.values();
        let [x4, y4, _] = quad.p4.values();
        [x1, y1, x2, y2, x3, y3, x4, y4]
    }
}

/// Convert `[x1, y1, x2, y2, x3, y3]` to [Quad].
impl<T: Number, U: Into<T>> From<[U; 8]> for Quad<T> {
    fn from([x1, y1, x2, y2, x3, y3, x4, y4]: [U; 8]) -> Self {
        Self::new(
            x1.into(),
            y1.into(),
            x2.into(),
            y2.into(),
            x3.into(),
            y3.into(),
            x4.into(),
            y4.into(),
        )
    }
}

/// Convert `&[x1, y1, x2, y2, x3, y3]` to [Quad].
impl<T: Number, U: Copy + Into<T>> From<&[U; 8]> for Quad<T> {
    fn from(&[x1, y1, x2, y2, x3, y3, x4, y4]: &[U; 8]) -> Self {
        Self::new(
            x1.into(),
            y1.into(),
            x2.into(),
            y2.into(),
            x3.into(),
            y3.into(),
            x4.into(),
            y4.into(),
        )
    }
}

/// Convert [Quad] to `[x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]`.
impl<T: Number> From<Quad<T>> for [T; 12] {
    fn from(quad: Quad<T>) -> Self {
        quad.values()
    }
}

/// Convert &[Quad] to `[x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]`.
impl<T: Number> From<&Quad<T>> for [T; 12] {
    fn from(quad: &Quad<T>) -> Self {
        quad.values()
    }
}

/// Convert `[x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]` to [Quad].
impl<T: Number, U: Into<T>> From<[U; 12]> for Quad<T> {
    fn from([x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]: [U; 12]) -> Self {
        Self::with_points([x1, y1, z1], [x2, y2, z2], [x3, y3, z3], [x4, y4, z4])
    }
}

/// Convert `&[x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]` to [Quad].
impl<T: Number, U: Copy + Into<T>> From<&[U; 12]> for Quad<T> {
    fn from(&[x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]: &[U; 12]) -> Self {
        Self::with_points([x1, y1, z1], [x2, y2, z2], [x3, y3, z3], [x4, y4, z4])
    }
}

/// Convert `[Point<U>; 4]` to [Quad].
impl<T, U> From<[Point<U>; 4]> for Quad<T>
where
    T: Number,
    Point<U>: Into<Point<T>>,
{
    fn from([p1, p2, p3, p4]: [Point<U>; 4]) -> Self {
        Self::with_points(p1, p2, p3, p4)
    }
}

/// Convert `&[Point<U>; 4]` to [Quad].
impl<T, U> From<&[Point<U>; 4]> for Quad<T>
where
    T: Number,
    U: Copy,
    Point<U>: Into<Point<T>>,
{
    fn from(&[p1, p2, p3, p4]: &[Point<U>; 4]) -> Self {
        Self::with_points(p1, p2, p3, p4)
    }
}

/// Convert [Quad] to `[Point<U>; 4]`.
impl<T, U> From<Quad<U>> for [Point<T>; 4]
where
    T: Number,
    Point<U>: Into<Point<T>>,
{
    fn from(quad: Quad<U>) -> Self {
        [
            quad.p1.into(),
            quad.p2.into(),
            quad.p3.into(),
            quad.p4.into(),
        ]
    }
}

/// Convert &[Quad] to `[Point<U>; 4]`.
impl<T, U> From<&Quad<U>> for [Point<T>; 4]
where
    T: Number,
    U: Copy,
    Point<U>: Into<Point<T>>,
{
    fn from(quad: &Quad<U>) -> Self {
        quad.into()
    }
}

/// Convert `[Vector<U>; 4]` to [Quad].
impl<T, U> From<[Vector<U>; 4]> for Quad<T>
where
    T: Number,
    Vector<U>: Into<Point<T>>,
{
    fn from([v1, v2, v3, v4]: [Vector<U>; 4]) -> Self {
        Self::with_points(v1, v2, v3, v4)
    }
}

/// Convert `&[Vector<U>; 4]` to [Quad].
impl<T, U> From<&[Vector<U>; 4]> for Quad<T>
where
    T: Number,
    U: Copy,
    Vector<U>: Into<Point<T>>,
{
    fn from(&[v1, v2, v3, v4]: &[Vector<U>; 4]) -> Self {
        Self::with_points(v1, v2, v3, v4)
    }
}

/// Convert [Quad] to `[Vector<U>; 4]`.
impl<T, U> From<Quad<U>> for [Vector<T>; 4]
where
    T: Number,
    Point<U>: Into<Vector<T>>,
{
    fn from(quad: Quad<U>) -> Self {
        [
            quad.p1.into(),
            quad.p2.into(),
            quad.p3.into(),
            quad.p4.into(),
        ]
    }
}

/// Convert &[Quad] to `[Vector<U>; 4]`.
impl<T, U> From<&Quad<U>> for [Vector<T>; 4]
where
    T: Number,
    U: Copy,
    Point<U>: Into<Vector<T>>,
{
    fn from(quad: &Quad<U>) -> Self {
        quad.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    macro_rules! assert_approx_eq {
        ($i1:expr, $i2:expr) => {
            assert_approx_eq!($i1, $i2, Scalar::EPSILON);
        };
        ($i1:expr, $i2:expr, $e:expr) => {{
            match ($i1, $i2) {
                (Some((p1, t1)), Some((p2, t2))) => {
                    let [x1, y1, z1]: [Scalar; 3] = p1.into();
                    let [x2, y2, z2]: [Scalar; 3] = p2.into();
                    let xd = (x1 - x2).abs();
                    let yd = (y1 - y2).abs();
                    let zd = (z1 - z2).abs();
                    let td = (t1 - t2).abs();
                    assert!(xd < $e, "x: ({} - {}) < {}", x1, x2, $e);
                    assert!(yd < $e, "y: ({} - {}) < {}", y1, y2, $e);
                    assert!(zd < $e, "z: ({} - {}) < {}", z1, z2, $e);
                    assert!(td < $e, "t: ({} - {}) < {}", t1, t2, $e);
                }
                _ => assert_eq!($i1, $i2),
            }
        }};
    }

    #[test]
    fn test_intersects_line() {
        let rect: Rect = rect!(10.0, 10.0, 100.0, 100.0);

        // Left
        let line = Line::new([3.0, 7.0], [20.0, 30.0]);
        assert_approx_eq!(
            rect.intersects_line(&line),
            Some((point!(10.0, 16.471), 0.411)),
            0.001
        );

        // Right
        let line = Line::new([150, 50], [90, 30]);
        assert_approx_eq!(
            rect.intersects_line(&line),
            Some((point!(110.0, 36.667), 0.667)),
            0.001
        );

        // Top
        let line = Line::new([50, 5], [70, 30]);
        assert_approx_eq!(
            rect.intersects_line(&line),
            Some((point!(54.0, 10.0), 0.2)),
            0.001
        );

        // Bottom
        let line = Line::new([50, 150], [30, 30]);
        assert_approx_eq!(
            rect.intersects_line(&line),
            Some((point!(43.3333, 110.0), 0.333)),
            0.001
        );
    }
}
