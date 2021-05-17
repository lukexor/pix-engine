//! 2D Rect types used for drawing.

use super::Point;
use crate::vector::Vector;

/// A `Rectangle`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rect {
    /// X-coord
    pub x: i32,
    /// Y-coord
    pub y: i32,
    /// Width
    pub w: u32,
    /// Height
    pub h: u32,
}

/// # Create a [`Rect`].
///
/// ```
/// use pix_engine::prelude::*;
///
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
        $crate::prelude::Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    };
}

impl Rect {
    /// Creates a new [`Rect`].
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }
}

/// From tuple of (x, y, w, h) to [`Rect`].
impl From<(i32, i32, u32, u32)> for Rect {
    fn from((x, y, w, h): (i32, i32, u32, u32)) -> Self {
        Self::new(x, y, w, h)
    }
}

/// From tuple of (`Point`, w, h) to [`Rect`].
impl From<(Point, u32, u32)> for Rect {
    fn from((p, w, h): (Point, u32, u32)) -> Self {
        Self::new(p.x, p.y, w, h)
    }
}

/// From tuple of (`Vector`, w, h) to [`Rect`].
impl From<(Vector, u32, u32)> for Rect {
    fn from((v, w, h): (Vector, u32, u32)) -> Self {
        Self::new(v.x.round() as i32, v.y.round() as i32, w, h)
    }
}

/// From [`Square`] to [`Rect`].
impl From<Square> for Rect {
    fn from(s: Square) -> Self {
        Self::new(s.x, s.y, s.s, s.s)
    }
}

/// A `Square`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Square {
    /// X-coord
    pub x: i32,
    /// Y-coord
    pub y: i32,
    /// Size
    pub s: u32,
}

/// # Create a [`Square`].
///
/// ```
/// use pix_engine::prelude::*;
///
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
        $crate::prelude::Square::new($x as i32, $y as i32, $s as u32)
    };
}

impl Square {
    /// Creates a new `Square`.
    pub fn new(x: i32, y: i32, s: u32) -> Self {
        Self { x, y, s }
    }
}

/// From tuple of (x, y, s) to [`Square`].
impl From<(i32, i32, u32)> for Square {
    fn from((x, y, s): (i32, i32, u32)) -> Self {
        Self::new(x, y, s)
    }
}

/// From tuple of (`Point`, s) to [`Square`].
impl From<(Point, u32)> for Square {
    fn from((p, s): (Point, u32)) -> Self {
        Self::new(p.x, p.y, s)
    }
}

/// From tuple of (`Vector`, s) to [`Square`].
impl From<(Vector, u32)> for Square {
    fn from((v, s): (Vector, u32)) -> Self {
        Self::new(v.x.round() as i32, v.y.round() as i32, s)
    }
}
