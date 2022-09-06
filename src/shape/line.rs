//! A shape type representing lines used for drawing.
//!
//! # Examples
//!
//! You can create a [Line] using [`Line::new`]:
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! // 2D
//! let line = Line::new([10, 20], [30, 10]);
//!
//! let p1 = point![10, 20];
//! let p2 = point![30, 10];
//! let line = Line::new(p1, p2);
//!
//! // 3D
//! let line = Line::new([10, 20, 5], [30, 10, 5]);
//! ```

use crate::prelude::*;
#[cfg(feature = "serde")]
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// A `Line` with start and end [Point]s.
///
/// Please see the [module-level documentation] for examples.
///
/// [module-level documentation]: crate::shape::line
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
#[must_use]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound = "T: Serialize + DeserializeOwned"))]
pub struct Line<T = i32, const N: usize = 2>(pub(crate) [Point<T, N>; 2]);

/// Constructs a [Line] with two points.
///
/// ```
/// # use pix_engine::prelude::*;
///
/// let l = line_!([10, 20], [30, 10]);
/// assert_eq!(l.points(), [
///   point!(10, 20),
///   point!(30, 10),
/// ]);
///
/// let l = line_!([10, 20, 10], [30, 10, 40]);
/// assert_eq!(l.points(), [
///   point!(10, 20, 10),
///   point!(30, 10, 40),
/// ]);
/// ```
#[macro_export]
macro_rules! line_ {
    ($p1:expr, $p2:expr$(,)?) => {
        $crate::prelude::Line::new($p1, $p2)
    };
    ($x1:expr, $y1:expr, $x2:expr, $y2:expr$(,)?) => {
        $crate::prelude::Line::from_xy($x1, $y1, $x2, $y2)
    };
    ($x1:expr, $y1:expr, $z1:expr, $x2:expr, $y2:expr, $z2:expr$(,)?) => {
        $crate::prelude::Line::from_xyz($x1, $y1, $z2, $x2, $y2, $z2)
    };
}

impl<T, const N: usize> Line<T, N> {
    /// Constructs a `Line` from `start` to `end` [Point]s.
    ///
    /// # Example
    /// ```
    /// # use pix_engine::prelude::*;
    /// // 2D
    /// let line = Line::new([10, 20], [30, 10]);
    ///
    /// let p1 = point![10, 20];
    /// let p2 = point![30, 10];
    /// let line = Line::new(p1, p2);
    ///
    /// // 3D
    /// let line: Line<i32, 3> = Line::new([10, 20, 5], [30, 10, 5]);
    /// ```
    pub fn new<P1, P2>(start: P1, end: P2) -> Self
    where
        P1: Into<Point<T, N>>,
        P2: Into<Point<T, N>>,
    {
        Self([start.into(), end.into()])
    }
}

impl<T> Line<T> {
    /// Constructs a `Line` from individual x/y coordinates.
    #[inline]
    pub const fn from_xy(x1: T, y1: T, x2: T, y2: T) -> Self {
        Self([point!(x1, y1), point!(x2, y2)])
    }
}

impl<T: Copy> Line<T> {
    /// Returns `Line` coordinates as `[x1, y1, x2, y2]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p1 = point!(5, 10);
    /// let p2 = point!(100, 100);
    /// let l = Line::new(p1, p2);
    /// assert_eq!(l.coords(), [5, 10, 100, 100]);
    /// ```
    #[inline]
    pub fn coords(&self) -> [T; 4] {
        let [p1, p2] = self.points();
        let [x1, y1] = p1.coords();
        let [x2, y2] = p2.coords();
        [x1, y1, x2, y2]
    }
}

impl<T> Line<T, 3> {
    /// Constructs a `Line` from individual x/y/z coordinates.
    #[inline]
    pub const fn from_xyz(x1: T, y1: T, z1: T, x2: T, y2: T, z2: T) -> Self {
        Self([point!(x1, y1, z1), point!(x2, y2, z2)])
    }
}

impl<T: Copy> Line<T, 3> {
    /// Returns `Line` coordinates as `[x1, y1, z1, x2, y2, z2]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p1 = point!(5, 10);
    /// let p2 = point!(100, 100);
    /// let l = Line::new(p1, p2);
    /// assert_eq!(l.coords(), [5, 10, 100, 100]);
    /// ```
    #[inline]
    pub fn coords(&self) -> [T; 6] {
        let [p1, p2] = self.points();
        let [x1, y1, z1] = p1.coords();
        let [x2, y2, z2] = p2.coords();
        [x1, y1, z1, x2, y2, z2]
    }
}

impl<T: Copy, const N: usize> Line<T, N> {
    /// Returns the starting point of the line.
    #[inline]
    pub fn start(&self) -> Point<T, N> {
        self.0[0]
    }

    /// Sets the starting point of the line.
    #[inline]
    pub fn set_start<P: Into<Point<T, N>>>(&mut self, start: P) {
        self.0[0] = start.into();
    }

    /// Returns the ending point of the line.
    #[inline]
    pub fn end(&self) -> Point<T, N> {
        self.0[1]
    }

    /// Sets the ending point of the line.
    #[inline]
    pub fn set_end<P: Into<Point<T, N>>>(&mut self, end: P) {
        self.0[1] = end.into();
    }

    /// Returns `Line` points as `[Point<T, N>; 3]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p1 = point!(5, 10);
    /// let p2 = point!(100, 100);
    /// let l = Line::new(p1, p2);
    /// assert_eq!(l.points(), [point!(5, 10), point!(100, 100)]);
    /// ```
    #[inline]
    pub fn points(&self) -> [Point<T, N>; 2] {
        self.0
    }

    /// Returns `Line` points as a mutable slice `&mut [Point<T, N>; 3]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p1 = point!(5, 10);
    /// let p2 = point!(100, 100);
    /// let mut l = Line::new(p1, p2);
    /// for p in l.points_mut() {
    ///     *p += 5;
    /// }
    /// assert_eq!(l.points(), [point!(10, 15), point!(105, 105)]);
    /// ```
    #[inline]
    pub fn points_mut(&mut self) -> &mut [Point<T, N>; 2] {
        &mut self.0
    }

    /// Returns `Line` as a [Vec].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p1 = point!(5, 10);
    /// let p2 = point!(100, 100);
    /// let l = Line::new(p1, p2);
    /// assert_eq!(l.to_vec(), vec![[5, 10], [100, 100]]);
    /// ```
    pub fn to_vec(self) -> Vec<Vec<T>> {
        let start = self.start().to_vec();
        let end = self.end().to_vec();
        vec![start, end]
    }
}

impl<T: Float> Intersects<Line<T>> for Line<T> {
    type Result = (Point<T>, T);

    /// Returns the closest intersection point with a given line and distance along the line or
    /// `None` if there is no intersection.
    #[allow(clippy::many_single_char_names)]
    fn intersects(&self, line: Line<T>) -> Option<Self::Result> {
        let [x1, y1, x2, y2] = self.coords();
        let [x3, y3, x4, y4] = line.coords();
        let d = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if d == T::zero() {
            return None;
        }
        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / d;
        let u = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / d;
        if (T::zero()..).contains(&t) && (T::zero()..=T::one()).contains(&u) {
            let x = x1 + t * (x2 - x1);
            let y = y1 + t * (y2 - y1);
            Some((point!(x, y), t))
        } else {
            None
        }
    }
}

impl Draw for Line<i32> {
    /// Draw `Line` to the current [`PixState`] canvas.
    fn draw(&self, s: &mut PixState) -> Result<()> {
        s.line(*self)
    }
}

impl<T: Copy> From<[T; 4]> for Line<T> {
    /// Converts `[T; 4]` into `Line<T>`.
    #[inline]
    fn from([x1, y1, x2, y2]: [T; 4]) -> Self {
        Self::from_xy(x1, y1, x2, y2)
    }
}

impl<T: Copy> From<[T; 6]> for Line<T, 3> {
    /// Converts `[T; 6]` into `Line<T, 3>`.
    #[inline]
    fn from([x1, y1, z1, x2, y2, z2]: [T; 6]) -> Self {
        Self::from_xyz(x1, y1, z1, x2, y2, z2)
    }
}

impl<T: Copy> From<[[T; 2]; 2]> for Line<T> {
    /// Converts `[[T; 2]; 2]` into `Line<T>`.
    #[inline]
    fn from([[x1, y1], [x2, y2]]: [[T; 2]; 2]) -> Self {
        Self::from_xy(x1, y1, x2, y2)
    }
}

impl<T: Copy> From<[[T; 3]; 2]> for Line<T, 3> {
    /// Converts `[[T; 3]; 2]` into `Line<T, 3>`.
    #[inline]
    fn from([[x1, y1, z1], [x2, y2, z2]]: [[T; 3]; 2]) -> Self {
        Self::from_xyz(x1, y1, z1, x2, y2, z2)
    }
}
