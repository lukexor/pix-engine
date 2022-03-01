//! A shape type representing squares and rectangles used for drawing.
//!
//! # Examples
//!
//! You can create a [Rect] or square using [`Rect::new`] or [`Rect::square`]:
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! let r = Rect::new(10, 20, 100, 200);
//! let s = Rect::square(10, 20, 100);
//! ```
//!
//! ...or by using the [rect!] or [square!] macros:
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! let r = rect!(10, 20, 100, 200);
//! let s = square!(10, 20, 100);
//!
//! // using a point
//! let r = rect!([10, 20], 100, 200);
//! let r = rect!(point![10, 20], 100, 200);
//! let s = square!([10, 20], 100);
//! let s = square!(point![10, 20], 100);
//! ```

use crate::prelude::*;
use num_traits::{AsPrimitive, Bounded, NumCast};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

/// A `Rectangle` positioned at `(x, y)` with `width` and `height`. A square is a `Rectangle` where
/// `width` and `height` are equal.
///
/// Please see the [module-level documentation] for examples.
///
/// [module-level documentation]: crate::shape::rect
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
#[must_use]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rect<T = i32>(pub(crate) [T; 4]);

/// Constructs a [Rect] at position `(x, y)` with `width` and `height`.
///
/// ```
/// # use pix_engine::prelude::*;
/// let p = point!(10, 20);
/// let r = rect!(p, 100, 200);
/// assert_eq!(r.x(), 10);
/// assert_eq!(r.y(), 20);
/// assert_eq!(r.width(), 100);
/// assert_eq!(r.height(), 200);
///
/// let r = rect!(10, 20, 100, 200);
/// assert_eq!(r.x(), 10);
/// assert_eq!(r.y(), 20);
/// assert_eq!(r.width(), 100);
/// assert_eq!(r.height(), 200);
/// ```
#[macro_export]
macro_rules! rect {
    ($p1:expr, $p2:expr$(,)?) => {
        $crate::prelude::Rect::with_points($p1, $p2)
    };
    ($p:expr, $width:expr, $height:expr$(,)?) => {
        $crate::prelude::Rect::with_position($p, $width, $height)
    };
    ($x:expr, $y:expr, $width:expr, $height:expr$(,)?) => {
        $crate::prelude::Rect::new($x, $y, $width, $height)
    };
}

/// Constructs a square [Rect] at position `(x, y)` with the same `width` and `height`.
///
/// ```
/// # use pix_engine::prelude::*;
/// let p = point!(10, 20);
/// let s = square!(p, 100);
/// assert_eq!(s.x(), 10);
/// assert_eq!(s.y(), 20);
/// assert_eq!(s.width(), 100);
/// assert_eq!(s.height(), 100);
///
/// let s = square!(10, 20, 100);
/// assert_eq!(s.x(), 10);
/// assert_eq!(s.y(), 20);
/// assert_eq!(s.width(), 100);
/// assert_eq!(s.height(), 100);
/// ```
#[macro_export]
macro_rules! square {
    ($p:expr, $size:expr$(,)?) => {{
        $crate::prelude::Rect::square_with_position($p, $size)
    }};
    ($x:expr, $y:expr, $size:expr$(,)?) => {
        $crate::prelude::Rect::square($x, $y, $size)
    };
}

impl<T> Rect<T> {
    /// Constructs a `Rect` at position `(x, y)` with `width` and `height`.
    pub const fn new(x: T, y: T, width: T, height: T) -> Self {
        Self([x, y, width, height])
    }

    /// Constructs a square `Rect` at position `(x, y)` with `size`.
    pub fn square(x: T, y: T, size: T) -> Self
    where
        T: Copy,
    {
        Self::new(x, y, size, size)
    }
}

impl<T: Copy> Rect<T> {
    /// Returns `Rect` as `[x, y, width, height]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let r = rect!(5, 10, 100, 100);
    /// assert_eq!(r.as_array(), [5, 10, 100, 100]);
    /// ```
    #[inline]
    pub fn as_array(&self) -> [T; 4] {
        self.0
    }

    /// Returns `Rect` as a byte slice `&[x, y, width, height]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let r = rect!(5, 10, 100, 100);
    /// assert_eq!(r.as_bytes(), &[5, 10, 100, 100]);
    /// ```
    #[inline]
    pub fn as_bytes(&self) -> &[T; 4] {
        &self.0
    }

    /// Returns `Rect` as a mutable byte slice `&mut [x, y, width, height]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut r = rect!(5, 10, 100, 100);
    /// for p in r.as_bytes_mut() {
    ///     *p += 5;
    /// }
    /// assert_eq!(r.as_bytes(), &[10, 15, 105, 105]);
    /// ```
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [T; 4] {
        &mut self.0
    }

    /// Returns the `x-coordinate` of the rectangle.
    #[inline]
    pub fn x(&self) -> T {
        self.0[0]
    }

    /// Sets the `x-coordinate` of the rectangle.
    #[inline]
    pub fn set_x(&mut self, x: T) {
        self.0[0] = x;
    }

    /// Returns the `y-coordinate` of the rectangle.
    #[inline]
    pub fn y(&self) -> T {
        self.0[1]
    }

    /// Sets the `y-coordinate` of the rectangle.
    #[inline]
    pub fn set_y(&mut self, y: T) {
        self.0[1] = y;
    }

    /// Returns the `width` of the rectangle.
    #[inline]
    pub fn width(&self) -> T {
        self.0[2]
    }

    /// Sets the `width` of the rectangle.
    #[inline]
    pub fn set_width(&mut self, width: T) {
        self.0[2] = width;
    }

    /// Returns the `height` of the rectangle.
    #[inline]
    pub fn height(&self) -> T {
        self.0[3]
    }

    /// Sets the `height` of the rectangle.
    #[inline]
    pub fn set_height(&mut self, height: T) {
        self.0[3] = height;
    }
}

impl<T: Num> Rect<T> {
    /// Constructs a `Rect` at position [Point] with `width` and `height`.
    pub fn with_position<P: Into<Point<T>>>(p: P, width: T, height: T) -> Self {
        let p = p.into();
        Self::new(p.x(), p.y(), width, height)
    }

    /// Constructs a square `Rect` at position [Point] with `size`.
    pub fn square_with_position<P: Into<Point<T>>>(p: P, size: T) -> Self {
        Self::with_position(p, size, size)
    }

    /// Constructs a `Rect` by providing top-left and bottom-right [Point]s.
    ///
    /// # Panics
    ///
    /// Panics if `p2 <= p1`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let r = Rect::with_points([50, 50], [150, 150]);
    /// assert_eq!(r.as_array(), [50, 50, 100, 100]);
    /// ```
    pub fn with_points<P: Into<Point<T>>>(p1: P, p2: P) -> Self {
        let p1 = p1.into();
        let p2 = p2.into();
        assert!(p2 > p1, "bottom-right point must be greater than top-right",);
        let width = p2.x() - p1.x();
        let height = p2.y() - p1.y();
        Self::new(p1.x(), p1.y(), width, height)
    }

    /// Constructs a `Rect` centered at position `(x, y)` with `width` and `height`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let r = Rect::from_center([50, 50], 100, 100);
    /// assert_eq!(r.as_array(), [0, 0, 100, 100]);
    /// ```
    pub fn from_center<P: Into<Point<T>>>(p: P, width: T, height: T) -> Self {
        let p = p.into();
        let two = T::one() + T::one();
        Self::new(p.x() - width / two, p.y() - height / two, width, height)
    }

    /// Constructs a square `Rect` centered at position `(x, y)` with `size`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let s = Rect::square_from_center([50, 50], 100);
    /// assert_eq!(s.as_array(), [0, 0, 100, 100]);
    /// ```
    pub fn square_from_center<P: Into<Point<T>>>(p: P, size: T) -> Self {
        let p = p.into();
        let two = T::one() + T::one();
        let offset = size / two;
        Self::new(p.x() - offset, p.y() - offset, size, size)
    }

    /// Returns the `size` of the rectangle as a `Point`.
    #[inline]
    pub fn size(&self) -> Point<T> {
        point!(self.width(), self.height())
    }

    /// Reposition the the rectangle.
    #[inline]
    pub fn reposition(&self, x: T, y: T) -> Self {
        Self::new(x, y, self.width(), self.height())
    }

    /// Resize the the rectangle.
    #[inline]
    pub fn resize(&self, width: T, height: T) -> Self {
        Self::new(self.x(), self.y(), width, height)
    }

    /// Offsets a rectangle by shifting coordinates by given amount.
    #[inline]
    pub fn offset<P>(&self, offsets: P) -> Self
    where
        P: Into<Point<T>>,
    {
        let offsets = offsets.into();
        let mut rect = *self;
        for i in 0..=1 {
            rect[i] += offsets[i];
        }
        rect
    }

    /// Offsets a rectangle's size by shifting coordinates by given amount.
    #[inline]
    pub fn offset_size<P>(&self, offsets: P) -> Self
    where
        P: Into<Point<T>>,
    {
        let offsets = offsets.into();
        let mut rect = *self;
        for i in 2..=3 {
            rect[i] += offsets[i - 2];
        }
        rect
    }

    /// Grows a rectangle by a given size.
    #[inline]
    pub fn grow<P>(&self, offsets: P) -> Self
    where
        P: Into<Point<T>>,
    {
        let offsets = offsets.into();
        let mut rect = *self;
        for i in 0..=1 {
            rect[i] -= offsets[i];
        }
        for i in 2..=3 {
            rect[i] += (T::one() + T::one()) * offsets[i - 2];
        }
        rect
    }

    /// Shrinks a rectangle by a given size.
    #[inline]
    pub fn shrink<P>(&self, offsets: P) -> Self
    where
        P: Into<Point<T>>,
    {
        let offsets = offsets.into();
        let mut rect = *self;
        for i in 0..=1 {
            rect[i] += offsets[i];
        }
        for i in 2..=3 {
            rect[i] -= (T::one() + T::one()) * offsets[i - 2];
        }
        rect
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
    #[inline]
    pub fn to_vec(self) -> Vec<T> {
        self.0.to_vec()
    }

    /// Returns the horizontal position of the left edge.
    #[inline]
    pub fn left(&self) -> T {
        self.x()
    }

    /// Set the horizontal position of the left edge.
    #[inline]
    pub fn set_left(&mut self, left: T) {
        self.set_x(left);
    }

    /// Returns the horizontal position of the right edge.
    #[inline]
    pub fn right(&self) -> T {
        self.x() + self.width()
    }

    /// Set the horizontal position of the right edge.
    #[inline]
    pub fn set_right(&mut self, right: T) {
        self.set_x(right - self.width());
    }

    /// Returns the horizontal position of the top edge.
    #[inline]
    pub fn top(&self) -> T {
        self.y()
    }

    /// Set the vertical position of the top edge.
    #[inline]
    pub fn set_top(&mut self, top: T) {
        self.set_y(top);
    }

    /// Returns the vertical position of the bottom edge.
    #[inline]
    pub fn bottom(&self) -> T {
        self.y() + self.height()
    }

    /// Set the vertical position of the bottom edge.
    #[inline]
    pub fn set_bottom(&mut self, bottom: T) {
        self.set_y(bottom - self.height());
    }

    /// Returns the center position as [Point].
    #[inline]
    pub fn center(&self) -> Point<T> {
        let two = T::one() + T::one();
        point!(
            self.x() + self.width() / two,
            self.y() + self.height() / two
        )
    }

    /// Returns the top-left position as [Point].
    #[inline]
    pub fn top_left(&self) -> Point<T> {
        point!(self.left(), self.top())
    }

    /// Returns the top-right position as [Point].
    #[inline]
    pub fn top_right(&self) -> Point<T> {
        point!(self.right(), self.top())
    }

    /// Returns the bottom-left position as [Point].
    #[inline]
    pub fn bottom_left(&self) -> Point<T> {
        point!(self.left(), self.bottom())
    }

    /// Returns the bottom-right position as [Point].
    #[inline]
    pub fn bottom_right(&self) -> Point<T> {
        point!(self.right(), self.bottom())
    }

    /// Returns the four [Point]s that compose this `Rect` as `[top_left, top_right, bottom_right,
    /// bottom_left]`.
    #[inline]
    pub fn points(&self) -> [Point<T>; 4] {
        [
            self.top_left(),
            self.top_right(),
            self.bottom_right(),
            self.bottom_left(),
        ]
    }

    /// Set position centered on a [Point].
    #[inline]
    pub fn center_on<P: Into<Point<T>>>(&mut self, p: P) {
        let p = p.into();
        let two = T::one() + T::one();
        self.set_x(p.x() - self.width() / two);
        self.set_y(p.y() - self.height() / two);
    }

    /// Returns the bounding box for a given rectangle rotated about a `center` by a given
    /// `angle`. Passing `None` for `center` rotates about the top-left point of the rectangle.
    #[inline]
    pub fn rotated(&self, angle: f64, center: Option<Point<T>>) -> Self
    where
        T: Ord + Bounded + AsPrimitive<f64> + NumCast,
    {
        if angle == 0.0 {
            return *self;
        }

        let sin_cos = angle.sin_cos();
        // Determine rotated bounding box
        let [cx, cy]: [f64; 2] = center.unwrap_or_else(|| self.center()).as_().as_array();
        let (sin, cos) = sin_cos;
        let transformed_points = self.points().map(|p| {
            let [x, y]: [f64; 2] = p.as_().as_array();
            point![
                NumCast::from(((x - cx).mul_add(cos, cx) - (y - cy) * sin).round())
                    .expect("valid number cast"),
                NumCast::from(((y - cy).mul_add(cos, (x - cx).mul_add(sin, cy))).round())
                    .expect("valid number cast"),
            ]
        });
        let (min_x, min_y) = transformed_points
            .iter()
            .fold((T::max_value(), T::max_value()), |(min_x, min_y), point| {
                (min_x.min(point.x()), min_y.min(point.y()))
            });
        let (max_x, max_y) = transformed_points
            .iter()
            .fold((T::min_value(), T::min_value()), |(max_x, max_y), point| {
                (max_x.max(point.x()), max_y.max(point.y()))
            });
        Self::with_points([min_x, min_y], [max_x, max_y])
    }
}

impl<T: Num, const N: usize> Contains<T, N> for Rect<T> {
    type Shape = Rect<T>;

    /// Returns whether this rectangle contains a given [Point].
    fn contains_point<P>(&self, p: P) -> bool
    where
        P: Into<Point<T, N>>,
    {
        let p = p.into();
        p.x() >= self.left() && p.x() < self.right() && p.y() >= self.top() && p.y() < self.bottom()
    }

    /// Returns whether this rectangle completely contains another rectangle.
    fn contains_shape<O>(&self, other: O) -> bool
    where
        O: Into<Self::Shape>,
    {
        let other = other.into();
        other.left() >= self.left()
            && other.right() < self.right()
            && other.top() >= self.top()
            && other.bottom() < self.bottom()
    }
}

impl<T: Float> Intersects<T> for Rect<T> {
    type Shape = Rect<T>;

    /// Returns the closest intersection point with a given line and distance along the line or
    /// `None` if there is no intersection.
    fn intersects_line<L>(&self, line: L) -> Option<(Point<T>, T)>
    where
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
                let closest_t = closest.map_or_else(T::infinity, |c| c.1);
                let t = intersection.1;
                if t < closest_t {
                    Some(intersection)
                } else {
                    closest
                }
            })
    }

    /// Returns whether this rectangle intersects with another rectangle.
    fn intersects_shape<O>(&self, other: O) -> bool
    where
        O: Into<Self::Shape>,
    {
        let other = other.into();
        let tl = self.top_left();
        let br = self.bottom_right();
        let otl = other.top_left();
        let obr = other.bottom_right();
        // Both rectangle corner x and y values overlap ranges
        tl.x() < obr.x() && br.x() > otl.x() && tl.y() < otl.y() && br.y() > obr.y()
    }
}

impl Draw for Rect<i32> {
    /// Draw `Rect` to the current [`PixState`] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.rect(*self)
    }
}

impl<T: Copy> From<[T; 3]> for Rect<T> {
    /// Converts `[T; 3]` into `Rect<T>`.
    #[inline]
    fn from([x, y, s]: [T; 3]) -> Self {
        Self([x, y, s, s])
    }
}

impl<T: Copy> From<&[T; 3]> for Rect<T> {
    /// Converts `&[T; 3]` into `Rect<T>`.
    #[inline]
    fn from(&[x, y, s]: &[T; 3]) -> Self {
        Self([x, y, s, s])
    }
}

impl Add<Point<i32>> for Rect {
    type Output = Self;
    fn add(self, p: Point<i32>) -> Self::Output {
        self.offset(p)
    }
}

impl Sub<Point<i32>> for Rect {
    type Output = Self;
    fn sub(self, p: Point<i32>) -> Self::Output {
        self.offset(-p)
    }
}
