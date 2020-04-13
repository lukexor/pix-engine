mod ellipse;

pub const DEFAULT_STROKE_WEIGHT: u32 = 1;

pub use ellipse::ArcMode;

/// Determines the way ellipses/circles are drawn by changing how the parameters given to
/// State.draw_ellipse and State.draw_circle are interpreted. The default is Center.
///
/// Center: Uses x and y as the center of the shape
/// Radius: Uses x and y as the center, but the w/h or d values as half the shape's width/height
/// Corner: Uses x and y as the upper-left corner of the shape
pub enum EllipseMode {
    Center,
    Radius,
    Corner,
}

/// Determines the way rect/squares are drawn by changing how the parameters given to
/// State.draw_rect and State.draw_square are interpreted. The default is Corner.
///
/// Corner: Uses x and y as the upper-left corner of the shape
/// Center: Uses x and y as the center of the shape
/// Radius: Uses x and y as the center, but the w/h or d values as half the shape's width/height
pub enum RectMode {
    Corner,
    Center,
    Radius,
}

/// Sets the style for rendering line endings. More noticeable when stroke weight is set greater
/// than 1. The default is Round.
pub enum StrokeCap {
    Round,
    Square,
    Project,
}

/// Sets the style of the joints which connect line segments. More noticeable when stroke weight is
/// set greater than 1. The default is Miter.
pub enum StrokeJoin {
    Miter,
    Bevel,
    Round,
}

impl Default for EllipseMode {
    fn default() -> Self {
        Self::Center
    }
}

impl Default for RectMode {
    fn default() -> Self {
        Self::Corner
    }
}

impl Default for StrokeCap {
    fn default() -> Self {
        Self::Round
    }
}

impl Default for StrokeJoin {
    fn default() -> Self {
        Self::Miter
    }
}
