//! [`Circle`], [`Ellipse`], and [`Sphere`] types used for drawing.

use crate::prelude::{Point, Vector};
use num::Num;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An `Ellipse` positioned at `(x, y)`, with `width` and `height`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ellipse<T> {
    /// Center x-coord
    pub x: T,
    /// Center y-coord
    pub y: T,
    /// Width
    pub w: T,
    /// Height
    pub h: T,
}

/// # Construct an [`Ellipse<T>`].
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
    /// Construct an `Ellipse`.
    pub fn new(x: T, y: T, w: T, h: T) -> Self {
        Self { x, y, w, h }
    }

    /// Whether a 2D [`Point<T>`] lies inside this ellipse.
    pub fn contains(&self, p: impl Into<Point<T>>) -> bool {
        let _p = p.into();
        todo!("ellipse contains");
    }

    /// Whether another ellipse overlaps this one.
    pub fn overlaps(&self, _other: Ellipse<T>) -> bool {
        todo!("ellipse overlaps");
    }
}

/// Convert `(x, y, w, h)` to [`Ellipse<T>`].
impl<T> From<(T, T, T, T)> for Ellipse<T>
where
    T: Num,
{
    fn from((x, y, w, h): (T, T, T, T)) -> Self {
        Self { x, y, w, h }
    }
}

/// Convert ([`Point<T>`], `w`, `h`) to [`Ellipse<T>`].
impl<T> From<(Point<T>, T, T)> for Ellipse<T> {
    fn from((p, w, h): (Point<T>, T, T)) -> Self {
        Self {
            x: p.x,
            y: p.y,
            w,
            h,
        }
    }
}

/// Convert ([`Vector<T>`], `w`, `h`) to [`Ellipse<T>`].
impl<T> From<(Vector<T>, T, T)> for Ellipse<T> {
    fn from((v, w, h): (Vector<T>, T, T)) -> Self {
        Self {
            x: v.x,
            y: v.y,
            w,
            h,
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
            w: c.r,
            h: c.r,
        }
    }
}

/// A `Circle` positioned at `(x, y)` with `radius`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Circle<T> {
    /// Center x-coord
    pub x: T,
    /// Center y-coord
    pub y: T,
    /// Radius
    pub r: T,
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
    pub fn new(x: T, y: T, r: T) -> Self {
        Self { x, y, r }
    }

    /// Whether a 2D [`Point<T>`] lies inside this circle.
    pub fn contains(&self, p: impl Into<Point<T>>) -> bool
    where
        T: PartialOrd + Copy,
    {
        let p = p.into();
        let px = p.x - self.x;
        let py = p.y - self.y;
        (px * px + py * py) < self.r * self.r
    }

    /// Whether another circle overlaps this one.
    pub fn overlaps(&self, other: Circle<T>) -> bool
    where
        T: PartialOrd + Copy,
    {
        let px = self.x - other.x;
        let py = self.y - other.y;
        let r = self.r + other.r;
        (px * px + py * py) <= r * r
    }
}

/// Convert `(x, y, r)` to [`Circle<T>`].
impl<T> From<(T, T, T)> for Circle<T> {
    fn from((x, y, r): (T, T, T)) -> Self {
        Self { x, y, r }
    }
}

/// Convert ([`Point<T>`], `radius`) to [`Circle<T>`].
impl<T> From<(Point<T>, T)> for Circle<T> {
    fn from((p, r): (Point<T>, T)) -> Self {
        Self { x: p.x, y: p.y, r }
    }
}

/// Convert ([`Vector<T>`], `radius`) to [`Circle<T>`].
impl<T> From<(Vector<T>, T)> for Circle<T> {
    fn from((v, r): (Vector<T>, T)) -> Self {
        Self { x: v.x, y: v.y, r }
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

    /// Whether a 3D [`Point<T>`] lies inside this sphere.
    pub fn contains(&self, _p: impl Into<Point<T>>) -> bool
    where
        T: PartialOrd + Copy,
    {
        todo!("sphere contains")
    }

    /// Whether another sphere overlaps this one.
    pub fn overlaps(&self, _other: Sphere<T>) -> bool
    where
        T: PartialOrd + Copy,
    {
        todo!("sphere overlaps")
    }
}
