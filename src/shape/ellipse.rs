use crate::state::State;

/// Determines the way ellipses/circles are drawn by changing how the parameters given to
/// `State::draw_ellipse()` and `State::draw_circle()` are interpreted. The default is Center.
///
/// Center: Uses x and y as the center of the shape.
/// Radius: Uses x and y as the center, but the w/h or d values as half the shape's width/height.
/// Corner: Uses x and y as the upper-left corner of the shape.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EllipseMode {
    Center,
    Radius,
    Corner,
}

impl Default for EllipseMode {
    fn default() -> Self {
        Self::Center
    }
}

/// Determines the way arcs are filled. Has no effect if fill is disabled. The default is Pie.
///
/// Pie: Filled as a closed pie segment.
/// Open: Filled like an open semi-circle.
/// Chord: Filled like a closed semi-circle.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ArcMode {
    Pie,
    Open,
    Chord,
}

impl Default for ArcMode {
    fn default() -> Self {
        Self::Pie
    }
}

impl State {}
