//! 2D Triangle type used for drawing.

use super::Point;
use crate::vector::Vector;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A Triangle.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Triangle {
    /// Point 1
    pub p1: Point,
    /// Point 2
    pub p2: Point,
    /// Point 3
    pub p3: Point,
}

impl Triangle {
    /// Creates a new [Triangle].
    pub fn new<P: Into<Point>>(p1: P, p2: P, p3: P) -> Self {
        Self {
            p1: p1.into(),
            p2: p2.into(),
            p3: p3.into(),
        }
    }
}

/// From tuple of (x1, y1, x2, y2, x3, y3) to [Triangle].
impl From<(i32, i32, i32, i32, i32, i32)> for Triangle {
    fn from((x1, y1, x2, y2, x3, y3): (i32, i32, i32, i32, i32, i32)) -> Self {
        Self::new((x1, y1), (x2, y2), (x3, y3))
    }
}

/// From tuple of (Point, Point, Point) to [Triangle].
impl From<(Point, Point, Point)> for Triangle {
    fn from((p1, p2, p3): (Point, Point, Point)) -> Self {
        Self::new(p1, p2, p3)
    }
}

/// From tuple of (Vector, Vector, Vector) to [Triangle].
impl From<(Vector, Vector, Vector)> for Triangle {
    fn from((v1, v2, v3): (Vector, Vector, Vector)) -> Self {
        Self::new(v1, v2, v3)
    }
}
