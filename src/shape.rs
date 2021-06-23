//! Shape functions for drawing.

use crate::prelude::{PixResult, PixState};

#[macro_use]
pub mod ellipse;
#[macro_use]
pub mod line;
#[macro_use]
pub mod point;
#[macro_use]
pub mod rect;
pub mod triangle;

pub use ellipse::{Circle, Ellipse, Sphere};
pub use line::Line;
pub use point::Point;
pub use rect::Rect;
pub use triangle::Triangle;

/// Trait for operations on a geometric shape.
pub trait Shape<T> {
    type Item;
    type DrawType;

    /// Returns whether this shape contains a given [`Point<T>`].
    fn contains_point(&self, p: impl Into<Point<T>>) -> bool;

    /// Returns whether this shape completely contains another shape.
    fn contains(&self, other: impl Into<Self::Item>) -> bool;

    /// Returns whether this rectangle intersects with a line.
    fn intersects_line(&self, line: impl Into<Line<f64>>) -> Option<(Point<f64>, Point<f64>)>;

    /// Returns whether this shape intersects with another shape.
    fn intersects(&self, other: impl Into<Self::Item>) -> bool;
}
