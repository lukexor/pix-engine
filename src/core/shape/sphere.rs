//! [Sphere] type used for drawing.

use crate::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A `Sphere` positioned at `(x, y, z)` with `radius`.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Sphere<T = Scalar> {
    /// Center position
    pub center: Point<T>,
    /// Radius
    pub radius: T,
}

/// # Constructs a [Sphere].
///
/// ```
/// use pix_engine::prelude::*;
/// let s = sphere!([10, 20, 10], 100);
/// assert_eq!(s.center, point!(10, 20, 10));
/// assert_eq!(s.radius, 100);
/// ```
#[macro_export]
macro_rules! sphere {
    ($p:expr, $r:expr$(,)?) => {
        $crate::prelude::Sphere::new($p, $r)
    };
    ([$x:expr, $y:expr, $z:expr], $r:expr$(,)?) => {
        $crate::prelude::Sphere::new([$x, $y, $z], $r)
    };
}

impl<T> Sphere<T> {
    /// Constructs a `Sphere`.
    pub fn new<P>(center: P, radius: T) -> Self
    where
        P: Into<Point<T>>,
    {
        Self {
            center: center.into(),
            radius,
        }
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
        let px = other.center.x - self.center.x;
        let py = other.center.y - self.center.y;
        let pz = other.center.z - self.center.z;
        let r = self.radius;
        (px * px + py * py + pz * pz) < r * r
    }

    /// Returns whether this sphere intersects another sphere.
    fn intersects<O>(&self, other: O) -> bool
    where
        O: Into<Self::Item>,
    {
        let other = other.into();
        let px = other.center.x - self.center.x;
        let py = other.center.y - self.center.y;
        let pz = other.center.z - self.center.z;
        let r = other.radius + self.radius;
        (px * px + py * py + pz * pz) < r * r
    }
}
