//! Transformation functions and types.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Enum representing which direction to flip during drawing.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
