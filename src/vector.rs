//! 1D, 2D and 3D Euclidean [`Vector`] functions.
//!
//! Each [`Vector`] represents a 1D, 2D or 3D Euclidean (or geometric) vector with a magnitude and a
//! direction. The `Vector`, however, contains 3 values for `x`, `y`, and `z`. The magnitude and direction are
//! retrieved with the [`mag`](Vector::mag) and [`heading`](Vector::heading) methods.
//!
//! Some example uses of a `Vector` include modeling a position, velocity, or acceleration of an
//! object or particle.
//!
//! The [`vector!`] macro allows for flexible construction which takes 0-3 parameters:
//!
//! - Zero parameters constructs a vector at the origin `(0.0, 0.0, 0.0)`
//! - One, Two, or Three parameters constructs a vector with `x`, `y`, and `z` set with remaining
//!   values set to `0.0`.
//!
//! If you want randomized vectors, use the [`random_1d`](Vector::random_1d),
//! [`random_2d`](Vector::random_2d) and [`random_3d`](Vector::random_3d) methods which create unit
//! vectors with magnitudes in the range `-1.0..=1.0`.
//!
//! # Examples
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! let v: Vector<f64> = vector!(); // Vector placed at the origin (0.0, 0.0, 0.0)
//! assert_eq!(v.get(), [0.0, 0.0, 0.0]);
//!
//! let v = vector!(5.0); // 1D Vector parallel with the X-axis, magnitude 5
//! assert_eq!(v.get(), [5.0, 0.0, 0.0]);
//!
//! let v = vector!(1.0, -3.0); // 2D Vector in the XY-plane
//! assert_eq!(v.get(), [1.0, -3.0, 0.0]);
//!
//! let v = vector!(-1.5, 3.0, 2.2); // 3D Vector
//! assert_eq!(v.get(), [-1.5, 3.0, 2.2]);
//! ```
//!
//! # Other Examples
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! let v: Vector<f64> = Vector::random_1d();
//! // `v.get()` will return something like:
//! // [-0.9993116191591512, 0.03709835324533284, 0.0]
//! assert!(v.x >= -1.0 && v.x <= 1.0);
//! assert_eq!(v.y, 0.0);
//! assert_eq!(v.z, 0.0);
//!
//! let v: Vector<f64> = Vector::random_2d();
//! // `v.get()` will return something like:
//! // [-0.9993116191591512, 0.03709835324533284, 0.0]
//! assert!(v.x >= -1.0 && v.x <= 1.0);
//! assert!(v.y >= -1.0 && v.y <= 1.0);
//! assert_eq!(v.z, 0.0);
//!
//! let v: Vector<f64> = Vector::random_3d();
//! // `v.get()` will return something like:
//! // [-0.40038099206441835, 0.8985763512414204, 0.17959844705110184]
//! assert!(v.x >= -1.0 && v.x <= 1.0);
//! assert!(v.y >= -1.0 && v.y <= 1.0);
//! assert!(v.z >= -1.0 && v.z <= 1.0);
//! ```

use crate::prelude::{constants::TAU, random, Point, Scalar};
use num_traits::{clamp, AsPrimitive, Float, Num, NumCast};
use rand::distributions::uniform::SampleUniform;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    array::IntoIter,
    fmt,
    iter::{once, Chain, FromIterator, Once, Sum},
    ops::*,
};

/// Represents a Euclidiean (also known as geometric) `Vector` in 2D or 3D space. A `Vector` has
/// both a magnitude and a direction. The `Vector`, however, contains 3 values for `x`, `y`, and `z`.
///
/// The magnitude and direction are retrieved with the [`mag`](Vector::mag) and
/// [`heading`](Vector::heading) methods.
///
/// Some example uses of a `Vector` include modeling a position, velocity, or acceleration of an
/// object or particle.
///
/// `Vector`s can be combined using "vector" math, so for example two `Vector`s can be added together
/// to form a new `Vector` using `let v3 = v1 + v2` or you can add one `Vector` to another by calling
/// `v1 += v2`.
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

/// # Constructs a [`Vector<T>`].
///
/// # Examples
///
/// ```
/// use pix_engine::prelude::*;
///
/// let v: Vector<f64> = vector!();
/// assert_eq!(v.get(), [0.0, 0.0, 0.0]);
///
/// let v = vector!(1.0);
/// assert_eq!(v.get(), [1.0, 0.0, 0.0]);
///
/// let v = vector!(1.0, 2.0);
/// assert_eq!(v.get(), [1.0, 2.0, 0.0]);
///
/// let v = vector!(1.0, -2.0, 1.0);
/// assert_eq!(v.get(), [1.0, -2.0, 1.0]);
/// ```
#[macro_export]
macro_rules! vector {
    () => {
        $crate::vector::Vector::default()
    };
    ($x:expr) => {
        $crate::vector::Vector::new_x($x)
    };
    ($x:expr, $y:expr$(,)?) => {
        $crate::vector::Vector::new_xy($x, $y)
    };
    ($x:expr, $y:expr, $z:expr$(,)?) => {
        $crate::vector::Vector::new($x, $y, $z)
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
    /// assert_eq!(v.get(), [2.1, 3.5, 1.0]);
    /// ```
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Constructs a `Vector<T>` from a [`Point<T>`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p: Point<f64> = point!(1.0, 2.0);
    /// let v = Vector::from_point(p);
    /// assert_eq!(v.get(), [1.0, 2.0, 0.0]);
    /// ```
    pub fn from_point(p: impl Into<Point<T>>) -> Self {
        let p = p.into();
        Self::new(p.x, p.y, p.z)
    }

    /// Copy the current `Vector`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v1 = vector!(1.0, 0.0, 1.0);
    /// let mut v2 = v1.copy();
    /// v2.x = 2.0;
    /// assert_eq!(v1.get(), [1.0, 0.0, 1.0]);
    /// assert_eq!(v2.get(), [2.0, 0.0, 1.0]);
    /// ```
    pub fn copy(&self) -> Self
    where
        T: Copy,
    {
        *self
    }

    /// Get `Vector` coordinates as `[x, y, z]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = vector!(2.0, 1.0, 3.0);
    /// assert_eq!(v.get(), [2.0, 1.0, 3.0]);
    /// ```
    pub fn get(&self) -> [T; 3]
    where
        T: Copy,
    {
        [self.x, self.y, self.z]
    }

    /// Set `Vector` coordinates from any type that implements [`Into<Vector<T>>`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v1 = Vector::new(2.0, 1.0, 3.0);
    /// assert_eq!(v1.get(), [2.0, 1.0, 3.0]);
    /// v1.set([1.0, 2.0, 4.0]);
    /// assert_eq!(v1.get(), [1.0, 2.0, 4.0]);
    ///
    /// let v2 = Vector::new(-2.0, 5.0, 1.0);
    /// v1.set(v2);
    /// assert_eq!(v1.get(), [-2.0, 5.0, 1.0]);
    /// ```
    pub fn set(&mut self, v: impl Into<Vector<T>>) {
        let v = v.into();
        self.x = v.x;
        self.y = v.y;
        self.z = v.z;
    }

    /// Returns an itereator over the `Vector`s coordinates `[x, y, z]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v: Vector<f64> = vector!(1.0, 2.0, -4.0);
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

    /// Returns an itereator over the `Vector` that allows modifying each value.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v: Vector<f64> = vector!(1.0, 2.0, -4.0);
    /// for value in v.iter_mut() {
    ///     *value *= 2.0;
    /// }
    /// assert_eq!(v.get(), [2.0, 4.0, -8.0]);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut::new(self)
    }

    /// Convert [`Vector<T>`] to [`Vector<U>`] using `as` operator.
    #[inline]
    pub fn as_<U>(self) -> Vector<U>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy,
    {
        Vector::new(self.x.as_(), self.y.as_(), self.z.as_())
    }
}

impl<T: Num> Vector<T> {
    /// Constructs a `Vector<T>` with only an `x` magnitude.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = Vector::new_x(2.1);
    /// assert_eq!(v.get(), [2.1, 0.0, 0.0]);
    /// ```
    pub fn new_x(x: T) -> Self {
        Self::new(x, T::zero(), T::zero())
    }

    /// Constructs a `Vector<T>` with only `x` and `y magnitudes.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = Vector::new_xy(2.1, 3.5);
    /// assert_eq!(v.get(), [2.1, 3.5, 0.0]);
    /// ```
    pub fn new_xy(x: T, y: T) -> Self {
        Self::new(x, y, T::zero())
    }
}

impl<T> Vector<T>
where
    T: Float,
{
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
    /// assert_eq!(v2.get(), [-1.0, 1.0, 0.0]);
    /// ```
    pub fn reflection<V>(v: V, normal: V) -> Self
    where
        V: Into<Vector<T>>,
        T: MulAssign,
    {
        let mut v = v.into();
        v.reflect(normal);
        v
    }

    /// Constructs a unit `Vector<T>` of length `1.0` from another `Vector`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v1 = Vector::new(0.0, 5.0, 0.0);
    /// let v2 = Vector::normalized(v1);
    /// assert_eq!(v2.get(), [0.0, 1.0, 0.0]);
    /// ```
    pub fn normalized(v: impl Into<Vector<T>>) -> Self
    where
        T: MulAssign,
    {
        let mut v = v.into();
        v.normalize();
        v
    }

    /// Constructs a 2D unit `Vector<T>` in the XY plane from a given angle. Angle is given as radians
    /// and is unaffected by [`AngleMode`](crate::prelude::AngleMode).
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v: Vector<f64> = Vector::from_angle(30.0, 15.0);
    /// let abs_difference_x = (v.x - 2.3137).abs();
    /// let abs_difference_y = (v.y - (-14.8204)).abs();
    /// assert!(abs_difference_x <= 1e-4);
    /// assert!(abs_difference_y <= 1e-4);
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
    /// let v: Vector<f64> = vector!(1.0, 2.0, 3.0);
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
    pub fn dot(&self, v: impl Into<Vector<T>>) -> T {
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
    /// assert_eq!(cross.get(), [0.0, 0.0, 0.0]);
    /// ```
    pub fn cross(&self, v: impl Into<Vector<T>>) -> Self {
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
    pub fn reflect(&mut self, normal: impl Into<Vector<T>>)
    where
        T: MulAssign,
    {
        let normal = Self::normalized(normal);
        *self = normal * ((T::one() + T::one()) * self.dot(normal)) - *self;
    }

    /// Returns `Vector` as a [`Vec<T>`].
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

    /// Constructs a random unit `Vector<T>` in 1D space.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v: Vector<f64> = Vector::random_1d();
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
    /// let v: Vector<f64> = Vector::random_2d();
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
    /// let v: Vector<f64> = Vector::random_3d();
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

    /// Set the magnitude (length) of the `Vector`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v: Vector<f64> = vector!(10.0, 20.0, 2.0);
    /// v.set_mag(10.0);
    ///
    /// let abs_difference_mag = (v.mag() - 10.0).abs();
    /// let abs_difference_x = (v.x - 4.4543).abs();
    /// let abs_difference_y = (v.y - 8.9087).abs();
    /// let abs_difference_z = (v.z - 0.8908).abs();
    ///
    /// assert!(abs_difference_mag <= 1e-4);
    /// assert!(abs_difference_x <= 1e-4);
    /// assert!(abs_difference_y <= 1e-4);
    /// assert!(abs_difference_z <= 1e-4);
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
    /// let v1: Vector<f64> = vector!(1.0, 0.0, 0.0);
    /// let v2: Vector<f64> = vector!(0.0, 1.0, 0.0);
    /// let dist = v1.dist(v2);
    ///
    /// let abs_difference = (dist - std::f64::consts::SQRT_2).abs();
    /// assert!(abs_difference <= 1e-4);
    /// ```
    pub fn dist(&self, v: impl Into<Vector<T>>) -> T {
        let v = v.into();
        (*self - v).mag()
    }

    /// Normalize the `Vector` to length `1` making it a unit vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v: Vector<f64> = vector!(10.0, 20.0, 2.0);
    /// v.normalize();
    ///
    /// let abs_difference_mag = (v.mag() - 1.0).abs();
    /// assert!(abs_difference_mag <= 1e-4);
    ///
    /// let abs_difference_x = (v.x - 0.4454).abs();
    /// let abs_difference_y = (v.y - 0.8908).abs();
    /// let abs_difference_z = (v.z - 0.0890).abs();
    ///
    /// assert!(abs_difference_x <= 1e-4);
    /// assert!(abs_difference_y <= 1e-4);
    /// assert!(abs_difference_z <= 1e-4);
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
    /// let mut v: Vector<f64> = vector!(10.0, 20.0, 2.0);
    /// v.limit(5.0);
    ///
    /// let abs_difference_x = (v.x - 2.2271).abs();
    /// let abs_difference_y = (v.y - 4.4543).abs();
    /// let abs_difference_z = (v.z - 0.4454).abs();
    ///
    /// assert!(abs_difference_x <= 1e-4, "x {}", abs_difference_x);
    /// assert!(abs_difference_y <= 1e-4, "y {}", abs_difference_y);
    /// assert!(abs_difference_z <= 1e-4, "z {}", abs_difference_z);
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
    /// let v: Vector<f64> = vector!(10.0, 10.0);
    /// let heading = v.heading();
    /// assert_eq!(heading.to_degrees(), 45.0);
    /// ```
    pub fn heading(&self) -> T {
        self.y.atan2(self.x)
    }

    /// Rotate a 2D `Vector` by an angle in radians, magnitude remains the same. Unaffected by
    /// [`AngleMode`](crate::prelude::AngleMode).
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v: Vector<f64> = vector!(10.0, 20.0);
    /// v.rotate(std::f64::consts::FRAC_PI_2);
    ///
    /// let abs_difference_x = (v.x - (-20.0)).abs();
    /// let abs_difference_y = (v.y - 10.0).abs();
    ///
    /// assert!(abs_difference_x <= 1e-4);
    /// assert!(abs_difference_y <= 1e-4);
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
    pub fn angle_between(&self, v: impl Into<Vector<T>>) -> T {
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
    /// assert_eq!(v3.get(), [2.0, 2.0, 0.0]);
    /// ```
    pub fn lerp(&self, v: impl Into<Vector<T>>, amt: T) -> Self {
        let lerp = |start, stop, amt| amt * (stop - start) + start;
        let amt = clamp(amt, T::zero(), T::one());

        let v = v.into();
        Self::new(
            lerp(self.x, v.x, amt),
            lerp(self.y, v.y, amt),
            lerp(self.z, v.z, amt),
        )
    }

    /// Wraps `Vector` around the given width, height, and size (radius).
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v = vector!(200.0, 300.0);
    /// v.wrap_2d(150.0, 400.0, 10.0);
    /// assert_eq!(v.x, -10.0);
    /// assert_eq!(v.y, 300.0);
    ///
    /// let mut v = vector!(200.0, 300.0);
    /// v.wrap_2d(300.0, 200.0, 10.0);
    /// assert_eq!(v.x, 200.0);
    /// assert_eq!(v.y, -10.0);
    /// ```
    pub fn wrap_2d(&mut self, width: T, height: T, size: T) {
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
}

impl<T> ExactSizeIterator for Iter<'_, T> {}
impl<T> ExactSizeIterator for IterMut<'_, T> {}

impl<T> FromIterator<T> for Vector<T>
where
    T: Num,
{
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
    /// This struct is created by the [`into_iter`](Vector::into_iter) method on [`Vector`]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v: Vector<f64> = vector!(1.0, 2.0, -4.0);
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

/// Immutable `Vector<T>` iterator over `[x, y, z]`.
///
/// This struct is created by the [`iter`](Vector::iter) method on [`Vector`]s.
///
/// # Example
///
/// ```
/// # use pix_engine::prelude::*;
/// let v: Vector<f64> = vector!(1.0, 2.0, -4.0);
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

impl<'a, T> Iter<'a, T> {
    #[inline]
    fn new(v: &'a Vector<T>) -> Self {
        Self {
            inner: [&v.x, &v.y, &v.z],
            current: 0,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
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

impl<'a, T> IntoIterator for &'a Vector<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

type ThreeChain<T> = Chain<Chain<Once<T>, Once<T>>, Once<T>>;

/// Mutable `Vector<T>` iterator over `[x, y, z]`.
///
/// This struct is created by the [`iter_mut`](Vector::iter_mut) method on [`Vector`]s.
///
/// # Example
///
/// ```
/// # use pix_engine::prelude::*;
/// let mut v = vector!(1.0, 2.0, -4.0);
/// for value in v.iter_mut() {
///     *value *= 2.0;
/// }
/// assert_eq!(v.get(), [2.0, 4.0, -8.0]);
/// ```
#[derive(Debug)]
pub struct IterMut<'a, T = Scalar> {
    inner: ThreeChain<&'a mut T>,
}

impl<'a, T> IterMut<'a, T> {
    #[inline]
    fn new(v: &'a mut Vector<T>) -> Self {
        Self {
            inner: once(&mut v.x).chain(once(&mut v.y)).chain(once(&mut v.z)),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, T> IntoIterator for &'a mut Vector<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

// Operations

impl<T> Index<usize> for Vector<T> {
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

impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds: the len is 3 but the index is {}", idx),
        }
    }
}

impl<T> Add for Vector<T>
where
    T: Num,
{
    type Output = Self;
    fn add(self, v: Vector<T>) -> Self::Output {
        Vector::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

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

impl<T> Sub for Vector<T>
where
    T: Num,
{
    type Output = Self;
    fn sub(self, v: Vector<T>) -> Self::Output {
        Vector::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

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

impl<T> Neg for Vector<T>
where
    T: Num + Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl<T, U> Mul<U> for Vector<T>
where
    T: Num + Mul<U, Output = T>,
    U: Num + Copy,
{
    type Output = Self;
    fn mul(self, s: U) -> Self::Output {
        Vector::new(self.x * s, self.y * s, self.z * s)
    }
}

impl<T, U> MulAssign<U> for Vector<T>
where
    T: MulAssign<U>,
    U: Num + Copy,
{
    fn mul_assign(&mut self, s: U) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }
}

impl<T, U> Div<U> for Vector<T>
where
    T: Num + Div<U, Output = T>,
    U: Num + Copy,
{
    type Output = Self;
    fn div(self, s: U) -> Self::Output {
        Vector::new(self.x / s, self.y / s, self.z / s)
    }
}

impl<T, U> DivAssign<U> for Vector<T>
where
    T: Num + DivAssign<U>,
    U: Num + Copy,
{
    fn div_assign(&mut self, s: U) {
        self.x /= s;
        self.y /= s;
        self.z /= s;
    }
}

impl<T> Sum for Vector<T>
where
    Self: Add<Output = Self>,
    T: Num,
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
    Self: Add<Output = Self>,
    T: Num + Copy,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        let v = Vector::new(T::zero(), T::zero(), T::zero());
        iter.fold(v, |a, b| a + *b)
    }
}

// Required because of orphan rules
macro_rules! impl_ops {
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

impl_ops!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);

// Conversions from U to Vector<T>

/// Convert [`Vector<T>`] to `[x]`.
impl<T> From<Vector<T>> for [T; 1] {
    fn from(v: Vector<T>) -> Self {
        [v.x]
    }
}
/// Convert [`&Vector<T>`] to `[x]`.
impl<T: Copy> From<&Vector<T>> for [T; 1] {
    fn from(v: &Vector<T>) -> Self {
        [v.x]
    }
}

/// Convert [`Vector<T>`] to `[x, y]`.
impl<T> From<Vector<T>> for [T; 2] {
    fn from(v: Vector<T>) -> Self {
        [v.x, v.y]
    }
}
/// Convert [`&Vector<T>`] to `[x, y]`.
impl<T: Copy> From<&Vector<T>> for [T; 2] {
    fn from(v: &Vector<T>) -> Self {
        [v.x, v.y]
    }
}

/// Convert [`Vector<T>`] to `[x, y, z]`.
impl<T> From<Vector<T>> for [T; 3] {
    fn from(v: Vector<T>) -> Self {
        [v.x, v.y, v.z]
    }
}
/// Convert [`&Vector<T>`] to `[x, y, z]`.
impl<T: Copy> From<&Vector<T>> for [T; 3] {
    fn from(v: &Vector<T>) -> Self {
        [v.x, v.y, v.z]
    }
}

/// Convert `[U; 1]` to [`Vector<T>`].
impl<T: Num, U: Into<T>> From<[U; 1]> for Vector<T> {
    fn from([x]: [U; 1]) -> Self {
        Self::new(x.into(), T::zero(), T::zero())
    }
}
/// Convert `&[U; 1]` to [`Vector<T>`].
impl<T: Num, U: Copy + Into<T>> From<&[U; 1]> for Vector<T> {
    fn from(&[x]: &[U; 1]) -> Self {
        Self::new(x.into(), T::zero(), T::zero())
    }
}

/// Convert `[U; 2]` to [`Vector<T>`].
impl<T: Num, U: Into<T>> From<[U; 2]> for Vector<T> {
    fn from([x, y]: [U; 2]) -> Self {
        Self::new(x.into(), y.into(), T::zero())
    }
}
/// Convert `&[U; 2]` to [`Vector<T>`].
impl<T: Num, U: Copy + Into<T>> From<&[U; 2]> for Vector<T> {
    fn from(&[x, y]: &[U; 2]) -> Self {
        Self::new(x.into(), y.into(), T::zero())
    }
}

/// Convert `[U; 3]` to [`Vector<T>`].
impl<T, U: Into<T>> From<[U; 3]> for Vector<T> {
    fn from([x, y, z]: [U; 3]) -> Self {
        Self::new(x.into(), y.into(), z.into())
    }
}
/// Convert `&[U; 3]` to [`Vector<T>`].
impl<T, U: Copy + Into<T>> From<&[U; 3]> for Vector<T> {
    fn from(&[x, y, z]: &[U; 3]) -> Self {
        Self::new(x.into(), y.into(), z.into())
    }
}

/// Converts [`Point<U>`] to [`Vector<T>`].
impl<T, U: Into<T>> From<Point<U>> for Vector<T> {
    fn from(p: Point<U>) -> Self {
        Self::new(p.x.into(), p.y.into(), p.z.into())
    }
}

/// Converts [`&Point<U>`] to [`Vector<T>`].
impl<T, U: Copy + Into<T>> From<&Point<U>> for Vector<T> {
    fn from(p: &Point<U>) -> Self {
        Self::new(p.x.into(), p.y.into(), p.z.into())
    }
}

/// Converts [`Vector<U>`] to [`Point<T>`].
impl<T, U: Into<T>> From<Vector<U>> for Point<T> {
    fn from(v: Vector<U>) -> Self {
        Self::new(v.x.into(), v.y.into(), v.z.into())
    }
}

/// Converts [`&Vector<U>`] to [`Point<T>`].
impl<T, U: Copy + Into<T>> From<&Vector<U>> for Point<T> {
    fn from(v: &Vector<U>) -> Self {
        Self::new(v.x.into(), v.y.into(), v.z.into())
    }
}

/// Display [`Vector<T>`] as "[x, y, z]".
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
            assert_approx_eq!(v.get(), [4.0, -10.0, 0.0], $e);

            // Mul<Vector> for T
            let v = $val * vector!(2.0, -5.0, 0.0);
            assert_approx_eq!(v.get(), [4.0, -10.0, 0.0], $e);

            // MulAssign<T> for Vector
            let mut v = vector!(2.0, -5.0, 0.0);
            v *= $val;
            assert_approx_eq!(v.get(), [4.0, -10.0, 0.0], $e);

            // Div<T> for Vector
            let v = vector!(1.0, -5.0, 0.0) / $val;
            assert_approx_eq!(v.get(), [0.5, -2.5, 0.0], $e);

            // DivAssign<T> for Vector
            let mut v = vector!(2.0, -5.0, 0.0);
            v /= $val;
            assert_approx_eq!(v.get(), [1.0, -2.5, 0.0], $e);
        };
    }

    #[test]
    fn test_ops() {
        // Add
        let v1 = vector!(2.0, 5.0, 1.0);
        let v2 = vector!(1.0, 5.0, -1.0);
        let v3 = v1 + v2;
        assert_approx_eq!(v3.get(), [3.0, 10.0, 0.0]);

        // AddAssign
        let mut v1 = vector!(2.0, 5.0, 1.0);
        let v2 = vector!(1.0, 5.0, -1.0);
        v1 += v2;
        assert_approx_eq!(v1.get(), [3.0, 10.0, 0.0]);

        // Sub
        let v1 = vector!(2.0, 1.0, 2.0);
        let v2 = vector!(1.0, 5.0, 3.0);
        let v3 = v1 - v2;
        assert_approx_eq!(v3.get(), [1.0, -4.0, -1.0]);

        // SubAssign
        let mut v1 = vector!(2.0, 1.0, 2.0);
        let v2 = vector!(1.0, 5.0, 3.0);
        v1 -= v2;
        assert_approx_eq!(v1.get(), [1.0, -4.0, -1.0]);

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
