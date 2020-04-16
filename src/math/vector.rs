use crate::{constants::*, shape::Point, State};
use rand::{self, Rng};
use std::fmt;

/// Represents a Euclidiean (also known as geometric) Vector in 2D or 3D space. A Vector has both a magnitude and a direction,
/// but this data type stores the components of the vector as (x, y, 0) for 2D or (x, y, z) for 3D.
///
/// The magnitude and direction can be accessed by calling `mag()` or `heading()` on the vector.
///
/// Some example uses of a vector include modeling a position, velocity, or acceleration of an
/// object or particle.
///
/// Vectors can be combined using "vector" math, so for example two vectors can be added together
/// to form a new vector using `Vector::add_vectors(v1, v2)` or you can add one vector to another
/// by calling `v1.add(v2)`.
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl<'a> Vector {
    /// Creates a new Vector in 3D space. Shortcut for `Vector::new_2d()` and `Vector::new_3d()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::math::Vector;
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
    /// use pix_engine::math::Vector;
    ///
    /// let v = Vector::new((1, 2));
    /// assert_eq!(v.get(), (1.0, 2.0, 0.0));
    /// ```
    pub fn new_2d(x: f32, y: f32) -> Self {
        Self::new_3d(x, y, 0.0)
    }

    /// Creates a new Vector in 3D space.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let v = Vector::new((2.1, 3.5, 1.0));
    /// assert_eq!(v.get(), (2.1, 3.5, 1.0));
    /// ```
    pub fn new_3d(x: f32, y: f32, z: f32) -> Self {
        if !Self::valid_coordinates(x, y, z) {
            eprintln!("Vector::new: vector contains components that are either undefined or not finite numbers: {:?}", (x, y, z));
        }
        Self { x, y, z }
    }

    /// Creates a new Vector in 3D space from a Point.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::{shape::Point, math::Vector};
    ///
    /// let p = Point::new(1, 2, 3);
    /// let v = Vector::from_point(p);
    /// assert_eq!(v.get(), (1.0, 2.0, 3.0));
    /// ```
    pub fn from_point(p: Point) -> Self {
        Self::new_3d(p.x as f32, p.y as f32, p.z as f32)
    }

    /// Copies the current Vector into a new Vector.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let v1 = Vector::new((1, 0, 1));
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
    /// as Radians and is unaffected by `State::angle_mode()`.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let v = Vector::from_angle(30.0, 15.0);
    ///
    /// let abs_difference_x = (v.x - 2.3137717).abs();
    /// let abs_difference_y = (v.y - (-14.820475)).abs();
    ///
    /// assert!(abs_difference_x <= std::f32::EPSILON);
    /// assert!(abs_difference_y <= std::f32::EPSILON);
    /// ```
    pub fn from_angle(angle: f32, length: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new_2d(length * cos, length * sin)
    }

    /// Make a random unit Vector in 2D space.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let v = Vector::random_2d();
    ///
    /// // May make v's (x, y, z) values something like:
    /// // (0.61554617, -0.51195765, 0.0) or
    /// // (-0.4695841, -0.14366731, 0.0) or
    /// // (0.6091097, -0.22805278, 0.0)
    /// ```
    pub fn random_2d() -> Self {
        Self::from_angle(rand::thread_rng().gen::<f32>() * TWO_PI, 1.0)
    }

    /// Make a random unit Vector in 3D space.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let v = Vector::random_3d();
    ///
    /// // May make v's (x, y, z) values something like:
    /// // (0.61554617, -0.51195765, 0.599168) or
    /// // (-0.4695841, -0.14366731, -0.8711202) or
    /// // (0.6091097, -0.22805278, -0.7595902)
    /// ```
    pub fn random_3d() -> Self {
        let (sin, cos) = (rand::thread_rng().gen::<f32>() * TWO_PI).sin_cos();
        let z = rand::thread_rng().gen::<f32>() * 2.0 - 1.0; // Range from -1.0 to 1.0
        let z_base = (1.0 - z * z).sqrt();
        let x = z_base * cos;
        let y = z_base * sin;
        Self::new_3d(x, y, z)
    }

    /// Get the xyz coordinates as a tuple.
    pub const fn get(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    /// Set the xyz coordinates.
    pub fn set<V: Into<Vector>>(&mut self, v: V) {
        let v = v.into();
        self.x = v.x;
        self.y = v.y;
        self.z = v.z;
    }

    /// Adds a vector to the current Vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let mut v = Vector::new((1, 2, 3));
    ///
    /// v.add((4, 5));
    /// assert_eq!(v.get(), (5.0, 7.0, 3.0));
    ///
    /// v.add((2, 0, 6));
    /// assert_eq!(v.get(), (7.0, 7.0, 9.0));
    /// ```
    pub fn add<V: Into<Vector>>(&mut self, v: V) {
        let v = v.into();
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }

    /// Subtracts a vector from the current Vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let mut v = Vector::new((1, 2, 3));
    ///
    /// v.sub((4, 5));
    /// assert_eq!(v.get(), (-3.0, -3.0, 3.0));
    ///
    /// v.sub((2, 0, 6));
    /// assert_eq!(v.get(), (-5.0, -3.0, -3.0));
    /// ```
    pub fn sub<V: Into<Vector>>(&mut self, v: V) {
        let v = v.into();
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
    }

    /// Multiplies the current Vector by a scaler.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let mut v = Vector::new((1, 2, 3));
    /// v.mul(2.0);
    /// assert_eq!(v.get(), (2.0, 4.0, 6.0));
    /// ```
    pub fn mul(&mut self, s: f32) {
        if s.is_infinite() || s.is_nan() {
            eprintln!(
                "Vector::mul: scaler is either undefined or not finite: {}",
                s
            );
        } else {
            self.x *= s;
            self.y *= s;
            self.z *= s;
        }
    }

    /// Divides the current Vector by a scaler.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let mut v = Vector::new((2, 4, 6));
    /// v.div(2.0);
    /// assert_eq!(v.get(), (1.0, 2.0, 3.0));
    /// ```
    pub fn div(&mut self, s: f32) {
        if s == 0.0 || s.is_infinite() || s.is_nan() {
            eprintln!(
                "Vector::mul: scaler is either zero, undefined or not finite: {}",
                s
            );
        } else {
            self.x /= s;
            self.y /= s;
            self.z /= s;
        }
    }

    /// Calculates the remainder of a Vector when divided by another Vector.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let v = Vector::new((3, 4, 5));
    /// let rem = v.rem((2, 3, 3));
    /// assert_eq!(rem.get(), (1.0, 1.0, 2.0));
    /// ```
    pub fn rem<V: Into<Vector>>(&self, v: V) -> Vector {
        let v = v.into();
        let mut v2 = self.copy();
        if v.x != 0.0 {
            v2.x %= v.x;
        }
        if v.y != 0.0 {
            v2.y %= v.y;
        }
        if v.z != 0.0 {
            v2.z %= v.z;
        }
        v2
    }

    /// Calculates and returns the magnitude (length) of the Vector.
    ///
    /// The formula is `sqrt(x*x + y*y + z*z)`.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let v = Vector::new((1, 2, 3));
    /// let abs_difference = (v.mag() - 3.7416575).abs();
    /// assert!(abs_difference <= std::f32::EPSILON);
    /// ```
    pub fn mag(&self) -> f32 {
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
    /// use pix_engine::math::Vector;
    ///
    /// let v = Vector::new((1, 2, 3));
    /// assert_eq!(v.mag_sq(), 14.0);
    /// ```
    pub fn mag_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Set the magnitude (length) of the Vector.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let mut v = Vector::new((10, 20, 2));
    /// v.set_mag(10.0);
    ///
    /// let abs_difference_mag = (v.mag() - 10.0).abs();
    /// let abs_difference_x = (v.x - 4.4543543).abs();
    /// let abs_difference_y = (v.y - 8.908709).abs();
    /// let abs_difference_z = (v.z - 0.8908708).abs();
    ///
    /// assert!(abs_difference_mag <= std::f32::EPSILON);
    /// assert!(abs_difference_x <= std::f32::EPSILON);
    /// assert!(abs_difference_y <= std::f32::EPSILON);
    /// assert!(abs_difference_z <= std::f32::EPSILON);
    /// ```
    pub fn set_mag(&mut self, mag: f32) {
        self.normalize();
        self.mul(mag);
    }

    /// Calculates and returns the dot product with another Vector.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let v = Vector::new((1, 2, 3));
    /// let dot_product = v.dot((2, 3, 4));
    /// assert_eq!(dot_product, 20.0);
    /// ```
    pub fn dot<V: Into<Vector>>(&self, v: V) -> f32 {
        let v = v.into();
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    /// Calculates and returns the Vector cross product with another Vector.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let v1 = Vector::new((1, 2, 3));
    /// let v2 = Vector::new((1, 2, 3));
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
    /// use pix_engine::math::Vector;
    ///
    /// let v1 = Vector::new((1, 0, 0));
    /// let v2 = Vector::new((0, 1, 0));
    /// let dist = v1.dist(v2);
    ///
    /// let abs_difference = (dist - 2f32.sqrt()).abs();
    /// assert!(abs_difference <= std::f32::EPSILON);
    /// ```
    pub fn dist<V: Into<Vector>>(&self, v: V) -> f32 {
        let v = v.into();
        let mut v2 = self.copy();
        v2.sub(v);
        v2.mag()
    }

    /// Normalize the Vector to length 1 making it a unit vector.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let mut v = Vector::new((10, 20, 2));
    /// v.normalize();
    ///
    /// let abs_difference_mag = (v.mag() - 1.0).abs();
    /// assert!(abs_difference_mag <= std::f32::EPSILON);
    ///
    /// let abs_difference_x = (v.x - 0.4454354).abs();
    /// let abs_difference_y = (v.y - 0.8908708).abs();
    /// let abs_difference_z = (v.z - 0.089087084).abs();
    ///
    /// assert!(abs_difference_x <= std::f32::EPSILON);
    /// assert!(abs_difference_y <= std::f32::EPSILON);
    /// assert!(abs_difference_z <= std::f32::EPSILON);
    /// ```
    pub fn normalize(&mut self) {
        let len = self.mag();
        if len != 0.0 {
            // Multiply by the reciprocol so we don't duploicate a div by zero check
            self.mul(1.0 / len);
        }
    }

    /// Limit the magnitude (length) of this vector to the value given by max.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let mut v = Vector::new((10, 20, 2));
    /// v.limit(5.0);
    ///
    /// let abs_difference_x = (v.x - 2.2271771).abs();
    /// let abs_difference_y = (v.y - 4.4543543).abs();
    /// let abs_difference_z = (v.z - 0.4454354).abs();
    ///
    /// assert!(abs_difference_x <= std::f32::EPSILON, "x {}", abs_difference_x);
    /// assert!(abs_difference_y <= std::f32::EPSILON, "y {}", abs_difference_y);
    /// assert!(abs_difference_z <= std::f32::EPSILON, "z {}", abs_difference_z);
    /// ```
    pub fn limit(&mut self, max: f32) {
        let mag_sq = self.mag_sq();
        if mag_sq > max * max {
            self.div(mag_sq.sqrt()); // Normalize vector
            self.mul(max);
        }
    }

    /// Calculate the angle of rotation for a 2D Vector in radians. To convert to degrees you can
    /// call `to_degrees()` on the result.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let v = Vector::new((10, 10));
    /// let heading = v.heading();
    /// assert_eq!(heading.to_degrees(), 45.0);
    /// ```
    pub fn heading(&self) -> f32 {
        self.y.atan2(self.x)
    }

    /// Rotate a 2D Vector by an angle in radians, magnitude remains the same.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::{constants::HALF_PI, math::Vector};
    ///
    /// let mut v = Vector::new((10, 20));
    /// v.rotate(HALF_PI);
    ///
    /// let abs_difference_x = (v.x - (-20.000002)).abs();
    /// let abs_difference_y = (v.y - 9.999998).abs();
    ///
    /// assert!(abs_difference_x <= std::f32::EPSILON);
    /// assert!(abs_difference_y <= std::f32::EPSILON);
    /// ```
    pub fn rotate(&mut self, angle: f32) {
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
    /// use pix_engine::{constants::HALF_PI, math::Vector};
    ///
    /// let v1 = Vector::new((1, 0, 0));
    /// let v2 = Vector::new((0, 1, 0));
    /// let angle = v1.angle_between(v2);
    /// assert_eq!(angle, HALF_PI);
    /// ```
    pub fn angle_between<V: Into<Vector>>(&self, v: V) -> f32 {
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
    /// use pix_engine::math::Vector;
    ///
    /// let mut v = Vector::new((4, 6)); // Vector heading right and down
    /// let n = Vector::new((0, -1)); // Surface normal facing up
    /// v.reflect(n); // Reflect about the surface normal (e.g. the x-axis)
    ///
    /// assert_eq!(v.x, 4.0);
    /// assert_eq!(v.y, -6.0);
    ///
    /// ```
    pub fn reflect<V: Into<Vector>>(&mut self, normal: V) {
        let mut normal = normal.into();
        normal.mul(2.0 * self.dot(normal));
        self.sub(normal);
    }

    /// Linear interpolate the current vector to another vector.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let mut v1 = Vector::new((1, 1, 0));
    /// let v2 = Vector::new((3, 3, 0));
    /// v1.lerp(v2, 0.5);
    ///
    /// assert_eq!(v1.get(), (2.0, 2.0, 0.0));
    /// ```
    pub fn lerp<V: Into<Vector>>(&mut self, v: V, amt: f32) {
        let v = v.into();
        self.x += (v.x - self.x) * amt;
        self.y += (v.y - self.y) * amt;
        self.z += (v.z - self.z) * amt;
    }

    /// Returns a representation of this vector as a Vec of f32 values. Useful for temporary use.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let mut v = Vector::new((1, 1, 0));
    /// assert_eq!(v.to_vec(), vec![1.0, 1.0, 0.0]);
    /// ```
    pub fn to_vec(&self) -> Vec<f32> {
        vec![self.x, self.y, self.z]
    }

    /// Gets a Vector as a slice of xyz f32 values.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let mut v = Vector::new((1, 1, 0));
    /// assert_eq!(v.as_slice(), &[1.0, 1.0, 0.0]);
    /// ```
    pub fn as_slice(&self) -> &'a [f32] {
        unsafe { core::slice::from_raw_parts(self as *const Self as *const f32, 3) }
    }

    /// Gets a Vector as a mutable slice of xyz f32 values.
    pub fn as_slice_mut(&mut self) -> &'a mut [f32] {
        unsafe { core::slice::from_raw_parts_mut(self as *mut Self as *mut f32, 3) }
    }

    /// Helper function to validate coordinates are finite and defined.
    fn valid_coordinates(x: f32, y: f32, z: f32) -> bool {
        x.is_finite() && y.is_finite() && z.is_finite() && !x.is_nan() && !y.is_nan() && !z.is_nan()
    }
}

/// From 2D tuple of (x, y) i32 to Vector.
impl From<(i32, i32)> for Vector {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new_2d(x as f32, y as f32)
    }
}

/// From 3D tuple of (x, y, z) i32 to Vector.
impl From<(i32, i32, i32)> for Vector {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self::new_3d(x as f32, y as f32, z as f32)
    }
}

/// From 2D tuple of (x, y) f32 to Vector.
impl From<(f32, f32)> for Vector {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new_2d(x, y)
    }
}

/// From 3D tuple of (x, y, z) f32 to Vector.
impl From<(f32, f32, f32)> for Vector {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self::new_3d(x, y, z)
    }
}

impl From<Point> for Vector {
    fn from(p: Point) -> Self {
        Self::from_point(p)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl State {}
