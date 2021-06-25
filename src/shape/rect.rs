//! [`Rect`] types used for drawing.

use crate::prelude::{Draw, Line, PixResult, PixState, Point, Scalar, Shape, ShapeNum};
use num_traits::{AsPrimitive, Num};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
/// assert_eq!(s.width, 100);
/// assert_eq!(s.height, 100);
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

    /// Converts [`Rect<T>`] to [`Rect<i16>`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let r: Rect<f32> = rect!(f32::MIN, 2.0, 3.0, f32::MAX);
    /// let r = r.as_i16();
    /// assert_eq!(r.x, i16::MIN);
    /// assert_eq!(r.y, 2);
    /// assert_eq!(r.width, 3);
    /// assert_eq!(r.height, i16::MAX);
    /// ```
    pub fn as_i16(&self) -> Rect<i16>
    where
        T: AsPrimitive<i16>,
    {
        Rect::new(
            self.x.as_(),
            self.y.as_(),
            self.width.as_(),
            self.height.as_(),
        )
    }

    /// Converts [`Rect<T>`] to [`Rect<Scalar>`].
    pub fn as_scalar(&self) -> Rect<Scalar>
    where
        T: AsPrimitive<Scalar>,
    {
        Rect::new(
            self.x.as_(),
            self.y.as_(),
            self.width.as_(),
            self.height.as_(),
        )
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
        Self::new(x - width / two, y - height / two, width, height)
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

impl<T: ShapeNum> Shape<T> for Rect<T> {
    type Item = Rect<T>;

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

    /// Returns the closest intersection point with a given line and distance along the line or
    /// `None` if there is no intersection.
    fn intersects_line(&self, line: impl Into<Line<Scalar>>) -> Option<(Point<Scalar>, Scalar)> {
        let rect: Rect<Scalar> = self.as_scalar();
        let line = line.into();
        let left = line.intersects(Line::new(rect.top_left(), rect.bottom_left()));
        let right = line.intersects(Line::new(rect.top_right(), rect.bottom_right()));
        let top = line.intersects(Line::new(rect.top_left(), rect.top_right()));
        let bottom = line.intersects(Line::new(rect.bottom_left(), rect.bottom_right()));
        [left, right, top, bottom]
            .iter()
            .filter_map(|&p| p)
            .fold(None, |closest, intersection| {
                let closest_t = closest.map(|c| c.1).unwrap_or(Scalar::INFINITY);
                let t = intersection.1;
                if t < closest_t {
                    Some(intersection)
                } else {
                    closest
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

impl<T> Draw for Rect<T>
where
    Rect<T>: Copy + Into<Rect<Scalar>>,
{
    /// Draw rectangle to the current [`PixState`] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.rect(*self)
    }
}

macro_rules! impl_from {
    ($from:ty => $to:ty) => {
        impl From<Rect<$from>> for Rect<$to> {
            fn from(r: Rect<$from>) -> Self {
                Rect::new(r.x.into(), r.y.into(), r.width.into(), r.height.into())
            }
        }
    };
}

impl_from!(i8 => Scalar);
impl_from!(u8 => Scalar);
impl_from!(i16 => Scalar);
impl_from!(u16 => Scalar);
impl_from!(i32 => Scalar);
impl_from!(u32 => Scalar);
impl_from!(f32 => Scalar);

/// Convert `[x, y, size]` to [`Rect<T>`].
impl<T: Copy, U: Into<T>> From<[U; 3]> for Rect<T> {
    fn from([x, y, size]: [U; 3]) -> Self {
        let size = size.into();
        Self::new(x.into(), y.into(), size, size)
    }
}

/// Convert `&[x, y, size]` to [`Rect<T>`].
impl<T: Copy, U: Copy + Into<T>> From<&[U; 3]> for Rect<T> {
    fn from(&[x, y, size]: &[U; 3]) -> Self {
        let size = size.into();
        Self::new(x.into(), y.into(), size, size)
    }
}

/// Convert `[x, y, width, height]` to [`Rect<T>`].
impl<T, U: Into<T>> From<[U; 4]> for Rect<T> {
    fn from([x, y, width, height]: [U; 4]) -> Self {
        Self::new(x.into(), y.into(), width.into(), height.into())
    }
}

/// Convert `&[x, y, width, height]` to [`Rect<T>`].
impl<T, U: Copy + Into<T>> From<&[U; 4]> for Rect<T> {
    fn from(&[x, y, width, height]: &[U; 4]) -> Self {
        Self::new(x.into(), y.into(), width.into(), height.into())
    }
}

/// Convert [`Rect<T>`] to `[x, y, width, height]`.
impl<T, U: Into<T>> From<Rect<U>> for [T; 4] {
    fn from(r: Rect<U>) -> Self {
        [r.x.into(), r.y.into(), r.width.into(), r.height.into()]
    }
}

/// Convert [`&Rect<T>`] to `[x, y, width, height]`.
impl<T, U: Copy + Into<T>> From<&Rect<U>> for [T; 4] {
    fn from(r: &Rect<U>) -> Self {
        [r.x.into(), r.y.into(), r.width.into(), r.height.into()]
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    macro_rules! assert_approx_eq {
        ($i1:expr, $i2:expr) => {
            assert_approx_eq!($i1, $i2, Scalar::EPSILON);
        };
        ($i1:expr, $i2:expr, $e:expr) => {{
            match ($i1, $i2) {
                (Some((p1, t1)), Some((p2, t2))) => {
                    let [x1, y1, z1]: [Scalar; 3] = p1.into();
                    let [x2, y2, z2]: [Scalar; 3] = p2.into();
                    let xd = (x1 - x2).abs();
                    let yd = (y1 - y2).abs();
                    let zd = (z1 - z2).abs();
                    let td = (t1 - t2).abs();
                    assert!(xd < $e, "x: ({} - {}) < {}", x1, x2, $e);
                    assert!(yd < $e, "y: ({} - {}) < {}", y1, y2, $e);
                    assert!(zd < $e, "z: ({} - {}) < {}", z1, z2, $e);
                    assert!(td < $e, "t: ({} - {}) < {}", t1, t2, $e);
                }
                _ => assert_eq!($i1, $i2),
            }
        }};
    }

    #[test]
    fn test_intersects_line() {
        let rect: Rect<i32> = rect!(10, 10, 100, 100);

        // Left
        let line = Line::new([3, 7], [20, 30]);
        assert_approx_eq!(
            rect.intersects_line(line),
            Some((point!(10.0, 16.471), 0.411)),
            0.001
        );

        // Right
        let line = Line::new([150, 50], [90, 30]);
        assert_approx_eq!(
            rect.intersects_line(line),
            Some((point!(110.0, 36.667), 0.667)),
            0.001
        );

        // Top
        let line = Line::new([50, 5], [70, 30]);
        assert_approx_eq!(
            rect.intersects_line(line),
            Some((point!(54.0, 10.0), 0.2)),
            0.001
        );

        // Bottom
        let line = Line::new([50, 150], [30, 30]);
        assert_approx_eq!(
            rect.intersects_line(line),
            Some((point!(43.3333, 110.0), 0.333)),
            0.001
        );
    }
}
