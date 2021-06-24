//! Shape functions for drawing.

use num_traits::{AsPrimitive, Num};

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

/// Trait constraint for implementing [`Shape`].
pub trait ShapeNum: Num + Copy + PartialOrd + AsPrimitive<f64> {}

impl<T> ShapeNum for T where T: Num + Copy + PartialOrd + AsPrimitive<f64> {}

/// Trait for operations on a geometric shape.
pub trait Shape<T>
where
    T: ShapeNum,
{
    /// The shape type. e.g. [`Rect<T>`].
    type Item;

    /// Returns whether this shape contains a given [`Point<T>`].
    fn contains_point(&self, p: impl Into<Point<T>>) -> bool;

    /// Returns whether this shape completely contains another shape.
    fn contains(&self, other: impl Into<Self::Item>) -> bool;

    /// Returns the closest intersection point with a given line and distance along the line or
    /// `None` if there is no intersection.
    fn intersects_line(&self, line: impl Into<Line<f64>>) -> Option<(Point<f64>, f64)>;

    /// Returns whether this shape intersects with another shape.
    fn intersects(&self, other: impl Into<Self::Item>) -> bool;
}
