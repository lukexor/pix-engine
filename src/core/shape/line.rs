//! A shape type representing lines used for drawing.
//!
//! # Examples
//!
//! You can create a [Line] using [Line::new]:
//!
//! ```
//! use pix_engine::prelude_3d::*;
//!
//! // 2D
//! let line: LineI2 = Line::new([10, 20], [30, 10]);
//!
//! let p1 = point![10, 20];
//! let p2 = point![30, 10];
//! let line: LineI2 = Line::new(p1, p2);
//!
//! // 3D
//! let line: LineI3 = Line::new([10, 20, 5], [30, 10, 5]);
//! ```

use crate::prelude::*;
use num_traits::AsPrimitive;
// #[cfg(feature = "serde")]
// use serde::{Deserialize, Serialize};

/// A `Line` with start and end [Point]s.
///
/// Please see the [module-level documentation] for examples.
///
/// [module-level documentation]: crate::core::shape::line
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
// TODO: serde is not ready for const generics yet
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Line<T, const N: usize>(pub(crate) [Point<T, N>; 2]);

/// A 2D `Line` represented by integers.
pub type LineI2 = Line<i32, 2>;

/// A 3D `Line` represented by integers.
pub type LineI3 = Line<i32, 3>;

/// A 2D `Line` represented by floating point numbers.
pub type LineF2 = Line<Scalar, 2>;

/// A 3D `Line` represented by floating point numbers.
pub type LineF3 = Line<Scalar, 3>;

/// Constructs a [Line] with two points.
///
/// ```
/// # use pix_engine::prelude_3d::*;
///
/// let l: LineI2 = line_!([10, 20], [30, 10]);
/// assert_eq!(l.values(), [
///   point!(10, 20),
///   point!(30, 10),
/// ]);
///
/// let l: LineI3 = line_!([10, 20, 10], [30, 10, 40]);
/// assert_eq!(l.values(), [
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
        $crate::prelude::Line::new([$x1, $y1], [$x2, $y2])
    };
    ($x1:expr, $y1:expr, $z1:expr, $x2:expr, $y2:expr, $z2:expr$(,)?) => {
        $crate::prelude::Line::new([$x1, $y1, $z2], [$x2, $y2, $z2])
    };
}

impl<T, const N: usize> Line<T, N> {
    /// Constructs a `Line` from `start` to `end` [Point]s.
    ///
    /// # Example
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// // 2D
    /// let line: LineI2 = Line::new([10, 20], [30, 10]);
    ///
    /// let p1 = point![10, 20];
    /// let p2 = point![30, 10];
    /// let line: LineI2 = Line::new(p1, p2);
    ///
    /// // 3D
    /// let line: LineI3 = Line::new([10, 20, 5], [30, 10, 5]);
    /// ```
    pub fn new<P>(start: P, end: P) -> Self
    where
        P: Into<Point<T, N>>,
    {
        Self([start.into(), end.into()])
    }

    /// Constructs a `Line` with from an array `[Point<T, N>; 2]`.
    pub const fn from_array(arr: [Point<T, N>; 2]) -> Self {
        Self(arr)
    }
}

impl<T, const N: usize> Line<T, N>
where
    T: Copy + Default,
{
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

    /// Returns `Line` coordinates as `[x1, y1, z1, x2, y2, z2]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p1 = point!(5, 10);
    /// let p2 = point!(100, 100);
    /// let l: LineI2 = Line::new(p1, p2);
    /// assert_eq!(l.values(), [point!(5, 10), point!(100, 100)]);
    /// ```
    pub fn values(&self) -> [Point<T, N>; 2] {
        self.0
    }

    /// Returns `Line` as a [Vec].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p1 = point!(5, 10);
    /// let p2 = point!(100, 100);
    /// let l: LineI2 = Line::new(p1, p2);
    /// assert_eq!(l.to_vec(), vec![[5, 10], [100, 100]]);
    /// ```
    pub fn to_vec(self) -> Vec<Vec<T>> {
        let start = self.start().to_vec();
        let end = self.end().to_vec();
        vec![start, end]
    }
}

impl<T: Float> Intersects<T, 2> for Line<T, 2> {
    type Shape = Line<T, 2>;

    /// Returns the closest intersection point with a given line and distance along the line or
    /// `None` if there is no intersection.
    #[allow(clippy::many_single_char_names)]
    fn intersects_line<L>(&self, other: L) -> Option<(Point<T, 2>, T)>
    where
        L: Into<Line<T, 2>>,
    {
        let other = other.into();
        let [start1, end1] = self.values();
        let [start2, end2] = other.values();
        let [x1, y1] = start1.values();
        let [x2, y2] = end1.values();
        let [x3, y3] = start2.values();
        let [x4, y4] = end2.values();
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

    /// Returns whether this line intersections with another line
    fn intersects_shape<O>(&self, other: O) -> bool
    where
        O: Into<Self::Shape>,
    {
        self.intersects_line(other).is_some()
    }
}

impl<T, const N: usize> Draw for Line<T, N>
where
    Self: Into<LineI2>,
    T: Default + AsPrimitive<i32>,
{
    /// Draw `Line` to the current [PixState] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.line(*self)
    }
}
