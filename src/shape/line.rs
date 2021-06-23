//! [`Line`] type used for drawing.

use crate::prelude::{point, Draw, PixResult, PixState, Point, Vector};
use num_traits::Num;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A `Line` with a starting [`Point<T>`] and ending [`Point<T>`].
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Line<T> {
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

    /// Constructs a `Line` from `start` and `end` coordinates.
    pub fn with_xy(start_x: T, start_y: T, end_x: T, end_y: T) -> Self
    where
        T: Num,
    {
        let start = point!(start_x, start_y);
        let end = point!(end_x, end_y);
        Self { start, end }
    }

    /// Returns whether this line intersects with another line.
    pub fn intersects(&self, other: impl Into<[f64; 4]>) -> Option<Point<f64>>
    where
        T: Num + Copy + Into<[f64; 4]>,
    {
        let [p1x, p1y, p2x, p2y]: [f64; 4] = self.into();
        let [p3x, p3y, p4x, p4y]: [f64; 4] = other.into();
        let ua = ((p4x - p3x) * (p1y - p3y) - (p4y - p3y) * (p1x - p3x))
            / ((p4y - p3y) * (p2x - p1x) - (p4x - p3x) * (p2y - p1y));
        let ub = ((p2x - p1x) * (p1y - p3y) - (p2y - p1y) * (p1x - p3x))
            / ((p4y - p3y) * (p2x - p1x) - (p4x - p3x) * (p2y - p1y));
        // If ua and ub are between 0.0 and 1.0, intersection
        if (0.0..=1.0).contains(&ua) && (0.0..=1.0).contains(&ub) {
            let x = p1x + ua * (p2x - p1x);
            let y = p1y + ua * (p2y - p1y);
            Some(point!(x, y))
        } else {
            None
        }
    }
}

impl<T> Draw for Line<T> {
    /// Draw line to the current [`PixState`] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.line(self)
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
        let (x1, y1) = line.start.into();
        let (x2, y2) = line.end.into();
        [x1, y1, x2, y2]
    }
}

/// Convert [`&Line<U>`] to `[x1, y1, x2, y2]`.
impl<T: Num, U: Into<T> + Copy> From<&Line<U>> for [T; 4] {
    fn from(line: &Line<U>) -> Self {
        let (x1, y1) = line.start.into();
        let (x2, y2) = line.end.into();
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
