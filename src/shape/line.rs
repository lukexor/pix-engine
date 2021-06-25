//! [`Line`] type used for drawing.

use crate::prelude::{point, Draw, PixResult, PixState, Point, Scalar, Vector};
use num_traits::{AsPrimitive, Num};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A `Line` with a starting [`Point<T>`] and ending [`Point<T>`].
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Line<T = Scalar> {
    /// Start of line.
    pub start: Point<T>,
    /// End of line.
    pub end: Point<T>,
}

impl<T> Line<T> {
    /// Constructs a `Line` from `start` to `end` [`Point<T>`]s.
    pub fn new<P>(start: P, end: P) -> Self
    where
        P: Into<Point<T>>,
    {
        Self {
            start: start.into(),
            end: end.into(),
        }
    }

    /// Returns whether this line intersects with another line.
    #[allow(clippy::many_single_char_names)]
    pub fn intersects(&self, other: impl Into<Line<Scalar>>) -> Option<(Point<Scalar>, Scalar)>
    where
        T: Num + Copy + PartialOrd + Into<Scalar>,
    {
        let [x1, y1, x2, y2]: [Scalar; 4] = self.into();
        let other = other.into();
        let [x3, y3, x4, y4]: [Scalar; 4] = other.into();
        let d = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if d == 0.0 {
            return None;
        }
        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / d;
        let u = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / d;
        if (0.0..).contains(&t) && (0.0..=1.0).contains(&u) {
            let x = x1 + t * (x2 - x1);
            let y = y1 + t * (y2 - y1);
            Some((point!(x, y), t))
        } else {
            None
        }
    }

    /// Converts [`Line<T>`] to [`Line<i16>`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let l: Line<f32> = Line::new([f32::MAX, 2.0], [3.0, f32::MIN]);
    /// let l = l.as_i16();
    /// assert_eq!(l.start.get(), [i16::MAX, 2, 0]);
    /// assert_eq!(l.end.get(), [3, i16::MIN, 0]);
    /// ```
    pub fn as_i16(&self) -> Line<i16>
    where
        T: AsPrimitive<i16>,
    {
        Line::new(self.start.as_i16(), self.end.as_i16())
    }
}

impl<T> Draw for Line<T>
where
    Line<T>: Copy + Into<Line<Scalar>>,
{
    /// Draw line to the current [`PixState`] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.line(*self)
    }
}

/// Convert `[x1, y1, x2, y2]` to [`Line<T>`].
impl<T: Num, U: Into<T>> From<[U; 4]> for Line<T> {
    fn from([x1, y1, x2, y2]: [U; 4]) -> Self {
        Self::new([x1, y1], [x2, y2])
    }
}

/// Convert [`Line<U>`] to `[x1, y1, x2, y2]`.
impl<T: Num, U: Into<T>> From<Line<U>> for [T; 4] {
    fn from(line: Line<U>) -> Self {
        let [x1, y1]: [T; 2] = line.start.into();
        let [x2, y2]: [T; 2] = line.end.into();
        [x1, y1, x2, y2]
    }
}

/// Convert [`&Line<U>`] to `[x1, y1, x2, y2]`.
impl<T: Num, U: Into<T> + Copy> From<&Line<U>> for [T; 4] {
    fn from(line: &Line<U>) -> Self {
        let [x1, y1]: [T; 2] = line.start.into();
        let [x2, y2]: [T; 2] = line.end.into();
        [x1, y1, x2, y2]
    }
}

/// Convert ([`Point<U>`], [`Point<U>`]) to [`Line<T>`].
impl<T, U: Into<T>> From<(Point<U>, Point<U>)> for Line<T>
where
    Point<U>: Into<Point<T>>,
{
    fn from((p1, p2): (Point<U>, Point<U>)) -> Self {
        Self::new(p1, p2)
    }
}

/// Convert [`Line<U>`] to ([`Point<T>`], [`Point<T>`]).
impl<T: Num, U: Into<T>> From<Line<U>> for (Point<T>, Point<T>)
where
    Point<U>: Into<Point<T>>,
{
    fn from(line: Line<U>) -> Self {
        (line.start.into(), line.end.into())
    }
}

/// Convert [`Line<U>`] to ([`Point<T>`], [`Point<T>`]).
impl<T: Num, U: Into<T> + Copy> From<&Line<U>> for (Point<T>, Point<T>)
where
    Point<U>: Into<Point<T>>,
{
    fn from(line: &Line<U>) -> Self {
        (line.start.into(), line.end.into())
    }
}

/// Convert ([`Vector<U>`], [`Vector<U>`]) to [`Line<T>`].
impl<T: Num, U: Into<T>> From<(Vector<U>, Vector<U>)> for Line<T>
where
    Vector<U>: Into<Point<T>>,
{
    fn from((v1, v2): (Vector<U>, Vector<U>)) -> Self {
        Self::new(v1, v2)
    }
}

/// Convert [`Line<U>`] to ([`Vector<T>`], [`Vector<T>`]).
impl<T: Num, U: Into<T> + Copy> From<&Line<U>> for (Vector<T>, Vector<T>)
where
    Point<U>: Into<Vector<T>>,
{
    fn from(line: &Line<U>) -> Self {
        (line.start.into(), line.end.into())
    }
}
