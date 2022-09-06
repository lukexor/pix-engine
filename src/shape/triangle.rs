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
pub struct Tri<T = i32, const N: usize = 2>(pub(crate) [Point<T, N>; 3]);

/// Constructs a [Triangle][Tri] with three points.
///
/// ```
/// # use pix_engine::prelude::*;
///
/// let t = tri!([10, 20], [30, 10], [20, 25]);
/// assert_eq!(t.points(), [
///   point!(10, 20),
///   point!(30, 10),
///   point!(20, 25),
/// ]);
///
/// let t = tri!([10, 20, 10], [30, 10, 40], [20, 25, 20]);
/// assert_eq!(t.points(), [
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
    /// assert_eq!(tri.p1().coords(), [10, 20]);
    /// assert_eq!(tri.p2().coords(), [30, 10]);
    /// assert_eq!(tri.p3().coords(), [20, 25]);
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

impl<T> Tri<T> {
    /// Constructs a `Triangle` from individual x/y coordinates.
    #[inline]
    pub const fn from_xy(x1: T, y1: T, x2: T, y2: T, x3: T, y3: T) -> Self {
        Self([point!(x1, y1), point!(x2, y2), point!(x3, y3)])
    }
}

impl<T: Copy> Tri<T> {
    /// Returns `Triangle` coordinates as `[x1, y1, x2, y2, x3, y3]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let tri = Tri::new([10, 20], [30, 10], [20, 25]);
    /// assert_eq!(tri.coords(), [10, 20, 30, 10, 20, 25]);
    /// ```
    #[inline]
    pub fn coords(&self) -> [T; 6] {
        let [p1, p2, p3] = self.points();
        let [x1, y1] = p1.coords();
        let [x2, y2] = p2.coords();
        let [x3, y3] = p3.coords();
        [x1, y1, x2, y2, x3, y3]
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

impl<T: Copy> Tri<T, 3> {
    /// Returns `Triangle` coordinates as `[x1, y1, z1, x2, y2, z2, x3, y3, z3]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let tri = Tri::new([10, 20, 5], [30, 10, 5], [20, 25, 5]);
    /// assert_eq!(tri.coords(), [10, 20, 5, 30, 10, 5, 20, 25, 5]);
    /// ```
    #[inline]
    pub fn coords(&self) -> [T; 9] {
        let [p1, p2, p3] = self.points();
        let [x1, y1, z1] = p1.coords();
        let [x2, y2, z2] = p2.coords();
        let [x3, y3, z3] = p3.coords();
        [x1, y1, z1, x2, y2, z2, x3, y3, z3]
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
    /// assert_eq!(tri.points(), [
    ///     point!(10, 20),
    ///     point!(30, 10),
    ///     point!(20, 25),
    /// ]);
    /// ```
    #[inline]
    pub fn points(&self) -> [Point<T, N>; 3] {
        self.0
    }

    /// Returns `Triangle` points as a mutable slice `&mut [Point<T, N>; 3]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut tri = Tri::new([10, 20], [30, 10], [20, 25]);
    /// for p in tri.points_mut() {
    ///     *p += 5;
    /// }
    /// assert_eq!(tri.points(), [
    ///     point!(15, 25),
    ///     point!(35, 15),
    ///     point!(25, 30),
    /// ]);
    /// ```
    #[inline]
    pub fn points_mut(&mut self) -> &mut [Point<T, N>; 3] {
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

impl<T: Num> Contains<Point<T>> for Tri<T> {
    /// Returns whether this rectangle contains a given [Point].
    fn contains(&self, p: Point<T>) -> bool {
        let [p1, p2, p3] = self.points();
        let b1 = ((p.x() - p2.x()) * (p1.y() - p2.y()) - (p.y() - p2.y()) * (p1.x() - p2.x()))
            < T::zero();
        let b2 = ((p.x() - p3.x()) * (p2.y() - p3.y()) - (p.y() - p3.y()) * (p2.x() - p3.x()))
            < T::zero();
        let b3 = ((p.x() - p1.x()) * (p3.y() - p1.y()) - (p.y() - p1.y()) * (p3.x() - p1.x()))
            < T::zero();
        (b1 == b2) && (b2 == b3)
    }
}

impl Draw for Tri<i32> {
    /// Draw `Triangle` to the current [`PixState`] canvas.
    fn draw(&self, s: &mut PixState) -> Result<()> {
        s.triangle(*self)
    }
}

impl<T: Copy> From<[T; 6]> for Tri<T> {
    /// Converts `[T; 6]` into `Tri<T>`.
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

impl<T: Copy> From<[[T; 2]; 3]> for Tri<T> {
    /// Converts `[[T; 2]; 3]` into `Tri<T>`.
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
