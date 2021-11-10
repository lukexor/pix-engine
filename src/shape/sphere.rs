//! A shape type representing spheres used for drawing.
//!
//! # Examples
//!
//! You can create a [Sphere] using [Sphere::new]:
//!
//! ```
//! # use pix_engine::prelude::*;
//! let s = Sphere::new(10, 20, 100, 200);
//! ```
//!
//! ...or by using the [sphere!] macro:
//!
//! ```
//! # use pix_engine::prelude::*;
//! let s = sphere!(10, 20, 15, 200);
//!
//! // using a point
//! let s = sphere!([10, 20, 15], 200);
//! let s = sphere!(point![10, 20, 15], 200);
//! ```

use crate::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A `Sphere` positioned at `(x, y, z)` with `radius`.
///
/// Please see the [module-level documentation] for examples.
///
/// [module-level documentation]: crate::shape::sphere
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Sphere<T = i32>(pub(crate) [T; 4]);

/// Constructs a [Sphere] at position `(x, y, z)` with `radius`.
///
/// ```
/// # use pix_engine::prelude::*;
/// let p = point!(10, 20, 10);
/// let s = sphere!(p, 100);
/// assert_eq!(s.x(), 10);
/// assert_eq!(s.y(), 20);
/// assert_eq!(s.z(), 10);
/// assert_eq!(s.radius(), 100);
///
/// let s = sphere!(10, 20, 10, 100);
/// assert_eq!(s.x(), 10);
/// assert_eq!(s.y(), 20);
/// assert_eq!(s.z(), 10);
/// assert_eq!(s.radius(), 100);
/// ```
#[macro_export]
macro_rules! sphere {
    ($p:expr, $r:expr$(,)?) => {
        $crate::prelude::Sphere::with_position($p, $r)
    };
    ($x:expr, $y:expr, $z:expr, $r:expr$(,)?) => {
        $crate::prelude::Sphere::new($x, $y, $z, $r)
    };
}

impl<T> Sphere<T> {
    /// Constructs a `Sphere` at position `(x, y, z)` with `radius`.
    pub const fn new(x: T, y: T, z: T, radius: T) -> Self {
        Self([x, y, z, radius])
    }
}

impl<T: Copy> Sphere<T> {
    /// Returns `Sphere` as_array as `[x, y, z, radius]`.
    #[inline]
    pub fn as_array(&self) -> [T; 4] {
        self.0
    }

    /// Returns `Sphere` as_array as a byte slice `&[x, y, z, radius]`.
    #[inline]
    pub fn as_bytes(&self) -> &[T; 4] {
        &self.0
    }

    /// Returns `Sphere` as_array as a mutable byte slice `&mut [x, y, z, radius]`.
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [T; 4] {
        &mut self.0
    }
}

impl<T: Num> Sphere<T> {
    /// Constructs a `Sphere` at position [Point] with `radius`.
    pub fn with_position<P: Into<Point<T, 3>>>(p: P, radius: T) -> Self {
        let p = p.into();
        Self::new(p.x(), p.y(), p.z(), radius)
    }

    /// Returns the `x-coordinate` of the sphere.
    #[inline]
    pub fn x(&self) -> T {
        self.0[0]
    }

    /// Sets the `x-coordinate` of the sphere.
    #[inline]
    pub fn set_x(&mut self, x: T) {
        self.0[0] = x;
    }

    /// Returns the `y-coordinate` of the sphere.
    #[inline]
    pub fn y(&self) -> T {
        self.0[1]
    }

    /// Sets the `y-coordinate` of the sphere.
    #[inline]
    pub fn set_y(&mut self, y: T) {
        self.0[1] = y;
    }

    /// Returns the `z-coordinate` of the sphere.
    #[inline]
    pub fn z(&self) -> T {
        self.0[2]
    }

    /// Sets the `z-coordinate` of the sphere.
    #[inline]
    pub fn set_z(&mut self, z: T) {
        self.0[2] = z;
    }

    /// Returns the `radius` of the sphere.
    #[inline]
    pub fn radius(&self) -> T {
        self.0[3]
    }

    /// Sets the `radius` of the sphere.
    #[inline]
    pub fn set_radius(&mut self, radius: T) {
        self.0[3] = radius;
    }

    /// Returns the center [Point].
    pub fn center(&self) -> Point<T, 3> {
        point!(self.x(), self.y(), self.z())
    }
}

impl<T: Num, const N: usize> Contains<T, N> for Sphere<T> {
    type Shape = Sphere<T>;

    /// Returns whether this sphere contains a given [Point].
    fn contains_point<P>(&self, p: P) -> bool
    where
        P: Into<Point<T, N>>,
    {
        let p = p.into();
        let px = p.x() - self.x();
        let py = p.y() - self.y();
        let pz = p.z() - self.z();
        let r = self.radius() * self.radius();
        (px * px + py * py + pz * pz) < r
    }

    /// Returns whether this sphere completely contains another sphere.
    fn contains_shape<O>(&self, other: O) -> bool
    where
        O: Into<Self::Shape>,
    {
        let other = other.into();
        let px = other.x() - self.x();
        let py = other.y() - self.y();
        let pz = other.z() - self.z();
        let r = self.radius() * self.radius();
        (px * px + py * py + pz * pz) < r
    }
}

impl<T: Num> Intersects<T, 3> for Sphere<T> {
    type Shape = Sphere<T>;

    /// Returns the closest intersection point with a given line and distance along the line or
    /// `None` if there is no intersection.
    fn intersects_line<L>(&self, _line: L) -> Option<(Point<T, 3>, T)>
    where
        L: Into<Line<T, 3>>,
    {
        todo!("sphere intersects_line")
    }

    /// Returns whether this sphere intersects another sphere.
    fn intersects_shape<O>(&self, other: O) -> bool
    where
        O: Into<Self::Shape>,
    {
        let other = other.into();
        let px = other.x() - self.x();
        let py = other.y() - self.y();
        let pz = other.z() - self.z();
        let r = other.radius() + self.radius();
        (px * px + py * py + pz * pz) < r * r
    }
}