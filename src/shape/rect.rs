use super::Point;
use crate::math::Vector;
use std::fmt;

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

/// From tuple of (width, height) to `Rect`.
impl From<(u32, u32)> for Rect {
    fn from((w, h): (u32, u32)) -> Self {
        Self::new(0, 0, w, h)
    }
}

/// From tuple of (x, y, size) to `Rect` makes a Square.
impl From<(i32, i32, u32)> for Rect {
    fn from((x, y, s): (i32, i32, u32)) -> Self {
        Self::new(x, y, s, s)
    }
}

/// From tuple of (`Point`, size) to `Rect` makes a Square.
impl From<(Point, u32)> for Rect {
    fn from((p, s): (Point, u32)) -> Self {
        Self::new(p.x, p.y, s, s)
    }
}

/// From tuple of (`Vector`, size) to `Rect` makes a Square.
impl From<(Vector, u32)> for Rect {
    fn from((v, s): (Vector, u32)) -> Self {
        Self::new(v.x as i32, v.y as i32, s, s)
    }
}

/// From tuple of (x, y, width, height) to `Rect`.
impl From<(i32, i32, u32, u32)> for Rect {
    fn from((x, y, w, h): (i32, i32, u32, u32)) -> Self {
        Self::new(x, y, w, h)
    }
}

/// From tuple of (`Point`, width, height) to `Rect`.
impl From<(Point, u32, u32)> for Rect {
    fn from((p, w, h): (Point, u32, u32)) -> Self {
        Self::new(p.x, p.y, w, h)
    }
}

/// From tuple of (`Vector`, width, height) to `Rect`.
impl From<(Vector, u32, u32)> for Rect {
    fn from((v, w, h): (Vector, u32, u32)) -> Self {
        Self::new(v.x as i32, v.y as i32, w, h)
    }
}

/// From upper-left `Point` to lower-right `Point`.
impl From<(Point, Point)> for Rect {
    fn from((p1, p2): (Point, Point)) -> Self {
        Self::new(p1.x, p1.y, (p2.x - p1.x) as u32, (p2.y - p1.y) as u32)
    }
}

/// From upper-left `Vector` to lower-right `Vector`.
impl From<(Vector, Vector)> for Rect {
    fn from((v1, v2): (Vector, Vector)) -> Self {
        Self::new(
            v1.x as i32,
            v1.y as i32,
            (v2.x - v1.x) as u32,
            (v2.y - v1.y) as u32,
        )
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
