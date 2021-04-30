//! Shape types and functions for Circle, Square, Rectangle, etc.

use crate::{math::Scalar, vector::Vector};

/// A Circle shape
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Circle {
    /// Center Position (x, y) of circle
    pub pos: Vector,
    /// Radius of circle
    pub radius: Scalar,
}

impl Circle {
    /// Creates a new Circle shape
    pub fn new<V: Into<Vector>>(pos: V, radius: Scalar) -> Self {
        Self {
            pos: pos.into(),
            radius,
        }
    }

    /// Detects whether a 2D point (x, y) lies inside this circle.
    pub fn contains<V: Into<Vector>>(&self, point: V) -> bool {
        let point = point.into();
        ((point.x - self.pos.x).powf(2.0) + (point.y - self.pos.y).powf(2.0)).abs()
            < self.radius.powf(2.0)
    }

    /// Detects whether another circle overlaps this one.
    pub fn overlaps(&self, other: Circle) -> bool {
        ((self.pos.x - other.pos.x).powf(2.0) + (self.pos.y - other.pos.y).powf(2.0)).abs()
            <= (self.radius + other.radius).powf(2.0)
    }
}
