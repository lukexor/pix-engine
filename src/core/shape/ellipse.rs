//! 2D shape types representing circles and ellipses used for drawing.
//!
//! # Examples
//!
//! You can create an [Ellipse] or [Circle] using [Ellipse::new] or [Circle::new]:
//!
//! ```
//! # use pix_engine::prelude::*;
//! let e = Ellipse::new(10, 20, 100, 200);
//! let c = Circle::new(10, 20, 100);
//! ```
//!
//! ...or by using the [ellipse!] or [circle!] macros:
//!
//! ```
//! # use pix_engine::prelude::*;
//! let e = ellipse!(10, 20, 100, 200);
//! let c = circle!(10, 20, 100);
//!
//! // using a point
//! let e = ellipse!([10, 20], 100, 200);
//! let e = ellipse!(point![10, 20], 100, 200);
//! let c = circle!([10, 20], 100);
//! let c = circle!(point![10, 20], 100);
//! ```

use crate::prelude::*;
use num_traits::{AsPrimitive, Float};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// Constructs an `Ellipse<T>` at position `(x, y)` with `width` and `height`.
///
/// ```
/// # use pix_engine::prelude::*;
/// let p = point!(10, 20);
/// let e = ellipse!(p, 100, 200);
/// assert_eq!(e.x(), 10);
/// assert_eq!(e.y(), 20);
/// assert_eq!(e.width(), 100);
/// assert_eq!(e.height(), 200);
///
/// let e = ellipse!(10, 20, 100, 200);
/// assert_eq!(e.x(), 10);
/// assert_eq!(e.y(), 20);
/// assert_eq!(e.width(), 100);
/// assert_eq!(e.height(), 200);
/// ```
#[macro_export]
macro_rules! ellipse {
    ($p:expr, $r:expr$(,)?) => {
        ellipse!($p, $r, $r)
    };
    ($p:expr, $width:expr, $height:expr$(,)?) => {
        $crate::prelude::Ellipse::with_position($p, $width, $height)
    };
    ($x:expr, $y:expr, $width:expr, $height:expr$(,)?) => {
        $crate::prelude::Ellipse::new($x, $y, $width, $height)
    };
}

/// # Constructs a `Circle<T>` at position `(x, y`) with `radius`.
///
/// ```
/// # use pix_engine::prelude::*;
/// let p = point!(10, 20);
/// let c = circle!(p, 100);
/// assert_eq!(c.x(), 10);
/// assert_eq!(c.y(), 20);
/// assert_eq!(c.radius(), 100);
///
/// let c = circle!(10, 20, 100);
/// assert_eq!(c.x(), 10);
/// assert_eq!(c.y(), 20);
/// assert_eq!(c.radius(), 100);
/// ```
#[macro_export]
macro_rules! circle {
    ($p:expr, $r:expr$(,)?) => {
        $crate::prelude::Circle::with_position($p, $r)
    };
    ($x:expr, $y:expr, $r:expr$(,)?) => {
        $crate::prelude::Circle::new($x, $y, $r)
    };
}

/// An `Ellipse` positioned at `(x, y)`, with `width` and `height`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ellipse<T = Scalar>([T; 4]);

impl<T> Ellipse<T> {
    /// Constructs an `Ellipse<T>` at position `(x, y)` with `width` and `height`.
    pub const fn new(x: T, y: T, width: T, height: T) -> Self {
        Self([x, y, width, height])
    }
}

impl<T: Number> Ellipse<T> {
    /// Constructs an `Ellipse<T>` at position [Point] with `width` and `height`.
    pub fn with_position<P: Into<Point<T>>>(p: P, width: T, height: T) -> Self {
        let p = p.into();
        Self::new(p.x(), p.y(), width, height)
    }

    /// Constructs a `Ellipse<T>` centered at position `(x, y)` with `width` and `height`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let e = Ellipse::from_center([50, 50], 100, 100);
    /// assert_eq!(e.values(), [0, 0, 100, 100]);
    /// ```
    pub fn from_center<P: Into<Point<T>>>(p: P, width: T, height: T) -> Self {
        let p = p.into();
        let two = T::one() + T::one();
        Self::new(p.x() - width / two, p.y() - height / two, width, height)
    }

    /// Returns the `x-coordinate` of the ellipse.
    #[inline(always)]
    pub fn x(&self) -> T {
        self.0[0]
    }

    /// Sets the `x-coordinate` of the ellipse.
    #[inline(always)]
    pub fn set_x(&mut self, x: T) {
        self.0[0] = x;
    }

    /// Returns the `y-coordinate` of the ellipse.
    #[inline(always)]
    pub fn y(&self) -> T {
        self.0[1]
    }

    /// Sets the `y-coordinate` of the ellipse.
    #[inline(always)]
    pub fn set_y(&mut self, y: T) {
        self.0[1] = y;
    }

    /// Returns the `width` of the ellipse.
    #[inline(always)]
    pub fn width(&self) -> T {
        self.0[2]
    }

    /// Sets the `width` of the ellipse.
    #[inline(always)]
    pub fn set_width(&mut self, width: T) {
        self.0[2] = width;
    }

    /// Returns the `height` of the ellipse.
    #[inline(always)]
    pub fn height(&self) -> T {
        self.0[3]
    }

    /// Sets the `height` of the ellipse.
    #[inline(always)]
    pub fn set_height(&mut self, height: T) {
        self.0[3] = height;
    }

    /// Convert `Ellipse<T>` to another primitive type using the `as` operator.
    #[inline]
    pub fn as_<U>(self) -> Ellipse<U>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy,
    {
        Ellipse::new(
            self.x().as_(),
            self.y().as_(),
            self.width().as_(),
            self.height().as_(),
        )
    }

    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let e = ellipse!(5, 10, 100, 100);
    /// assert_eq!(e.values(), [5, 10, 100, 100]);
    /// ```
    pub fn values(&self) -> [T; 4] {
        self.0
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
        self.0.to_vec()
    }

    /// Returns the horizontal position of the left edge.
    pub fn left(&self) -> T {
        self.x()
    }

    /// Returns the horizontal position of the right edge.
    pub fn right(&self) -> T {
        self.x() + self.width()
    }

    /// Returns the horizontal position of the top edge.
    pub fn top(&self) -> T {
        self.y()
    }

    /// Returns the vertical position of the bottom edge.
    pub fn bottom(&self) -> T {
        self.y() + self.height()
    }

    /// Set the horizontal position of the left edge.
    pub fn set_left(&mut self, left: T) {
        self.set_x(left);
    }

    /// Set the horizontal position of the right edge.
    pub fn set_right(&mut self, right: T) {
        self.set_x(right - self.width());
    }

    /// Set the vertical position of the top edge.
    pub fn set_top(&mut self, top: T) {
        self.set_y(top);
    }

    /// Set the vertical position of the bottom edge.
    pub fn set_bottom(&mut self, bottom: T) {
        self.set_y(bottom - self.height());
    }

    /// Returns the center position as [Point].
    pub fn center(&self) -> Point<T> {
        let two = T::one() + T::one();
        point!(
            self.x() + self.width() / two,
            self.y() + self.height() / two
        )
    }

    /// Returns the top-left position as [Point].
    pub fn top_left(&self) -> Point<T> {
        point!(self.left(), self.top())
    }

    /// Returns the top-right position as [Point].
    pub fn top_right(&self) -> Point<T> {
        point!(self.right(), self.top())
    }

    /// Returns the bottom-left position as [Point].
    pub fn bottom_left(&self) -> Point<T> {
        point!(self.left(), self.bottom())
    }

    /// Returns the bottom-right position as [Point].
    pub fn bottom_right(&self) -> Point<T> {
        point!(self.right(), self.bottom())
    }

    /// Set position centered on a [Point].
    pub fn center_on<P: Into<Point<T>>>(&mut self, p: P) {
        let p = p.into();
        let two = T::one() + T::one();
        self.set_x(p.x() - self.width() / two);
        self.set_y(p.y() - self.height() / two);
    }
}

impl<T: Float> Ellipse<T> {
    /// Returns `Ellipse` with values rounded to the nearest integer number. Round half-way cases
    /// away from `0.0`.
    pub fn round(&self) -> Self {
        Self::new(
            self.x().round(),
            self.y().round(),
            self.width().round(),
            self.height().round(),
        )
    }
}

impl<T: Number> Shape<T> for Ellipse<T> {
    type Item = Ellipse<T>;

    /// Returns whether this ellipse contains a given [Point].
    fn contains_point<P: Into<Point<T>>>(&self, p: P) -> bool {
        let p = p.into();
        let px = p.x() - self.x();
        let py = p.y() - self.y();
        let rx = self.width();
        let ry = self.height();
        (px * px) / (rx * rx) + (py * py) / (ry * ry) <= T::one()
    }

    /// Returns whether this ellipse intersects with another ellipse.
    fn contains<O: Into<Self::Item>>(&self, other: O) -> bool {
        let other = other.into();
        let px = self.x() - other.x();
        let py = self.y() - other.y();
        let rx = self.width() + other.width();
        let ry = self.height() + other.height();
        (px * px) / (rx * rx) + (py * py) / (ry * ry) <= T::one()
    }
}

impl<T> Draw for Ellipse<T>
where
    T: Number,
    Self: Into<Ellipse>,
{
    /// Draw `Ellipse` to the current [PixState] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.ellipse(*self)
    }
}

impl<T> Deref for Ellipse<T> {
    type Target = [T; 4];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Ellipse<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Number> From<&mut Ellipse<T>> for Ellipse<T> {
    /// Convert `&mut Ellipse<T>` to [Ellipse].
    fn from(ellipse: &mut Ellipse<T>) -> Self {
        *ellipse
    }
}

impl<T: Number> From<&Ellipse<T>> for Ellipse<T> {
    /// Convert `&Ellipse<T>` to [Ellipse].
    fn from(ellipse: &Ellipse<T>) -> Self {
        *ellipse
    }
}

impl<T: Number, U: Number + Into<T>> From<[U; 4]> for Ellipse<T> {
    /// Convert `[x, y, width, height]` to [Ellipse].
    fn from([x, y, width, height]: [U; 4]) -> Self {
        Self::new(x.into(), y.into(), width.into(), height.into())
    }
}

impl<T: Number, U: Number + Into<T>> From<&[U; 4]> for Ellipse<T> {
    /// Convert `&[x, y, width, height]` to [Ellipse].
    fn from(&[x, y, width, height]: &[U; 4]) -> Self {
        Self::new(x.into(), y.into(), width.into(), height.into())
    }
}

impl<T: Number> From<Circle<T>> for Ellipse<T> {
    /// Convert [Circle] to [Ellipse].
    fn from(c: Circle<T>) -> Self {
        Self::new(c.x(), c.y(), c.radius(), c.radius())
    }
}

/// Convert &[Circle] to [Ellipse].
impl<T: Number> From<&Circle<T>> for Ellipse<T> {
    fn from(c: &Circle<T>) -> Self {
        Self::new(c.x(), c.y(), c.radius(), c.radius())
    }
}

/// A `Circle` positioned at `(x, y)` with `radius`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Circle<T = Scalar>([T; 3]);

impl<T> Circle<T> {
    /// Constructs a `Circle<T>` at position `(x, y)` with `radius`.
    pub const fn new(x: T, y: T, radius: T) -> Self {
        Self([x, y, radius])
    }
}

impl<T: Number> Circle<T> {
    /// Constructs a `Circle<T>` at position [Point] with `radius`.
    pub fn with_position<P: Into<Point<T>>>(p: P, radius: T) -> Self {
        let p = p.into();
        Self::new(p.x(), p.y(), radius)
    }

    /// Constructs a `Circle<T>` centered at position `(x, y)` with `radius`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Circle::from_center([50, 50], 100);
    /// assert_eq!(c.values(), [0, 0, 100]);
    /// ```
    pub fn from_center<P: Into<Point<T>>>(p: P, radius: T) -> Self {
        let p = p.into();
        let two = T::one() + T::one();
        Self::new(p.x() - radius / two, p.y() - radius / two, radius)
    }

    /// Returns the `x-coordinate` of the circle.
    #[inline(always)]
    pub fn x(&self) -> T {
        self.0[0]
    }

    /// Sets the `x-coordinate` of the circle.
    #[inline(always)]
    pub fn set_x(&mut self, x: T) {
        self.0[0] = x;
    }

    /// Returns the `y-coordinate` of the circle.
    #[inline(always)]
    pub fn y(&self) -> T {
        self.0[1]
    }

    /// Sets the `y-coordinate` of the circle.
    #[inline(always)]
    pub fn set_y(&mut self, y: T) {
        self.0[1] = y;
    }

    /// Returns the `radius` of the circle.
    #[inline(always)]
    pub fn radius(&self) -> T {
        self.0[2]
    }

    /// Sets the `radius` of the circle.
    #[inline(always)]
    pub fn set_radius(&mut self, radius: T) {
        self.0[2] = radius;
    }

    /// Convert `Circle<T>` to another primitive type using the `as` operator.
    #[inline]
    pub fn as_<U>(self) -> Circle<U>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy,
    {
        Circle::new(self.x().as_(), self.y().as_(), self.radius().as_())
    }

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
        self.0
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
        self.0.to_vec()
    }

    /// Returns the horizontal position of the left edge.
    pub fn left(&self) -> T {
        self.x()
    }

    /// Returns the horizontal position of the right edge.
    pub fn right(&self) -> T {
        self.x() + self.radius()
    }

    /// Returns the horizontal position of the top edge.
    pub fn top(&self) -> T {
        self.y()
    }

    /// Returns the vertical position of the bottom edge.
    pub fn bottom(&self) -> T {
        self.y() + self.radius()
    }

    /// Set the horizontal position of the left edge.
    pub fn set_left(&mut self, left: T) {
        self.set_x(left);
    }

    /// Set the horizontal position of the right edge.
    pub fn set_right(&mut self, right: T) {
        self.set_x(right - self.radius());
    }

    /// Set the vertical position of the top edge.
    pub fn set_top(&mut self, top: T) {
        self.set_y(top);
    }

    /// Set the vertical position of the bottom edge.
    pub fn set_bottom(&mut self, bottom: T) {
        self.set_y(bottom - self.radius());
    }

    /// Returns the center position as [Point].
    pub fn center(&self) -> Point<T> {
        point!(self.x() + self.radius(), self.y() + self.radius())
    }

    /// Returns the top-left position as [Point].
    pub fn top_left(&self) -> Point<T> {
        point!(self.left(), self.top())
    }

    /// Returns the top-right position as [Point].
    pub fn top_right(&self) -> Point<T> {
        point!(self.right(), self.top())
    }

    /// Returns the bottom-left position as [Point].
    pub fn bottom_left(&self) -> Point<T> {
        point!(self.left(), self.bottom())
    }

    /// Returns the bottom-right position as [Point].
    pub fn bottom_right(&self) -> Point<T> {
        point!(self.right(), self.bottom())
    }

    /// Set position centered on a [Point].
    pub fn center_on<P: Into<Point<T>>>(&mut self, p: P) {
        let p = p.into();
        self.set_x(p.x() - self.radius());
        self.set_y(p.y() - self.radius());
    }
}

impl<T: Float> Circle<T> {
    /// Returns `Circle` with values rounded to the nearest integer number. Round half-way cases
    /// away from `0.0`.
    pub fn round(&self) -> Self {
        Self::new(self.x().round(), self.y().round(), self.radius().round())
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
        let px = p.x() - self.x();
        let py = p.y() - self.y();
        let r = self.radius() * self.radius();
        (px * px + py * py) < r
    }

    /// Returns whether this circle intersects with another circle.
    fn intersects<O>(&self, other: O) -> bool
    where
        O: Into<Self::Item>,
    {
        let other = other.into();
        let px = self.x() - other.x();
        let py = self.y() - other.y();
        let r = self.radius() + other.radius();
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

impl<T> Deref for Circle<T> {
    type Target = [T; 3];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Circle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Number> From<&mut Circle<T>> for Circle<T> {
    fn from(circle: &mut Circle<T>) -> Self {
        *circle
    }
}

impl<T: Number> From<&Circle<T>> for Circle<T> {
    fn from(circle: &Circle<T>) -> Self {
        *circle
    }
}

/// Convert `[x, y, radius]` to [Circle].
impl<T: Number, U: Number + Into<T>> From<[U; 3]> for Circle<T> {
    fn from([x, y, radius]: [U; 3]) -> Self {
        Self::new(x.into(), y.into(), radius.into())
    }
}

/// Convert `&[x, y, radius]` to [Circle].
impl<T: Number, U: Number + Into<T>> From<&[U; 3]> for Circle<T> {
    fn from(&[x, y, radius]: &[U; 3]) -> Self {
        Self::new(x.into(), y.into(), radius.into())
    }
}
