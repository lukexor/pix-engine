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
        rect!($p.x, $p.y, $width, $height)
    };
    ($x:expr, $y:expr, $width:expr, $height:expr$(,)?) => {
        $crate::shape::rect::Rect::new($x, $y, $width, $height)
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
        square!($p.x, $p.y, $size)
    }};
    ($x:expr, $y:expr, $size:expr$(,)?) => {
        $crate::shape::rect::Rect::new($x, $y, $size, $size)
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

/// A `Quad` or quadrilateral, a four-sided polygon. Similar to [Rect] but the
/// angles between edges are not constrained to 90 degrees.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Quad<T = Scalar> {
    /// Point 1
    pub p1: Point<T>,
    /// Point 2
    pub p2: Point<T>,
    /// Point 3
    pub p3: Point<T>,
    /// Point 4
    pub p4: Point<T>,
}

impl<T> Quad<T> {
    /// Constructs a `Quad<T>` with the given [Point]s.
    ///
    /// ```
    /// use pix_engine::prelude::*;
    /// let quad: Quad<i32> = Quad::new([10, 20], [30, 10], [20, 25], [15, 15]);
    /// assert_eq!(quad.p1.values(), [10, 20, 0]);
    /// assert_eq!(quad.p2.values(), [30, 10, 0]);
    /// assert_eq!(quad.p3.values(), [20, 25, 0]);
    /// assert_eq!(quad.p4.values(), [15, 15, 0]);
    /// ```
    pub fn new<P>(p1: P, p2: P, p3: P, p4: P) -> Self
    where
        P: Into<Point<T>>,
    {
        Self {
            p1: p1.into(),
            p2: p2.into(),
            p3: p3.into(),
            p4: p4.into(),
        }
    }

    /// Convert `Quad<T>` to another primitive type using the `as` operator.
    #[inline]
    pub fn as_<U>(self) -> Quad<U>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy,
    {
        Quad::new(self.p1.as_(), self.p2.as_(), self.p3.as_(), self.p4.as_())
    }
}

impl<T: Number> Quad<T> {
    /// Returns `Quad` coordinates as `[x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let quad: Quad<i32> = Quad::new([10, 20], [30, 10], [20, 25], [15, 15]);
    /// assert_eq!(quad.values(), [10, 20, 0, 30, 10, 0, 20, 25, 0, 15, 15, 0]);
    /// ```
    pub fn values(&self) -> [T; 12] {
        let [x1, y1, z1] = self.p1.values();
        let [x2, y2, z2] = self.p2.values();
        let [x3, y3, z3] = self.p3.values();
        let [x4, y4, z4] = self.p4.values();
        [x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]
    }

    /// Returns `Quad` as a [Vec].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let quad: Quad<i32> = Quad::new([10, 20], [30, 10], [20, 25], [15, 15]);
    /// assert_eq!(quad.to_vec(), vec![10, 20, 0, 30, 10, 0, 20, 25, 0, 15, 15, 0]);
    /// ```
    pub fn to_vec(self) -> Vec<T> {
        let [x1, y1, z1] = self.p1.values();
        let [x2, y2, z2] = self.p2.values();
        let [x3, y3, z3] = self.p3.values();
        let [x4, y4, z4] = self.p4.values();
        vec![x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]
    }
}

impl<T: Float> Quad<T> {
    /// Returns `Quad` with values rounded to the nearest integer number. Round half-way cases
    /// away from `0.0`.
    pub fn round(&self) -> Self {
        Self::new(
            self.p1.round(),
            self.p2.round(),
            self.p3.round(),
            self.p4.round(),
        )
    }
}

impl<T> Draw for Quad<T>
where
    T: Number,
    Self: Into<Quad>,
{
    /// Draw `Quad` to the current [PixState] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.quad(*self)
    }
}

impl<T: Number> From<&mut Quad<T>> for Quad<T> {
    fn from(quad: &mut Quad<T>) -> Self {
        quad.to_owned()
    }
}

impl<T: Number> From<&Quad<T>> for Quad<T> {
    fn from(quad: &Quad<T>) -> Self {
        *quad
    }
}

/// Convert [Quad] to `[x1, y1, x2, y2, x3, y3, x4, y4]`.
impl<T: Number> From<Quad<T>> for [T; 8] {
    fn from(quad: Quad<T>) -> Self {
        let [x1, y1, _] = quad.p1.values();
        let [x2, y2, _] = quad.p2.values();
        let [x3, y3, _] = quad.p3.values();
        let [x4, y4, _] = quad.p4.values();
        [x1, y1, x2, y2, x3, y3, x4, y4]
    }
}

/// Convert &[Quad] to `[x1, y1, x2, y2, x3, y3, x4, y4]`.
impl<T: Number> From<&Quad<T>> for [T; 8] {
    fn from(quad: &Quad<T>) -> Self {
        let [x1, y1, _] = quad.p1.values();
        let [x2, y2, _] = quad.p2.values();
        let [x3, y3, _] = quad.p3.values();
        let [x4, y4, _] = quad.p4.values();
        [x1, y1, x2, y2, x3, y3, x4, y4]
    }
}

/// Convert `[x1, y1, x2, y2, x3, y3]` to [Quad].
impl<T: Number, U: Into<T>> From<[U; 8]> for Quad<T> {
    fn from([x1, y1, x2, y2, x3, y3, x4, y4]: [U; 8]) -> Self {
        Self::new([x1, y1], [x2, y2], [x3, y3], [x4, y4])
    }
}

/// Convert `&[x1, y1, x2, y2, x3, y3]` to [Quad].
impl<T: Number, U: Copy + Into<T>> From<&[U; 8]> for Quad<T> {
    fn from(&[x1, y1, x2, y2, x3, y3, x4, y4]: &[U; 8]) -> Self {
        Self::new([x1, y1], [x2, y2], [x3, y3], [x4, y4])
    }
}

/// Convert [Quad] to `[x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]`.
impl<T: Number> From<Quad<T>> for [T; 12] {
    fn from(quad: Quad<T>) -> Self {
        quad.values()
    }
}

/// Convert &[Quad] to `[x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]`.
impl<T: Number> From<&Quad<T>> for [T; 12] {
    fn from(quad: &Quad<T>) -> Self {
        quad.values()
    }
}

/// Convert `[x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]` to [Quad].
impl<T: Number, U: Into<T>> From<[U; 12]> for Quad<T> {
    fn from([x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]: [U; 12]) -> Self {
        Self::new([x1, y1, z1], [x2, y2, z2], [x3, y3, z3], [x4, y4, z4])
    }
}

/// Convert `&[x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]` to [Quad].
impl<T: Number, U: Copy + Into<T>> From<&[U; 12]> for Quad<T> {
    fn from(&[x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4]: &[U; 12]) -> Self {
        Self::new([x1, y1, z1], [x2, y2, z2], [x3, y3, z3], [x4, y4, z4])
    }
}

/// Convert `[Point<U>; 4]` to [Quad].
impl<T, U> From<[Point<U>; 4]> for Quad<T>
where
    T: Number,
    Point<U>: Into<Point<T>>,
{
    fn from([p1, p2, p3, p4]: [Point<U>; 4]) -> Self {
        Self::new(p1, p2, p3, p4)
    }
}

/// Convert `&[Point<U>; 4]` to [Quad].
impl<T, U> From<&[Point<U>; 4]> for Quad<T>
where
    T: Number,
    U: Copy,
    Point<U>: Into<Point<T>>,
{
    fn from(&[p1, p2, p3, p4]: &[Point<U>; 4]) -> Self {
        Self::new(p1, p2, p3, p4)
    }
}

/// Convert [Quad] to `[Point<U>; 4]`.
impl<T, U> From<Quad<U>> for [Point<T>; 4]
where
    T: Number,
    Point<U>: Into<Point<T>>,
{
    fn from(quad: Quad<U>) -> Self {
        [
            quad.p1.into(),
            quad.p2.into(),
            quad.p3.into(),
            quad.p4.into(),
        ]
    }
}

/// Convert &[Quad] to `[Point<U>; 4]`.
impl<T, U> From<&Quad<U>> for [Point<T>; 4]
where
    T: Number,
    U: Copy,
    Point<U>: Into<Point<T>>,
{
    fn from(quad: &Quad<U>) -> Self {
        quad.into()
    }
}

/// Convert `[Vector<U>; 4]` to [Quad].
impl<T, U> From<[Vector<U>; 4]> for Quad<T>
where
    T: Number,
    Vector<U>: Into<Point<T>>,
{
    fn from([v1, v2, v3, v4]: [Vector<U>; 4]) -> Self {
        Self::new(v1, v2, v3, v4)
    }
}

/// Convert `&[Vector<U>; 4]` to [Quad].
impl<T, U> From<&[Vector<U>; 4]> for Quad<T>
where
    T: Number,
    U: Copy,
    Vector<U>: Into<Point<T>>,
{
    fn from(&[v1, v2, v3, v4]: &[Vector<U>; 4]) -> Self {
        Self::new(v1, v2, v3, v4)
    }
}

/// Convert [Quad] to `[Vector<U>; 4]`.
impl<T, U> From<Quad<U>> for [Vector<T>; 4]
where
    T: Number,
    Point<U>: Into<Vector<T>>,
{
    fn from(quad: Quad<U>) -> Self {
        [
            quad.p1.into(),
            quad.p2.into(),
            quad.p3.into(),
            quad.p4.into(),
        ]
    }
}

/// Convert &[Quad] to `[Vector<U>; 4]`.
impl<T, U> From<&Quad<U>> for [Vector<T>; 4]
where
    T: Number,
    U: Copy,
    Point<U>: Into<Vector<T>>,
{
    fn from(quad: &Quad<U>) -> Self {
        quad.into()
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
        let rect: Rect = rect!(10.0, 10.0, 100.0, 100.0);

        // Left
        let line = Line::new([3.0, 7.0], [20.0, 30.0]);
        assert_approx_eq!(
            rect.intersects_line(&line),
            Some((point!(10.0, 16.471), 0.411)),
            0.001
        );

        // Right
        let line = Line::new([150, 50], [90, 30]);
        assert_approx_eq!(
            rect.intersects_line(&line),
            Some((point!(110.0, 36.667), 0.667)),
            0.001
        );

        // Top
        let line = Line::new([50, 5], [70, 30]);
        assert_approx_eq!(
            rect.intersects_line(&line),
            Some((point!(54.0, 10.0), 0.2)),
            0.001
        );

        // Bottom
        let line = Line::new([50, 150], [30, 30]);
        assert_approx_eq!(
            rect.intersects_line(&line),
            Some((point!(43.3333, 110.0), 0.333)),
            0.001
        );
    }
}
