use crate::state::State;

mod ellipse;

pub const DEFAULT_STROKE_WEIGHT: u32 = 1;

pub use ellipse::{ArcMode, EllipseMode};

/// Determines the way rect/squares are drawn by changing how the parameters given to
/// `State::draw_rect()` and `State::draw_square()` are interpreted. The default is Corner.
///
/// Corner: Uses x and y as the upper-left corner of the shape.
/// Center: Uses x and y as the center of the shape.
/// Radius: Uses x and y as the center, but the w/h or d values as half the shape's width/height.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RectMode {
    Corner,
    Center,
    Radius,
}

/// Represents a rectangle.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

/// Represents a single point.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Sets the style for rendering line endings. More noticeable when stroke weight is set greater
/// than 1. The default is Round.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StrokeCap {
    Round,
    Square,
    Project,
}

/// Sets the style of the joints which connect line segments. More noticeable when stroke weight is
/// set greater than 1. The default is Miter.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StrokeJoin {
    Miter,
    Bevel,
    Round,
}

impl Rect {
    fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Default for RectMode {
    fn default() -> Self {
        Self::Corner
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

/// From tuple of (width, height) to Rect
impl From<(u32, u32)> for Rect {
    fn from((w, h): (u32, u32)) -> Self {
        Self::new(0, 0, w, h)
    }
}
/// From tuple of (x, y, width, height) to Rect
impl From<(i32, i32, u32, u32)> for Rect {
    fn from((x, y, w, h): (i32, i32, u32, u32)) -> Self {
        Self::new(x, y, w, h)
    }
}

/// From tuple of (x, y) to Point
impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
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

impl State {}
