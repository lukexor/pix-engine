//! [Rect] types used for drawing.

use crate::prelude::*;
use num_traits::{AsPrimitive, Float};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// # Constructs a [Rect].
///
/// ```
/// # use pix_engine::prelude::*;
/// let p = point!(10, 20);
/// let r = rect!(p, 100, 200);
/// assert_eq!(r.x, 10);
/// assert_eq!(r.y, 20);
/// assert_eq!(r.width, 100);
/// assert_eq!(r.height, 200);
///
/// let r = rect!(10, 20, 100, 200);
/// assert_eq!(r.x, 10);
/// assert_eq!(r.y, 20);
/// assert_eq!(r.width, 100);
/// assert_eq!(r.height, 200);
/// ```
#[macro_export]
macro_rules! rect {
    ($p:expr, $size:expr$(,)?) => {
        rect!($p, $size, $size)
    };
    ($p:expr, $width:expr, $height:expr$(,)?) => {
        $crate::prelude::Rect::with_point($p, $width, $height)
    };
    ($x:expr, $y:expr, $width:expr, $height:expr$(,)?) => {
        $crate::prelude::Rect::new($x, $y, $width, $height)
    };
}

/// # Constructs a [Rect] with the same `width` and `height`.
///
/// ```
/// # use pix_engine::prelude::*;
/// let p = point!(10, 20);
/// let s = square!(p, 100);
/// assert_eq!(s.x, 10);
/// assert_eq!(s.y, 20);
/// assert_eq!(s.width, 100);
/// assert_eq!(s.height, 100);
///
/// let s = square!(10, 20, 100);
/// assert_eq!(s.x, 10);
/// assert_eq!(s.y, 20);
/// assert_eq!(s.width, 100);
/// assert_eq!(s.height, 100);
/// ```
#[macro_export]
macro_rules! square {
    ($p:expr, $size:expr$(,)?) => {{
        $crate::prelude::Rect::square_with_point($p, $size)
    }};
    ($x:expr, $y:expr, $size:expr$(,)?) => {
        $crate::prelude::Rect::square($x, $y, $size)
    };
}

/// A `Rectangle` positioned at `(x, y)` with `width` and `height`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rect<T = Scalar> {
    /// X-coord
    pub x: T,
    /// Y-coord
    pub y: T,
    /// Width
    pub width: T,
    /// Height
    pub height: T,
}

impl<T> Rect<T> {
    /// Constructs a `Rect<T>` at position `(x, y)` with `width` and `height`.
    pub const fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Constructs a `Rect<T>` at position [Point] with `width` and `height`.
    pub fn with_point<P: Into<Point<T>>>(p: P, width: T, height: T) -> Self {
        let p = p.into();
        Self::new(p.x, p.y, width, height)
    }

    /// Constructs a square `Rect<T>` at position `(x, y)` with `size`.
    pub fn square(x: T, y: T, size: T) -> Self
    where
        T: Copy,
    {
        Self::new(x, y, size, size)
    }

    /// Constructs a square `Rect<T>` at position [Point] with `size`.
    pub fn square_with_point<P: Into<Point<T>>>(p: P, size: T) -> Self
    where
        T: Copy,
    {
        Self::with_point(p, size, size)
    }

    /// Convert `Rect<T>` to another primitive type using the `as` operator.
    #[inline]
    pub fn as_<U>(self) -> Rect<U>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy,
    {
        Rect::new(
            self.x.as_(),
            self.y.as_(),
            self.width.as_(),
            self.height.as_(),
        )
    }
}

impl<T: Number> Rect<T> {
    /// Constructs a `Rect<T>` centered at position `(x, y)` with `width` and `height`.
    pub fn from_center<P>(p: P, width: T, height: T) -> Self
    where
        P: Into<Point<T>>,
    {
        let p = p.into();
        let two = T::one() + T::one();
        Self::new(p.x - width / two, p.y - height / two, width, height)
    }

    /// Returns `Rect` values as `[x, y, width, height]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let r = rect!(5, 10, 100, 100);
    /// assert_eq!(r.values(), [5, 10, 100, 100]);
    /// ```
    pub fn values(&self) -> [T; 4] {
        [self.x, self.y, self.width, self.height]
    }

    /// Returns `Rect` as a [Vec].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let r = rect!(5, 10, 100, 100);
    /// assert_eq!(r.to_vec(), vec![5, 10, 100, 100]);
    /// ```
    pub fn to_vec(self) -> Vec<T> {
        vec![self.x, self.y, self.width, self.height]
    }

    /// Returns the horizontal position of the left edge.
    pub fn left(&self) -> T {
        self.x
    }

    /// Returns the horizontal position of the right edge.
    pub fn right(&self) -> T {
        self.x + self.width
    }

    /// Returns the horizontal position of the top edge.
    pub fn top(&self) -> T {
        self.y
    }

    /// Returns the vertical position of the bottom edge.
    pub fn bottom(&self) -> T {
        self.y + self.height
    }

    /// Set the horizontal position of the left edge.
    pub fn set_left(&mut self, left: T) {
        self.x = left;
    }

    /// Set the horizontal position of the right edge.
    pub fn set_right(&mut self, right: T) {
        self.x = right - self.width;
    }

    /// Set the vertical position of the top edge.
    pub fn set_top(&mut self, top: T) {
        self.y = top;
    }

    /// Set the vertical position of the bottom edge.
    pub fn set_bottom(&mut self, bottom: T) {
        self.y = bottom - self.height;
    }

    /// Returns the center position as [Point].
    pub fn center(&self) -> Point<T> {
        let two = T::one() + T::one();
        let x = self.x + (self.width / two);
        let y = self.y + (self.height / two);
        point!(x, y)
    }

    /// Returns the top-left position as [Point].
    pub fn top_left(&self) -> Point<T> {
        point!(self.x, self.y)
    }

    /// Returns the top-right position as [Point].
    pub fn top_right(&self) -> Point<T> {
        point!(self.x + self.width, self.y)
    }

    /// Returns the bottom-left position as [Point].
    pub fn bottom_left(&self) -> Point<T> {
        point!(self.x, self.y + self.height)
    }

    /// Returns the bottom-right position as [Point].
    pub fn bottom_right(&self) -> Point<T> {
        point!(self.x + self.width, self.y + self.height)
    }

    /// Set position centered on a [Point].
    pub fn center_on<P>(&mut self, p: P)
    where
        P: Into<Point<T>>,
    {
        let p = p.into();
        let two = T::one() + T::one();
        self.x = p.x - self.width / two;
        self.y = p.y - self.height / two;
    }
}

impl<T: Float> Rect<T> {
    /// Returns `Rect` with values rounded to the nearest integer number. Round half-way cases
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

impl<T: Number> Shape<T> for Rect<T> {
    type Item = Rect<T>;

    /// Returns whether this rectangle contains a given [Point].
    fn contains_point<P>(&self, p: P) -> bool
    where
        P: Into<Point<T>>,
    {
        let p = p.into();
        p.x >= self.left() && p.x < self.right() && p.y >= self.top() && p.y < self.bottom()
    }

    /// Returns whether this rectangle completely contains another rectangle.
    fn contains<O>(&self, other: O) -> bool
    where
        O: Into<Self::Item>,
    {
        let other = other.into();
        other.left() >= self.left()
            && other.right() < self.right()
            && other.top() >= self.top()
            && other.bottom() < self.bottom()
    }

    /// Returns the closest intersection point with a given line and distance along the line or
    /// `None` if there is no intersection.
    fn intersects_line<L>(&self, line: L) -> Option<(Point<T>, T)>
    where
        T: Float,
        L: Into<Line<T>>,
    {
        let line = line.into();
        let left = line.intersects_line([self.top_left(), self.bottom_left()]);
        let right = line.intersects_line([self.top_right(), self.bottom_right()]);
        let top = line.intersects_line([self.top_left(), self.top_right()]);
        let bottom = line.intersects_line([self.bottom_left(), self.bottom_right()]);
        [left, right, top, bottom]
            .iter()
            .filter_map(|&p| p)
            .fold(None, |closest, intersection| {
                let closest_t = closest.map(|c| c.1).unwrap_or_else(Float::infinity);
                let t = intersection.1;
                if t < closest_t {
                    Some(intersection)
                } else {
                    closest
                }
            })
    }

    /// Returns whether this rectangle intersects with another rectangle.
    fn intersects<O>(&self, other: O) -> bool
    where
        O: Into<Self::Item>,
    {
        let other = other.into();
        let tl = self.top_left();
        let br = self.bottom_right();
        let otl = other.top_left();
        let obr = other.bottom_right();
        // Both rectangle corner x and y values overlap ranges
        tl.x < obr.x && br.x > otl.x && tl.y < otl.y && br.y > obr.y
    }
}

impl<T> Draw for Rect<T>
where
    T: Number,
    Self: Into<Rect>,
{
    /// Draw rectangle to the current [PixState] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.rect(*self)
    }
}

impl<T: Number> From<&mut Rect<T>> for Rect<T> {
    fn from(rect: &mut Rect<T>) -> Self {
        rect.to_owned()
    }
}

impl<T: Number> From<&Rect<T>> for Rect<T> {
    fn from(rect: &Rect<T>) -> Self {
        *rect
    }
}

/// Convert [Rect] to `[x, y, width, height]`.
impl<T: Number> From<Rect<T>> for [T; 4] {
    fn from(r: Rect<T>) -> Self {
        [r.x, r.y, r.width, r.height]
    }
}

/// Convert &[Rect] to `[x, y, width, height]`.
impl<T: Number> From<&Rect<T>> for [T; 4] {
    fn from(r: &Rect<T>) -> Self {
        [r.x, r.y, r.width, r.height]
    }
}

/// Convert `[x, y, size]` to [Rect].
impl<T: Number, U: Into<T>> From<[U; 3]> for Rect<T> {
    fn from([x, y, size]: [U; 3]) -> Self {
        let size = size.into();
        Self::new(x.into(), y.into(), size, size)
    }
}

/// Convert `&[x, y, size]` to [Rect].
impl<T: Number, U: Copy + Into<T>> From<&[U; 3]> for Rect<T> {
    fn from(&[x, y, size]: &[U; 3]) -> Self {
        let size = size.into();
        Self::new(x.into(), y.into(), size, size)
    }
}

/// Convert `[x, y, width, height]` to [Rect].
impl<T: Number, U: Into<T>> From<[U; 4]> for Rect<T> {
    fn from([x, y, width, height]: [U; 4]) -> Self {
        Self::new(x.into(), y.into(), width.into(), height.into())
    }
}

/// Convert `&[x, y, width, height]` to [Rect].
impl<T: Number, U: Copy + Into<T>> From<&[U; 4]> for Rect<T> {
    fn from(&[x, y, width, height]: &[U; 4]) -> Self {
        Self::new(x.into(), y.into(), width.into(), height.into())
    }
}