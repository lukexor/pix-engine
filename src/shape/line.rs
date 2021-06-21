//! [`Line`] type used for drawing.

use super::Point;
use crate::{point, vector::Vector};
use num_traits::{Float, Num};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A `Line` with a starting [`Point<T>`] and ending [`Point<T>`].
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Line<T> {
    /// Start [Point<T>].
    pub p1: Point<T>,
    /// End [Point<T>].
    pub p2: Point<T>,
}

impl<T> Line<T>
where
    T: Num,
{
    /// Constructs a `Line`.
    pub fn new<P>(p1: P, p2: P) -> Self
    where
        P: Into<Point<T>>,
    {
        Self {
            p1: p1.into(),
            p2: p2.into(),
        }
    }

    /// Returns whether this line intersects with another line.
    pub fn intersects(&self, other: impl Into<Line<T>>) -> Option<Point<T>>
    where
        T: Float + PartialOrd + Copy,
    {
        let (p1, p2) = self.into();
        let (p3, p4) = other.into().into();
        let ua = ((p4.x - p3.x) * (p1.y - p3.y) - (p4.y - p3.y) * (p1.x - p3.x))
            / ((p4.y - p3.y) * (p2.x - p1.x) - (p4.x - p3.x) * (p2.y - p1.y));
        let ub = ((p2.x - p1.x) * (p1.y - p3.y) - (p2.y - p1.y) * (p1.x - p3.x))
            / ((p4.y - p3.y) * (p2.x - p1.x) - (p4.x - p3.x) * (p2.y - p1.y));
        // If ua and ub are between 0.0 and 1.0, intersection
        if ua >= T::zero() && ub <= T::one() && ub >= T::zero() && ub <= T::one() {
            let x = p1.x + ua * (p2.x - p1.x);
            let y = p1.y + ua * (p2.y - p1.y);
            Some(point!(x, y))
        } else {
            None
        }
    }
}

/// Convert `(x1, y1, x2, y2)` to [`Line<T>`].
impl<T> From<(T, T, T, T)> for Line<T>
where
    T: Num + Copy,
{
    fn from((x1, y1, x2, y2): (T, T, T, T)) -> Self {
        Self::new((x1, y1), (x2, y2))
    }
}

/// Convert ([`Point<T>`], [`Point<T>`]) to [`Line<T>`].
impl<T> From<(Point<T>, Point<T>)> for Line<T>
where
    T: Num + Copy,
{
    fn from((p1, p2): (Point<T>, Point<T>)) -> Self {
        Self::new(p1, p2)
    }
}

/// Convert [`Line<T>`] to ([`Point<T>`], [`Point<T>`]).
impl<T> From<Line<T>> for (Point<T>, Point<T>)
where
    T: Num + Copy,
{
    fn from(line: Line<T>) -> Self {
        (line.p1, line.p2)
    }
}

/// Convert [`Line<T>`] to ([`Point<T>`], [`Point<T>`]).
impl<T> From<&Line<T>> for (Point<T>, Point<T>)
where
    T: Num + Copy,
{
    fn from(line: &Line<T>) -> Self {
        (line.p1, line.p2)
    }
}

/// Convert ([`Vector<T>`], [`Vector<T>`]) to [`Line<T>`].
impl<T> From<(Vector<T>, Vector<T>)> for Line<T>
where
    Vector<T>: Into<Point<T>>,
    T: Num + Copy,
{
    fn from((v1, v2): (Vector<T>, Vector<T>)) -> Self {
        Self::new(v1, v2)
    }
}
