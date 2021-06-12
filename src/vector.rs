//! 2D and 3D [Vector] functions.Self::
//!
//! Each Vector is represented by 3 values for x, y, and z. Values can be provided as either
//! integer or floating point.
//!
//! The number of parameters can vary. Optional values are in square brackets:
//!
//! # Syntax
//!
//! ```text
//! vector!(x, [y], [z])
//! ```
//!
//! There are also methods for creating unit and randomized vectors. See `Other Examples` for details.
//!
//! # Examples
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! let v = vector!(); // New Vector placed at the origin (0.0, 0.0)
//! assert_eq!(v.values(), [0.0, 0.0, 0.0]);
//!
//! let v = vector!(5.0); // Vector parallel with the X-axis, magnitude of 5
//! assert_eq!(v.values(), [5.0, 0.0, 0.0]);
//!
//! let v = vector!(1.0, -3.0); // Vector in the XY-plane
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
//! let v: Vector<f64> = Vector::random_2d();
//! // `v.values()` will return something like:
//! // [-0.9993116191591512, 0.03709835324533284, 0.0]
//! assert!(v.x >= -1.0 && v.x <= 1.0);
//! assert!(v.y >= -1.0 && v.y <= 1.0);
//! assert_eq!(v.z, 0.0);
//!
//! let v: Vector<f64> = Vector::random_3d();
//! // `v.values()` will return something like:
//! // [-0.40038099206441835, 0.8985763512414204, 0.17959844705110184]
//! assert!(v.x >= -1.0 && v.x <= 1.0);
//! assert!(v.y >= -1.0 && v.y <= 1.0);
//! assert!(v.z >= -1.0 && v.z <= 1.0);
//! ```

use crate::{random, shape::Point};
use num::{clamp, Float, Num, NumCast};
use num_traits::AsPrimitive;
use rand::distributions::uniform::SampleUniform;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    f64::consts::TAU,
    fmt,
    iter::Sum,
    ops::*,
};

/// Represents a Euclidiean (also known as geometric) Vector in 2D or 3D space. A Vector has both a magnitude and a direction,
/// but this data type stores the components of the vector as (x, y, 0) for 2D or (x, y, z) for 3D.
///
/// The magnitude and direction can be accessed by calling `mag()` or `heading()` on the vector.
///
/// Some example uses of a vector include modeling a position, velocity, or acceleration of an
/// object or particle.
///
/// Vectors can be combined using "vector" math, so for example two vectors can be added together
/// to form a new vector using `let v3 = v1 + v2` or you can add one vector to another by calling
/// `v1 += v2`.
#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector<T> {
    /// X magnitude
    pub x: T,
    /// Y magnitude
    pub y: T,
    /// Z magnitude
    pub z: T,
}

/// # Create a [Vector<T>].
///
/// ```
/// use pix_engine::prelude::*;
/// let v = vector!(1.0, 2.0, 0.0);
/// assert_eq!(v.values(), [1.0, 2.0, 0.0]);
/// ```
#[macro_export]
macro_rules! vector {
    () => {
        vector!(0.0, 0.0, 0.0)
    };
    ($x:expr) => {
        vector!($x, 0.0, 0.0)
    };
    ($x:expr, $y:expr$(,)?) => {
        vector!($x, $y, 0.0)
    };
    ($x:expr, $y:expr, $z:expr$(,)?) => {
        $crate::vector::Vector::new_3d($x, $y, $z)
    };
}

impl<T> Vector<T> {
    /// Creates a new Vector in 3D space.
    ///
    /// # Panics
    ///
    /// Panics if any coordinate is `Infinity`, or `NaN`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = Vector::new_3d(2.1, 3.5, 1.0);
    /// assert_eq!(v.get(), (2.1, 3.5, 1.0));
    /// ```
    pub const fn new_3d(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Set `Vector` coordinates from (x, y, z).
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v = Vector::new_3d(2.0, 1.0, 3.0);
    /// assert_eq!(v.get(), (2.0, 1.0, 3.0));
    /// v.set((1.0, 2.0, 4.0));
    /// assert_eq!(v.get(), (1.0, 2.0, 4.0));
    /// ```
    pub fn set(&mut self, v: impl Into<Vector<T>>) {
        let v = v.into();
        self.x = v.x;
        self.y = v.y;
        self.z = v.z;
    }
}

impl<T> Vector<T>
where
    T: Num + Copy,
{
    /// Creates a new Vector in 2D space.
    ///
    /// # Panics
    ///
    /// Panics if any coordinate is `Infinity`, or `NaN`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = Vector::new_2d(1.0, 2.0);
    /// assert_eq!(v.get(), (1.0, 2.0, 0.0));
    /// ```
    pub fn new_2d(x: T, y: T) -> Self {
        Self::new_3d(x, y, T::zero())
    }

    /// Copies the current Vector into a new Vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v1 = vector!(1.0, 0.0, 1.0);
    /// let mut v2 = v1.copy();
    /// v2.x = 2.0;
    /// assert_eq!(v1.get(), (1.0, 0.0, 1.0));
    /// assert_eq!(v2.get(), (2.0, 0.0, 1.0));
    /// ```
    pub fn copy(&self) -> Self {
        *self
    }

    /// Get `Vector` coordinates as (x, y, z).
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = vector!(2.0, 1.0, 3.0);
    /// assert_eq!(v.get(), (2.0, 1.0, 3.0));
    /// ```
    pub fn get(&self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }

    /// Calculates and returns the squared magnitude (length) of the Vector. This is faster if the
    /// real length is not required in the case of comparing vectors.
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

    /// Calculates and returns the dot product with another Vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = vector!(1.0, 2.0, 3.0);
    /// let dot_product = v.dot((2.0, 3.0, 4.0));
    /// assert_eq!(dot_product, 20.0);
    /// ```
    pub fn dot(&self, v: impl Into<Vector<T>>) -> T {
        let v = v.into();
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    /// Calculates and returns the Vector cross product with another Vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v1 = vector!(1.0, 2.0, 3.0);
    /// let v2 = vector!(1.0, 2.0, 3.0);
    /// let cross = v1.cross(v2);
    /// assert_eq!(cross.get(), (0.0, 0.0, 0.0));
    /// ```
    pub fn cross(&self, v: impl Into<Vector<T>>) -> Self {
        let v = v.into();
        Self::new_3d(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }

    /// Reflect the current Vector about a normal to a line in 2D space, or about a normal to
    /// a plane in 3D space.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v = vector!(4.0, 6.0); // Vector heading right and down
    /// let n = vector!(0.0, -1.0); // Surface normal facing up
    /// v.reflect(n); // Reflect about the surface normal (e.g. the x-axis)
    ///
    /// assert_eq!(v.x, 4.0);
    /// assert_eq!(v.y, -6.0);
    /// ```
    pub fn reflect(&mut self, normal: impl Into<Vector<T>>)
    where
        Self: SubAssign,
    {
        let normal = normal.into();
        *self -= normal * (T::one() + T::one()) * self.dot(normal);
    }

    /// Returns a representation of this vector as a Vec of T values. Useful for temporary use.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = vector!(1.0, 1.0, 0.0);
    /// assert_eq!(v.to_vec(), vec![1.0, 1.0, 0.0]);
    /// ```
    pub fn to_vec(&self) -> Vec<T> {
        vec![self.x, self.y, self.z]
    }

    /// Get `Vector` coordinates as [x, y, z].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = vector!(1.0, 1.0, 0.0);
    /// assert_eq!(v.values(), [1.0, 1.0, 0.0]);
    /// ```
    pub fn values(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}

impl<T> Vector<T>
where
    T: Float,
{
    /// Creates a new unit Vector in 2D space from a given angle. Angle is given
    /// as Radians and is unaffected by angle_mode.
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
        Self::new_2d(length * cos, length * sin)
    }

    /// Make a random unit Vector in 2D space.
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

    /// Make a random unit Vector in 3D space.
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
        Self::new_3d(x, y, z)
    }

    /// Calculates and returns the magnitude (length) of the Vector.
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

    /// Set the magnitude (length) of the Vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
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

    /// Calculates the Euclidean distance between the current Vector and another vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
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

    /// Normalize the Vector to length 1 making it a unit vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
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
            *self *= T::one() / len;
        }
    }

    /// Limit the magnitude (length) of this vector to the value given by max.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
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

    /// Calculate the angle of rotation for a 2D Vector in radians. To convert to degrees you can
    /// call `to_degrees()` on the result.
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

    /// Rotate a 2D Vector by an angle in radians, magnitude remains the same.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
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

    /// Calculates and returns the angle between the current Vector and another Vector in radians.
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

    /// Linear interpolate the current vector to another vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v1 = vector!(1.0, 1.0, 0.0);
    /// let v2 = vector!(3.0, 3.0, 0.0);
    /// v1.lerp(v2, 0.5);
    /// assert_eq!(v1.get(), (2.0, 2.0, 0.0));
    /// ```
    pub fn lerp(&mut self, v: impl Into<Vector<T>>, amt: T)
    where
        T: AddAssign,
    {
        let v = v.into();
        self.x += (v.x - self.x) * amt;
        self.y += (v.y - self.y) * amt;
        self.z += (v.z - self.z) * amt;
    }

    /// Wraps `Vector` around given width, height with a size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
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

    /// Convert [Vector<T>] to [Point<U>].
    pub fn as_point<U>(&self) -> Point<U>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy,
    {
        Point {
            x: self.x.as_(),
            y: self.y.as_(),
            z: self.z.as_(),
        }
    }
}

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
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl<T, U> Add<U> for Vector<T>
where
    T: Num + Add<U, Output = T>,
    U: Num + Copy,
{
    type Output = Self;
    fn add(self, s: U) -> Self::Output {
        Self {
            x: self.x + s,
            y: self.y + s,
            z: self.z + s,
        }
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

impl<T, U> AddAssign<U> for Vector<T>
where
    T: AddAssign<U>,
    U: Num + Copy,
{
    fn add_assign(&mut self, s: U) {
        self.x += s;
        self.y += s;
        self.z += s;
    }
}

impl<T> Sub for Vector<T>
where
    T: Num,
{
    type Output = Self;
    fn sub(self, v: Vector<T>) -> Self::Output {
        Self {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}

impl<T, U> Sub<U> for Vector<T>
where
    T: Num + Sub<U, Output = T>,
    U: Num + Copy,
{
    type Output = Self;
    fn sub(self, s: U) -> Self::Output {
        Self {
            x: self.x - s,
            y: self.y - s,
            z: self.z - s,
        }
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

impl<T, U> SubAssign<U> for Vector<T>
where
    T: SubAssign<U>,
    U: Num + Copy,
{
    fn sub_assign(&mut self, s: U) {
        self.x -= s;
        self.y -= s;
        self.z -= s;
    }
}

impl<T> Neg for Vector<T>
where
    T: Num + Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T, U> Mul<U> for Vector<T>
where
    T: Num + Mul<U, Output = T>,
    U: Num + Copy,
{
    type Output = Self;
    fn mul(self, s: U) -> Self::Output {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
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
        if s == U::zero() {
            panic!("divisor is zero");
        } else {
            Self {
                x: self.x / s,
                y: self.y / s,
                z: self.z / s,
            }
        }
    }
}

impl<T, U> DivAssign<U> for Vector<T>
where
    T: Num + DivAssign<U>,
    U: Num + Copy,
{
    fn div_assign(&mut self, s: U) {
        if s == U::zero() {
            panic!("divisor is zero");
        }
        self.x /= s;
        self.y /= s;
        self.z /= s;
    }
}

impl<T> Rem for Vector<T>
where
    T: Num,
{
    type Output = Self;
    fn rem(mut self, v: Vector<T>) -> Self::Output {
        if v.x != T::zero() {
            self.x = self.x % v.x;
        }
        if v.y != T::zero() {
            self.y = self.y % v.y;
        }
        if v.z != T::zero() {
            self.z = self.z % v.z;
        }
        self
    }
}

impl<T> RemAssign for Vector<T>
where
    T: Num + RemAssign,
{
    fn rem_assign(&mut self, v: Vector<T>) {
        if v.x != T::zero() {
            self.x %= v.x;
        }
        if v.y != T::zero() {
            self.y %= v.y;
        }
        if v.z != T::zero() {
            self.z %= v.z;
        }
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
        let v = Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        };
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
        let v = Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        };
        iter.fold(v, |a, b| a + *b)
    }
}

macro_rules! impl_op {
    ($target:ty, $zero:expr) => {
        impl Mul<Vector<$target>> for $target {
            type Output = Vector<$target>;
            fn mul(self, v: Vector<$target>) -> Self::Output {
                Vector::new_3d(self * v.x, self * v.y, self * v.z)
            }
        }

        impl Div<Vector<$target>> for $target {
            type Output = Vector<$target>;
            fn div(self, v: Vector<$target>) -> Self::Output {
                if v.x == $zero || v.y == $zero || v.z == $zero {
                    panic!("divisor is zero");
                }
                Vector::new_3d(self / v.x, self / v.y, self / v.z)
            }
        }
    };
}

impl_op!(i8, 0);
impl_op!(u8, 0);
impl_op!(i16, 0);
impl_op!(u16, 0);
impl_op!(i32, 0);
impl_op!(u32, 0);
impl_op!(i64, 0);
impl_op!(u64, 0);
impl_op!(i128, 0);
impl_op!(u128, 0);
impl_op!(isize, 0);
impl_op!(usize, 0);
impl_op!(f32, 0.0);
impl_op!(f64, 0.0);

/// Convert `T` to [Vector<T>].
impl<T> From<T> for Vector<T>
where
    T: Num + Copy,
{
    fn from(v: T) -> Self {
        Self { x: v, y: v, z: v }
    }
}

/// Convert `(T, T)` to [Vector<T>].
impl<T> From<(T, T)> for Vector<T>
where
    T: Num,
{
    fn from((x, y): (T, T)) -> Self {
        Self { x, y, z: T::zero() }
    }
}

/// Convert `(T, T, T)` to [Vector<T>].
impl<T> From<(T, T, T)> for Vector<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self { x, y, z }
    }
}

/// Convert [Vector<T>] to `(x, y)`.
impl<T> From<Vector<T>> for (T, T) {
    fn from(v: Vector<T>) -> Self {
        (v.x, v.y)
    }
}

/// Convert [Vector<T>] to `(x, y, z)`.
impl<T> From<Vector<T>> for (T, T, T) {
    fn from(v: Vector<T>) -> Self {
        (v.x, v.y, v.z)
    }
}

/// Convert [Point<U>] to [Vector<T>].
impl<T, U> TryFrom<Point<U>> for Vector<T>
where
    U: TryInto<T>,
{
    type Error = <U as TryInto<T>>::Error;
    fn try_from(p: Point<U>) -> Result<Self, Self::Error> {
        Ok(Self {
            x: p.x.try_into()?,
            y: p.y.try_into()?,
            z: p.z.try_into()?,
        })
    }
}

/// Convert [Vector<U>] to [Point<T>].
impl<T, U> TryFrom<Vector<U>> for Point<T>
where
    U: TryInto<T>,
{
    type Error = <U as TryInto<T>>::Error;
    fn try_from(v: Vector<U>) -> Result<Self, Self::Error> {
        Ok(Self {
            x: v.x.try_into()?,
            y: v.y.try_into()?,
            z: v.z.try_into()?,
        })
    }
}

/// Display [Vector<T>] as "[x, y, z]".
impl<T> fmt::Display for Vector<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}
