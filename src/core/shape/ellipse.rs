//! [Circle], [Ellipse] types used for drawing.

use crate::prelude::*;
use num_traits::{AsPrimitive, Float};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// # Constructs an [Ellipse].
///
/// ```
/// # use pix_engine::prelude::*;
/// let p = point!(10, 20);
/// let e = ellipse!(p, 100, 200);
/// assert_eq!(e.x, 10);
/// assert_eq!(e.y, 20);
/// assert_eq!(e.width, 100);
/// assert_eq!(e.height, 200);
///
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
        $crate::prelude::Ellipse::with_point($p, $width, $height)
    };
    ($x:expr, $y:expr, $width:expr, $height:expr$(,)?) => {
        $crate::prelude::Ellipse::new($x, $y, $width, $height)
    };
}

/// # Constructs a [Circle].
///
/// ```
/// # use pix_engine::prelude::*;
/// let p = point!(10, 20);
/// let c = circle!(p, 100);
/// assert_eq!(c.x, 10);
/// assert_eq!(c.y, 20);
/// assert_eq!(c.radius, 100);
///
/// let c = circle!(10, 20, 100);
/// assert_eq!(c.x, 10);
/// assert_eq!(c.y, 20);
/// assert_eq!(c.radius, 100);
/// ```
#[macro_export]
macro_rules! circle {
    ($p:expr, $r:expr$(,)?) => {
        $crate::prelude::Circle::with_point($p, $r)
    };
    ($x:expr, $y:expr, $r:expr$(,)?) => {
        $crate::prelude::Circle::new($x, $y, $r)
    };
}

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

impl<T> Ellipse<T> {
    /// Constructs an `Ellipse<T>` at position `(x, y)` with `width` and `height`.
    pub const fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Constructs an `Ellipse<T>` at position [Point] with `width` and `height`.
    pub fn with_point<P: Into<Point<T>>>(p: P, width: T, height: T) -> Self {
        let p = p.into();
        Self::new(p.x, p.y, width, height)
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

impl<T: Number> Ellipse<T> {
    /// Returns `Ellipse` values as `[x, y, width, height]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let e = ellipse!(5, 10, 100, 100);
    /// assert_eq!(e.values(), [5, 10, 100, 100]);
    /// ```
    pub fn values(&self) -> [T; 4] {
        [self.x, self.y, self.width, self.height]
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
    pub fn to_vec(self) -> Vec<T> {
        vec![self.x, self.y, self.width, self.height]
    }
}

impl<T: Float> Ellipse<T> {
    /// Returns `Ellipse` with values rounded to the nearest integer number. Round half-way cases
    /// away from `0.0`.
    pub fn round(&self) -> Self {
        Self::new(
            self.x.round(),
            self.y.round(),
            self.width.round(),
            self.height.round(),
        )
    }
}

impl<T: Number> Shape<T> for Ellipse<T> {
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
    T: Number,
    Self: Into<Ellipse>,
{
    /// Draw ellipse to the current [PixState] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.ellipse(*self)
    }
}

impl<T: Number> From<&mut Ellipse<T>> for Ellipse<T> {
    fn from(ellipse: &mut Ellipse<T>) -> Self {
        ellipse.to_owned()
    }
}

impl<T: Number> From<&Ellipse<T>> for Ellipse<T> {
    fn from(ellipse: &Ellipse<T>) -> Self {
        *ellipse
    }
}

/// Convert [Ellipse] to `[x, y, width, height]`.
impl<T: Number> From<Ellipse<T>> for [T; 4] {
    fn from(e: Ellipse<T>) -> Self {
        [e.x, e.y, e.width, e.height]
    }
}

/// Convert `&Ellipse<T>` to `[x, y, width, height]`.
impl<T: Number> From<&Ellipse<T>> for [T; 4] {
    fn from(e: &Ellipse<T>) -> Self {
        [e.x, e.y, e.width, e.height]
    }
}

/// Convert `[x, y, width, height]` to [Ellipse].
impl<T: Number, U: Into<T>> From<[U; 4]> for Ellipse<T> {
    fn from([x, y, width, height]: [U; 4]) -> Self {
        Self::new(x.into(), y.into(), width.into(), height.into())
    }
}

/// Convert `&[x, y, width, height]` to [Ellipse].
impl<T: Number, U: Copy + Into<T>> From<&[U; 4]> for Ellipse<T> {
    fn from(&[x, y, width, height]: &[U; 4]) -> Self {
        Self::new(x.into(), y.into(), width.into(), height.into())
    }
}

/// Convert [Circle] to [Ellipse].
impl<T: Number> From<Circle<T>> for Ellipse<T> {
    fn from(c: Circle<T>) -> Self {
        Self::new(c.x, c.y, c.radius, c.radius)
    }
}

/// Convert &[Circle] to [Ellipse].
impl<T: Number> From<&Circle<T>> for Ellipse<T> {
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

impl<T> Circle<T> {
    /// Constructs a `Circle<T>` at position `(x, y)` with `radius`.
    pub const fn new(x: T, y: T, radius: T) -> Self {
        Self { x, y, radius }
    }

    /// Constructs a `Circle<T>` at position [Point] with `radius`.
    pub fn with_point<P: Into<Point<T>>>(p: P, radius: T) -> Self {
        let p = p.into();
        Self::new(p.x, p.y, radius)
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

impl<T: Number> Circle<T> {
    /// Returns `Circle` values as `[x, y, radius]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = circle!(5, 10, 100);
    /// assert_eq!(c.values(), [5, 10, 100]);
    /// ```
    pub fn values(&self) -> [T; 3] {
        [self.x, self.y, self.radius]
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
    pub fn to_vec(self) -> Vec<T> {
        vec![self.x, self.y, self.radius]
    }
}

impl<T: Float> Circle<T> {
    /// Returns `Circle` with values rounded to the nearest integer number. Round half-way cases
    /// away from `0.0`.
    pub fn round(&self) -> Self {
        Self::new(self.x.round(), self.y.round(), self.radius.round())
    }
}

impl<T: Number> Shape<T> for Circle<T> {
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
    T: Number,
    Self: Into<Circle>,
{
    /// Draw circle to the current [PixState] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.circle(*self)
    }
}

impl<T: Number> From<&mut Circle<T>> for Circle<T> {
    fn from(circle: &mut Circle<T>) -> Self {
        circle.to_owned()
    }
}

impl<T: Number> From<&Circle<T>> for Circle<T> {
    fn from(circle: &Circle<T>) -> Self {
        *circle
    }
}

/// Convert [Circle] to `[x, y, radius]`.
impl<T: Number> From<Circle<T>> for [T; 3] {
    fn from(c: Circle<T>) -> Self {
        [c.x, c.y, c.radius]
    }
}

/// Convert &[Circle] to `[x, y, radius]`.
impl<T: Number> From<&Circle<T>> for [T; 3] {
    fn from(c: &Circle<T>) -> Self {
        [c.x, c.y, c.radius]
    }
}

/// Convert `[x, y, radius]` to [Circle].
impl<T: Number, U: Into<T>> From<[U; 3]> for Circle<T> {
    fn from([x, y, radius]: [U; 3]) -> Self {
        Self::new(x.into(), y.into(), radius.into())
    }
}

/// Convert `&[x, y, radius]` to [Circle].
impl<T: Number, U: Copy + Into<T>> From<&[U; 3]> for Circle<T> {
    fn from(&[x, y, radius]: &[U; 3]) -> Self {
        Self::new(x.into(), y.into(), radius.into())
    }
}
