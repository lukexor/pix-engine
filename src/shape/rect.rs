//! [`Square`] and [`Rect`] types used for drawing.

use super::{Line, Point};
use crate::vector::Vector;
use num_traits::{Float, Num};
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
    pub width: T,
    /// Height
    pub height: T,
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
    ($x:expr, $y:expr, $width:expr$(,)?) => {
        rect!($x, $y, $width, $width)
    };
    ($x:expr, $y:expr, $width:expr, $height:expr$(,)?) => {
        $crate::shape::rect::Rect::new($x, $y, $width, $height)
    };
}

impl<T> Rect<T>
where
    T: Num + Copy,
{
    /// Constructs a `Rect<T>` at position `(x, y)` with `width` and `height`.
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Constructs a `Rect<T>` centered at position `(x, y)` with `width` and `height`.
    pub fn from_center(p: impl Into<(T, T)>, width: T, height: T) -> Self {
        let (x, y) = p.into();
        let two = T::one() + T::one();
        Self {
            x: x - width / two,
            y: y - height / two,
            width,
            height,
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
        self.width
    }

    /// Returns the height.
    pub fn height(&self) -> T {
        self.height
    }

    /// Returns the dimensions as `(width, height)`.
    pub fn dimensions(&self) -> (T, T) {
        (self.width, self.height)
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
        self.width = width;
    }

    /// Set the height.
    pub fn set_height(&mut self, height: T) {
        self.height = height;
    }

    /// Set the dimensions as `(width, height)`.
    pub fn set_dimensions(&mut self, (w, h): (T, T)) {
        self.width = w;
        self.height = h;
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
        self.set_x(left);
    }

    /// Set the horizontal position of the right edge.
    pub fn set_right(&mut self, right: T) {
        self.set_x(right - self.width);
    }

    /// Set the vertical position of the top edge.
    pub fn set_top(&mut self, top: T) {
        self.set_y(top);
    }

    /// Set the vertical position of the bottom edge.
    pub fn set_bottom(&mut self, bottom: T) {
        self.set_y(bottom - self.height);
    }

    /// Returns the center position as [`Point<T>`].
    pub fn center(&self) -> Point<T> {
        let two = T::one() + T::one();
        let x = self.x + (self.width / two);
        let y = self.y + (self.height / two);
        point!(x, y)
    }

    /// Returns the top-left position as [`Point<T>`].
    pub fn top_left(&self) -> Point<T> {
        point!(self.x, self.y)
    }

    /// Returns the top-right position as [`Point<T>`].
    pub fn top_right(&self) -> Point<T> {
        point!(self.x + self.width, self.y)
    }

    /// Returns the bottom-left position as [`Point<T>`].
    pub fn bottom_left(&self) -> Point<T> {
        point!(self.x, self.y + self.height)
    }

    /// Returns the bottom-right position as [`Point<T>`].
    pub fn bottom_right(&self) -> Point<T> {
        point!(self.x + self.width, self.y + self.height)
    }

    /// Set position centered on a [`Point<T>`].
    pub fn center_on(&mut self, p: impl Into<(T, T)>) {
        let (x, y) = p.into();
        let two = T::one() + T::one();
        self.x = x - self.width / two;
        self.y = y - self.height / two;
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
    pub fn intersects(&self, other: impl Into<Rect<T>>) -> bool
    where
        T: PartialOrd,
    {
        let other = other.into();
        let tl = self.top_left();
        let br = self.bottom_right();
        let otl = other.top_left();
        let obr = other.bottom_right();
        // Both rectangle corner x and y values overlap ranges
        tl.x < obr.x && br.x > otl.x && tl.y < otl.y && br.y > obr.y
    }

    /// Returns whether this rectangle intersects with a line.
    pub fn intersects_line(&self, line: impl Into<Line<T>>) -> Option<(Point<T>, Point<T>)>
    where
        T: Float + PartialOrd,
    {
        let line = line.into();
        let left = line.intersects((self.top_left(), self.bottom_left()));
        let right = line.intersects((self.top_right(), self.bottom_right()));
        let top = line.intersects((self.top_left(), self.top_right()));
        let bottom = line.intersects((self.bottom_left(), self.bottom_right()));
        match (left, right, top, bottom) {
            (Some(l), Some(r), None, None) => Some((l, r)),
            (Some(l), None, Some(t), None) => Some((l, t)),
            (Some(l), None, None, Some(b)) => Some((l, b)),
            (Some(l), None, None, None) => {
                let p2 = if l == line.p1 { line.p2 } else { line.p1 };
                Some((l, p2))
            }
            (None, Some(r), Some(t), None) => Some((r, t)),
            (None, Some(r), None, Some(b)) => Some((r, b)),
            (None, Some(r), None, None) => {
                let p2 = if r == line.p1 { line.p2 } else { line.p1 };
                Some((r, p2))
            }
            (None, None, Some(t), Some(b)) => Some((t, b)),
            (None, None, Some(t), None) => {
                let p2 = if t == line.p1 { line.p2 } else { line.p1 };
                Some((t, p2))
            }
            (None, None, None, Some(b)) => {
                let p2 = if b == line.p1 { line.p2 } else { line.p1 };
                Some((b, p2))
            }
            _ => None,
        }
    }
}

/// Convert `(x, y, w, h)` to [`Rect<T>`].
impl<T> From<(T, T, T, T)> for Rect<T> {
    fn from((x, y, width, height): (T, T, T, T)) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// Convert ([`Point<T>`], `w`, `h`) to [`Rect<T>`].
impl<T> From<(Point<T>, T, T)> for Rect<T> {
    fn from((p, width, height): (Point<T>, T, T)) -> Self {
        Self {
            x: p.x,
            y: p.y,
            width,
            height,
        }
    }
}

/// Convert ([`Vector<T>`], `w`, `h`) to [`Rect<T>`].
impl<T> From<(Vector<T>, T, T)> for Rect<T> {
    fn from((v, width, height): (Vector<T>, T, T)) -> Self {
        Self {
            x: v.x,
            y: v.y,
            width,
            height,
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
            width: s.s,
            height: s.s,
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
