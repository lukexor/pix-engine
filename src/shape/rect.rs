//! 2D Rect types used for drawing.

use super::Point;
use crate::vector::Vector;
use num::Num;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A `Rectangle` positioned at (x, y) with width and height.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
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

/// # Create new [Rect<T>].
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
    T: Num,
{
    /// Create new `Rect`.
    pub fn new(x: T, y: T, w: T, h: T) -> Self {
        Self { x, y, w, h }
    }
}

/// Convert `(x, y, w, h)` to [Rect<T>].
impl<T> From<(T, T, T, T)> for Rect<T> {
    fn from((x, y, w, h): (T, T, T, T)) -> Self {
        Self { x, y, w, h }
    }
}

/// Convert `([Point<T>], w, h)` to [Rect<T>].
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

/// Convert `([Vector<T>], w, h)` to [Rect<T>].
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

/// Convert [Square<T>] to [Rect<T>].
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

/// A `Square` positioned at (x, y) with size.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Square<T> {
    /// X-coord
    pub x: T,
    /// Y-coord
    pub y: T,
    /// Size
    pub s: T,
}

/// # Create new [Square<T>].
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
    /// Create new `Square`.
    pub fn new(x: T, y: T, s: T) -> Self {
        Self { x, y, s }
    }
}

/// Convert `(x, y, s)` to [Square<T>].
impl<T> From<(T, T, T)> for Square<T> {
    fn from((x, y, s): (T, T, T)) -> Self {
        Self { x, y, s }
    }
}

/// Convert `([Point<T>], size)` to [Square<T>].
impl<T> From<(Point<T>, T)> for Square<T> {
    fn from((p, s): (Point<T>, T)) -> Self {
        Self { x: p.x, y: p.y, s }
    }
}

/// Convert `([Vector<T>], size)` to [Square<T>].
impl<T> From<(Vector<T>, T)> for Square<T> {
    fn from((v, s): (Vector<T>, T)) -> Self {
        Self { x: v.x, y: v.y, s }
    }
}
