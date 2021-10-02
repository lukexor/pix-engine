//! A shape type representing circles and ellipses used for drawing.

//!
//! # Examples
//!
//! You can create an [Ellipse] or circle using [Ellipse::new]::new] or [Ellipse::circle]:
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! let e = Ellipse::new(10, 20, 100, 200);
//! let c = Ellipse::circle(10, 20, 100);
//! ```
//!
//! ...or by using the [ellipse!] [circle!] macros:
//!
//! ```
//! use pix_engine::prelude::*;
//!
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
use num_traits::AsPrimitive;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An `Ellipse` positioned at `(x, y)`, with `width` and `height`. A circle is an `Ellipse` where
/// `width` and `height` are equal.
///
/// Please see the [module-level documentation] for examples.
///
/// [module-level documentation]: crate::core::shape::ellipse
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ellipse<T = i32>(pub(crate) [T; 4]);

/// Constructs an [Ellipse] at position `(x, y)` with `width` and `height`.
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

/// Constructs a circle [Ellipse] at position `(x, y`) with `radius`.
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
        $crate::prelude::Ellipse::circle_with_position($p, $r)
    };
    ($x:expr, $y:expr, $r:expr$(,)?) => {
        $crate::prelude::Ellipse::circle($x, $y, $r)
    };
}

impl<T> Ellipse<T> {
    /// Constructs an `Ellipse` at position `(x, y)` with `width` and `height`.
    pub const fn new(x: T, y: T, width: T, height: T) -> Self {
        Self([x, y, width, height])
    }

    /// Constructs a circle `Ellipse` at position `(x, y)` with `radius`.
    pub fn circle(x: T, y: T, radius: T) -> Self
    where
        T: Copy,
    {
        Self::new(x, y, radius, radius)
    }
}

impl<T: Copy + Default> Ellipse<T> {
    /// Returns `Ellipse` values as `[x, y, width, height]`.
    ///
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
}

impl<T: Num> Ellipse<T> {
    /// Constructs an `Ellipse` at position [Point] with `width` and `height`.
    pub fn with_position<P: Into<Point<T, 2>>>(p: P, width: T, height: T) -> Self {
        let p = p.into();
        Self::new(p.x(), p.y(), width, height)
    }

    /// Constructs a circle `Ellipse` at position [Point] with `radius`.
    pub fn circle_with_position<P: Into<Point<T, 2>>>(p: P, radius: T) -> Self {
        let p = p.into();
        Self::new(p.x(), p.y(), radius, radius)
    }

    /// Constructs an `Ellipse` centered at position `(x, y)` with `width` and `height`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let e = Ellipse::from_center([50, 50], 100, 100);
    /// assert_eq!(e.values(), [0, 0, 100, 100]);
    /// ```
    pub fn from_center<P: Into<Point<T, 2>>>(p: P, width: T, height: T) -> Self {
        let p = p.into();
        let two = T::one() + T::one();
        Self::new(p.x() - width / two, p.y() - height / two, width, height)
    }

    /// Constructs a circle `Ellipse` centered at position `(x, y)` with `radius`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Ellipse::circle_from_center([50, 50], 100);
    /// assert_eq!(c.values(), [0, 0, 100, 100]);
    /// ```
    pub fn circle_from_center<P: Into<Point<T, 2>>>(p: P, radius: T) -> Self {
        let p = p.into();
        let two = T::one() + T::one();
        let offset = radius / two;
        Self::new(p.x() - offset, p.y() - offset, radius, radius)
    }

    /// Returns the `x-coordinate` of the ellipse.
    #[inline]
    pub fn x(&self) -> T {
        self.0[0]
    }

    /// Sets the `x-coordinate` of the ellipse.
    #[inline]
    pub fn set_x(&mut self, x: T) {
        self.0[0] = x;
    }

    /// Returns the `y-coordinate` of the ellipse.
    #[inline]
    pub fn y(&self) -> T {
        self.0[1]
    }

    /// Sets the `y-coordinate` of the ellipse.
    #[inline]
    pub fn set_y(&mut self, y: T) {
        self.0[1] = y;
    }

    /// Returns the `width` of the ellipse.
    #[inline]
    pub fn width(&self) -> T {
        self.0[2]
    }

    /// Sets the `width` of the ellipse.
    #[inline]
    pub fn set_width(&mut self, width: T) {
        self.0[2] = width;
    }

    /// Returns the `height` of the ellipse.
    #[inline]
    pub fn height(&self) -> T {
        self.0[3]
    }

    /// Sets the `height` of the ellipse.
    #[inline]
    pub fn set_height(&mut self, height: T) {
        self.0[3] = height;
    }

    /// Returns the `radius` of the circle.
    #[inline]
    pub fn radius(&self) -> T {
        self.0[2]
    }

    /// Sets the `width` of the circle.
    #[inline]
    pub fn set_radius(&mut self, radius: T) {
        self.0[2] = radius;
        self.0[3] = radius;
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
    pub fn center(&self) -> Point<T, 2> {
        let two = T::one() + T::one();
        point!(
            self.x() + self.width() / two,
            self.y() + self.height() / two
        )
    }

    /// Returns the top-left position as [Point].
    pub fn top_left(&self) -> Point<T, 2> {
        point!(self.left(), self.top())
    }

    /// Returns the top-right position as [Point].
    pub fn top_right(&self) -> Point<T, 2> {
        point!(self.right(), self.top())
    }

    /// Returns the bottom-left position as [Point].
    pub fn bottom_left(&self) -> Point<T, 2> {
        point!(self.left(), self.bottom())
    }

    /// Returns the bottom-right position as [Point].
    pub fn bottom_right(&self) -> Point<T, 2> {
        point!(self.right(), self.bottom())
    }

    /// Set position centered on a [Point].
    pub fn center_on<P: Into<Point<T, 2>>>(&mut self, p: P) {
        let p = p.into();
        let two = T::one() + T::one();
        self.set_x(p.x() - self.width() / two);
        self.set_y(p.y() - self.height() / two);
    }
}

impl<T: Num, const N: usize> Contains<T, N> for Ellipse<T> {
    type Shape = Ellipse<T>;

    /// Returns whether this ellipse contains a given [Point].
    fn contains_point<P>(&self, p: P) -> bool
    where
        P: Into<Point<T, N>>,
    {
        let p = p.into();
        let px = p.x() - self.x();
        let py = p.y() - self.y();
        let rx = self.width();
        let ry = self.height();
        (px * px) / (rx * rx) + (py * py) / (ry * ry) <= T::one()
    }

    /// Returns whether this ellipse completely contains another ellipse.
    fn contains_shape<O>(&self, other: O) -> bool
    where
        O: Into<Self::Shape>,
    {
        let other = other.into();
        let px = self.x() - other.x();
        let py = self.y() - other.y();
        let rx = self.width() + other.width();
        let ry = self.height() + other.height();
        (px * px) / (rx * rx) + (py * py) / (ry * ry) <= T::one()
    }
}

impl<T: Num> Intersects<T, 2> for Ellipse<T> {
    type Shape = Ellipse<T>;

    /// Returns the closest intersection point with a given line and distance along the line or
    /// `None` if there is no intersection.
    fn intersects_line<L>(&self, _line: L) -> Option<(Point<T, 2>, T)>
    where
        L: Into<Line<T, 2>>,
    {
        todo!("ellipse intersects_line")
    }

    /// Returns whether this circle intersects with another circle.
    fn intersects_shape<O>(&self, other: O) -> bool
    where
        O: Into<Self::Shape>,
    {
        let other = other.into();
        let px = self.x() - other.x();
        let py = self.y() - other.y();
        if self.width() == self.height() {
            let r = self.width() + other.width();
            (px * px + py * py) <= r * r
        } else {
            todo!("ellipse intersects_shape")
        }
    }
}

impl<T> Draw for Ellipse<T>
where
    Self: Into<Ellipse<i32>>,
    T: Default + AsPrimitive<i32>,
{
    /// Draw `Ellipse` to the current [PixState] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.ellipse(*self)
    }
}

impl<T, U> From<[U; 3]> for Ellipse<T>
where
    T: 'static + Copy,
    U: AsPrimitive<T>,
{
    /// Converts `[U; 3]` into `Ellipse<T>`.
    #[inline]
    fn from([x, y, r]: [U; 3]) -> Self {
        let r = r.as_();
        Self([x.as_(), y.as_(), r, r])
    }
}

impl<T, U> From<&[U; 3]> for Ellipse<T>
where
    T: 'static + Copy,
    U: AsPrimitive<T>,
{
    /// Converts `&[U; 3]` into `Ellipse<T>`.
    #[inline]
    fn from(&[x, y, r]: &[U; 3]) -> Self {
        let r = r.as_();
        Self([x.as_(), y.as_(), r, r])
    }
}
