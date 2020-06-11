//! Vector handling and operations in 3D space.
//!
//! Each `Vector` is represented by 3 values for x, y, and z. Values can be provided as either
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
//! let v = vector!(5); // Vector parallel with the X-axis, magnitude of 5
//! assert_eq!(v.values(), [5.0, 0.0, 0.0]);
//!
//! let v = vector!(1, -3); // Vector in the XY-plane
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
//! let v = Vector::random_2d();
//! // `v.values()` will return something like:
//! // [-0.9993116191591512, 0.03709835324533284, 0.0]
//! assert!(v.x >= -1.0 && v.x <= 1.0);
//! assert!(v.y >= -1.0 && v.y <= 1.0);
//! assert_eq!(v.z, 0.0);
//!
//! let v = Vector::random_3d();
//! // `v.values()` will return something like:
//! // [-0.40038099206441835, 0.8985763512414204, 0.17959844705110184]
//! assert!(v.x >= -1.0 && v.x <= 1.0);
//! assert!(v.y >= -1.0 && v.y <= 1.0);
//! assert!(v.z >= -1.0 && v.z <= 1.0);
//! ```

use crate::{math::constants::*, randomf};
use std::{fmt, ops::*};

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
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    /// X magnitude
    pub x: f64,
    /// Y magnitude
    pub y: f64,
    /// Z magnitude
    pub z: f64,
}

/// # Create an `Vector`.
///
/// ```
/// use pix_engine::prelude::*;
///
/// let v = vector!(1, 2, 0);
/// assert_eq!(v.values(), [1.0, 2.0, 0.0]);
/// ```
#[macro_export]
macro_rules! vector {
    ($x:expr) => {
        vector!($x, 0.0, 0.0)
    };
    ($x:expr, $y:expr) => {
        vector!($x, $y, 0.0)
    };
    ($x:expr, $y:expr, $z:expr) => {
        $crate::prelude::Vector::new_3d($x as f64, $y as f64, $z as f64)
    };
}

impl Vector {
    /// Creates a new Vector in 3D space. Shortcut for `Vector::new_2d()` and `Vector::new_3d()`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let v1 = Vector::new((1, 2));
    /// assert_eq!(v1.get(), (1.0, 2.0, 0.0));
    ///
    /// let v2 = Vector::new((2.1, 3.5, 1.0));
    /// assert_eq!(v2.get(), (2.1, 3.5, 1.0));
    /// ```
    pub fn new<V: Into<Vector>>(v: V) -> Self {
        let v = v.into();
        if !Self::valid_coordinates(v.x, v.y, v.z) {
            eprintln!("Vector::new: vector contains components that are either undefined or not finite numbers: {}", v);
        }
        v
    }

    /// Creates a new Vector in 2D space.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let v = Vector::new_2d(1.0, 2.0);
    /// assert_eq!(v.get(), (1.0, 2.0, 0.0));
    /// ```
    pub fn new_2d(x: f64, y: f64) -> Self {
        Self::new_3d(x, y, 0.0)
    }

    /// Creates a new Vector in 3D space.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let v = Vector::new_3d(2.1, 3.5, 1.0);
    /// assert_eq!(v.get(), (2.1, 3.5, 1.0));
    /// ```
    pub fn new_3d(x: f64, y: f64, z: f64) -> Self {
        if !Self::valid_coordinates(x, y, z) {
            eprintln!("Vector::new: vector contains components that are either undefined or not finite numbers: {:?}", (x, y, z));
        }
        Self { x, y, z }
    }

    /// Copies the current Vector into a new Vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let v1 = vector!(1, 0, 1);
    /// let mut v2 = v1.copy();
    /// v2.x = 2.0;
    ///
    /// assert_eq!(v1.get(), (1.0, 0.0, 1.0));
    /// assert_eq!(v2.get(), (2.0, 0.0, 1.0));
    /// ```
    pub fn copy(&self) -> Self {
        *self
    }

    /// Creates a new unit Vector in 2D space from a given angle. Angle is given
    /// as Radians and is unaffected by `StateData::angle_mode()`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let v = Vector::from_angle(30.0, 15.0);
    ///
    /// let abs_difference_x = (v.x - 2.3137).abs();
    /// let abs_difference_y = (v.y - (-14.8204)).abs();
    ///
    /// assert!(abs_difference_x <= 1e-4);
    /// assert!(abs_difference_y <= 1e-4);
    /// ```
    pub fn from_angle(angle: f64, length: f64) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new_2d(length * cos, length * sin)
    }

    /// Make a random unit Vector in 2D space.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let v = Vector::random_2d();
    ///
    /// // May make v's (x, y, z) values something like:
    /// // (0.61554617, -0.51195765, 0.0) or
    /// // (-0.4695841, -0.14366731, 0.0) or
    /// // (0.6091097, -0.22805278, 0.0)
    /// ```
    pub fn random_2d() -> Self {
        Self::from_angle(randomf!(TWO_PI), 1.0)
    }

    /// Make a random unit Vector in 3D space.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let v = Vector::random_3d();
    ///
    /// // May make v's (x, y, z) values something like:
    /// // (0.61554617, -0.51195765, 0.599168) or
    /// // (-0.4695841, -0.14366731, -0.8711202) or
    /// // (0.6091097, -0.22805278, -0.7595902)
    /// ```
    pub fn random_3d() -> Self {
        let (sin, cos) = randomf!(TWO_PI).sin_cos();
        let z: f64 = randomf!(-1.0, 1.0); // Range from -1.0 to 1.0
        let z_base = (1.0 - z * z).sqrt();
        let x = z_base * cos;
        let y = z_base * sin;
        Self::new_3d(x, y, z)
    }

    /// Get the xyz coordinates as a tuple.
    pub const fn get(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    /// Set the xyz coordinates.
    pub fn set<V: Into<Vector>>(&mut self, v: V) {
        let v = v.into();
        self.x = v.x;
        self.y = v.y;
        self.z = v.z;
    }

    /// Calculates and returns the magnitude (length) of the Vector.
    ///
    /// The formula is `sqrt(x*x + y*y + z*z)`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let v = vector!(1, 2, 3);
    /// let abs_difference = (v.mag() - 3.7416).abs();
    /// assert!(abs_difference <= 1e-4);
    /// ```
    pub fn mag(&self) -> f64 {
        self.mag_sq().sqrt()
    }

    /// Calculates and returns the squared magnitude (length) of the Vector. This is faster if the
    /// real length is not required in the case of comparing vectors.
    ///
    /// The formula is `x*x + y*y + z*z`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let v = vector!(1, 2, 3);
    /// assert_eq!(v.mag_sq(), 14.0);
    /// ```
    pub fn mag_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Set the magnitude (length) of the Vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let mut v = vector!(10, 20, 2);
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
    pub fn set_mag(&mut self, mag: f64) {
        self.normalize();
        *self *= mag;
    }

    /// Calculates and returns the dot product with another Vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let v = vector!(1, 2, 3);
    /// let dot_product = v.dot((2, 3, 4));
    /// assert_eq!(dot_product, 20.0);
    /// ```
    pub fn dot<V: Into<Vector>>(&self, v: V) -> f64 {
        let v = v.into();
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    /// Calculates and returns the Vector cross product with another Vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let v1 = vector!(1, 2, 3);
    /// let v2 = vector!(1, 2, 3);
    /// let cross = v1.cross(v2);
    /// assert_eq!(cross.get(), (0.0, 0.0, 0.0));
    /// ```
    pub fn cross<V: Into<Vector>>(&self, v: V) -> Self {
        let v = v.into();
        Self::new_3d(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }

    /// Calculates the Euclidean distance between the current Vector and another vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let v1 = vector!(1, 0, 0);
    /// let v2 = vector!(0, 1, 0);
    /// let dist = v1.dist(v2);
    ///
    /// let abs_difference = (dist - SQRT_2).abs();
    /// assert!(abs_difference <= 1e-4);
    /// ```
    pub fn dist<V: Into<Vector>>(&self, v: V) -> f64 {
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
    /// let mut v = vector!(10, 20, 2);
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
    pub fn normalize(&mut self) {
        let len = self.mag();
        if len != 0.0 {
            // Multiply by the reciprocol so we don't duploicate a div by zero check
            *self *= 1.0 / len;
        }
    }

    /// Limit the magnitude (length) of this vector to the value given by max.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let mut v = vector!(10, 20, 2);
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
    pub fn limit(&mut self, max: f64) {
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
    ///
    /// let v = vector!(10, 10);
    /// let heading = v.heading();
    /// assert_eq!(heading.to_degrees(), 45.0);
    /// ```
    pub fn heading(&self) -> f64 {
        self.y.atan2(self.x)
    }

    /// Rotate a 2D Vector by an angle in radians, magnitude remains the same.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let mut v = vector!(10, 20);
    /// v.rotate(HALF_PI);
    ///
    /// let abs_difference_x = (v.x - (-20.0)).abs();
    /// let abs_difference_y = (v.y - 10.0).abs();
    ///
    /// assert!(abs_difference_x <= 1e-4);
    /// assert!(abs_difference_y <= 1e-4);
    /// ```
    pub fn rotate(&mut self, angle: f64) {
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
    ///
    /// let v1 = vector!(1, 0, 0);
    /// let v2 = vector!(0, 1, 0);
    /// let angle = v1.angle_between(v2);
    /// assert_eq!(angle, HALF_PI);
    /// ```
    pub fn angle_between<V: Into<Vector>>(&self, v: V) -> f64 {
        let v = v.into();
        // This should range from -1.0 to 1.0, inclusive but could possibly land outside this range
        // due to floating-point rounding, so we'll need to clamp it to the correct range.
        let dot_mag_product = (self.dot(v) / (self.mag() * v.mag())).max(-1.0).min(1.0);
        dot_mag_product.acos() * self.cross(v).z.signum()
    }

    /// Reflect the current Vector about a normal to a line in 2D space, or about a normal to
    /// a plane in 3D space.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let mut v = vector!(4, 6); // Vector heading right and down
    /// let n = vector!(0, -1); // Surface normal facing up
    /// v.reflect(n); // Reflect about the surface normal (e.g. the x-axis)
    ///
    /// assert_eq!(v.x, 4.0);
    /// assert_eq!(v.y, -6.0);
    ///
    /// ```
    pub fn reflect<V: Into<Vector>>(&mut self, normal: V) {
        let normal = normal.into();
        *self -= normal * 2.0 * self.dot(normal);
    }

    /// Linear interpolate the current vector to another vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let mut v1 = vector!(1, 1, 0);
    /// let v2 = vector!(3, 3, 0);
    /// v1.lerp(v2, 0.5);
    ///
    /// assert_eq!(v1.get(), (2.0, 2.0, 0.0));
    /// ```
    pub fn lerp<V: Into<Vector>>(&mut self, v: V, amt: f64) {
        let v = v.into();
        self.x += (v.x - self.x) * amt;
        self.y += (v.y - self.y) * amt;
        self.z += (v.z - self.z) * amt;
    }

    /// Returns a representation of this vector as a Vec of f64 values. Useful for temporary use.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let mut v = vector!(1, 1, 0);
    /// assert_eq!(v.to_vec(), vec![1.0, 1.0, 0.0]);
    /// ```
    pub fn to_vec(&self) -> Vec<f64> {
        vec![self.x, self.y, self.z]
    }

    /// Gets a Vector as an array of xyz f64 values.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let mut v = vector!(1, 1, 0);
    /// assert_eq!(v.values(), [1.0, 1.0, 0.0]);
    /// ```
    pub fn values(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    /// Helper function to validate a single coordinate is finite and defined.
    fn valid_coordinate(v: f64) -> bool {
        v.is_finite() && !v.is_nan()
    }

    /// Helper function to validate all coordinates are finite and defined.
    fn valid_coordinates(x: f64, y: f64, z: f64) -> bool {
        Self::valid_coordinate(x) && Self::valid_coordinate(y) && Self::valid_coordinate(z)
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, v: Vector) -> Vector {
        Vector::new_3d(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, v: Vector) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, v: Vector) -> Vector {
        Vector::new_3d(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, v: Vector) {
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, s: f64) -> Vector {
        if s.is_infinite() || s.is_nan() {
            eprintln!(
                "Vector::mul: scaler is either undefined or not finite: {}",
                s
            );
            self
        } else {
            Vector::new_3d(self.x * s, self.y * s, self.z * s)
        }
    }
}

impl MulAssign<f64> for Vector {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn mul_assign(&mut self, s: f64) {
        if s.is_infinite() || s.is_nan() {
            eprintln!(
                "Vector::mul_assign: scaler is either undefined or not finite: {}",
                s
            );
        } else {
            self.x *= s;
            self.y *= s;
            self.z *= s;
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, s: f64) -> Vector {
        if s == 0.0 || s.is_infinite() || s.is_nan() {
            eprintln!(
                "Vector::div: scaler is either zero, undefined or not finite: {}",
                s
            );
            self
        } else {
            Vector::new_3d(self.x / s, self.y / s, self.z / s)
        }
    }
}

impl DivAssign<f64> for Vector {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn div_assign(&mut self, s: f64) {
        if s == 0.0 || s.is_infinite() || s.is_nan() {
            eprintln!(
                "Vector::div_assign: scaler is either zero, undefined or not finite: {}",
                s
            );
        } else {
            self.x /= s;
            self.y /= s;
            self.z /= s;
        }
    }
}

impl Rem for Vector {
    type Output = Vector;
    fn rem(mut self, v: Vector) -> Vector {
        if v.x != 0.0 {
            self.x %= v.x;
        }
        if v.y != 0.0 {
            self.y %= v.y;
        }
        if v.z != 0.0 {
            self.z %= v.z;
        }
        self
    }
}

impl RemAssign for Vector {
    fn rem_assign(&mut self, v: Vector) {
        if v.x != 0.0 {
            self.x %= v.x;
        }
        if v.y != 0.0 {
            self.y %= v.y;
        }
        if v.z != 0.0 {
            self.z %= v.z;
        }
    }
}

impl Deref for Vector {
    type Target = [f64];
    fn deref(&self) -> &[f64] {
        unsafe { ::std::slice::from_raw_parts(self as *const Self as *const f64, 3) }
    }
}

impl DerefMut for Vector {
    fn deref_mut(&mut self) -> &mut [f64] {
        unsafe { ::std::slice::from_raw_parts_mut(self as *mut Self as *mut f64, 3) }
    }
}

/// From 1D tuple of i32 to 3D `Vector` with all the same value.
impl From<i32> for Vector {
    fn from(v: i32) -> Self {
        let v = v as f64;
        Self::new_3d(v, v, v)
    }
}

/// From 1D tuple of i64 to 3D `Vector` with all the same value.
impl From<i64> for Vector {
    fn from(v: i64) -> Self {
        let v = v as f64;
        Self::new_3d(v, v, v)
    }
}

/// From 2D tuple of (x, y) i32 to `Vector`.
impl From<(i32, i32)> for Vector {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new_2d(x as f64, y as f64)
    }
}

/// From 2D tuple of (x, y) u32 to `Vector`.
impl From<(u32, u32)> for Vector {
    fn from((x, y): (u32, u32)) -> Self {
        Self::new_2d(x as f64, y as f64)
    }
}

/// From 2D tuple of (x, y) i64 to `Vector`.
impl From<(i64, i64)> for Vector {
    fn from((x, y): (i64, i64)) -> Self {
        Self::new_2d(x as f64, y as f64)
    }
}

/// From 3D tuple of (x, y, z) i32 to `Vector`.
impl From<(i32, i32, i32)> for Vector {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self::new_3d(x as f64, y as f64, z as f64)
    }
}

/// From 3D tuple of (x, y, z) i64 to `Vector`.
impl From<(i64, i64, i64)> for Vector {
    fn from((x, y, z): (i64, i64, i64)) -> Self {
        Self::new_3d(x as f64, y as f64, z as f64)
    }
}

/// From 2D tuple of (x, y) f64 to `Vector`.
impl From<(f64, f64)> for Vector {
    fn from((x, y): (f64, f64)) -> Self {
        Self::new_2d(x, y)
    }
}

/// From 3D tuple of (x, y, z) f64 to `Vector`.
impl From<(f64, f64, f64)> for Vector {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self::new_3d(x, y, z)
    }
}

/// Convert to f64 tuple of (x, y).
impl Into<(f64, f64)> for Vector {
    fn into(self) -> (f64, f64) {
        (self.x, self.y)
    }
}

/// Convert to f64 tuple of (x, y, z).
impl Into<(f64, f64, f64)> for Vector {
    fn into(self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}
