//! Shape types and functions for Circle, Rectangle, etc.

/// A Circle shape
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Circle {
    /// Center x-coord
    pub x: i32,
    /// Center y-coord
    pub y: i32,
    /// Radius of circle
    pub r: u32,
}

impl Circle {
    /// Creates a new Circle shape.
    pub fn new(x: i32, y: i32, r: u32) -> Self {
        Self { x, y, r }
    }

    /// Detects whether a 2D point (x, y) lies inside this circle.
    pub fn contains(&self, x: i32, y: i32) -> bool {
        ((x - self.x).pow(2) + (y - self.y).pow(2)) < self.r.pow(2) as i32
    }

    /// Detects whether another circle overlaps this one.
    pub fn overlaps(&self, other: Circle) -> bool {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) <= (self.r + other.r).pow(2) as i32
    }
}

/// A Rectangle shape
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rect {
    /// X-coord
    pub x: i32,
    /// Y-coord
    pub y: i32,
    /// Width
    pub w: u32,
    /// Height
    pub h: u32,
}

impl Rect {
    /// Creates a new Rect shape.
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    /// Creates a new Rect shape with equal width and height.
    pub fn square(x: i32, y: i32, w: u32) -> Self {
        Self { x, y, w, h: w }
    }
}

/// From tuple of (x, y, w, h) to `Rect`
impl From<(i32, i32, u32, u32)> for Rect {
    fn from((x, y, w, h): (i32, i32, u32, u32)) -> Self {
        Self::new(x, y, w, h)
    }
}
