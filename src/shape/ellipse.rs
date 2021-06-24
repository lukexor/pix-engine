//! [`Circle`], [`Ellipse`], and [`Sphere`] types used for drawing.

use crate::prelude::{Draw, Line, PixResult, PixState, Point, Shape, ShapeNum, Vector};
use num_traits::{AsPrimitive, Num, Signed};
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
/// assert_eq!(e.width, 100);
/// assert_eq!(e.height, 200);
/// ```
#[macro_export]
macro_rules! ellipse {
    ($p:expr, $r:expr$(,)?) => {
        ellipse!($p, $r, $r)
    };
    ($p:expr, $width:expr, $height:expr$(,)?) => {
        ellipse!($p.x, $p.y, $width, $height)
    };
    ($x:expr, $y:expr, $width:expr, $height:expr$(,)?) => {
        $crate::shape::ellipse::Ellipse::new($x, $y, $width, $height)
    };
}

impl<T> Ellipse<T> {
    /// Constructs an `Ellipse`.
    pub const fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Converts [`Ellipse<T>`] to [`Ellipse<i16>`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let e: Ellipse<f32> = Ellipse::new(f32::MAX, 2.0, 3.0, f32::MIN);
    /// let e = e.as_i16();
    /// assert_eq!(e.x, i16::MAX);
    /// assert_eq!(e.y, 2);
    /// assert_eq!(e.width, 3);
    /// assert_eq!(e.height, i16::MIN);
    /// ```
    pub fn as_i16(&self) -> Ellipse<i16>
    where
        T: AsPrimitive<i16>,
    {
        Ellipse::new(
            self.x.as_(),
            self.y.as_(),
            self.width.as_(),
            self.height.as_(),
        )
    }
}

impl<T> Ellipse<T>
where
    T: Num,
{
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

impl<T: ShapeNum> Shape<T> for Ellipse<T> {
    type Item = Ellipse<T>;

    /// Returns whether this ellipse contains a given [`Point<T>`].
    fn contains_point(&self, _p: impl Into<Point<T>>) -> bool {
        todo!()
    }

    /// Returns whether this ellipse completely contains another ellipse.
    fn contains(&self, _other: impl Into<Self::Item>) -> bool {
        todo!()
    }

    /// Returns the closest intersection point with a given line and distance along the line or
    /// `None` if there is no intersection.
    fn intersects_line(&self, _line: impl Into<Line<f64>>) -> Option<(Point<f64>, f64)> {
        todo!()
    }

    /// Returns whether this ellipse intersects with another ellipse.
    fn intersects(&self, _other: impl Into<Self::Item>) -> bool {
        todo!()
    }
}

impl<T> Draw for Ellipse<T>
where
    Ellipse<T>: Copy + Into<Ellipse<f64>>,
{
    /// Draw ellipse to the current [`PixState`] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.ellipse(*self)
    }
}

/// Convert `[x, y, width, height]` to [`Ellipse<T>`].
impl<T, U: Into<T>> From<[U; 4]> for Ellipse<T> {
    fn from([x, y, width, height]: [U; 4]) -> Self {
        Self::new(x.into(), y.into(), width.into(), height.into())
    }
}

/// Convert `&[x, y, width, height]` to [`Ellipse<T>`].
impl<T, U: Copy + Into<T>> From<&[U; 4]> for Ellipse<T> {
    fn from(&[x, y, width, height]: &[U; 4]) -> Self {
        Self::new(x.into(), y.into(), width.into(), height.into())
    }
}

/// Convert [`Ellipse<T>`] to `[x, y, width, height]`.
impl<T: Copy, U: Into<T>> From<Ellipse<U>> for [T; 4] {
    fn from(e: Ellipse<U>) -> Self {
        [e.x.into(), e.y.into(), e.width.into(), e.height.into()]
    }
}

/// Convert [`&Ellipse<T>`] to `[x, y, width, height]`.
impl<T, U: Copy + Into<T>> From<&Ellipse<U>> for [T; 4] {
    fn from(e: &Ellipse<U>) -> Self {
        [e.x.into(), e.y.into(), e.width.into(), e.height.into()]
    }
}

/// Convert [`Circle<U>`] to [`Ellipse<T>`].
impl<T: Copy, U: Into<T>> From<Circle<U>> for Ellipse<T> {
    fn from(c: Circle<U>) -> Self {
        let radius = c.radius.into();
        Self::new(c.x.into(), c.y.into(), radius, radius)
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
/// assert_eq!(c.radius, 100);
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

impl<T> Circle<T> {
    /// Constructs a `Circle`.
    pub const fn new(x: T, y: T, radius: T) -> Self {
        Self { x, y, radius }
    }

    /// Constructs a `Circle` from [`Point<T>`].
    pub fn from_point(p: Point<T>, radius: T) -> Self {
        Self::new(p.x, p.y, radius)
    }

    /// Constructs a `Circle` from [`Vector<T>`].
    pub fn from_vector(v: Vector<T>, radius: T) -> Self {
        Self::new(v.x, v.y, radius)
    }

    /// Converts [`Circle<T>`] to [`Circle<i16>`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c: Circle<f32> = Circle::new(f32::MIN, 2.0, f32::MAX);
    /// let c = c.as_i16();
    /// assert_eq!(c.x, i16::MIN);
    /// assert_eq!(c.y, 2);
    /// assert_eq!(c.radius, i16::MAX);
    /// ```
    pub fn as_i16(&self) -> Circle<i16>
    where
        T: AsPrimitive<i16>,
    {
        Circle::new(self.x.as_(), self.y.as_(), self.radius.as_())
    }
}

impl<T: ShapeNum> Shape<T> for Circle<T> {
    type Item = Circle<T>;

    /// Returns whether this circle contains a given [`Point<T>`].
    fn contains_point(&self, p: impl Into<Point<T>>) -> bool {
        let p = p.into();
        let px = p.x - self.x;
        let py = p.y - self.y;
        let r = self.radius;
        (px * px + py * py) < r
    }

    /// Returns whether this circle completely contains another circle.
    fn contains(&self, _other: impl Into<Self::Item>) -> bool {
        todo!()
    }

    /// Returns the closest intersection point with a given line and distance along the line or
    /// `None` if there is no intersection.
    fn intersects_line(&self, _line: impl Into<Line<f64>>) -> Option<(Point<f64>, f64)> {
        todo!()
    }

    /// Returns whether this circle intersects with another circle.
    fn intersects(&self, other: impl Into<Self::Item>) -> bool {
        let other = other.into();
        let px = self.x - other.x;
        let py = self.y - other.y;
        let r = self.radius + other.radius;
        (px * px + py * py) <= r * r
    }
}

impl<T> Draw for Circle<T>
where
    Circle<T>: Copy + Into<Circle<f64>>,
{
    /// Draw circle to the current [`PixState`] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.circle(*self)
    }
}

/// Convert `[x, y, radius]` to [`Circle<T>`].
impl<T, U: Into<T>> From<[U; 3]> for Circle<T> {
    fn from([x, y, radius]: [U; 3]) -> Self {
        Self::new(x.into(), y.into(), radius.into())
    }
}

/// Convert `&[x, y, radius]` to [`Circle<T>`].
impl<T, U: Copy + Into<T>> From<&[U; 3]> for Circle<T> {
    fn from(&[x, y, radius]: &[U; 3]) -> Self {
        Self::new(x.into(), y.into(), radius.into())
    }
}

/// Convert [`Circle<T>`] to `[x, y, radius]`.
impl<T, U: Into<T>> From<Circle<U>> for [T; 3] {
    fn from(c: Circle<U>) -> Self {
        [c.x.into(), c.y.into(), c.radius.into()]
    }
}

/// Convert [`&Circle<T>`] to `[x, y, radius]`.
impl<T, U: Copy + Into<T>> From<&Circle<U>> for [T; 3] {
    fn from(c: &Circle<U>) -> Self {
        [c.x.into(), c.y.into(), c.radius.into()]
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
/// let s = sphere!([10, 20, 10], 100);
/// assert_eq!(s.center, point!(10, 20, 10));
/// assert_eq!(s.radius, 100);
/// ```
#[macro_export]
macro_rules! sphere {
    ($p:expr, $r:expr$(,)?) => {
        $crate::shape::ellipse::Sphere::new($p, $r)
    };
    ([$x:expr, $y:expr, $z:expr], $r:expr$(,)?) => {
        $crate::shape::ellipse::Sphere::new([$x, $y, $z], $r)
    };
}

impl<T> Sphere<T> {
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
}

impl<T: ShapeNum> Shape<T> for Sphere<T> {
    type Item = Sphere<T>;

    /// Returns whether this sphere contains a given [`Point<T>`].
    fn contains_point(&self, _p: impl Into<Point<T>>) -> bool {
        todo!()
    }

    /// Returns whether this sphere contains a given [`Point<T>`].
    fn contains(&self, s: impl Into<Self::Item>) -> bool {
        let s = s.into();
        let px = s.center.x - self.center.x;
        let py = s.center.y - self.center.y;
        let pz = s.center.z - self.center.z;
        let r = self.radius;
        (px * px + py * py + pz * pz) < r * r
    }

    /// Returns the closest intersection point with a given line and distance along the line or
    /// `None` if there is no intersection.
    fn intersects_line(&self, _line: impl Into<Line<f64>>) -> Option<(Point<f64>, f64)> {
        todo!()
    }

    /// Returns whether this sphere intersects another sphere.
    fn intersects(&self, other: impl Into<Self::Item>) -> bool {
        let other = other.into();
        let px = other.center.x - self.center.x;
        let py = other.center.y - self.center.y;
        let pz = other.center.z - self.center.z;
        let r = other.radius + self.radius;
        (px * px + py * py + pz * pz) < r * r
    }
}
