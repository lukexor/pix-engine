use crate::{
    math::Vector,
    renderer::Renderer,
    state_data::{StateData, StateDataResult},
};
use std::fmt;

/// Represents a single point on the screen.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
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
        Self::new_3d(v.x.round() as i32, v.y.round() as i32, v.z.round() as i32)
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

/// Convert to an i32 tuple of (x, y).
impl Into<(i32, i32)> for Point {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

/// Convert to an i32 tuple of (x, y, z).
impl Into<(i32, i32, i32)> for Point {
    fn into(self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }
}

/// Convert to an f64 tuple of (x, y).
impl Into<(f64, f64)> for Point {
    fn into(self) -> (f64, f64) {
        (self.x as f64, self.y as f64)
    }
}

/// Convert to an f64 tuple of (x, y, z).
impl Into<(f64, f64, f64)> for Point {
    fn into(self) -> (f64, f64, f64) {
        (self.x as f64, self.y as f64, self.z as f64)
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

impl StateData {
    /// Draw a point.
    pub fn point<P: Into<Point>>(&mut self, point: P) -> StateDataResult<()> {
        Ok(self.renderer.point(point.into())?)
    }
}
