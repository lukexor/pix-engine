//! 2D shape type representing circles used for drawing.
//!
//! # Examples
//!
//! You can create a [Circle] using [Circle::new]:
//!
//! ```
//! # use pix_engine::prelude::*;
//! let c = Circle::new(10, 20, 100);
//! ```
//!
//! ...or by using the [circle!] macro:
//!
//! ```
//! # use pix_engine::prelude::*;
//! let c = circle!(10, 20, 100);
//!
//! // using a point
//! let c = circle!([10, 20], 100);
//! let c = circle!(point![10, 20], 100);
//! ```

use crate::prelude::*;
use num_traits::AsPrimitive;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

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

/// A `Circle` positioned at `(x, y)` with `radius`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Circle<T = i32>([T; 3]);

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

impl<T: Number> Contains for Circle<T> {
    type Type = T;
    type Shape = Circle<Self::Type>;

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

    /// Returns whether this circle completely contains another circle.
    fn contains_shape<O>(&self, other: O) -> bool
    where
        O: Into<Self::Shape>,
    {
        let other = other.into();
        let px = other.x() - self.x();
        let py = other.y() - self.y();
        let r = self.radius() * self.radius();
        (px * px + py * py) < r
    }
}

impl<T: Number> Intersects for Circle<T> {
    type Type = T;
    type Shape = Circle<Self::Type>;

    /// Returns the closest intersection point with a given line and distance along the line or
    /// `None` if there is no intersection.
    fn intersects_line<L>(&self, _line: L) -> Option<(Point<f64>, f64)>
    where
        L: Into<Line<Self::Type>>,
    {
        todo!("circle intersects_line")
    }

    /// Returns whether this circle intersects with another circle.
    fn intersects_shape<O>(&self, other: O) -> bool
    where
        O: Into<Self::Shape>,
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
    Self: Into<Circle<i32>>,
    T: Number,
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
