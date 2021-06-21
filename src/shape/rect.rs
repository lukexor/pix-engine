//! [`Square`] and [`Rect`] types used for drawing.

use super::{Line, Point};
use crate::vector::Vector;
use num_traits::Num;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A `Rectangle` positioned at `(x, y)` with `width` and `height`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rect<T> {
    /// X-coord
    pub x: T,
    /// Y-coord
    pub y: T,
    /// Width
    pub w: T,
    /// Height
    pub h: T,
}

/// # Constructs a [`Rect<T>`].
///
/// ```
/// use pix_engine::prelude::*;
/// let r = rect!(10, 20, 100, 200);
/// assert_eq!(r.x, 10);
/// assert_eq!(r.y, 20);
/// assert_eq!(r.w, 100);
/// assert_eq!(r.h, 200);
/// ```
#[macro_export]
macro_rules! rect {
    () => {
        rect!(0, 0)
    };
    ($x:expr, $y:expr$(,)?) => {
        rect!($x, $y, 100, 100)
    };
    ($x:expr, $y:expr, $w:expr$(,)?) => {
        rect!($x, $y, $w, $w)
    };
    ($x:expr, $y:expr, $w:expr, $h:expr$(,)?) => {
        $crate::shape::rect::Rect::new($x, $y, $w, $h)
    };
}

impl<T> Rect<T>
where
    T: Num + Copy,
{
    /// Constructs a `Rect<T>` at position `(x, y)` with `width` and `height`.
    pub fn new(x: T, y: T, w: T, h: T) -> Self {
        Self { x, y, w, h }
    }

    /// Constructs a `Rect<T>` centered at position `(x, y)` with `width` and `height`.
    pub fn from_center(p: impl Into<(T, T)>, width: T, height: T) -> Self {
        let (x, y) = p.into();
        let two = T::one() + T::one();
        Self {
            x: x - width / two,
            y: y - height / two,
            w: width,
            h: height,
        }
    }

    /// Returns the horizontal position of the left edge.
    pub fn x(&self) -> T {
        self.x
    }

    /// Returns the vertical position of the top edge.
    pub fn y(&self) -> T {
        self.y
    }

    /// Returns the width.
    pub fn width(&self) -> T {
        self.w
    }

    /// Returns the height.
    pub fn height(&self) -> T {
        self.h
    }

    /// Returns the dimensions as `(width, height)`.
    pub fn dimensions(&self) -> (T, T) {
        (self.w, self.h)
    }

    /// Set the horizontal position of the left edge.
    pub fn set_x(&mut self, x: T) {
        self.x = x;
    }

    /// Set the vertical position of the top edge.
    pub fn set_y(&mut self, y: T) {
        self.y = y;
    }

    /// Set the width.
    pub fn set_width(&mut self, width: T) {
        self.w = width;
    }

    /// Set the height.
    pub fn set_height(&mut self, height: T) {
        self.h = height;
    }

    /// Set the dimensions as `(width, height)`.
    pub fn set_dimensions(&mut self, (w, h): (T, T)) {
        self.w = w;
        self.h = h;
    }

    /// Returns the horizontal position of the left edge.
    pub fn left(&self) -> T {
        self.x
    }

    /// Returns the horizontal position of the right edge.
    pub fn right(&self) -> T {
        self.x + self.w
    }

    /// Returns the horizontal position of the top edge.
    pub fn top(&self) -> T {
        self.y
    }

    /// Returns the vertical position of the bottom edge.
    pub fn bottom(&self) -> T {
        self.y + self.h
    }

    /// Set the horizontal position of the left edge.
    pub fn set_left(&mut self, left: T) {
        self.set_x(left);
    }

    /// Set the horizontal position of the right edge.
    pub fn set_right(&mut self, right: T) {
        self.set_x(right - self.w);
    }

    /// Set the vertical position of the top edge.
    pub fn set_top(&mut self, top: T) {
        self.set_y(top);
    }

    /// Set the vertical position of the bottom edge.
    pub fn set_bottom(&mut self, bottom: T) {
        self.set_y(bottom - self.h);
    }

    /// Returns the center position as [`Point<T>`].
    pub fn center(&self) -> Point<T> {
        let two = T::one() + T::one();
        let x = self.x + (self.w / two);
        let y = self.y + (self.h / two);
        point!(x, y)
    }

    /// Returns the top-left position as [`Point<T>`].
    pub fn top_left(&self) -> Point<T> {
        point!(self.x, self.y)
    }

    /// Returns the top-right position as [`Point<T>`].
    pub fn top_right(&self) -> Point<T> {
        point!(self.x + self.w, self.y)
    }

    /// Returns the bottom-left position as [`Point<T>`].
    pub fn bottom_left(&self) -> Point<T> {
        point!(self.x, self.y + self.h)
    }

    /// Returns the bottom-right position as [`Point<T>`].
    pub fn bottom_right(&self) -> Point<T> {
        point!(self.x + self.w, self.y + self.h)
    }

    /// Set position centered on a [`Point<T>`].
    pub fn center_on(&mut self, p: impl Into<(T, T)>) {
        let (x, y) = p.into();
        let two = T::one() + T::one();
        self.x = x - self.w / two;
        self.y = y - self.h / two;
    }

    /// Returns whether this rectangle contains a given [`Point<T>`].
    pub fn contains_point(&self, p: impl Into<(T, T)>) -> bool
    where
        T: PartialOrd,
    {
        let (x, y) = p.into();
        x >= self.left() && x < self.right() && y >= self.top() && y < self.bottom()
    }

    /// Returns whether this rectangle completely contains another rectangle.
    pub fn contains_rect(&self, other: impl Into<Rect<T>>) -> bool
    where
        T: PartialOrd,
    {
        let other = other.into();
        other.left() >= self.left()
            && other.right() < self.right()
            && other.top() >= self.top()
            && other.bottom() < self.bottom()
    }

    /// Returns whether this rectangle intersects with another rectangle.
    pub fn intersects(&self, _other: impl Into<Rect<T>>) -> bool {
        todo!();
    }

    /// Returns whether this rectangle intersects with a line.
    pub fn intersects_line(&self, _line: impl Into<Line<T>>) -> Option<(Point<T>, Point<T>)> {
        todo!();
    }
}

/// Convert `(x, y, w, h)` to [`Rect<T>`].
impl<T> From<(T, T, T, T)> for Rect<T> {
    fn from((x, y, w, h): (T, T, T, T)) -> Self {
        Self { x, y, w, h }
    }
}

/// Convert ([`Point<T>`], `w`, `h`) to [`Rect<T>`].
impl<T> From<(Point<T>, T, T)> for Rect<T> {
    fn from((p, w, h): (Point<T>, T, T)) -> Self {
        Self {
            x: p.x,
            y: p.y,
            w,
            h,
        }
    }
}

/// Convert ([`Vector<T>`], `w`, `h`) to [`Rect<T>`].
impl<T> From<(Vector<T>, T, T)> for Rect<T> {
    fn from((v, w, h): (Vector<T>, T, T)) -> Self {
        Self {
            x: v.x,
            y: v.y,
            w,
            h,
        }
    }
}

/// Convert [`Square<T>`] to [`Rect<T>`].
impl<T> From<Square<T>> for Rect<T>
where
    T: Copy,
{
    fn from(s: Square<T>) -> Self {
        Self {
            x: s.x,
            y: s.y,
            w: s.s,
            h: s.s,
        }
    }
}

/// A `Square` positioned at `(x, y)` with `size`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Square<T> {
    /// X-coord
    pub x: T,
    /// Y-coord
    pub y: T,
    /// Size
    pub s: T,
}

/// # Constructs a [`Square<T>`].
///
/// ```
/// use pix_engine::prelude::*;
/// let s = square!(10, 20, 100);
/// assert_eq!(s.x, 10);
/// assert_eq!(s.y, 20);
/// assert_eq!(s.s, 100);
/// ```
#[macro_export]
macro_rules! square {
    () => {
        square!(0, 0)
    };
    ($x:expr, $y:expr$(,)?) => {
        square!($x, $y, 100)
    };
    ($x:expr, $y:expr, $s:expr$(,)?) => {
        $crate::shape::rect::Square::new($x, $y, $s)
    };
}

impl<T> Square<T>
where
    T: Num,
{
    /// Constructs a `Square`.
    pub fn new(x: T, y: T, s: T) -> Self {
        Self { x, y, s }
    }
}

/// Convert `(x, y, s)` to [`Square<T>`].
impl<T> From<(T, T, T)> for Square<T> {
    fn from((x, y, s): (T, T, T)) -> Self {
        Self { x, y, s }
    }
}

/// Convert ([`Point<T>`], `size`) to [`Square<T>`].
impl<T> From<(Point<T>, T)> for Square<T> {
    fn from((p, s): (Point<T>, T)) -> Self {
        Self { x: p.x, y: p.y, s }
    }
}

/// Convert ([`Vector<T>`], `size`) to [`Square<T>`].
impl<T> From<(Vector<T>, T)> for Square<T> {
    fn from((v, s): (Vector<T>, T)) -> Self {
        Self { x: v.x, y: v.y, s }
    }
}
