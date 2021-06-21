//! [`Circle`], [`Ellipse`], and [`Sphere`] types used for drawing.

use crate::prelude::{Point, Vector};
use num_traits::{Num, Signed};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An `Ellipse` positioned at `(x, y)`, with `width` and `height`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ellipse<T> {
    /// Center x-coord
    pub x: T,
    /// Center y-coord
    pub y: T,
    /// Width
    pub width: T,
    /// Height
    pub height: T,
}

/// # Constructs an [`Ellipse<T>`].
///
/// ```
/// use pix_engine::prelude::*;
/// let e = ellipse!(10, 20, 100, 200);
/// assert_eq!(e.x, 10);
/// assert_eq!(e.y, 20);
/// assert_eq!(e.w, 100);
/// assert_eq!(e.h, 200);
/// ```
#[macro_export]
macro_rules! ellipse {
    ($p:expr, $r:expr$(,)?) => {
        ellipse!($p, $r, $r)
    };
    ($p:expr, $w:expr, $h:expr$(,)?) => {
        ellipse!($p.x, $p.y, $w, $h)
    };
    ($x:expr, $y:expr, $w:expr, $h:expr$(,)?) => {
        $crate::shape::ellipse::Ellipse::new($x, $y, $w, $h)
    };
}

impl<T> Ellipse<T>
where
    T: Num,
{
    /// Constructs an `Ellipse`.
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Returns whether this ellipse contains a given [`Point<T>`].
    pub fn contains_point(&self, p: impl Into<(T, T)>) -> bool
    where
        T: Signed + PartialOrd + Copy,
    {
        let (x, y) = p.into();
        let px = x - self.x;
        let py = y - self.y;
        let rx = self.width;
        let ry = self.height;
        (px * px) / (rx * rx) + (py * py) / (ry * ry) <= T::one()
    }

    /// Returns whether this ellipse intersects another ellipse.
    pub fn intersects(&self, other: Ellipse<T>) -> bool
    where
        T: Signed + PartialOrd + Copy,
    {
        let px = self.x - other.x;
        let py = self.y - other.y;
        let rx = self.width + other.width;
        let ry = self.height + other.height;
        (px * px) / (rx * rx) + (py * py) / (ry * ry) <= T::one()
    }
}

/// Convert `(x, y, w, h)` to [`Ellipse<T>`].
impl<T> From<(T, T, T, T)> for Ellipse<T>
where
    T: Num,
{
    fn from((x, y, width, height): (T, T, T, T)) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// Convert ([`Point<T>`], `w`, `h`) to [`Ellipse<T>`].
impl<T> From<(Point<T>, T, T)> for Ellipse<T> {
    fn from((p, width, height): (Point<T>, T, T)) -> Self {
        Self {
            x: p.x,
            y: p.y,
            width,
            height,
        }
    }
}

/// Convert ([`Vector<T>`], `w`, `h`) to [`Ellipse<T>`].
impl<T> From<(Vector<T>, T, T)> for Ellipse<T> {
    fn from((v, width, height): (Vector<T>, T, T)) -> Self {
        Self {
            x: v.x,
            y: v.y,
            width,
            height,
        }
    }
}

/// Convert [`Circle<T>`] to [`Ellipse<T>`].
impl<T> From<Circle<T>> for Ellipse<T>
where
    T: Copy,
{
    fn from(c: Circle<T>) -> Self {
        Self {
            x: c.x,
            y: c.y,
            width: c.radius,
            height: c.radius,
        }
    }
}

/// A `Circle` positioned at `(x, y)` with `radius`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Circle<T> {
    /// Center x-coord
    pub x: T,
    /// Center y-coord
    pub y: T,
    /// Radius
    pub radius: T,
}

/// # Constructs a [`Circle<T>`].
///
/// ```
/// use pix_engine::prelude::*;
/// let c = circle!(10, 20, 100);
/// assert_eq!(c.x, 10);
/// assert_eq!(c.y, 20);
/// assert_eq!(c.r, 100);
/// ```
#[macro_export]
macro_rules! circle {
    ($p:expr, $r:expr$(,)?) => {
        circle!($p.x, $p.y, $r)
    };
    ($x:expr, $y:expr, $r:expr$(,)?) => {
        $crate::shape::ellipse::Circle::new($x, $y, $r)
    };
}

impl<T> Circle<T>
where
    T: Num,
{
    /// Constructs a `Circle`.
    pub fn new(x: T, y: T, radius: T) -> Self {
        Self { x, y, radius }
    }

    /// Returns whether this circle contains a given [`Point<T>`].
    pub fn contains_point(&self, p: impl Into<(T, T)>) -> bool
    where
        T: Signed + PartialOrd + Copy,
    {
        let (x, y) = p.into();
        let px = x - self.x;
        let py = y - self.y;
        let r = self.radius;
        (px * px + py * py) < r
    }

    /// Returns whether this ellipse intersects another ellipse.
    pub fn intersects(&self, other: Circle<T>) -> bool
    where
        T: Signed + PartialOrd + Copy,
    {
        let px = self.x - other.x;
        let py = self.y - other.y;
        let r = self.radius + other.radius;
        (px * px + py * py) <= r * r
    }
}

/// Convert `(x, y, r)` to [`Circle<T>`].
impl<T> From<(T, T, T)> for Circle<T> {
    fn from((x, y, radius): (T, T, T)) -> Self {
        Self { x, y, radius }
    }
}

/// Convert ([`Point<T>`], `radius`) to [`Circle<T>`].
impl<T> From<(Point<T>, T)> for Circle<T> {
    fn from((p, radius): (Point<T>, T)) -> Self {
        Self {
            x: p.x,
            y: p.y,
            radius,
        }
    }
}

/// Convert ([`Vector<T>`], `radius`) to [`Circle<T>`].
impl<T> From<(Vector<T>, T)> for Circle<T> {
    fn from((v, radius): (Vector<T>, T)) -> Self {
        Self {
            x: v.x,
            y: v.y,
            radius,
        }
    }
}

/// A `Sphere` positioned at `(x, y, z)` with `radius`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Sphere<T> {
    /// Center position
    pub center: Point<T>,
    /// Radius
    pub radius: T,
}

/// # Constructs a [`Sphere<T>`].
///
/// ```
/// use pix_engine::prelude::*;
/// let s = sphere!((10, 20, 10), 100);
/// assert_eq!(s.center, point!(10, 20, 10));
/// assert_eq!(s.radius, 100);
/// ```
#[macro_export]
macro_rules! sphere {
    ($p:expr, $r:expr$(,)?) => {
        $crate::shape::ellipse::Sphere::new($p, $r)
    };
    (($x:expr, $y:expr, $z:expr), $r:expr$(,)?) => {
        $crate::shape::ellipse::Sphere::new(($x, $y, $z), $r)
    };
}

impl<T> Sphere<T>
where
    T: Num,
{
    /// Constructs a `Sphere`.
    pub fn new<P>(center: P, radius: T) -> Self
    where
        P: Into<Point<T>>,
    {
        Self {
            center: center.into(),
            radius,
        }
    }

    /// Returns whether this sphere contains a given [`Point<T>`].
    pub fn contains(&self, point: impl Into<(T, T, T)>) -> bool
    where
        T: Signed + PartialOrd + Copy,
    {
        let (x, y, z) = point.into();
        let px = x - self.center.x;
        let py = y - self.center.y;
        let pz = z - self.center.z;
        let r = self.radius;
        (px * px + py * py + pz * pz) < r * r
    }

    /// Returns whether this sphere intersects another sphere.
    pub fn intersects(&self, other: Sphere<T>) -> bool
    where
        T: Signed + PartialOrd + Copy,
    {
        let px = other.center.x - self.center.x;
        let py = other.center.y - self.center.y;
        let pz = other.center.z - self.center.z;
        let r = other.radius + self.radius;
        (px * px + py * py + pz * pz) < r * r
    }
}
