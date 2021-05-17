//! 2D/3D Point type used for drawing.

use crate::math::Scalar;
use std::ops::*;

/// A `Point`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    /// X-coord
    pub x: i32,
    /// Y-coord
    pub y: i32,
    /// Z-coord
    pub z: i32,
}

/// # Create an [`Point`].
///
/// ```
/// use pix_engine::prelude::*;
///
/// let p = point!(1, 2, 0);
/// assert_eq!(p.values(), [1, 2, 0]);
/// ```
#[macro_export]
macro_rules! point {
    () => {
        point!(0, 0, 0)
    };
    ($x:expr) => {
        point!($x, 0, 0)
    };
    ($x:expr, $y:expr) => {
        point!($x, $y, 0)
    };
    ($x:expr, $y:expr, $z:expr) => {
        $crate::prelude::Point::new_3d($x as i32, $y as i32, $z as i32)
    };
}

impl Point {
    /// Create new `Point`.
    pub fn new<P>(p: P) -> Self
    where
        P: Into<Point>,
    {
        p.into()
    }

    /// Create new 2D `Point`.
    pub fn new_2d(x: i32, y: i32) -> Self {
        Self { x, y, z: 0 }
    }

    /// Create new 3D `Point`.
    pub fn new_3d(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Gets a Point as an array of xyz i32 values.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let p = point!(1, 2, 0);
    /// assert_eq!(p.values(), [1, 2, 0]);
    /// ```
    pub fn values(&self) -> [i32; 3] {
        [self.x, self.y, self.z]
    }
}

impl Index<usize> for Point {
    type Output = i32;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds: the len is 3 but the index is {}", idx),
        }
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds: the len is 3 but the index is {}", idx),
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, v: Point) -> Self::Output {
        Point::new_3d(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, v: Point) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, v: Point) -> Self::Output {
        Point::new_3d(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point::new_3d(-self.x, -self.y, -self.z)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, v: Point) {
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, s: i32) -> Self::Output {
        Point::new_3d(self.x * s, self.y * s, self.z * s)
    }
}

impl Mul<Point> for i32 {
    type Output = Point;

    fn mul(self, v: Point) -> Self::Output {
        Point::new_3d(self * v.x, self * v.x, self * v.z)
    }
}

impl MulAssign<i32> for Point {
    fn mul_assign(&mut self, s: i32) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }
}

impl Div<i32> for Point {
    type Output = Self;

    fn div(self, s: i32) -> Self::Output {
        Point::new_3d(self.x / s, self.y / s, self.z / s)
    }
}

impl Div<Point> for i32 {
    type Output = Point;

    fn div(self, v: Point) -> Self::Output {
        Point::new_3d(self / v.x, self / v.x, self / v.z)
    }
}

impl DivAssign<i32> for Point {
    fn div_assign(&mut self, s: i32) {
        self.x /= s;
        self.y /= s;
        self.z /= s;
    }
}

impl Rem for Point {
    type Output = Self;

    fn rem(mut self, v: Point) -> Self::Output {
        if v.x != 0 {
            self.x %= v.x;
        }
        if v.y != 0 {
            self.y %= v.y;
        }
        if v.z != 0 {
            self.z %= v.z;
        }
        self
    }
}

impl RemAssign for Point {
    fn rem_assign(&mut self, v: Point) {
        if v.x != 0 {
            self.x %= v.x;
        }
        if v.y != 0 {
            self.y %= v.y;
        }
        if v.z != 0 {
            self.z %= v.z;
        }
    }
}

/// From tuple of (i32, i32) to [`Point`].
impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new_2d(x, y)
    }
}

/// From tuple of (i32, i32, i32) to [`Point`].
impl From<(i32, i32, i32)> for Point {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self::new_3d(x, y, z)
    }
}

/// From 2D tuple of (x, y) Scalar to [`Point`].
impl From<(Scalar, Scalar)> for Point {
    fn from((x, y): (Scalar, Scalar)) -> Self {
        let x = x.round() as i32;
        let y = y.round() as i32;
        Self::new_2d(x, y)
    }
}

/// From 3D tuple of (x, y, z) Scalar to [`Point`].
impl From<(Scalar, Scalar, Scalar)> for Point {
    fn from((x, y, z): (Scalar, Scalar, Scalar)) -> Self {
        let x = x.round() as i32;
        let y = y.round() as i32;
        let z = z.round() as i32;
        Self::new_3d(x, y, z)
    }
}

/// Into tuple of (x, y) i32 from [`Point`].
impl Into<(i32, i32)> for Point {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

/// Into tuple of (x, y, z) i32 from [`Point`].
impl Into<(i32, i32, i32)> for Point {
    fn into(self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }
}
