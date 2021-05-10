//! Shape types and functions for [`Point`], [`Circle`], [`Rectangle`], [`Triangle`]

use std::ops::{Index, IndexMut};

/// A `Point`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    /// X-coord
    pub x: i32,
    /// Y-coord
    pub y: i32,
    /// Z-coord
    pub z: i32,
}

impl Point {
    /// Create new `Point`.
    pub fn new<P>(p: P) -> Self
    where
        P: Into<Point>,
    {
        p.into()
    }

    /// Create new 2D `Point`.
    pub fn new_2d(x: i32, y: i32) -> Self {
        Self { x, y, z: 0 }
    }

    /// Create new 3D `Point`.
    pub fn new_3d(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl Index<usize> for Point {
    type Output = i32;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds: the len is 3 but the index is {}", idx),
        }
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds: the len is 3 but the index is {}", idx),
        }
    }
}

/// From tuple of (i32, i32) to [`Point`].
impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new_2d(x, y)
    }
}

/// From tuple of (i32, i32, i32) to [`Point`].
impl From<(i32, i32, i32)> for Point {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self::new_3d(x, y, z)
    }
}

/// From 2D tuple of (x, y) f64 to [`Point`].
impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Self {
        let x = x.round() as i32;
        let y = y.round() as i32;
        Self::new_2d(x, y)
    }
}

/// From 3D tuple of (x, y, z) f64 to [`Point`].
impl From<(f64, f64, f64)> for Point {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        let x = x.round() as i32;
        let y = y.round() as i32;
        let z = z.round() as i32;
        Self::new_3d(x, y, z)
    }
}

/// Convert to i32 tuple of (x, y).
impl Into<(i32, i32)> for Point {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

/// Convert to i32 tuple of (x, y, z).
impl Into<(i32, i32, i32)> for Point {
    fn into(self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }
}

/// A `Line`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Line {
    /// Start Point.
    pub p1: Point,
    /// End Point.
    pub p2: Point,
}

impl Line {
    /// Creates a new `Line`.
    pub fn new<P>(p1: P, p2: P) -> Self
    where
        P: Into<Point>,
    {
        Self {
            p1: p1.into(),
            p2: p2.into(),
        }
    }
}

/// From tuple of (x1, y1, x2, y2) to `Line`.
impl From<(i32, i32, i32, i32)> for Line {
    fn from((x1, y1, x2, y2): (i32, i32, i32, i32)) -> Self {
        Self::new((x1, y1), (x2, y2))
    }
}

/// From tuple of (Point, Point) to `Line`.
impl From<(Point, Point)> for Line {
    fn from((p1, p2): (Point, Point)) -> Self {
        Self::new(p1, p2)
    }
}

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
    fn from((p, r): (Point, u32)) -> Self {
        Self::new(p.x, p.y, r)
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
    fn from((p, w, h): (Point, u32, u32)) -> Self {
        Self::new(p.x, p.y, w, h)
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
