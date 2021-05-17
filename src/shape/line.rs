//! 2D Line type used for drawing.

use super::Point;
use crate::vector::Vector;

/// A `Line`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Line {
    /// Start Point.
    pub p1: Point,
    /// End Point.
    pub p2: Point,
}

impl Line {
    /// Creates a new `Line`.
    pub fn new<P>(p1: P, p2: P) -> Self
    where
        P: Into<Point>,
    {
        Self {
            p1: p1.into(),
            p2: p2.into(),
        }
    }
}

/// From tuple of (x1, y1, x2, y2) to `Line`.
impl From<(i32, i32, i32, i32)> for Line {
    fn from((x1, y1, x2, y2): (i32, i32, i32, i32)) -> Self {
        Self::new((x1, y1), (x2, y2))
    }
}

/// From tuple of (x1, y1, x2, y2) to `Line`.
impl From<(u32, u32, u32, u32)> for Line {
    fn from((x1, y1, x2, y2): (u32, u32, u32, u32)) -> Self {
        Self::new((x1 as i32, y1 as i32), (x2 as i32, y2 as i32))
    }
}

/// From tuple of (`Point`, `Point`) to `Line`.
impl From<(Point, Point)> for Line {
    fn from((p1, p2): (Point, Point)) -> Self {
        Self::new(p1, p2)
    }
}

/// From tuple of (`Vector`, `Vector`) to `Line`.
impl From<(Vector, Vector)> for Line {
    fn from((v1, v2): (Vector, Vector)) -> Self {
        Self::new(v1, v2)
    }
}
