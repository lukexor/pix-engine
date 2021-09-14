//! A 2D/3D shape type representing a line used for drawing.
//!
//! # Examples
//!
//! You can create a [Line] using [Line::new]:
//!
//! ```
//! # use pix_engine::prelude::*;
//! // 2D
//! let line: Line<i32> = Line::new([10, 20], [30, 10]);
//!
//! let p1 = point![10, 20];
//! let p2 = point![30, 10];
//! let line: Line<i32> = Line::new(p1, p2);
//!
//! // 3D
//! let line: Line<i32> = Line::new([10, 20, 5], [30, 10, 5]);
//! ```

use crate::prelude::*;
use num_traits::AsPrimitive;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// A `Line` with a starting [Point] and ending [Point<T>].
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Line<T = f64>([Point<T>; 2]);

impl<T> Line<T> {
    /// Constructs a `Line` from `start` to `end` [Point]s.
    pub fn new<P>(start: P, end: P) -> Self
    where
        P: Into<Point<T>>,
    {
        Self([start.into(), end.into()])
    }
}

impl<T: Number> Line<T> {
    /// Returns the starting point of the line.
    #[inline(always)]
    pub fn start(&self) -> Point<T> {
        self.0[0]
    }

    /// Sets the starting point of the line.
    #[inline(always)]
    pub fn set_start<P: Into<Point<T>>>(&mut self, start: P) {
        self.0[0] = start.into();
    }

    /// Returns the ending point of the line.
    #[inline(always)]
    pub fn end(&self) -> Point<T> {
        self.0[1]
    }

    /// Sets the ending point of the line.
    #[inline(always)]
    pub fn set_end<P: Into<Point<T>>>(&mut self, end: P) {
        self.0[1] = end.into();
    }

    /// Convert `Line` to another primitive type using the `as` operator.
    #[inline]
    pub fn as_<U>(self) -> Line<U>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy,
    {
        Line::new(self.start().as_(), self.end().as_())
    }

    /// Returns `Line` coordinates as `[x1, y1, z1, x2, y2, z2]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p1 = point!(5, 10);
    /// let p2 = point!(100, 100);
    /// let l: Line<i32> = Line::new(p1, p2);
    /// assert_eq!(l.values(), [5, 10, 0, 100, 100, 0]);
    /// ```
    pub fn values(&self) -> [T; 6] {
        let [x1, y1, z1] = self.start().values();
        let [x2, y2, z2] = self.end().values();
        [x1, y1, z1, x2, y2, z2]
    }

    /// Returns `Line` as a [Vec].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p1 = point!(5, 10);
    /// let p2 = point!(100, 100);
    /// let l: Line<i32> = Line::new(p1, p2);
    /// assert_eq!(l.to_vec(), vec![5, 10, 0, 100, 100, 0]);
    /// ```
    pub fn to_vec(self) -> Vec<T> {
        let [x1, y1, z1] = self.start().values();
        let [x2, y2, z2] = self.end().values();
        vec![x1, y1, z1, x2, y2, z2]
    }
}

impl<T: Number + AsPrimitive<f64>> Intersects for Line<T> {
    type Type = T;
    type Shape = Line<Self::Type>;

    /// Returns the closest intersection point with a given line and distance along the line or
    /// `None` if there is no intersection.
    #[allow(clippy::many_single_char_names)]
    fn intersects_line<L>(&self, other: L) -> Option<(Point<f64>, f64)>
    where
        L: Into<Line<T>>,
    {
        let other = other.into();
        let [x1, y1, _, x2, y2, _] = self.values();
        let [x3, y3, _, x4, y4, _] = other.values();
        let d = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if d == T::zero() {
            return None;
        }
        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / d;
        let u = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / d;
        if (T::zero()..).contains(&t) && (T::zero()..=T::one()).contains(&u) {
            let x = x1 + t * (x2 - x1);
            let y = y1 + t * (y2 - y1);
            Some((point!(x, y).as_(), t.as_()))
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

impl<T> Draw for Line<T>
where
    Self: Into<Line<i32>>,
    T: Number,
{
    /// Draw `Line` to the current [PixState] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.line(*self)
    }
}

impl<T> Deref for Line<T> {
    type Target = [Point<T>; 2];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Line<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

macro_rules! impl_from_as {
    ($($from:ty),* => $to:ty) => {
        $(
            impl From<Line<$from>> for Line<$to> {
                fn from(line: Line<$from>) -> Self {
                    let p1: Point<$to> = line.start().into();
                    let p2: Point<$to> = line.end().into();
                    Self::new(p1, p2)
                }
            }

            /// Convert `[x1, y1, x2, y2]` to [Line].
            impl From<[$from; 4]> for Line<$to> {
                fn from([x1, y1, x2, y2]: [$from; 4]) -> Self {
                    Self::new([x1 , y1], [x2, y2])
                }
            }

            /// Convert `&[x1, y1, x2, y2]` to [Line].
            impl From<&[$from; 4]> for Line<$to> {
                fn from(&[x1, y1, x2, y2]: &[$from; 4]) -> Self {
                    Self::new([x1, y1], [x2, y2])
                }
            }

            /// Convert `[x1, y1, z1, x2, y2, z2]` to [Line].
            impl From<[$from; 6]> for Line<$to> {
                fn from([x1, y1, z1, x2, y2, z2]: [$from; 6]) -> Self {
                    Self::new([x1, y1, z1], [x2, y2, z2])
                }
            }

            /// Convert `&[x1, y1, z1, x2, y2, z2]` to [Line].
            impl From<&[$from; 6]> for Line<$to> {
                fn from(&[x1, y1, z1, x2, y2, z2]: &[$from; 6]) -> Self {
                    Self::new([x1, y1, z1], [x2, y2, z2])
                }
            }

        )*
    };
}

impl_from_as!(i8, u8, i16, u16, u32, i64, u64, isize, usize, f32, f64 => i32);
impl_from_as!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32 => f64);

impl<T: Number> From<&mut Line<T>> for Line<T> {
    fn from(line: &mut Line<T>) -> Self {
        *line
    }
}

impl<T: Number> From<&Line<T>> for Line<T> {
    fn from(line: &Line<T>) -> Self {
        *line
    }
}

/// Convert [Line] to `[x1, y1, x2, y2]`.
impl<T: Number> From<Line<T>> for [T; 4] {
    fn from(line: Line<T>) -> Self {
        let [x1, y1, _] = line.start().values();
        let [x2, y2, _] = line.end().values();
        [x1, y1, x2, y2]
    }
}

/// Convert &[Line] to `[x1, y1, x2, y2]`.
impl<T: Number> From<&Line<T>> for [T; 4] {
    fn from(line: &Line<T>) -> Self {
        let [x1, y1, _] = line.start().values();
        let [x2, y2, _] = line.end().values();
        [x1, y1, x2, y2]
    }
}

/// Convert [Line] to `[x1, y1, z1, x2, y2, z2]`.
impl<T: Number> From<Line<T>> for [T; 6] {
    fn from(line: Line<T>) -> Self {
        line.values()
    }
}

/// Convert &[Line] to `[x1, y1, z1, x2, y2, z2]`.
impl<T: Number> From<&Line<T>> for [T; 6] {
    fn from(line: &Line<T>) -> Self {
        line.values()
    }
}

/// Convert `[Point<U>; 2]` to [Line].
impl<T, U> From<[Point<U>; 2]> for Line<T>
where
    T: Number,
    U: Number,
    Point<U>: Into<Point<T>>,
{
    fn from([p1, p2]: [Point<U>; 2]) -> Self {
        Self::new(p1, p2)
    }
}

/// Convert `&[Point<U>; 2]` to [Line].
impl<T, U> From<&[Point<U>; 2]> for Line<T>
where
    T: Number,
    U: Number,
    Point<U>: Into<Point<T>>,
{
    fn from(&[p1, p2]: &[Point<U>; 2]) -> Self {
        Self::new(p1, p2)
    }
}

/// Convert `[Vector<U>; 2]` to [Line].
impl<T, U> From<[Vector<U>; 2]> for Line<T>
where
    T: Number,
    U: Number,
    Vector<U>: Into<Point<T>>,
{
    fn from([v1, v2]: [Vector<U>; 2]) -> Self {
        Self::new(v1, v2)
    }
}

/// Convert `&[Vector<U>; 2]` to [Line].
impl<T, U> From<&[Vector<U>; 2]> for Line<T>
where
    T: Number,
    U: Number,
    Vector<U>: Into<Point<T>>,
{
    fn from(&[v1, v2]: &[Vector<U>; 2]) -> Self {
        Self::new(v1, v2)
    }
}

/// Convert [Line] to `[Vector<T>; 2]`.
impl<T, U> From<Line<U>> for [Vector<T>; 2]
where
    T: Number,
    U: Number,
    Point<U>: Into<Vector<T>>,
{
    fn from(line: Line<U>) -> Self {
        [line.start().into(), line.end().into()]
    }
}

/// Convert &[Line] to `[Vector<T>; 2]`.
impl<T, U> From<&Line<U>> for [Vector<T>; 2]
where
    T: Number,
    U: Number,
    Point<U>: Into<Vector<T>>,
{
    fn from(line: &Line<U>) -> Self {
        [line.start().into(), line.end().into()]
    }
}
