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
//! use pix_engine::prelude::*;
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

/// A `Quad` or quadrilateral, a four-sided polygon.
///
/// `Quad` is similar to [Rect] but the angles between edges are not constrained to 90 degrees.
///
/// Please see the [module-level documentation] for examples.
///
/// [module-level documentation]: crate::shape::quad
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Quad<T, const N: usize>(pub(crate) [Point<T, N>; 4]);

/// A 2D `Quad` represented by integers.
pub type QuadI2 = Quad<i32, 2>;

/// A 3D `Quad` represented by integers.
pub type QuadI3 = Quad<i32, 3>;

/// A 2D `Quad` represented by floating point numbers.
pub type QuadF2 = Quad<Scalar, 2>;

/// A 3D `Quad` represented by floating point numbers.
pub type QuadF3 = Quad<Scalar, 3>;

/// Constructs a [Quad] with four points.
///
/// ```
/// # use pix_engine::prelude::*;
/// let q = quad!([10, 20], [30, 10], [20, 25], [15, 15]);
/// assert_eq!(q.values(), [
///   point!(10, 20),
///   point!(30, 10),
///   point!(20, 25),
///   point!(15, 15),
/// ]);
/// ```
#[macro_export]
macro_rules! quad {
    ($p1:expr, $p2:expr, $p3:expr, $p4: expr$(,)?) => {
        $crate::prelude::Quad::new($p1, $p2, $p3, $p4)
    };
}

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
    T: Copy + Default,
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
        self.0
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
    T: Default + AsPrimitive<i32>,
{
    /// Draw `Quad` to the current [PixState] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.quad(*self)
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
        let line: LineF2 = line_!([3.0, 7.0], [20.0, 30.0]);
        assert_approx_eq!(
            rect.intersects_line(line),
            Some((point!(10.0, 16.471), 0.411)),
            0.001
        );

        // Right
        let line: LineF2 = line_!([150.0, 50.0], [90.0, 30.0]);
        assert_approx_eq!(
            rect.intersects_line(line),
            Some((point!(110.0, 36.667), 0.667)),
            0.001
        );

        // Top
        let line: LineF2 = line_!([50.0, 5.0], [70.0, 30.0]);
        assert_approx_eq!(
            rect.intersects_line(line),
            Some((point!(54.0, 10.0), 0.2)),
            0.001
        );

        // Bottom
        let line: LineF2 = line_!([50.0, 150.0], [30.0, 30.0]);
        assert_approx_eq!(
            rect.intersects_line(line),
            Some((point!(43.3333, 110.0), 0.333)),
            0.001
        );
    }
}
