//! Geometric shape types, manipulation, and drawing routines.

mod ellipse;
mod line;
mod point;
mod rect;
mod triangle;

pub mod prelude {
    pub use super::*;
}

pub use ellipse::{ArcMode, EllipseMode};
pub use line::{Line, StrokeCap, StrokeJoin, DEFAULT_STROKE_WEIGHT};
pub use point::Point;
pub use rect::{Rect, RectMode};
