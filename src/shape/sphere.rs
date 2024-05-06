//! A shape type representing spheres used for drawing.
//!
//! # Examples
//!
//! You can create a [Sphere] using [`Sphere::new`]:
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
/// [module-level documentation]: mod@crate::shape::sphere
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
#[must_use]
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
    /// Returns `Sphere` coordinates  as `[x, y, z, radius]`.
    #[inline]
    pub fn coords(&self) -> [T; 4] {
        self.0
    }

    /// Returns `Sphere` coordinates as a mutable slice `&mut [x, y, z, radius]`.
    #[inline]
    pub fn coords_mut(&mut self) -> &mut [T; 4] {
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

impl<T: Num> Contains<Point<T>> for Sphere<T> {
    /// Returns whether this sphere contains a given [Point].
    fn contains(&self, p: Point<T>) -> bool {
        let px = p.x() - self.x();
        let py = p.y() - self.y();
        let pz = p.z() - self.z();
        let r = self.radius() * self.radius();
        (px * px + py * py + pz * pz) < r
    }
}

impl<T: Num> Contains<Sphere<T>> for Sphere<T> {
    /// Returns whether this sphere completely contains another sphere.
    fn contains(&self, sphere: Sphere<T>) -> bool {
        let px = sphere.x() - self.x();
        let py = sphere.y() - self.y();
        let pz = sphere.z() - self.z();
        let r = self.radius() * self.radius();
        (px * px + py * py + pz * pz) < r
    }
}

impl<T: Num> Intersects<Sphere<T>> for Sphere<T> {
    // FIXME: Provide a better intersection result
    type Result = ();

    /// Returns whether this sphere intersects another sphere.
    fn intersects(&self, sphere: Sphere<T>) -> Option<Self::Result> {
        let px = sphere.x() - self.x();
        let py = sphere.y() - self.y();
        let pz = sphere.z() - self.z();
        let r = sphere.radius() + self.radius();
        if (px * px + py * py + pz * pz) < r * r {
            Some(())
        } else {
            None
        }
    }
}
