//! Shape functions for drawing.

use num_traits::Float;

#[macro_use]
pub mod ellipse;
#[macro_use]
pub mod line;
#[macro_use]
pub mod point;
#[macro_use]
pub mod rect;
pub mod quad;
#[macro_use]
pub mod sphere;
pub mod triangle;

pub use ellipse::{Circle, Ellipse};
pub use line::Line;
pub use point::Point;
pub use quad::Quad;
pub use rect::Rect;
pub use sphere::Sphere;
pub use triangle::Triangle;

/// Trait for operations on a geometric shape.
pub trait Shape<T> {
    /// The shape type. e.g. [Rect<T>].
    type Item;

    /// Returns whether this shape contains a given [Point].
    fn contains_point<P: Into<Point<T>>>(&self, _p: P) -> bool {
        unimplemented!("contains_point is not implemented")
    }

    /// Returns whether this shape completely contains another shape of the same type.
    fn contains<O: Into<Self::Item>>(&self, _other: O) -> bool {
        unimplemented!("contains is not implemented")
    }

    /// Returns the closest intersection point with a given line and distance along the line or
    /// `None` if there is no intersection.
    fn intersects_line<L>(&self, _line: L) -> Option<(Point<T>, T)>
    where
        T: Float,
        L: Into<Line<T>>,
    {
        unimplemented!("intersects_line is not implemented")
    }

    /// Returns whether this shape intersects with another shape of the same type.
    fn intersects<O: Into<Self::Item>>(&self, _other: O) -> bool {
        unimplemented!("intersects is not implemented")
    }
}
