//! [Circle], [Ellipse], and [Sphere] types used for drawing.

use crate::prelude::*;
use num_traits::{AsPrimitive, Num};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An `Ellipse` positioned at `(x, y)`, with `width` and `height`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ellipse<T = Scalar> {
    /// Center x-coord
    pub x: T,
    /// Center y-coord
    pub y: T,
    /// Width
    pub width: T,
    /// Height
    pub height: T,
}

/// # Constructs an [Ellipse].
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

    /// Returns `Ellipse` as a [Vec].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let e = ellipse!(5, 10, 100, 100);
    /// assert_eq!(e.to_vec(), vec![5, 10, 100, 100]);
    /// ```
    pub fn to_vec(self) -> Vec<T>
    where
        T: Copy,
    {
        vec![self.x, self.y, self.width, self.height]
    }

    /// Convert `Ellipse<T>` to another primitive type using the `as` operator.
    #[inline]
    pub fn as_<U>(self) -> Ellipse<U>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy,
    {
        Ellipse::new(
            self.x.as_(),
            self.y.as_(),
            self.width.as_(),
            self.height.as_(),
        )
    }
}

impl<T> Shape<T> for Ellipse<T>
where
    T: Num + Copy + PartialOrd,
{
    type Item = Ellipse<T>;

    /// Returns whether this ellipse contains a given [Point].
    fn contains_point<P>(&self, p: P) -> bool
    where
        P: Into<Point<T>>,
    {
        let p = p.into();
        let px = p.x - self.x;
        let py = p.y - self.y;
        let rx = self.width;
        let ry = self.height;
        (px * px) / (rx * rx) + (py * py) / (ry * ry) <= T::one()
    }

    /// Returns whether this ellipse intersects with another ellipse.
    fn intersects<O>(&self, other: O) -> bool
    where
        O: Into<Self::Item>,
    {
        let other = other.into();
        let px = self.x - other.x;
        let py = self.y - other.y;
        let rx = self.width + other.width;
        let ry = self.height + other.height;
        (px * px) / (rx * rx) + (py * py) / (ry * ry) <= T::one()
    }
}

impl<T> Draw for Ellipse<T>
where
    T: Copy,
    Self: Into<Ellipse<Scalar>>,
{
    /// Draw ellipse to the current [PixState] canvas.
    #[inline]
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.ellipse(*self)
    }
}

/// Convert [Ellipse] to `[x, y, width, height]`.
impl<T> From<Ellipse<T>> for [T; 4] {
    fn from(e: Ellipse<T>) -> Self {
        [e.x, e.y, e.width, e.height]
    }
}

/// Convert `&Ellipse<T>` to `[x, y, width, height]`.
impl<T: Copy> From<&Ellipse<T>> for [T; 4] {
    fn from(e: &Ellipse<T>) -> Self {
        [e.x, e.y, e.width, e.height]
    }
}

/// Convert `[x, y, width, height]` to [Ellipse].
impl<T, U: Into<T>> From<[U; 4]> for Ellipse<T> {
    fn from([x, y, width, height]: [U; 4]) -> Self {
        Self::new(x.into(), y.into(), width.into(), height.into())
    }
}

/// Convert `&[x, y, width, height]` to [Ellipse].
impl<T, U: Copy + Into<T>> From<&[U; 4]> for Ellipse<T> {
    fn from(&[x, y, width, height]: &[U; 4]) -> Self {
        Self::new(x.into(), y.into(), width.into(), height.into())
    }
}

/// Convert [Circle] to [Ellipse].
impl<T: Copy> From<Circle<T>> for Ellipse<T> {
    fn from(c: Circle<T>) -> Self {
        Self::new(c.x, c.y, c.radius, c.radius)
    }
}

/// Convert &[Circle] to [Ellipse].
impl<T: Copy> From<&Circle<T>> for Ellipse<T> {
    fn from(c: &Circle<T>) -> Self {
        Self::new(c.x, c.y, c.radius, c.radius)
    }
}

/// A `Circle` positioned at `(x, y)` with `radius`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Circle<T = Scalar> {
    /// Center x-coord
    pub x: T,
    /// Center y-coord
    pub y: T,
    /// Radius
    pub radius: T,
}

/// # Constructs a [Circle].
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

    /// Constructs a `Circle` from [Point].
    pub fn from_point(p: Point<T>, radius: T) -> Self {
        Self::new(p.x, p.y, radius)
    }

    /// Constructs a `Circle` from [Vector].
    pub fn from_vector(v: Vector<T>, radius: T) -> Self {
        Self::new(v.x, v.y, radius)
    }

    /// Returns `Circle` as a [Vec].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = circle!(5, 10, 100);
    /// assert_eq!(c.to_vec(), vec![5, 10, 100]);
    /// ```
    pub fn to_vec(self) -> Vec<T>
    where
        T: Copy,
    {
        vec![self.x, self.y, self.radius]
    }

    /// Convert `Circle<T>` to another primitive type using the `as` operator.
    #[inline]
    pub fn as_<U>(self) -> Circle<U>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy,
    {
        Circle::new(self.x.as_(), self.y.as_(), self.radius.as_())
    }
}

impl<T> Shape<T> for Circle<T>
where
    T: Num + Copy + PartialOrd,
{
    type Item = Circle<T>;

    /// Returns whether this circle contains a given [Point].
    fn contains_point<P>(&self, p: P) -> bool
    where
        P: Into<Point<T>>,
    {
        let p = p.into();
        let px = p.x - self.x;
        let py = p.y - self.y;
        let r = self.radius * self.radius;
        (px * px + py * py) < r
    }

    /// Returns whether this circle intersects with another circle.
    fn intersects<O>(&self, other: O) -> bool
    where
        O: Into<Self::Item>,
    {
        let other = other.into();
        let px = self.x - other.x;
        let py = self.y - other.y;
        let r = self.radius + other.radius;
        (px * px + py * py) <= r * r
    }
}

impl<T> Draw for Circle<T>
where
    T: Copy,
    Self: Into<Circle<Scalar>>,
{
    /// Draw circle to the current [PixState] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.circle(*self)
    }
}

/// Convert [Circle] to `[x, y, radius]`.
impl<T> From<Circle<T>> for [T; 3] {
    fn from(c: Circle<T>) -> Self {
        [c.x, c.y, c.radius]
    }
}

/// Convert &[Circle] to `[x, y, radius]`.
impl<T: Copy> From<&Circle<T>> for [T; 3] {
    fn from(c: &Circle<T>) -> Self {
        [c.x, c.y, c.radius]
    }
}

/// Convert `[x, y, radius]` to [Circle].
impl<T, U: Into<T>> From<[U; 3]> for Circle<T> {
    fn from([x, y, radius]: [U; 3]) -> Self {
        Self::new(x.into(), y.into(), radius.into())
    }
}

/// Convert `&[x, y, radius]` to [Circle].
impl<T, U: Copy + Into<T>> From<&[U; 3]> for Circle<T> {
    fn from(&[x, y, radius]: &[U; 3]) -> Self {
        Self::new(x.into(), y.into(), radius.into())
    }
}

/// A `Sphere` positioned at `(x, y, z)` with `radius`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Sphere<T = Scalar> {
    /// Center position
    pub center: Point<T>,
    /// Radius
    pub radius: T,
}

/// # Constructs a [Sphere].
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

impl<T> Shape<T> for Sphere<T>
where
    T: Num + Copy + PartialOrd,
{
    type Item = Sphere<T>;

    /// Returns whether this sphere contains a given [Point].
    fn contains<O>(&self, other: O) -> bool
    where
        O: Into<Self::Item>,
    {
        let other = other.into();
        let px = other.center.x - self.center.x;
        let py = other.center.y - self.center.y;
        let pz = other.center.z - self.center.z;
        let r = self.radius;
        (px * px + py * py + pz * pz) < r * r
    }

    /// Returns whether this sphere intersects another sphere.
    fn intersects<O>(&self, other: O) -> bool
    where
        O: Into<Self::Item>,
    {
        let other = other.into();
        let px = other.center.x - self.center.x;
        let py = other.center.y - self.center.y;
        let pz = other.center.z - self.center.z;
        let r = other.radius + self.radius;
        (px * px + py * py + pz * pz) < r * r
    }
}
