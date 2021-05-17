//! 2D Circle types used for drawing.

use super::Point;
use crate::{math::Scalar, vector::Vector};

/// An `Ellipse`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Ellipse {
    /// Center x-coord
    pub x: i32,
    /// Center y-coord
    pub y: i32,
    /// Width
    pub w: u32,
    /// Height
    pub h: u32,
}

/// # Create a [`Ellipse`].
///
/// ```
/// use pix_engine::prelude::*;
///
/// let e = ellipse!(10, 20, 100, 200);
/// assert_eq!(e.x, 10);
/// assert_eq!(e.y, 20);
/// assert_eq!(e.w, 100);
/// assert_eq!(e.h, 200);
/// ```
#[macro_export]
macro_rules! ellipse {
    () => {
        ellipse!(0, 0)
    };
    ($x:expr, $y:expr$(,)?) => {
        ellipse!($x, $y, 100)
    };
    ($x:expr, $y:expr, $s:expr$(,)?) => {
        ellipse!($x, $y, $s, $s)
    };
    ($x:expr, $y:expr, $w:expr, $h:expr$(,)?) => {
        $crate::prelude::Ellipse::new($x as i32, $y as i32, $w as u32, $h as u32)
    };
}

impl Ellipse {
    /// Creates a new [`Ellipse`].
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    /// Detects whether a 2D point (x, y) lies inside this ellipse.
    pub fn contains<P>(&self, p: P) -> bool
    where
        P: Into<Point>,
    {
        let _p = p.into();
        todo!("ellipse contains");
    }

    /// Detects whether another ellipse overlaps this one.
    pub fn overlaps(&self, _other: Ellipse) -> bool {
        todo!("ellipse overlaps");
    }
}

/// From tuple of (x, y, w, h) to [`Ellipse`].
impl From<(i32, i32, u32, u32)> for Ellipse {
    fn from((x, y, w, h): (i32, i32, u32, u32)) -> Self {
        Self::new(x, y, w, h)
    }
}

/// From tuple of (x, y, w, h) to [`Ellipse`].
impl From<(Scalar, Scalar, u32, u32)> for Ellipse {
    fn from((x, y, w, h): (Scalar, Scalar, u32, u32)) -> Self {
        Self::new(x.round() as i32, y.round() as i32, w, h)
    }
}

/// From tuple of (`Point`, r) to [`Ellipse`].
impl From<(Point, u32, u32)> for Ellipse {
    fn from((p, w, h): (Point, u32, u32)) -> Self {
        Self::new(p.x, p.y, w, h)
    }
}

/// From tuple of (`Vector`, r) to [`Ellipse`].
impl From<(Vector, u32, u32)> for Ellipse {
    fn from((v, w, h): (Vector, u32, u32)) -> Self {
        Self::new(v.x.round() as i32, v.y.round() as i32, w, h)
    }
}

/// From [`Circle`] to [`Ellipse`].
impl From<Circle> for Ellipse {
    fn from(c: Circle) -> Self {
        Self::new(c.x, c.y, c.r, c.r)
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

/// # Create a [`Circle`].
///
/// ```
/// use pix_engine::prelude::*;
///
/// let c = circle!(10, 20, 100);
/// assert_eq!(c.x, 10);
/// assert_eq!(c.y, 20);
/// assert_eq!(c.r, 100);
/// ```
#[macro_export]
macro_rules! circle {
    () => {
        circle!(0, 0)
    };
    ($x:expr, $y:expr$(,)?) => {
        circle!($x, $y, 100)
    };
    ($x:expr, $y:expr, $r:expr$(,)?) => {
        $crate::prelude::Circle::new($x as i32, $y as i32, $r as u32)
    };
}

impl Circle {
    /// Creates a new `Circle`.
    pub fn new(x: i32, y: i32, r: u32) -> Self {
        Self { x, y, r }
    }

    /// Detects whether a 2D point (x, y) lies inside this circle.
    pub fn contains<P>(&self, p: P) -> bool
    where
        P: Into<Point>,
    {
        let p = p.into();
        ((p.x - self.x).pow(2) + (p.y - self.y).pow(2)) < self.r.pow(2) as i32
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

/// From tuple of (x, y, r) to `Circle`.
impl From<(Scalar, Scalar, u32)> for Circle {
    fn from((x, y, r): (Scalar, Scalar, u32)) -> Self {
        Self::new(x.round() as i32, y.round() as i32, r)
    }
}

/// From tuple of (`Point`, r) to `Circle`.
impl From<(Point, u32)> for Circle {
    fn from((p, r): (Point, u32)) -> Self {
        Self::new(p.x, p.y, r)
    }
}

/// From tuple of (`Vector`, r) to `Circle`.
impl From<(Vector, u32)> for Circle {
    fn from((v, r): (Vector, u32)) -> Self {
        Self::new(v.x.round() as i32, v.y.round() as i32, r)
    }
}
