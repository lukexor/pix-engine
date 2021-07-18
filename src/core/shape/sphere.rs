//! 3D shape type representing a sphere used for drawing.
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
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Sphere<T = Scalar> {
    /// Center x-coord
    pub x: T,
    /// Center y-coord
    pub y: T,
    /// Center z-coord
    pub z: T,
    /// Radius
    pub radius: T,
}

/// # Constructs a [Sphere].
///
/// ```
/// # use pix_engine::prelude::*;
/// let p = point!(10, 20, 10);
/// let s = sphere!(p, 100);
/// assert_eq!(s.x, 10);
/// assert_eq!(s.y, 20);
/// assert_eq!(s.z, 10);
/// assert_eq!(s.radius, 100);
///
/// let s = sphere!(10, 20, 10, 100);
/// assert_eq!(s.x, 10);
/// assert_eq!(s.y, 20);
/// assert_eq!(s.z, 10);
/// assert_eq!(s.radius, 100);
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
    /// Constructs a `Sphere<T>` at position `(x, y, z)` with `radius.
    pub const fn new(x: T, y: T, z: T, radius: T) -> Self {
        Self { x, y, z, radius }
    }

    /// Constructs a `Sphere<T>` at position [Point] with `radius.
    pub fn with_position<P: Into<Point<T>>>(p: P, radius: T) -> Self {
        let p = p.into();
        Self::new(p.x, p.y, p.z, radius)
    }
}

impl<T: Number> Sphere<T> {
    /// Returns the center [Point].
    pub fn center(&self) -> Point<T> {
        point!(self.x, self.y, self.z)
    }
}

impl<T: Number> Shape<T> for Sphere<T> {
    type Item = Sphere<T>;

    /// Returns whether this sphere contains a given [Point].
    fn contains<O>(&self, other: O) -> bool
    where
        O: Into<Self::Item>,
    {
        let other = other.into();
        let px = other.x - self.x;
        let py = other.y - self.y;
        let pz = other.z - self.z;
        let r = self.radius;
        (px * px + py * py + pz * pz) < r * r
    }

    /// Returns whether this sphere intersects another sphere.
    fn intersects<O>(&self, other: O) -> bool
    where
        O: Into<Self::Item>,
    {
        let other = other.into();
        let px = other.x - self.x;
        let py = other.y - self.y;
        let pz = other.z - self.z;
        let r = other.radius + self.radius;
        (px * px + py * py + pz * pz) < r * r
    }
}
