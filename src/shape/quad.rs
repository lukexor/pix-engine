//! A shape type representing quadrilaterals used for drawing.
//!
//! `Quad` is similar to [Rect] but the angles between edges are not constrained to 90 degrees and
//! can also be used to represent a `Plane` in 3D space.
//!
//! # Examples
//!
//! You can create a [Quad] using [`Quad::new`]:
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! let quad = Quad::new(
//!     [10, 20],
//!     [30, 10],
//!     [20, 25],
//!     [15, 15]
//! );
//!
//! let plane = Quad::new(
//!     [10, 20, 10],
//!     [30, 10, 5],
//!     [20, 25, 20],
//!     [15, 15, 10],
//! );
//! ```

use crate::prelude::*;
#[cfg(feature = "serde")]
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// A `Quad` or quadrilateral, a four-sided polygon.
///
/// `Quad` is similar to [Rect] but the angles between edges are not constrained to 90 degrees.
///
/// Please see the [module-level documentation] for examples.
///
/// [module-level documentation]: crate::shape::quad
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
#[must_use]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound = "T: Serialize + DeserializeOwned"))]
pub struct Quad<T, const N: usize>(pub(crate) [Point<T, N>; 4]);

/// A 2D `Quad` represented by `i32`.
pub type QuadI2 = Quad<i32, 2>;

/// A 3D `Quad` represented by `i32`.
pub type QuadI3 = Quad<i32, 3>;

/// A 2D `Quad` represented by `f32` or `f64` depending on platform.
pub type QuadF2 = Quad<Scalar, 2>;

/// A 3D `Quad` represented by `f32` or `f64` depending on platform.
pub type QuadF3 = Quad<Scalar, 3>;

/// Constructs a [Quad] with four points.
///
/// ```
/// # use pix_engine::prelude::*;
/// let p1: PointI2 = [10, 10].into();
/// let p2 = point!(100, 10);
/// let q = quad!(p1, p2, [90, 50], [10, 80]);
/// assert_eq!(q.as_array(), [
///   point!(10, 10),
///   point!(100, 10),
///   point!(90, 50),
///   point!(10, 80),
/// ]);
///
/// let q = quad!(10, 10, 100, 10, 90, 50, 10, 80);
/// assert_eq!(q.as_array(), [
///   point!(10, 10),
///   point!(100, 10),
///   point!(90, 50),
///   point!(10, 80),
/// ]);
/// ```
#[macro_export]
macro_rules! quad {
    ($p1:expr, $p2:expr, $p3:expr, $p4:expr$(,)?) => {
        $crate::prelude::Quad::new($p1, $p2, $p3, $p4)
    };
    ($x1:expr, $y1:expr, $x2:expr, $y2:expr, $x3:expr, $y3:expr, $x4:expr, $y4:expr$(,)?) => {
        $crate::prelude::Quad::from_xy($x1, $y1, $x2, $y2, $x3, $y3, $x4, $y4)
    };
    ($x1:expr, $y1:expr, $z1:expr, $x2:expr, $y2:expr, $z2:expr, $x3:expr, $y3:expr, $z3:expr, $x4:expr, $y4:expr, $z4:expr$(,)?) => {
        $crate::prelude::Quad::from_xy($x1, $y1, $z1, $x2, $y2, $z2, $x3, $y3, $z3, $x4, $y4, $z4)
    };
}

impl<T, const N: usize> Quad<T, N> {
    /// Constructs a `Quad` with the given [Point]s.
    ///
    /// ```
    /// use pix_engine::prelude::*;
    /// let quad = Quad::new([10, 20], [30, 10], [20, 25], [15, 15]);
    /// assert_eq!(quad.p1().as_array(), [10, 20]);
    /// assert_eq!(quad.p2().as_array(), [30, 10]);
    /// assert_eq!(quad.p3().as_array(), [20, 25]);
    /// assert_eq!(quad.p4().as_array(), [15, 15]);
    /// ```
    pub fn new<P1, P2, P3, P4>(p1: P1, p2: P2, p3: P3, p4: P4) -> Self
    where
        P1: Into<Point<T, N>>,
        P2: Into<Point<T, N>>,
        P3: Into<Point<T, N>>,
        P4: Into<Point<T, N>>,
    {
        Self([p1.into(), p2.into(), p3.into(), p4.into()])
    }
}

impl<T> Quad<T, 2> {
    /// Constructs a `Quad` from individual x/y coordinates.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub const fn from_xy(x1: T, y1: T, x2: T, y2: T, x3: T, y3: T, x4: T, y4: T) -> Self {
        Self([
            point!(x1, y1),
            point!(x2, y2),
            point!(x3, y3),
            point!(x4, y4),
        ])
    }
}

impl<T> Quad<T, 3> {
    /// Constructs a `Quad` from individual x/y/z coordinates.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub const fn from_xyz(
        x1: T,
        y1: T,
        z1: T,
        x2: T,
        y2: T,
        z2: T,
        x3: T,
        y3: T,
        z3: T,
        x4: T,
        y4: T,
        z4: T,
    ) -> Self {
        Self([
            point!(x1, y1, z1),
            point!(x2, y2, z2),
            point!(x3, y3, z3),
            point!(x4, y4, z4),
        ])
    }
}

impl<T: Copy, const N: usize> Quad<T, N> {
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
    /// let quad = Quad::new([10, 20], [30, 10], [20, 25], [15, 15]);
    /// assert_eq!(quad.as_array(), [
    ///     point!(10, 20),
    ///     point!(30, 10),
    ///     point!(20, 25),
    ///     point!(15, 15)
    /// ]);
    /// ```
    #[inline]
    pub fn as_array(&self) -> [Point<T, N>; 4] {
        self.0
    }

    /// Returns `Quad` points as a byte slice `&[Point<T, N>; 4]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let quad = Quad::new([10, 20], [30, 10], [20, 25], [15, 15]);
    /// assert_eq!(quad.as_bytes(), &[
    ///     point!(10, 20),
    ///     point!(30, 10),
    ///     point!(20, 25),
    ///     point!(15, 15)
    /// ]);
    /// ```
    #[inline]
    pub fn as_bytes(&self) -> &[Point<T, N>; 4] {
        &self.0
    }

    /// Returns `Quad` points as a mutable byte slice `&mut [Point<T, N>; 4]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut quad = Quad::new([10, 20], [30, 10], [20, 25], [15, 15]);
    /// for p in quad.as_bytes_mut() {
    ///     *p += 5;
    /// }
    /// assert_eq!(quad.as_bytes(), &[
    ///     point!(15, 25),
    ///     point!(35, 15),
    ///     point!(25, 30),
    ///     point!(20, 20)
    /// ]);
    /// ```
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [Point<T, N>; 4] {
        &mut self.0
    }

    /// Returns `Quad` points as a [Vec].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let quad = Quad::new([10, 20], [30, 10], [20, 25], [15, 15]);
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

impl Draw for QuadI2 {
    /// Draw `Quad` to the current [`PixState`] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.quad(*self)
    }
}

impl<T: Copy> From<[T; 8]> for Quad<T, 2> {
    /// Converts `[T; 8]` into `Quad<T, 2>`.
    #[inline]
    fn from([x1, y1, x2, y2, x3, y3, x4, y4]: [T; 8]) -> Self {
        Self::from_xy(x1, y1, x2, y2, x3, y3, x4, y4)
    }
}

impl<T: Copy> From<[T; 12]> for Quad<T, 3> {
    /// Converts `[T; 12]` into `Quad<T, 3>`.
    #[inline]
    fn from([x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]: [T; 12]) -> Self {
        Self::from_xyz(x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4)
    }
}

impl<T: Copy> From<[[T; 2]; 4]> for Quad<T, 2> {
    /// Converts `[[T; 2]; 4]` into `Quad<T, 2>`.
    #[inline]
    fn from([[x1, y1], [x2, y2], [x3, y3], [x4, y4]]: [[T; 2]; 4]) -> Self {
        Self::from_xy(x1, y1, x2, y2, x3, y3, x4, y4)
    }
}

impl<T: Copy> From<[[T; 3]; 4]> for Quad<T, 3> {
    /// Converts `[[T; 3]; 4]` into `Quad<T, 3>`.
    #[inline]
    fn from([[x1, y1, z1], [x2, y2, z2], [x3, y3, z3], [x4, y4, z4]]: [[T; 3]; 4]) -> Self {
        Self::from_xyz(x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4)
    }
}

#[cfg(test)]
mod tests {
    use crate::{prelude::*, shape::LineF2};

    macro_rules! assert_approx_eq {
        ($i1:expr, $i2:expr) => {
            assert_approx_eq!($i1, $i2, Scalar::EPSILON);
        };
        ($i1:expr, $i2:expr, $e:expr) => {{
            match ($i1, $i2) {
                (Some((p1, t1)), Some((p2, t2))) => {
                    let [x1, y1]: [Scalar; 2] = p1.as_array();
                    let [x2, y2]: [Scalar; 2] = p2.as_array();
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
