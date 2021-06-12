//! 2D Triangle<T> type used for drawing.

use super::Point;
use crate::vector::Vector;
use num::Num;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A `Triangle`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Triangle<T> {
    /// Point 1
    pub p1: Point<T>,
    /// Point 2
    pub p2: Point<T>,
    /// Point 3
    pub p3: Point<T>,
}

impl<T> Triangle<T>
where
    T: Num,
{
    /// Create new `Triangle<T>`.
    pub fn new<P>(p1: P, p2: P, p3: P) -> Self
    where
        P: Into<Point<T>>,
    {
        Self {
            p1: p1.into(),
            p2: p2.into(),
            p3: p3.into(),
        }
    }
}

/// Convert `(x1, y1, x2, y2, x3, y3)` to [Triangle<T>].
impl<T> From<(T, T, T, T, T, T)> for Triangle<T>
where
    T: Num + Copy,
{
    fn from((x1, y1, x2, y2, x3, y3): (T, T, T, T, T, T)) -> Self {
        Self::new((x1, y1), (x2, y2), (x3, y3))
    }
}

/// Convert `([Point<T>], [Point<T>], [Point<T>])` to [Triangle<T>].
impl<T> From<(Point<T>, Point<T>, Point<T>)> for Triangle<T>
where
    T: Num + Copy,
{
    fn from((p1, p2, p3): (Point<T>, Point<T>, Point<T>)) -> Self {
        Self::new(p1, p2, p3)
    }
}

/// Convert `([Vector<T>], [Vector<T>], [Vector<T>]) to [Triangle<T>].
impl<T> From<(Vector<T>, Vector<T>, Vector<T>)> for Triangle<T>
where
    Vector<T>: Into<Point<T>>,
    T: Num + Copy,
{
    fn from((v1, v2, v3): (Vector<T>, Vector<T>, Vector<T>)) -> Self {
        Self::new(v1, v2, v3)
    }
}
