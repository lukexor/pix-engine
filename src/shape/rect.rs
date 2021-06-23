//! [`Rect`] types used for drawing.

use crate::{
    prelude::{Draw, Line, PixResult, PixState, Point, Shape},
    vector::Vector,
};
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
/// assert_eq!(r.width, 100);
/// assert_eq!(r.height, 200);
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

/// # Constructs a [`Rect<T>`] with the same `width` and `height`.
///
/// ```
/// use pix_engine::prelude::*;
/// let s = square!(10, 20, 100);
/// assert_eq!(s.x, 10);
/// assert_eq!(s.y, 20);
/// assert_eq!(s.size, 100);
/// ```
#[macro_export]
macro_rules! square {
    () => {
        square!(0, 0)
    };
    ($x:expr, $y:expr$(,)?) => {
        square!($x, $y, 100)
    };
    ($x:expr, $y:expr, $size:expr$(,)?) => {
        $crate::shape::rect::Rect::new($x, $y, $size, $size)
    };
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
    pub fn set_dimensions(&mut self, (width, height): (T, T)) {
        self.width = width;
        self.height = height;
    }
}

impl<T> Rect<T>
where
    T: Copy,
{
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
}

impl<T> Rect<T>
where
    T: Num + Copy,
{
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
}

impl<T: Num + PartialOrd> Shape for Rect<T> {
    type Item = Rect<T>;
    type DrawType = i16;

    /// Returns whether this rectangle contains a given [`Point<T>`].
    fn contains_point(&self, p: impl Into<Point<T>>) -> bool {
        let p = p.into();
        p.x >= self.left() && p.x < self.right() && p.y >= self.top() && p.y < self.bottom()
    }

    /// Returns whether this rectangle completely contains another rectangle.
    fn contains(&self, other: impl Into<Rect<T>>) -> bool {
        let other = other.into();
        other.left() >= self.left()
            && other.right() < self.right()
            && other.top() >= self.top()
            && other.bottom() < self.bottom()
    }

    /// Returns whether this rectangle intersects with a line.
    fn intersects_line(&self, line: impl Into<Line<f64>>) -> Option<(Point<f64>, Point<f64>)>
    where
        T: Into<f64>,
    {
        let line = line.into();
        let left = line.intersects(Line::new(self.top_left(), self.bottom_left()));
        let right = line.intersects(Line::new(self.top_right(), self.bottom_right()));
        let top = line.intersects(Line::new(self.top_left(), self.top_right()));
        let bottom = line.intersects(Line::new(self.bottom_left(), self.bottom_right()));
        [left, right, top, bottom]
            .iter()
            .filter_map(|&p| p)
            .fold(None, |intersections, p1| {
                let p2 = if line.start == p1 {
                    line.end
                } else {
                    line.start
                };
                match intersections {
                    None => Some((p1, p2)),
                    Some((i1, _)) => Some((i1, p2)),
                }
            })
    }

    /// Returns whether this rectangle intersects with another rectangle.
    fn intersects(&self, other: impl Into<Rect<T>>) -> bool {
        let other = other.into();
        let tl = self.top_left();
        let br = self.bottom_right();
        let otl = other.top_left();
        let obr = other.bottom_right();
        // Both rectangle corner x and y values overlap ranges
        tl.x < obr.x && br.x > otl.x && tl.y < otl.y && br.y > obr.y
    }
}

impl<T> Draw for Rect<T> {
    /// Draw rectangle to the current [`PixState`] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.rect(self)
    }
}

/// Convert `[x, y, width, height]` to [`Rect<T>`].
impl<T, U: Into<T>> From<[U; 4]> for Rect<T> {
    fn from([x, y, width, height]: [U; 4]) -> Self {
        Self::new(x.into(), y.into(), width.into(), height.into())
    }
}

/// Convert `&[x, y, width, height]` to [`Rect<T>`].
impl<T: Copy, U: Into<T>> From<&[U; 4]> for Rect<T> {
    fn from(&[x, y, width, height]: &[U; 4]) -> Self {
        Self::new(x.into(), y.into(), width.into(), height.into())
    }
}

/// Convert [`Rect<T>`] to `[x, y, width, height]`.
impl<T: Copy, U: Into<T>> From<Rect<U>> for [T; 4] {
    fn from(r: Rect<U>) -> Self {
        [r.x.into(), r.y.into(), r.width.into(), r.height.into()]
    }
}

/// Convert [`&Rect<T>`] to `[x, y, width, height]`.
impl<T: Copy, U: Into<T>> From<&Rect<U>> for [T; 4] {
    fn from(r: &Rect<U>) -> Self {
        [r.x.into(), r.y.into(), r.width.into(), r.height.into()]
    }
}

/// Convert ([`Point<U>`], `width`, `height`) to [`Rect<T>`].
impl<T, U: Into<T>, V: Into<T>> From<(Point<U>, V, V)> for Rect<T> {
    fn from((p, width, height): (Point<U>, V, V)) -> Self {
        Self::new(p.x.into(), p.y.into(), width.into(), height.into())
    }
}

/// Convert [`Rect<T>`] to ([`Point<U>`], `width`, `height`).
impl<T, U: Into<T>, V: Into<T>> From<Rect<U>> for (Point<T>, V, V) {
    fn from(r: Rect<U>) -> Self {
        ((r.x, r.y).into(), r.width.into(), r.height.into())
    }
}

/// Convert [`&Rect<T>`] to ([`Point<U>`], `width`, `height`).
impl<T, U: Into<T>, V: Into<T>> From<&Rect<U>> for (Point<T>, V, V) {
    fn from(r: &Rect<U>) -> Self {
        ((r.x, r.y).into(), r.width.into(), r.height.into())
    }
}

/// Convert ([`Vector<U>`], `width`, `height`) to [`Rect<T>`].
impl<T, U: Into<T>, V: Into<T>> From<(Vector<U>, V, V)> for Rect<T> {
    fn from((v, width, height): (Vector<U>, V, V)) -> Self {
        Self::new(v.x.into(), v.y.into(), width.into(), height.into())
    }
}

/// Convert [`Rect<T>`] to ([`Vector<U>`], `width`, `height`).
impl<T, U: Into<T>, V: Into<T>> From<Rect<U>> for (Vector<T>, V, V) {
    fn from(r: Rect<U>) -> Self {
        ((r.x, r.y).into(), r.width.into(), r.height.into())
    }
}

/// Convert [`&Rect<T>`] to ([`Vector<U>`], `width`, `height`).
impl<T, U: Into<T>, V: Into<T>> From<&Rect<U>> for (Vector<T>, V, V) {
    fn from(r: &Rect<U>) -> Self {
        ((r.x, r.y).into(), r.width.into(), r.height.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_intersects_line() {
        let rect = rect!(10, 10, 100, 100);

        // (Left, p2)
        let line = Line::with_xy(5, 7, 20, 30);
        // assert_eq!(
        //     rect.intersects_line(line),
        //     Some((point!(0.0, 0.0), point!(0.0, 0.0)))
        // );
        // (Left, Right)
        // (Left, Top)
        // (Left, Bottom)

        // (Right, p1)
        // (Right, Left)
        // (Right, Top)
        // (Right, Bottom)

        // (Top, p2)
        // (Top, Left)
        // (Top, Right)
        // (Top, Bottom)

        // (Bottom, p1)
        // (Bottom, Left)
        // (Bottom, Right)
        // (Bottom, Top)
    }
}
