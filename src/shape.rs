//! Shape types and functions for [`Point`], [`Circle`], [`Rectangle`], [`Triangle`]

/// A `Point`.
pub type Point = (i32, i32);

/// A `Circle`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Circle {
    /// Center x-coord
    pub x: i32,
    /// Center y-coord
    pub y: i32,
    /// Radius
    pub r: u32,
}

impl Circle {
    /// Creates a new `Circle`.
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

/// From tuple of (x, y, r) to `Circle`.
impl From<(i32, i32, u32)> for Circle {
    fn from((x, y, r): (i32, i32, u32)) -> Self {
        Self::new(x, y, r)
    }
}

/// From tuple of (Point, r) to `Circle`.
impl From<(Point, u32)> for Circle {
    fn from(((x, y), r): (Point, u32)) -> Self {
        Self::new(x, y, r)
    }
}

/// A `Rectangle`.
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
    /// Creates a new `Rect`.
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    /// Creates a new `Rect` with equal width and height.
    pub fn square(x: i32, y: i32, w: u32) -> Self {
        Self { x, y, w, h: w }
    }
}

/// From tuple of (x, y, w, h) to `Rect`.
impl From<(i32, i32, u32, u32)> for Rect {
    fn from((x, y, w, h): (i32, i32, u32, u32)) -> Self {
        Self::new(x, y, w, h)
    }
}

/// From tuple of (Point, w, h) to `Rect`.
impl From<(Point, u32, u32)> for Rect {
    fn from(((x, y), w, h): (Point, u32, u32)) -> Self {
        Self::new(x, y, w, h)
    }
}

/// A `Triangle`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Triangle {
    /// Point 1
    pub p1: Point,
    /// Point 2
    pub p2: Point,
    /// Point 3
    pub p3: Point,
}

impl Triangle {
    /// Creates a new `Triangle`.
    pub fn new<P: Into<Point>>(p1: P, p2: P, p3: P) -> Self {
        Self {
            p1: p1.into(),
            p2: p2.into(),
            p3: p3.into(),
        }
    }
}

/// From tuple of (x1, y1, x2, y2, x3, y3) to `Triangle`.
impl From<(i32, i32, i32, i32, i32, i32)> for Triangle {
    fn from((x1, y1, x2, y2, x3, y3): (i32, i32, i32, i32, i32, i32)) -> Self {
        Self::new((x1, y1), (x2, y2), (x3, y3))
    }
}

/// From tuple of (Point1, Point2, Point3) to `Triangle`.
impl From<(Point, Point, Point)> for Triangle {
    fn from((p1, p2, p3): (Point, Point, Point)) -> Self {
        Self::new(p1, p2, p3)
    }
}
