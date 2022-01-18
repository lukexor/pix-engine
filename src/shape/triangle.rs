//! A shape type representing triangles used for drawing.
//!
//! # Examples
//!
//! You can create a [Triangle][Tri] using [`Tri::new`]:
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! // 2D
//! let tri = Tri::new([10, 20], [30, 10], [20, 25]);
//!
//! let p1 = point!(10, 20);
//! let p2 = point!(30, 10);
//! let p3 = point!(20, 25);
//! let tri = Tri::new(p1, p2, p3);
//! ```

use crate::prelude::*;
#[cfg(feature = "serde")]
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// A `Triangle` with three [Point]s.
///
/// Please see the [module-level documentation] for examples.
///
/// [module-level documentation]: crate::shape::triangle
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
#[must_use]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound = "T: Serialize + DeserializeOwned"))]
pub struct Tri<T, const N: usize>(pub(crate) [Point<T, N>; 3]);

/// A 2D `Triangle` represented by `i32`.
pub type TriI2 = Tri<i32, 2>;

/// A 3D `Tri` represented by `i32`.
pub type TriI3 = Tri<i32, 3>;

/// A 2D `Tri` represented by `f32` or `f64` depending on platform.
pub type TriF2 = Tri<Scalar, 2>;

/// A 3D `Tri` represented by `f32` or `f64` depending on platform.
pub type TriF3 = Tri<Scalar, 3>;

/// Constructs a [Triangle][Tri] with three points.
///
/// ```
/// # use pix_engine::prelude::*;
///
/// let t = tri!([10, 20], [30, 10], [20, 25]);
/// assert_eq!(t.as_array(), [
///   point!(10, 20),
///   point!(30, 10),
///   point!(20, 25),
/// ]);
///
/// let t = tri!([10, 20, 10], [30, 10, 40], [20, 25, 20]);
/// assert_eq!(t.as_array(), [
///   point!(10, 20, 10),
///   point!(30, 10, 40),
///   point!(20, 25, 20),
/// ]);
/// ```
#[macro_export]
macro_rules! tri {
    ($p1:expr, $p2:expr, $p3:expr$(,)?) => {
        $crate::prelude::Tri::new($p1, $p2, $p3)
    };
    ($x1:expr, $y1:expr, $x2:expr, $y2:expr, $x3:expr, $y3:expr$(,)?) => {
        $crate::prelude::Tri::from_xy($x1, $y1, $x2, $y2, $x3, $y3)
    };
    ($x1:expr, $y1:expr, $z1:expr, $x2:expr, $y2:expr, $z2:expr, $x3:expr, $y3:expr, $z3:expr$(,)?) => {
        $crate::prelude::Tri::from_xyz($x1, $y1, $z2, $x2, $y2, $z2, $x3, $y3, $z3)
    };
}

impl<T, const N: usize> Tri<T, N> {
    /// Constructs a `Triangle` with the given [Point]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let tri = Tri::new([10, 20], [30, 10], [20, 25]);
    /// assert_eq!(tri.p1().as_array(), [10, 20]);
    /// assert_eq!(tri.p2().as_array(), [30, 10]);
    /// assert_eq!(tri.p3().as_array(), [20, 25]);
    /// ```
    pub fn new<P1, P2, P3>(p1: P1, p2: P2, p3: P3) -> Self
    where
        P1: Into<Point<T, N>>,
        P2: Into<Point<T, N>>,
        P3: Into<Point<T, N>>,
    {
        Self([p1.into(), p2.into(), p3.into()])
    }
}

impl<T> Tri<T, 2> {
    /// Constructs a `Triangle` from individual x/y coordinates.
    #[inline]
    pub const fn from_xy(x1: T, y1: T, x2: T, y2: T, x3: T, y3: T) -> Self {
        Self([point!(x1, y1), point!(x2, y2), point!(x3, y3)])
    }
}

impl<T> Tri<T, 3> {
    /// Constructs a `Triangle` from individual x/y/z coordinates.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub const fn from_xyz(x1: T, y1: T, z1: T, x2: T, y2: T, z2: T, x3: T, y3: T, z3: T) -> Self {
        Self([point!(x1, y1, z1), point!(x2, y2, z2), point!(x3, y3, z3)])
    }
}

impl<T: Copy, const N: usize> Tri<T, N> {
    /// Returns the first point of the triangle.
    #[inline]
    pub fn p1(&self) -> Point<T, N> {
        self.0[0]
    }

    /// Sets the first point of the triangle.
    #[inline]
    pub fn set_p1<P>(&mut self, p: P)
    where
        P: Into<Point<T, N>>,
    {
        self.0[0] = p.into();
    }

    /// Returns the second point of the triangle.
    #[inline]
    pub fn p2(&self) -> Point<T, N> {
        self.0[1]
    }

    /// Sets the second point of the triangle.
    #[inline]
    pub fn set_p2<P>(&mut self, p: P)
    where
        P: Into<Point<T, N>>,
    {
        self.0[1] = p.into();
    }

    /// Returns the third point of the triangle.
    #[inline]
    pub fn p3(&self) -> Point<T, N> {
        self.0[2]
    }

    /// Sets the third point of the triangle.
    #[inline]
    pub fn set_p3<P>(&mut self, p: P)
    where
        P: Into<Point<T, N>>,
    {
        self.0[2] = p.into();
    }

    /// Returns `Triangle` points as `[Point<T, N>; 3]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let tri = Tri::new([10, 20], [30, 10], [20, 25]);
    /// assert_eq!(tri.as_array(), [
    ///     point!(10, 20),
    ///     point!(30, 10),
    ///     point!(20, 25),
    /// ]);
    /// ```
    #[inline]
    pub fn as_array(&self) -> [Point<T, N>; 3] {
        self.0
    }

    /// Returns `Triangle` points as a byte slice `&[Point<T, N>; 3]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let tri = Tri::new([10, 20], [30, 10], [20, 25]);
    /// assert_eq!(tri.as_bytes(), &[
    ///     point!(10, 20),
    ///     point!(30, 10),
    ///     point!(20, 25),
    /// ]);
    /// ```
    #[inline]
    pub fn as_bytes(&self) -> &[Point<T, N>; 3] {
        &self.0
    }

    /// Returns `Triangle` points as a mutable byte slice `&mut [Point<T, N>; 3]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut tri = Tri::new([10, 20], [30, 10], [20, 25]);
    /// for p in tri.as_bytes_mut() {
    ///     *p += 5;
    /// }
    /// assert_eq!(tri.as_bytes(), &[
    ///     point!(15, 25),
    ///     point!(35, 15),
    ///     point!(25, 30),
    /// ]);
    /// ```
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [Point<T, N>; 3] {
        &mut self.0
    }

    /// Returns `Triangle` as a [Vec].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let tri = Tri::new([10, 20], [30, 10], [20, 25]);
    /// assert_eq!(
    ///   tri.to_vec(),
    ///   vec![
    ///     point!(10, 20),
    ///     point!(30, 10),
    ///     point!(20, 25),
    ///   ]
    /// );
    /// ```
    pub fn to_vec(self) -> Vec<Point<T, N>> {
        self.0.to_vec()
    }
}

impl Draw for TriI2 {
    /// Draw `Triangle` to the current [`PixState`] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.triangle(*self)
    }
}

impl<T: Copy> From<[T; 6]> for Tri<T, 2> {
    /// Converts `[T; 6]` into `Tri<T, 2>`.
    #[inline]
    fn from([x1, y1, x2, y2, x3, y3]: [T; 6]) -> Self {
        Self::from_xy(x1, y1, x2, y2, x3, y3)
    }
}

impl<T: Copy> From<[T; 9]> for Tri<T, 3> {
    /// Converts `[T; 9]` into `Tri<T, 3>`.
    #[inline]
    fn from([x1, y1, z1, x2, y2, z2, x3, y3, z3]: [T; 9]) -> Self {
        Self::from_xyz(x1, y1, z1, x2, y2, z2, x3, y3, z3)
    }
}

impl<T: Copy> From<[[T; 2]; 3]> for Tri<T, 2> {
    /// Converts `[[T; 2]; 3]` into `Tri<T, 2>`.
    #[inline]
    fn from([[x1, y1], [x2, y2], [x3, y3]]: [[T; 2]; 3]) -> Self {
        Self::from_xy(x1, y1, x2, y2, x3, y3)
    }
}

impl<T: Copy> From<[[T; 3]; 3]> for Tri<T, 3> {
    /// Converts `[[T; 3]; 3]` into `Tri<T, 3>`.
    #[inline]
    fn from([[x1, y1, z1], [x2, y2, z2], [x3, y3, z3]]: [[T; 3]; 3]) -> Self {
        Self::from_xyz(x1, y1, z1, x2, y2, z2, x3, y3, z3)
    }
}
