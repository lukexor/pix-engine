//! A shape type representing quadrilaterals used for drawing.
//!
//! `Quad` is similar to [Rect] but the angles between edges are not constrained to 90 degrees and
//! can also be used to represent a `Plane` in 3D space.
//!
//! # Examples
//!
//! You can create a [Quad] using [Quad::new]:
//!
//! ```
//! use pix_engine::prelude_3d::*;
//!
//! let quad: QuadI2 = Quad::new(
//!     [10, 20],
//!     [30, 10],
//!     [20, 25],
//!     [15, 15]
//! );
//!
//! let plane: QuadI3 = Quad::new(
//!     [10, 20, 10],
//!     [30, 10, 5],
//!     [20, 25, 20],
//!     [15, 15, 10],
//! );
//! ```

use crate::prelude::*;
use num_traits::AsPrimitive;
// #[cfg(feature = "serde")]
// use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut, Index, IndexMut};

/// A `Quad` or quadrilateral, a four-sided polygon.
///
/// `Quad` is similar to [Rect] but the angles between edges are not constrained to 90 degrees.
///
/// Please see the [module-level documentation] for examples.
///
/// [module-level documentation]: crate::core::shape::quad
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Quad<T, const N: usize>([Point<T, N>; 4]);

/// A 2D `Quad` represented by integers.
pub type QuadI2 = Quad<i32, 2>;

/// A 3D `Quad` represented by integers.
pub type QuadI3 = Quad<i32, 3>;

/// A 2D `Quad` represented by floating point numbers.
pub type QuadF2 = Quad<Scalar, 2>;

/// A 3D `Quad` represented by floating point numbers.
pub type QuadF3 = Quad<Scalar, 3>;

impl<T, const N: usize> Quad<T, N> {
    /// Constructs a `Quad` with the given [Point]s.
    ///
    /// ```
    /// use pix_engine::prelude::*;
    /// let quad: QuadI2 = Quad::new([10, 20], [30, 10], [20, 25], [15, 15]);
    /// assert_eq!(quad.p1().values(), [10, 20]);
    /// assert_eq!(quad.p2().values(), [30, 10]);
    /// assert_eq!(quad.p3().values(), [20, 25]);
    /// assert_eq!(quad.p4().values(), [15, 15]);
    /// ```
    pub fn new<P>(p1: P, p2: P, p3: P, p4: P) -> Self
    where
        P: Into<Point<T, N>>,
    {
        Self([p1.into(), p2.into(), p3.into(), p4.into()])
    }
}

impl<T, const N: usize> Quad<T, N>
where
    T: Copy,
{
    /// Returns the first point of the quad.
    #[inline]
    pub fn p1(&self) -> Point<T, N> {
        self.0[0]
    }

    /// Sets the first point of the quad.
    #[inline]
    pub fn set_p1<P>(&mut self, p: P)
    where
        P: Into<Point<T, N>>,
    {
        self.0[0] = p.into();
    }

    /// Returns the second point of the quad.
    #[inline]
    pub fn p2(&self) -> Point<T, N> {
        self.0[1]
    }

    /// Sets the second point of the quad.
    #[inline]
    pub fn set_p2<P>(&mut self, p: P)
    where
        P: Into<Point<T, N>>,
    {
        self.0[1] = p.into();
    }

    /// Returns the third point of the quad.
    #[inline]
    pub fn p3(&self) -> Point<T, N> {
        self.0[2]
    }

    /// Sets the third point of the quad.
    #[inline]
    pub fn set_p3<P>(&mut self, p: P)
    where
        P: Into<Point<T, N>>,
    {
        self.0[2] = p.into();
    }

    /// Returns the fourth point of the quad.
    #[inline]
    pub fn p4(&self) -> Point<T, N> {
        self.0[3]
    }

    /// Sets the fourth point of the quad.
    #[inline]
    pub fn set_p4<P>(&mut self, p: P)
    where
        P: Into<Point<T, N>>,
    {
        self.0[3] = p.into();
    }

    /// Convert `Quad<T, N>` to `Quad<U, N>` using the `as` operator.
    #[inline]
    pub fn as_<U>(self) -> Quad<U, N>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy + Default,
    {
        Quad::new(
            self.p1().as_(),
            self.p2().as_(),
            self.p3().as_(),
            self.p4().as_(),
        )
    }

    /// Returns `Quad` points as `[Point<T, N>; 4]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let quad: QuadI2 = Quad::new([10, 20], [30, 10], [20, 25], [15, 15]);
    /// assert_eq!(quad.values(), [
    ///     point!(10, 20),
    ///     point!(30, 10),
    ///     point!(20, 25),
    ///     point!(15, 15)
    /// ]);
    /// ```
    pub fn values(&self) -> [Point<T, N>; 4] {
        [self.p1(), self.p2(), self.p3(), self.p4()]
    }

    /// Returns `Quad` points as a [Vec].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let quad: QuadI2 = Quad::new([10, 20], [30, 10], [20, 25], [15, 15]);
    /// assert_eq!(quad.to_vec(), vec![
    ///     point!(10, 20),
    ///     point!(30, 10),
    ///     point!(20, 25),
    ///     point!(15, 15)
    /// ]);
    /// ```
    pub fn to_vec(self) -> Vec<Point<T, N>> {
        self.0.to_vec()
    }
}

impl<T, const N: usize> Draw for Quad<T, N>
where
    Self: Into<QuadI2>,
    T: Copy,
{
    /// Draw `Quad` to the current [PixState] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.quad(*self)
    }
}

impl<T, const N: usize> Deref for Quad<T, N> {
    type Target = [Point<T, N>; 4];
    /// Deref `Quad` to `&[Point; 4]`.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for Quad<T, N> {
    /// Deref `Quad` to `&mut [Point; 4]`.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const N: usize> Index<usize> for Quad<T, N>
where
    T: Copy,
{
    type Output = Point<T, N>;
    /// Return `&T` by indexing `Quad` with `usize`.
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl<T, const N: usize> IndexMut<usize> for Quad<T, N>
where
    T: Copy,
{
    /// Return `&mut T` by indexing `Quad` with `usize`.
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl<T, const N: usize> From<&Quad<T, N>> for Quad<T, N>
where
    T: Copy,
{
    /// Convert `&Quad` to `Quad`.
    fn from(quad: &Quad<T, N>) -> Self {
        *quad
    }
}

impl<T, const N: usize> From<&mut Quad<T, N>> for Quad<T, N>
where
    T: Copy,
{
    /// Convert `&mut Quad` to `Quad`.
    fn from(quad: &mut Quad<T, N>) -> Self {
        *quad
    }
}

impl<T, U, const N: usize> From<[Point<U, N>; 4]> for Quad<T, N>
where
    Point<U, N>: Into<Point<T, N>>,
{
    /// Convert `[Point<U, N>; 4]` to [`Quad<T, N>`].
    fn from([p1, p2, p3, p4]: [Point<U, N>; 4]) -> Self {
        Self::new(p1, p2, p3, p4)
    }
}

impl<T, U, const N: usize> From<&[Point<U, N>; 4]> for Quad<T, N>
where
    U: Copy,
    Point<U, N>: Into<Point<T, N>>,
{
    /// Convert `&[Point<U, N>; 4]` to [`Quad<T, N>`].
    fn from(&[p1, p2, p3, p4]: &[Point<U, N>; 4]) -> Self {
        Self::new(p1, p2, p3, p4)
    }
}

impl<T, U, const N: usize> From<Quad<U, N>> for [Point<T, N>; 4]
where
    U: Copy,
    Point<U, N>: Into<Point<T, N>>,
{
    /// Convert [`Quad<U, N>`] to `[Point<T, N>; 4]`.
    fn from(quad: Quad<U, N>) -> Self {
        [
            quad.p1().into(),
            quad.p2().into(),
            quad.p3().into(),
            quad.p4().into(),
        ]
    }
}

impl<T, U, const N: usize> From<&Quad<U, N>> for [Point<T, N>; 4]
where
    Point<U, N>: Into<Point<T, N>>,
{
    /// Convert &[`Quad<U>`] to `[Point<T, N>; 4]`.
    fn from(quad: &Quad<U, N>) -> Self {
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
                    let [x1, y1]: [Scalar; 2] = p1.values();
                    let [x2, y2]: [Scalar; 2] = p2.values();
                    let xd = (x1 - x2).abs();
                    let yd = (y1 - y2).abs();
                    let td = (t1 - t2).abs();
                    assert!(xd < $e, "x: ({} - {}) < {}", x1, x2, $e);
                    assert!(yd < $e, "y: ({} - {}) < {}", y1, y2, $e);
                    assert!(td < $e, "t: ({} - {}) < {}", t1, t2, $e);
                }
                _ => assert_eq!($i1, $i2),
            }
        }};
    }

    #[test]
    fn test_intersects_line() {
        let rect = rect!(10.0, 10.0, 100.0, 100.0);

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
