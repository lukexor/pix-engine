//! Transformation functions and types.
//!

/// Enum representing which direction to flip during drawing.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Flipped {
    /// No flip direction.
    None,
    /// Flip in the horizontal (left/right) direction.
    Horizontal,
    /// Flip in the vertical (up/down) direction.
    Vertical,
    /// Flip in both the horizontal and vertical directions.
    Both,
}
