use crate::{math::Vector, state::State};
use std::fmt;

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

impl Default for RectMode {
    fn default() -> Self {
        Self::Corner
    }
}

/// Represents a rectangle shape with (x, y) position and width/height.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    /// Creates a new `Rect` instance.
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    /// Gets the (width, height) size of the `Rect`.
    pub fn dimensions(&self) -> (u32, u32) {
        (self.w, self.h)
    }

    /// Gets the left x-position of the `Rect`.
    pub fn left(&self) -> i32 {
        self.x
    }

    /// Gets the right x-position of the `Rect`.
    pub fn right(&self) -> i32 {
        self.x + self.w as i32
    }

    /// Gets the top y-position of the `Rect`.
    pub fn top(&self) -> i32 {
        self.y
    }

    /// Gets the bottom y-position of the `Rect`.
    pub fn bottom(&self) -> i32 {
        self.y + self.h as i32
    }

    /// Gets the center position of the `Rect`.
    pub fn center(&self) -> Point {
        Point::new((self.x + self.w as i32 / 2, self.y + self.h as i32 / 2))
    }

    /// Gets the top-left corner of the `Rect`.
    pub fn top_left(&self) -> Point {
        Point::new((self.x, self.y))
    }

    /// Gets the top-right corner of the `Rect`.
    pub fn top_right(&self) -> Point {
        Point::new((self.x + self.w as i32, self.y))
    }

    /// Gets the bottom-left corner of the `Rect`.
    pub fn bottom_left(&self) -> Point {
        Point::new((self.x, self.y + self.h as i32))
    }

    /// Gets the bottom-right corner of the `Rect`.
    pub fn bottom_right(&self) -> Point {
        Point::new((self.x + self.w as i32, self.y + self.h as i32))
    }

    /// Move the center of the `Rect` to a given `Point`.
    pub fn center_on<P: Into<Point>>(&mut self, p: P) {
        let p = p.into();
        self.x = p.x - self.w as i32 / 2;
        self.y = p.y - self.h as i32 / 2;
    }

    /// Move the top-left corner of the `Rect` to a given `Point`.
    pub fn move_to<P: Into<Point>>(&mut self, p: P) {
        let p = p.into();
        self.x = p.x;
        self.y = p.y;
    }

    /// Checks whether the `Rect` contains a given `Point`.
    ///
    /// Points along the right and bottom edges aren't considered inside the `Rect`. This means
    /// a 1-by-1 rectangle only contains a single `Point`.
    pub fn contains_point<P: Into<Point>>(&mut self, p: P) -> bool {
        let p = p.into();
        p.x >= self.left() && p.x < self.right() && p.y >= self.top() && p.y < self.bottom()
    }
}

/// From tuple of (width, height) to Rect.
impl From<(u32, u32)> for Rect {
    fn from((w, h): (u32, u32)) -> Self {
        Self::new(0, 0, w, h)
    }
}

/// From tuple of (x, y, width, height) to Rect.
impl From<(i32, i32, u32, u32)> for Rect {
    fn from((x, y, w, h): (i32, i32, u32, u32)) -> Self {
        Self::new(x, y, w, h)
    }
}

/// From upper-left `Point` to lower-right `Point`.
impl From<(Point, Point)> for Rect {
    fn from((p1, p2): (Point, Point)) -> Self {
        Self::new(p1.x, p1.y, (p2.x - p1.x) as u32, (p2.y - p1.y) as u32)
    }
}

/// Into a tuple of (x, y, width, height).
impl Into<(i32, i32, u32, u32)> for Rect {
    fn into(self) -> (i32, i32, u32, u32) {
        (self.x, self.y, self.w, self.h)
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.w, self.h)
    }
}

/// Sets the style for rendering line endings. More noticeable when stroke weight is set greater
/// than 1. The default is Round.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StrokeCap {
    Round,
    Square,
    Project,
}

impl Default for StrokeCap {
    fn default() -> Self {
        Self::Round
    }
}

/// Sets the style of the joints which connect line segments. More noticeable when stroke weight is
/// set greater than 1. The default is Miter.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StrokeJoin {
    Miter,
    Bevel,
    Round,
}

impl Default for StrokeJoin {
    fn default() -> Self {
        Self::Miter
    }
}

/// Represents a single point on the screen.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point {
    /// Creates a new Point in screen pixel space.
    pub fn new<P: Into<Point>>(p: P) -> Self {
        p.into()
    }

    /// Creates a new 2D Point in screen pixel space.
    pub fn new_2d(x: i32, y: i32) -> Self {
        Self::new_3d(x, y, 0)
    }

    /// Creates a new 3D Point in screen pixel space.
    pub fn new_3d(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Creates a new Point in screen space from a 3D Vector. Any decimal values of the Vector will be
    /// truncated and the z-axis ignored.
    pub fn from_vector(v: Vector) -> Self {
        Self::new_3d(v.x as i32, v.y as i32, v.z as i32)
    }
}

/// From an i32 tuple of (x, y) to Point.
impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new_2d(x, y)
    }
}

/// From an i32 tuple of (x, y, z) to Point.
impl From<(i32, i32, i32)> for Point {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self::new_3d(x, y, z)
    }
}

/// Convert to an i32 tuple of (x, y, z).
impl Into<(i32, i32, i32)> for Point {
    fn into(self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }
}

impl From<Vector> for Point {
    fn from(v: Vector) -> Self {
        Self::from_vector(v)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl State {}
