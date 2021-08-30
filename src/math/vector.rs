//! A [Euclidean] `Vector` in 1D, 2D or 3D space.
//!
//! Each [Vector] represents a 1D, 2D or 3D [Euclidean] (or geometric) vector with a magnitude
//! and a direction. The [Vector] `struct`, however, contains 3 values for `x`, `y`, and `z`. The
//! magnitude and direction are retrieved with the [Vector::mag] and [Vector::heading] methods.
//!
//! Some example uses of a [Vector] include modeling a position, velocity, or acceleration of an
//! object or particle.
//!
//! # Examples
//!
//! You can create a [Vector] using [Vector::new]:
//!
//! ```
//! # use pix_engine::prelude::*;
//! let v = Vector::new(10.0, 20.0, 15.0);
//! ```
//! ...or by using the [vector!] macro:
//!
//! ```
//! # use pix_engine::prelude::*;
//! let v: Vector<i32> = vector!(); // vector at the origin with no direction or magnitude
//!
//! let v = vector!(5.0); // 1D vector on the x-axis with magnitude 5
//!
//! let v = vector!(5.0, 10.0); // 2D vector in the x/y-plane
//!
//! let v = vector!(5.0, 10.0, 7.0); // 3D vector
//! ```
//!
//! You can also create random `Vector`s using [Vector::random_1d], [Vector::random_2d] or
//! [Vector::random_3d] which create unit vectors with magnitudes in the range `-1.0..=1.0`.
//!
//! ```
//! # use pix_engine::prelude::*;
//! let v: Vector<f64> = Vector::random_1d();
//!
//! let v: Vector<f64> = Vector::random_2d();
//!
//! let v: Vector<f64> = Vector::random_3d();
//! ```
//!
//! [Euclidean]: https://en.wikipedia.org/wiki/Euclidean_vector
//!
//! # Examples
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! let v: Vector = vector!(); // Vector placed at the origin (0.0, 0.0, 0.0)
//! assert_eq!(v.values(), [0.0, 0.0, 0.0]);
//!
//! let v = vector!(5.0); // 1D Vector parallel with the X-axis, magnitude 5
//! assert_eq!(v.values(), [5.0, 0.0, 0.0]);
//!
//! let v = vector!(1.0, -3.0); // 2D Vector in the XY-plane
//! assert_eq!(v.values(), [1.0, -3.0, 0.0]);
//!
//! let v = vector!(-1.5, 3.0, 2.2); // 3D Vector
//! assert_eq!(v.values(), [-1.5, 3.0, 2.2]);
//! ```
//!
//! # Other Examples
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! let v: Vector = Vector::random_1d();
//! // `v.values()` will return something like:
//! // [-0.9993116191591512, 0.03709835324533284, 0.0]
//! assert!(v.x >= -1.0 && v.x <= 1.0);
//! assert_eq!(v.y, 0.0);
//! assert_eq!(v.z, 0.0);
//!
//! let v: Vector = Vector::random_2d();
//! // `v.values()` will return something like:
//! // [-0.9993116191591512, 0.03709835324533284, 0.0]
//! assert!(v.x >= -1.0 && v.x <= 1.0);
//! assert!(v.y >= -1.0 && v.y <= 1.0);
//! assert_eq!(v.z, 0.0);
//!
//! let v: Vector = Vector::random_3d();
//! // `v.values()` will return something like:
//! // [-0.40038099206441835, 0.8985763512414204, 0.17959844705110184]
//! assert!(v.x >= -1.0 && v.x <= 1.0);
//! assert!(v.y >= -1.0 && v.y <= 1.0);
//! assert!(v.z >= -1.0 && v.z <= 1.0);
//! ```

use crate::prelude::*;
use num_traits::{clamp, AsPrimitive, Float, NumCast, Signed};
use rand::distributions::uniform::SampleUniform;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    array::IntoIter,
    convert::{TryFrom, TryInto},
    fmt,
    iter::{once, Chain, FromIterator, Once, Sum},
    ops::*,
};

/// A [Euclidean] `Vector` in 1D 2D or 3D space.
///
/// Also known as a geometric vector. A `Vector` has both a magnitude and a direction. The [Vector]
/// struct, however, contains 3 values for `x`, `y`, and `z`.
///
/// The magnitude and direction are retrieved with the [mag] and [heading] methods.
///
/// Some example uses of a [Vector] include modeling a position, velocity, or acceleration of an
/// object or particle.
///
/// [Vector]s can be combined using [vector math][vecmath], so for example two [Vector]s can be added together
/// to form a new [Vector] using `let v3 = v1 + v2` or you can add one [Vector] to another by calling
/// `v1 += v2`.
///
/// [Euclidean]: https://en.wikipedia.org/wiki/Euclidean_vector
/// [mag]: Vector::mag
/// [heading]: Vector::heading
/// [vecmath]: https://en.wikipedia.org/wiki/Vector_(mathematics_and_physics)
#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector<T = Scalar> {
    /// X magnitude
    pub x: T,
    /// Y magnitude
    pub y: T,
    /// Z magnitude
    pub z: T,
}

/// # Constructs a [Vector].
///
/// # Examples
///
/// ```
/// # use pix_engine::prelude::*;
/// let v: Vector = vector!();
/// assert_eq!(v.values(), [0.0, 0.0, 0.0]);
///
/// let v = vector!(1.0);
/// assert_eq!(v.values(), [1.0, 0.0, 0.0]);
///
/// let v = vector!(1.0, 2.0);
/// assert_eq!(v.values(), [1.0, 2.0, 0.0]);
///
/// let v = vector!(1.0, -2.0, 1.0);
/// assert_eq!(v.values(), [1.0, -2.0, 1.0]);
/// ```
#[macro_export]
macro_rules! vector {
    () => {
        $crate::prelude::Vector::default()
    };
    ($x:expr) => {
        $crate::prelude::Vector::with_x($x)
    };
    ($x:expr, $y:expr$(,)?) => {
        $crate::prelude::Vector::with_xy($x, $y)
    };
    ($x:expr, $y:expr, $z:expr$(,)?) => {
        $crate::prelude::Vector::new($x, $y, $z)
    };
}

impl<T> Vector<T> {
    /// Constructs a `Vector<T>`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = Vector::new(2.1, 3.5, 1.0);
    /// assert_eq!(v.values(), [2.1, 3.5, 1.0]);
    /// ```
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Constructs a `Vector<T>` from a [Point].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = point!(1.0, 2.0);
    /// let v = Vector::from_point(p);
    /// assert_eq!(v.values(), [1.0, 2.0, 0.0]);
    /// ```
    pub fn from_point(p: Point<T>) -> Self {
        Self::new(p.x, p.y, p.z)
    }

    /// Set `Vector` coordinates from any type that implements [Into<Vector<T>>].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v1 = Vector::new(2.0, 1.0, 3.0);
    /// assert_eq!(v1.values(), [2.0, 1.0, 3.0]);
    /// v1.set([1.0, 2.0, 4.0]);
    /// assert_eq!(v1.values(), [1.0, 2.0, 4.0]);
    /// ```
    pub fn set(&mut self, [x, y, z]: [T; 3]) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    /// Convert `Vector<T>` to another primitive type using the `as` operator.
    #[inline]
    pub fn as_<U>(self) -> Vector<U>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy,
    {
        Vector::new(self.x.as_(), self.y.as_(), self.z.as_())
    }
}

impl<T: Number> Vector<T> {
    /// Get `Vector` coordinates as `[x, y, z]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = vector!(2.0, 1.0, 3.0);
    /// assert_eq!(v.values(), [2.0, 1.0, 3.0]);
    /// ```
    pub fn values(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    /// Constructs a `Vector<T>` with only an `x` magnitude.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = Vector::with_x(2.1);
    /// assert_eq!(v.values(), [2.1, 0.0, 0.0]);
    /// ```
    pub fn with_x(x: T) -> Self {
        Self::new(x, T::zero(), T::zero())
    }

    /// Constructs a `Vector<T>` with only `x` and ``y` magnitudes.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = Vector::with_xy(2.1, 3.5);
    /// assert_eq!(v.values(), [2.1, 3.5, 0.0]);
    /// ```
    pub fn with_xy(x: T, y: T) -> Self {
        Self::new(x, y, T::zero())
    }

    /// Constructs a `Vector<T>` by multiplying it by the given scale factor.
    pub fn scale<U>(self, s: U) -> Self
    where
        T: Mul<U, Output = T>,
        U: Number,
    {
        self * s
    }

    /// Returns an iterator over the `Vector`s coordinates `[x, y, z]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = vector!(1.0, 2.0, -4.0);
    /// let mut iterator = v.iter();
    ///
    /// assert_eq!(iterator.next(), Some(&1.0));
    /// assert_eq!(iterator.next(), Some(&2.0));
    /// assert_eq!(iterator.next(), Some(&-4.0));
    /// assert_eq!(iterator.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self)
    }

    /// Returns an iterator over the `Vector` that allows modifying each value.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v = vector!(1.0, 2.0, -4.0);
    /// for value in v.iter_mut() {
    ///     *value *= 2.0;
    /// }
    /// assert_eq!(v.values(), [2.0, 4.0, -8.0]);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut::new(self)
    }

    /// Wraps `Vector` around the given width, height, and size (radius).
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v = vector!(200.0, 300.0);
    /// v.wrap_2d(150.0, 400.0, 10.0);
    /// assert_eq!(v.values(), [-10.0, 300.0, 0.0]);
    ///
    /// let mut v = vector!(-100.0, 300.0);
    /// v.wrap_2d(150.0, 400.0, 10.0);
    /// assert_eq!(v.values(), [160.0, 300.0, 0.0]);
    /// ```
    pub fn wrap_2d(&mut self, width: T, height: T, size: T)
    where
        T: Copy + PartialOrd + Signed,
    {
        if self.x > width + size {
            self.x = -size;
        } else if self.x < -size {
            self.x = width + size;
        }
        if self.y > height + size {
            self.y = -size;
        } else if self.y < -size {
            self.y = height + size;
        }
    }

    /// Wraps `Vector` around the given width, height, depth, and size (radius).
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v = vector!(200.0, 300.0, 250.0);
    /// v.wrap_3d(150.0, 400.0, 200.0, 10.0);
    /// assert_eq!(v.values(), [-10.0, 300.0, -10.0]);
    ///
    /// let mut v = vector!(-100.0, 300.0, 250.0);
    /// v.wrap_3d(150.0, 400.0, 200.0, 10.0);
    /// assert_eq!(v.values(), [160.0, 300.0, -10.0]);
    /// ```
    pub fn wrap_3d(&mut self, width: T, height: T, depth: T, size: T)
    where
        T: Copy + PartialOrd + Signed,
    {
        if self.x > width + size {
            self.x = -size;
        } else if self.x < -size {
            self.x = width + size;
        }
        if self.y > height + size {
            self.y = -size;
        } else if self.y < -size {
            self.y = height + size;
        }
        if self.z > depth + size {
            self.z = -size;
        } else if self.z < -size {
            self.z = depth + size;
        }
    }
}

impl<T: Float> Vector<T> {
    /// Returns `Vector` with values rounded to the nearest integer number. Round half-way cases
    /// away from `0.0`.
    pub fn round(&self) -> Self {
        Self::new(self.x.round(), self.y.round(), self.z.round())
    }

    /// Constructs a `Vector<T>` from a reflection about a normal to a line in 2D space or a plane in 3D
    /// space.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v1 = Vector::new(1.0, 1.0, 0.0);
    /// let normal = Vector::new(0.0, 1.0, 0.0);
    /// let v2 = Vector::reflection(v1, normal);
    /// assert_eq!(v2.values(), [-1.0, 1.0, 0.0]);
    /// ```
    pub fn reflection<V>(v: V, normal: V) -> Self
    where
        T: MulAssign,
        V: Into<Vector<T>>,
    {
        let mut v = v.into();
        v.reflect(normal);
        v
    }

    /// Constructs a unit `Vector<T>` of length `1` from another `Vector`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v1 = Vector::new(0.0, 5.0, 0.0);
    /// let v2 = Vector::normalized(v1);
    /// assert_eq!(v2.values(), [0.0, 1.0, 0.0]);
    /// ```
    pub fn normalized<V>(v: V) -> Self
    where
        T: MulAssign,
        V: Into<Vector<T>>,
    {
        let mut v = v.into();
        v.normalize();
        v
    }

    /// Constructs a `Vector<T>` from another `Vector`, rotated by an `angle`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// use pix_engine::math::constants::FRAC_PI_2;
    /// let v1 = Vector::new(10.0, 20.0, 0.0);
    /// let v2 = Vector::rotated(v1, FRAC_PI_2);
    /// assert!(v2.approx_eq(vector![-20.0, 10.0, 0.0], 1e-4));
    /// ```
    pub fn rotated<V>(v: V, angle: T) -> Self
    where
        T: MulAssign,
        V: Into<Vector<T>>,
    {
        let mut v = v.into();
        v.rotate(angle);
        v
    }

    /// Constructs a random unit `Vector<T>` in 1D space.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v: Vector = Vector::random_1d();
    /// assert!(v.x > -1.0 && v.x < 1.0);
    /// assert_eq!(v.y, 0.0);
    /// assert_eq!(v.z, 0.0);
    ///
    /// // May make v's (x, y, z) values something like:
    /// // (0.61554617, 0.0, 0.0) or
    /// // (-0.4695841, 0.0, 0.0) or
    /// // (0.6091097, 0.0, 0.0)
    /// ```
    pub fn random_1d() -> Self
    where
        T: SampleUniform,
    {
        Vector::new(random!(T::one()), T::zero(), T::zero())
    }

    /// Constructs a random unit `Vector<T>` in 2D space.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v: Vector = Vector::random_2d();
    /// assert!(v.x > -1.0 && v.x < 1.0);
    /// assert!(v.y > -1.0 && v.y < 1.0);
    /// assert_eq!(v.z, 0.0);
    ///
    /// // May make v's (x, y, z) values something like:
    /// // (0.61554617, -0.51195765, 0.0) or
    /// // (-0.4695841, -0.14366731, 0.0) or
    /// // (0.6091097, -0.22805278, 0.0)
    /// ```
    pub fn random_2d() -> Self
    where
        T: SampleUniform,
    {
        Self::from_angle(
            random!(NumCast::from(TAU).unwrap_or_else(T::zero)),
            T::one(),
        )
    }

    /// Constructs a random unit `Vector<T>` in 3D space.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v: Vector = Vector::random_3d();
    /// assert!(v.x > -1.0 && v.x < 1.0);
    /// assert!(v.y > -1.0 && v.y < 1.0);
    /// assert!(v.z > -1.0 && v.z < 1.0);
    ///
    /// // May make v's (x, y, z) values something like:
    /// // (0.61554617, -0.51195765, 0.599168) or
    /// // (-0.4695841, -0.14366731, -0.8711202) or
    /// // (0.6091097, -0.22805278, -0.7595902)
    /// ```
    pub fn random_3d() -> Self
    where
        T: SampleUniform,
    {
        let (sin, cos) = random!(NumCast::from(TAU).unwrap_or_else(T::zero)).sin_cos();
        let z: T = random!(-T::one(), T::one()); // Range from -1 to 1
        let z_base = (T::one() - z * z).sqrt();
        let x = z_base * cos;
        let y = z_base * sin;
        Self::new(x, y, z)
    }

    /// Returns `Vector` as a [Vec].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = vector!(1.0, 1.0, 0.0);
    /// assert_eq!(v.to_vec(), vec![1.0, 1.0, 0.0]);
    /// ```
    pub fn to_vec(self) -> Vec<T> {
        vec![self.x, self.y, self.z]
    }

    /// Constructs a 2D unit `Vector<T>` in the XY plane from a given angle. Angle is given as radians
    /// and is unaffected by [AngleMode](crate::prelude::AngleMode).
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = Vector::from_angle(FRAC_PI_4, 15.0);
    /// assert!(v.approx_eq(vector!(10.6066, 10.6066, 0.0), 1e-4));
    /// ```
    pub fn from_angle(angle: T, length: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new(length * cos, length * sin, T::zero())
    }

    /// Returns the magnitude (length) of the `Vector`.
    ///
    /// The formula used is `sqrt(x*x + y*y + z*z)`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v: Vector = vector!(1.0, 2.0, 3.0);
    /// let abs_difference = (v.mag() - 3.7416).abs();
    /// assert!(abs_difference <= 1e-4);
    /// ```
    pub fn mag(&self) -> T {
        self.mag_sq().sqrt()
    }

    /// Returns the squared magnitude (length) of the `Vector`. This is faster if the real length
    /// is not required in the case of comparing vectors.
    ///
    /// The formula used is `x*x + y*y + z*z`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = vector!(1.0, 2.0, 3.0);
    /// assert_eq!(v.mag_sq(), 14.0);
    /// ```
    pub fn mag_sq(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the [dot product](https://en.wikipedia.org/wiki/Dot_product) betwen two `Vector`s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v1 = vector!(1.0, 2.0, 3.0);
    /// let v2 = vector!(2.0, 3.0, 4.0);
    /// let dot_product = v1.dot(v2);
    /// assert_eq!(dot_product, 20.0);
    /// ```
    pub fn dot<V: Into<Vector<T>>>(&self, v: V) -> T {
        let v = v.into();
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    /// Returns the [cross product](https://en.wikipedia.org/wiki/Cross_product) between two
    /// `Vector`s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v1 = vector!(1.0, 2.0, 3.0);
    /// let v2 = vector!(1.0, 2.0, 3.0);
    /// let cross = v1.cross(v2);
    /// assert_eq!(cross.values(), [0.0, 0.0, 0.0]);
    /// ```
    pub fn cross<V: Into<Vector<T>>>(&self, v: V) -> Self {
        let v = v.into();
        Self::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }

    /// Reflect `Vector` about a normal to a line in 2D space or a plane in 3D space.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v = vector!(4.0, 6.0); // Vector heading right and down
    /// let n = vector!(0.0, 1.0); // Surface normal facing up
    /// v.reflect(n); // Reflect about the surface normal (e.g. the x-axis)
    /// assert_eq!(v.x, -4.0);
    /// assert_eq!(v.y, 6.0);
    /// ```
    pub fn reflect<V: Into<Vector<T>>>(&mut self, normal: V)
    where
        T: MulAssign,
    {
        let normal = Self::normalized(normal);
        *self = normal * ((T::one() + T::one()) * self.dot(normal)) - *self;
    }

    /// Set the magnitude (length) of the `Vector`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v = vector!(10.0, 20.0, 2.0);
    /// v.set_mag(10.0);
    /// dbg!(v);
    /// assert!(v.approx_eq(vector![4.4543, 8.9087, 0.8908], 1e-4));
    /// ```
    pub fn set_mag(&mut self, mag: T)
    where
        T: MulAssign,
    {
        self.normalize();
        *self *= mag;
    }

    /// Returns the Euclidean distance between two `Vector`s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v1 = vector!(1.0, 0.0, 0.0);
    /// let v2 = vector!(0.0, 1.0, 0.0);
    /// let dist = v1.dist(v2);
    /// let abs_difference: f64 = (dist - std::f64::consts::SQRT_2).abs();
    /// assert!(abs_difference <= 1e-4);
    /// ```
    pub fn dist<V: Into<Vector<T>>>(&self, v: V) -> T {
        let v = v.into();
        (*self - v).mag()
    }

    /// Normalize the `Vector` to length `1` making it a unit vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v = vector!(10.0, 20.0, 2.0);
    /// v.normalize();
    /// assert!(v.approx_eq(vector!(0.4454, 0.8908, 0.0890), 1e-4));
    /// ```
    pub fn normalize(&mut self)
    where
        T: MulAssign,
    {
        let len = self.mag();
        if len != T::zero() {
            // Multiply by the reciprocol so we don't duplicate a div by zero check
            *self *= len.recip();
        }
    }

    /// Clamp the magnitude (length) of `Vector` to the value given by `max`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v = vector!(10.0, 20.0, 2.0);
    /// v.limit(5.0);
    /// assert!(v.approx_eq(vector!(2.2271, 4.4543,  0.4454), 1e-4));
    /// ```
    pub fn limit(&mut self, max: T)
    where
        T: DivAssign + MulAssign,
    {
        let mag_sq = self.mag_sq();
        if mag_sq > max * max {
            *self /= mag_sq.sqrt();
            *self *= max;
        }
    }

    /// Returns the angular direction of the `Vector`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v: Vector = vector!(10.0, 10.0);
    /// let heading = v.heading();
    /// assert_eq!(heading.to_degrees(), 45.0);
    /// ```
    pub fn heading(&self) -> T {
        self.y.atan2(self.x)
    }

    /// Rotate a 2D `Vector` by an angle in radians, magnitude remains the same. Unaffected by
    /// [AngleMode](crate::prelude::AngleMode).
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// use pix_engine::math::constants::FRAC_PI_2;
    /// let mut v = vector!(10.0, 20.0);
    /// v.rotate(FRAC_PI_2);
    /// assert!(v.approx_eq(vector![-20.0, 10.0, 0.0], 1e-4));
    /// ```
    pub fn rotate(&mut self, angle: T) {
        let new_heading = self.heading() + angle;
        let mag = self.mag();
        let (sin, cos) = new_heading.sin_cos();
        self.x = cos * mag;
        self.y = sin * mag;
    }

    /// Returns the angle between two `Vector`s in radians.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v1 = vector!(1.0, 0.0, 0.0);
    /// let v2 = vector!(0.0, 1.0, 0.0);
    /// let angle = v1.angle_between(v2);
    /// assert_eq!(angle, std::f64::consts::FRAC_PI_2);
    /// ```
    pub fn angle_between<V: Into<Vector<T>>>(&self, v: V) -> T {
        let v = v.into();
        // This should range from -1.0 to 1.0, inclusive but could possibly land outside this range
        // due to floating-point rounding, so we'll need to clamp it to the correct range.
        let dot_mag_product = clamp(self.dot(v) / (self.mag() * v.mag()), -T::one(), T::one());
        dot_mag_product.acos() * self.cross(v).z.signum()
    }

    /// Constructs a `Vector<T>` by linear interpolating between two `Vector`s by a given amount
    /// between `0.0` and `1.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v1 = vector!(1.0, 1.0, 0.0);
    /// let v2 = vector!(3.0, 3.0, 0.0);
    /// let v3 = v1.lerp(v2, 0.5);
    /// assert_eq!(v3.values(), [2.0, 2.0, 0.0]);
    /// ```
    pub fn lerp<V: Into<Vector<T>>>(&self, v: V, amt: T) -> Self {
        let lerp = |start, stop, amt| amt * (stop - start) + start;
        let amt = clamp(amt, T::zero(), T::one());

        let v = v.into();
        Self::new(
            lerp(self.x, v.x, amt),
            lerp(self.y, v.y, amt),
            lerp(self.z, v.z, amt),
        )
    }

    /// Returns whether two `Vector`s are approximately equal.
    pub fn approx_eq(&self, other: Vector<T>, epsilon: T) -> bool {
        let xd = (self.x - other.x).abs();
        let yd = (self.y - other.y).abs();
        let zd = (self.z - other.z).abs();
        xd < epsilon && yd < epsilon && zd < epsilon
    }
}

impl<T: Number> From<&mut Vector<T>> for Vector<T> {
    fn from(v: &mut Vector<T>) -> Self {
        v.clone()
    }
}

impl<T: Number> From<&Vector<T>> for Vector<T> {
    fn from(v: &Vector<T>) -> Self {
        *v
    }
}

impl<T: Number> ExactSizeIterator for Iter<'_, T> {}
impl<T: Number> ExactSizeIterator for IterMut<'_, T> {}

impl<T: Number> FromIterator<T> for Vector<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut xyz = [T::zero(), T::zero(), T::zero()];
        for (i, v) in iter.into_iter().enumerate() {
            xyz[i] = v;
        }
        let [x, y, z] = xyz;
        Self::new(x, y, z)
    }
}

impl<T> IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item, 3>;

    /// Owned `Vector<T>` iterator over `[x, y, z]`.
    ///
    /// This struct is created by the [into_iter](Vector::into_iter) method on [Vector]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = vector!(1.0, 2.0, -4.0);
    /// let mut iterator = v.into_iter();
    ///
    /// assert_eq!(iterator.next(), Some(1.0));
    /// assert_eq!(iterator.next(), Some(2.0));
    /// assert_eq!(iterator.next(), Some(-4.0));
    /// assert_eq!(iterator.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new([self.x, self.y, self.z])
    }
}

/// Immutable iterator over `[x, y, z]` of `Vector`.
///
/// This struct is created by the [iter](Vector::iter) method on [Vector]s.
///
/// # Example
///
/// ```
/// # use pix_engine::prelude::*;
/// let v = vector!(1.0, 2.0, -4.0);
/// let mut iterator = v.iter();
///
/// assert_eq!(iterator.next(), Some(&1.0));
/// assert_eq!(iterator.next(), Some(&2.0));
/// assert_eq!(iterator.next(), Some(&-4.0));
/// assert_eq!(iterator.next(), None);
/// ```
#[derive(Debug, Clone)]
pub struct Iter<'a, T = Scalar> {
    inner: [&'a T; 3],
    current: usize,
}

impl<'a, T: Number> Iter<'a, T> {
    fn new(v: &'a Vector<T>) -> Self {
        Self {
            inner: [&v.x, &v.y, &v.z],
            current: 0,
        }
    }
}

impl<'a, T: Number> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current > 2 {
            return None;
        }
        let next = self.inner[self.current];
        self.current += 1;
        Some(next)
    }
}

impl<'a, T: Number> IntoIterator for &'a Vector<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

type ThreeChain<T> = Chain<Chain<Once<T>, Once<T>>, Once<T>>;

/// Mutable iterator over `[x, y, z]` of `Vector`.
///
/// This struct is created by the [iter_mut](Vector::iter_mut) method on [Vector]s.
///
/// # Example
///
/// ```
/// # use pix_engine::prelude::*;
/// let mut v = vector!(1.0, 2.0, -4.0);
/// for value in v.iter_mut() {
///     *value *= 2.0;
/// }
/// assert_eq!(v.values(), [2.0, 4.0, -8.0]);
/// ```
#[derive(Debug)]
pub struct IterMut<'a, T = Scalar> {
    inner: ThreeChain<&'a mut T>,
}

impl<'a, T: Number> IterMut<'a, T> {
    fn new(v: &'a mut Vector<T>) -> Self {
        Self {
            inner: once(&mut v.x).chain(once(&mut v.y)).chain(once(&mut v.z)),
        }
    }
}

impl<'a, T: Number> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, T: Number> IntoIterator for &'a mut Vector<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

// Operations

impl<T: Number> Index<usize> for Vector<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds: the len is 3 but the index is {}", idx),
        }
    }
}

impl<T: Number> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds: the len is 3 but the index is {}", idx),
        }
    }
}

/// [Vector] + [Vector] yields a [Vector].
impl<T: Number> Add for Vector<T> {
    type Output = Self;
    fn add(self, v: Vector<T>) -> Self::Output {
        Vector::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

/// [Vector] + U.
impl<T, U> Add<U> for Vector<T>
where
    T: Number + Add<U, Output = T>,
    U: Number,
{
    type Output = Self;
    fn add(self, val: U) -> Self::Output {
        Self::Output::new(self.x + val, self.y + val, self.z + val)
    }
}

/// [Vector] += [Vector].
impl<T> AddAssign for Vector<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, v: Vector<T>) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

/// [Vector] += U.
impl<T, U> AddAssign<U> for Vector<T>
where
    T: AddAssign<U>,
    U: Number,
{
    fn add_assign(&mut self, val: U) {
        self.x += val;
        self.y += val;
        self.z += val;
    }
}

/// [Vector] - [Vector] yields a [Vector].
impl<T: Number> Sub for Vector<T> {
    type Output = Self;
    fn sub(self, v: Vector<T>) -> Self::Output {
        Vector::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

/// [Vector] - U.
impl<T, U> Sub<U> for Vector<T>
where
    T: Number + Sub<U, Output = T>,
    U: Number,
{
    type Output = Self;
    fn sub(self, val: U) -> Self::Output {
        Self::Output::new(self.x - val, self.y - val, self.z - val)
    }
}

/// [Vector] -= [Vector].
impl<T> SubAssign for Vector<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, v: Vector<T>) {
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
    }
}

/// [Vector] -= U.
impl<T, U> SubAssign<U> for Vector<T>
where
    T: SubAssign<U>,
    U: Number,
{
    fn sub_assign(&mut self, val: U) {
        self.x -= val;
        self.y -= val;
        self.z -= val;
    }
}

/// ![Vector].
impl<T> Neg for Vector<T>
where
    T: Number + Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

/// [Vector] * U.
impl<T, U> Mul<U> for Vector<T>
where
    T: Number + Mul<U, Output = T>,
    U: Number,
{
    type Output = Self;
    fn mul(self, s: U) -> Self::Output {
        Vector::new(self.x * s, self.y * s, self.z * s)
    }
}

/// [Vector] *= U.
impl<T, U> MulAssign<U> for Vector<T>
where
    T: Number + MulAssign<U>,
    U: Number,
{
    fn mul_assign(&mut self, s: U) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }
}

/// [Vector] / U.
impl<T, U> Div<U> for Vector<T>
where
    T: Number + Div<U, Output = T>,
    U: Number,
{
    type Output = Self;
    fn div(self, s: U) -> Self::Output {
        Vector::new(self.x / s, self.y / s, self.z / s)
    }
}

/// [Vector] /= U.
impl<T, U> DivAssign<U> for Vector<T>
where
    T: Number + DivAssign<U>,
    U: Number,
{
    fn div_assign(&mut self, s: U) {
        self.x /= s;
        self.y /= s;
        self.z /= s;
    }
}

// Required because of orphan rules.
macro_rules! impl_primitive_mul {
    ($($target:ty),*) => {
        $(
            impl Mul<Vector<$target>> for $target {
                type Output = Vector<$target>;
                fn mul(self, v: Vector<$target>) -> Self::Output {
                    Vector::new(self * v.x, self * v.y, self * v.z)
                }
            }
        )*
    };
}

impl_primitive_mul!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);

impl<T> Sum for Vector<T>
where
    T: Number,
    Self: Add<Output = Self>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let v = Vector::new(T::zero(), T::zero(), T::zero());
        iter.fold(v, |a, b| a + b)
    }
}

impl<'a, T> Sum<&'a Vector<T>> for Vector<T>
where
    T: Number,
    Self: Add<Output = Self>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        let v = Vector::new(T::zero(), T::zero(), T::zero());
        iter.fold(v, |a, b| a + *b)
    }
}

macro_rules! impl_from {
    ($from:ty => $($to:ty),*) => {
        $(
            impl From<Vector<$from>> for Vector<$to> {
                fn from(v: Vector<$from>) -> Self {
                    Self::new(v.x.into(), v.y.into(), v.z.into())
                }
            }
        )*
    };
}

impl_from!(i8 => i16, i32, i64, isize, f32, f64);
impl_from!(u8 => i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);
impl_from!(i16 => i32, i64, isize, f32, f64);
impl_from!(u16 => i32, u32, i64, u64, usize, f32, f64);
impl_from!(i32 => i64, f64);
impl_from!(u32 => i64, u64, f64);
impl_from!(f32 => f64);

macro_rules! impl_try_from {
    ($from:ty => $($to:ty),*) => {
        $(
            impl TryFrom<Vector<$from>> for Vector<$to> {
                type Error = std::num::TryFromIntError;
                fn try_from(v: Vector<$from>) -> Result<Self, Self::Error> {
                    Ok(Self::new(v.x.try_into()?, v.y.try_into()?, v.z.try_into()?))
                }
            }
        )*
    };
}

impl_try_from!(i8 => u8, u16, u32, u64, usize);
impl_try_from!(u8 => i8);
impl_try_from!(i16 => i8, u8, u16, u32, u64, usize);
impl_try_from!(u16 => i8, u8, i16, isize);
impl_try_from!(i32 => i8, u8, i16, u32, u64, isize, usize);
impl_try_from!(u32 => i8, u8, i16, i32, isize, usize);
impl_try_from!(i64 => i8, u8, i16, i32, u32, u64, isize, usize);
impl_try_from!(u64 => i8, u8, i16, i32, u32, i64, isize, usize);
impl_try_from!(isize => i8, u8, i16, u16, i32, u32, i64, u64, usize);
impl_try_from!(usize => i8, u8, i16, u16, i32, u32, i64, u64, isize);

/// Convert [Vector] to `[x]`.
impl<T: Number> From<Vector<T>> for [T; 1] {
    fn from(v: Vector<T>) -> Self {
        [v.x]
    }
}
/// Convert &[Vector] to `[x]`.
impl<T: Number> From<&Vector<T>> for [T; 1] {
    fn from(v: &Vector<T>) -> Self {
        [v.x]
    }
}

/// Convert [Vector] to `[x, y]`.
impl<T: Number> From<Vector<T>> for [T; 2] {
    fn from(v: Vector<T>) -> Self {
        [v.x, v.y]
    }
}
/// Convert &[Vector] to `[x, y]`.
impl<T: Number> From<&Vector<T>> for [T; 2] {
    fn from(v: &Vector<T>) -> Self {
        [v.x, v.y]
    }
}

/// Convert [Vector] to `[x, y, z]`.
impl<T: Number> From<Vector<T>> for [T; 3] {
    fn from(v: Vector<T>) -> Self {
        [v.x, v.y, v.z]
    }
}
/// Convert &[Vector] to `[x, y, z]`.
impl<T: Number> From<&Vector<T>> for [T; 3] {
    fn from(v: &Vector<T>) -> Self {
        [v.x, v.y, v.z]
    }
}

/// Convert `[U; 1]` to [Vector].
impl<T: Number, U: Into<T>> From<[U; 1]> for Vector<T> {
    fn from([x]: [U; 1]) -> Self {
        Self::new(x.into(), T::zero(), T::zero())
    }
}
/// Convert `&[U; 1]` to [Vector].
impl<T: Number, U: Copy + Into<T>> From<&[U; 1]> for Vector<T> {
    fn from(&[x]: &[U; 1]) -> Self {
        Self::new(x.into(), T::zero(), T::zero())
    }
}

/// Convert `[U; 2]` to [Vector].
impl<T: Number, U: Into<T>> From<[U; 2]> for Vector<T> {
    fn from([x, y]: [U; 2]) -> Self {
        Self::new(x.into(), y.into(), T::zero())
    }
}
/// Convert `&[U; 2]` to [Vector].
impl<T: Number, U: Copy + Into<T>> From<&[U; 2]> for Vector<T> {
    fn from(&[x, y]: &[U; 2]) -> Self {
        Self::new(x.into(), y.into(), T::zero())
    }
}

/// Convert `[U; 3]` to [Vector].
impl<T: Number, U: Into<T>> From<[U; 3]> for Vector<T> {
    fn from([x, y, z]: [U; 3]) -> Self {
        Self::new(x.into(), y.into(), z.into())
    }
}
/// Convert `&[U; 3]` to [Vector].
impl<T: Number, U: Copy + Into<T>> From<&[U; 3]> for Vector<T> {
    fn from(&[x, y, z]: &[U; 3]) -> Self {
        Self::new(x.into(), y.into(), z.into())
    }
}

/// Converts [Point] to [Vector].
impl<T: Number, U: Into<T>> From<Point<U>> for Vector<T> {
    fn from(p: Point<U>) -> Self {
        Self::new(p.x.into(), p.y.into(), p.z.into())
    }
}

/// Converts &[Point] to [Vector].
impl<T: Number, U: Copy + Into<T>> From<&Point<U>> for Vector<T> {
    fn from(p: &Point<U>) -> Self {
        Self::new(p.x.into(), p.y.into(), p.z.into())
    }
}

/// Converts [Vector] to [Point].
impl<T: Number, U: Into<T>> From<Vector<U>> for Point<T> {
    fn from(v: Vector<U>) -> Self {
        Self::new(v.x.into(), v.y.into(), v.z.into())
    }
}

/// Converts &[Vector] to [Point].
impl<T: Number, U: Copy + Into<T>> From<&Vector<U>> for Point<T> {
    fn from(v: &Vector<U>) -> Self {
        Self::new(v.x.into(), v.y.into(), v.z.into())
    }
}

/// Display [Vector] as "[x, y, z]".
impl<T> fmt::Display for Vector<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_approx_eq {
        ($v1:expr, $v2:expr) => {
            assert_approx_eq!($v1, $v2, f64::EPSILON);
        };
        ($v1:expr, $v2:expr, $e:expr) => {
            let [v1, v2, v3] = $v1;
            let [ov1, ov2, ov3] = $v2;
            let v1d = (v1 - ov1).abs();
            let v2d = (v2 - ov2).abs();
            let v3d = (v3 - ov3).abs();
            assert!(v1d < $e, "v1: ({} - {}) < {}", v1, ov1, $e);
            assert!(v2d < $e, "v2: ({} - {}) < {}", v2, ov2, $e);
            assert!(v3d < $e, "v3: ({} - {}) < {}", v3, ov3, $e);
        };
    }

    macro_rules! test_ops {
        ($val:expr, $e:expr) => {
            // Mul<T> for Vector
            let v = vector!(2.0, -5.0, 0.0) * $val;
            assert_approx_eq!(v.values(), [4.0, -10.0, 0.0], $e);

            // Mul<Vector> for T
            let v = $val * vector!(2.0, -5.0, 0.0);
            assert_approx_eq!(v.values(), [4.0, -10.0, 0.0], $e);

            // MulAssign<T> for Vector
            let mut v = vector!(2.0, -5.0, 0.0);
            v *= $val;
            assert_approx_eq!(v.values(), [4.0, -10.0, 0.0], $e);

            // Div<T> for Vector
            let v = vector!(1.0, -5.0, 0.0) / $val;
            assert_approx_eq!(v.values(), [0.5, -2.5, 0.0], $e);

            // DivAssign<T> for Vector
            let mut v = vector!(2.0, -5.0, 0.0);
            v /= $val;
            assert_approx_eq!(v.values(), [1.0, -2.5, 0.0], $e);
        };
    }

    #[test]
    fn test_ops() {
        // Add
        let v1 = vector!(2.0, 5.0, 1.0);
        let v2 = vector!(1.0, 5.0, -1.0);
        let v3 = v1 + v2;
        assert_approx_eq!(v3.values(), [3.0, 10.0, 0.0]);

        // AddAssign
        let mut v1 = vector!(2.0, 5.0, 1.0);
        let v2 = vector!(1.0, 5.0, -1.0);
        v1 += v2;
        assert_approx_eq!(v1.values(), [3.0, 10.0, 0.0]);

        // Sub
        let v1 = vector!(2.0, 1.0, 2.0);
        let v2 = vector!(1.0, 5.0, 3.0);
        let v3 = v1 - v2;
        assert_approx_eq!(v3.values(), [1.0, -4.0, -1.0]);

        // SubAssign
        let mut v1 = vector!(2.0, 1.0, 2.0);
        let v2 = vector!(1.0, 5.0, 3.0);
        v1 -= v2;
        assert_approx_eq!(v1.values(), [1.0, -4.0, -1.0]);

        test_ops!(2.0f32, f32::EPSILON);
        test_ops!(2.0f64, f64::EPSILON);
    }

    #[test]
    fn test_slice_conversions() {
        let _: Vector<u8> = [50u8].into();
        let _: Vector<i8> = [50i8].into();
        let _: Vector<u16> = [50u16].into();
        let _: Vector<i16> = [50i16].into();
        let _: Vector<u32> = [50u32].into();
        let _: Vector<i32> = [50i32].into();
        let _: Vector<f32> = [50.0f32].into();
        let _: Vector<f64> = [50.0f64].into();

        let _: Vector<u8> = [50u8, 100].into();
        let _: Vector<i8> = [50i8, 100].into();
        let _: Vector<u16> = [50u16, 100].into();
        let _: Vector<i16> = [50i16, 100].into();
        let _: Vector<u32> = [50u32, 100].into();
        let _: Vector<i32> = [50i32, 100].into();
        let _: Vector<f32> = [50.0f32, 100.0].into();
        let _: Vector<f64> = [50.0f64, 100.0].into();

        let _: Vector<u8> = [50u8, 100, 55].into();
        let _: Vector<i8> = [50i8, 100, 55].into();
        let _: Vector<u16> = [50u16, 100, 55].into();
        let _: Vector<i16> = [50i16, 100, 55].into();
        let _: Vector<u32> = [50u32, 100, 55].into();
        let _: Vector<i32> = [50i32, 100, 55].into();
        let _: Vector<f32> = [50.0f32, 100.0, 55.0].into();
        let _: Vector<f64> = [50.0f64, 100.0, 55.0].into();
    }
}
