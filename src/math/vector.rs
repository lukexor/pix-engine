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
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    mag: f32,
    mag_sq: f32,
}

impl Vector {
    /// Creates a new Vector in 3D space. Shortcut for `Vector::new_3d()`.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        if !Self::valid_coordinates(x, y, z) {
            eprintln!("Vector::new: vector contains components that are either undefined or not finite numbers: {:?}", (x, y, z));
        }
        Self {
            x,
            y,
            z,
            mag: 0.0,
            mag_sq: 0.0,
        }
    }

    /// Creates a new Vector in 2D space.
    pub fn new_2d(x: f32, y: f32) -> Self {
        Self::new(x, y, 0.0)
    }

    /// Creates a new Vector in 3D space.
    pub fn new_3d(x: f32, y: f32, z: f32) -> Self {
        Self::new(x, y, z)
    }

    /// Creates a new Vector in 3D space from a Point.
    pub fn from_point(p: Point) -> Self {
        Self::new(p.x as f32, p.y as f32, p.z as f32)
    }

    /// Creates a new Vector in 3D space from given parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let v = Vector::create_vector((1, 2));
    /// assert_eq!(v.get(), (1.0, 2.0, 0.0));
    ///
    /// let v = Vector::create_vector((2.1, 3.5, 1.0));
    /// assert_eq!(v.get(), (2.1, 3.5, 1.0));
    ///
    /// let v = Vector::create_vector(Vector::new(0.0, 2.0, 1.0));
    /// assert_eq!(v.get(), (0.0, 2.0, 1.0));
    /// ```
    pub fn create_vector<V: Into<Vector>>(v: V) -> Self {
        v.into()
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
    /// let abs_difference_x = (v.x() - 2.3137717).abs();
    /// let abs_difference_y = (v.y() - (-14.820475)).abs();
    ///
    /// assert!(abs_difference_x <= std::f32::EPSILON);
    /// assert!(abs_difference_y <= std::f32::EPSILON);
    /// ```
    pub fn from_angle(angle: f32, length: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new(length * cos, length * sin, 0.0)
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
        Self::new(x, y, z)
    }

    /// Get the xyz coordinates as a tuple.
    pub const fn get(self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
    /// Set the xyz coordinates.
    pub fn set<V: Into<Vector>>(&mut self, v: V) {
        let v = v.into();
        self.x = v.x;
        self.y = v.y;
        self.z = v.z;
    }

    /// Get the x coordinate.
    pub const fn x(self) -> f32 {
        self.x
    }
    /// Set the x coordinate.
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    /// Get the y coordinate.
    pub const fn y(self) -> f32 {
        self.y
    }
    /// Set the y coordinate.
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    /// Get the z coordinate.
    pub const fn z(self) -> f32 {
        self.z
    }
    /// Set the z coordinate.
    pub fn set_z(&mut self, z: f32) {
        self.z = z;
    }

    /// Adds a vector to the current Vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let mut v = Vector::create_vector((1, 2, 3));
    ///
    /// v.add((4, 5));
    /// assert_eq!(v.get(), (5.0, 7.0, 3.0));
    ///
    /// v.add((2, 0, 6));
    /// assert_eq!(v.get(), (7.0, 7.0, 9.0));
    ///
    /// v.add(Vector::new(1.0, 2.0, -3.0));
    /// assert_eq!(v.get(), (8.0, 9.0, 6.0));
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
    /// let mut v = Vector::create_vector((1, 2, 3));
    ///
    /// v.sub((4, 5));
    /// assert_eq!(v.get(), (-3.0, -3.0, 3.0));
    ///
    /// v.sub((2, 0, 6));
    /// assert_eq!(v.get(), (-5.0, -3.0, -3.0));
    ///
    /// v.sub(Vector::new(1.0, 2.0, -3.0));
    /// assert_eq!(v.get(), (-6.0, -5.0, 0.0));
    /// ```
    pub fn sub<V: Into<Vector>>(&mut self, v: V) {
        let v = v.into();
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
    }

    /// Multiplies the current Vector by another 3D Vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let mut v = Vector::create_vector((1, 2, 3));
    ///
    /// v.mul((2, 0, -1));
    /// assert_eq!(v.get(), (2.0, 0.0, -3.0));
    ///
    /// v.mul(Vector::new(1.5, 2.0, -3.0));
    /// assert_eq!(v.get(), (3.0, 0.0, 9.0));
    /// ```
    pub fn mul<V: Into<Vector>>(&mut self, v: V) {
        let v = v.into();
        if !Self::valid_coordinates(v.x, v.y, v.z) {
            eprintln!("Vector::mul: vector contains components that are either undefined or not finite numbers: {}", v);
        } else {
            self.x *= v.x;
            self.y *= v.y;
            self.z *= v.z;
        }
    }

    /// Multiplies the current Vector by a scaler.
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let mut v = Vector::create_vector((1, 2, 3));
    /// v.mul_scaler(2.0);
    /// assert_eq!(v.get(), (2.0, 4.0, 6.0));
    /// ```
    pub fn mul_scaler(&mut self, s: f32) {
        if s.is_infinite() || s.is_nan() {
            eprintln!(
                "Vector::mul_scaler: scaler is either undefined or not finite: {}",
                s
            );
        } else {
            self.x *= s;
            self.y *= s;
            self.z *= s;
        }
    }

    /// Divides the current Vector by another 3D Vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let mut v = Vector::create_vector((4, 5, 6));
    ///
    /// v.div((1, 2, 3));
    /// assert_eq!(v.get(), (4.0, 2.5, 2.0));
    ///
    /// v.div((2.0, 1.0, 2.0));
    /// assert_eq!(v.get(), (2.0, 2.5, 1.0));
    ///
    /// v.div(Vector::new(1.0, 2.0, -0.5));
    /// assert_eq!(v.get(), (2.0, 1.25, -2.0));
    /// ```
    pub fn div<V: Into<Vector>>(&mut self, v: V) {
        let v = v.into();
        if v.x == 0.0 || v.y == 0.0 || v.z == 0.0 {
            eprintln!("Vector::div: divide by zero: {}", v);
        } else {
            self.x /= v.x;
            self.y /= v.y;
            self.z /= v.z;
        }
    }

    /// Divides the current Vector by a scaler.
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::math::Vector;
    ///
    /// let mut v = Vector::create_vector((2, 4, 6));
    /// v.div_scaler(2.0);
    /// assert_eq!(v.get(), (1.0, 2.0, 3.0));
    /// ```
    pub fn div_scaler(&mut self, s: f32) {
        if s == 0.0 || s.is_infinite() || s.is_nan() {
            eprintln!(
                "Vector::mul_scaler: scaler is either zero, undefined or not finite: {}",
                s
            );
        } else {
            self.x /= s;
            self.y /= s;
            self.z /= s;
        }
    }

    pub fn rem(&mut self, v: Vector) -> Vector {
        // TODO Vector::rem
        v
    }

    pub fn mag(&mut self, v: Vector) -> f32 {
        // TODO Vector::mag
        0.0
    }

    pub fn mag_sq(&mut self, v: Vector) -> f32 {
        // TODO Vector::mag_sq
        0.0
    }

    pub fn set_mag(&mut self, v: Vector) {
        // TODO Vector::set_mag
    }

    pub fn dot(&mut self, v: Vector) -> f32 {
        // TODO Vector::dot
        0.0
    }

    pub fn cross(&mut self, v: Vector) -> Self {
        // TODO Vector::cross
        v
    }

    pub fn dist(&mut self, v: Vector) -> f32 {
        // TODO Vector::dist
        0.0
    }

    pub fn normalize(&mut self) {
        // TODO Vector::normalize
    }

    pub fn limit(&mut self, max: f32) {
        // TODO Vector::limit
    }

    pub fn heading(&mut self) {
        // TODO Vector::heading
    }

    pub fn rotate(&mut self, angle: f32) {
        // TODO Vector::rotate
    }

    pub fn angle_between(&mut self, v: Vector) {
        // TODO Vector::angle_between
    }

    pub fn reflect(&mut self, normal: Vector) {
        // TODO Vector::reflect
    }

    fn valid_coordinates(x: f32, y: f32, z: f32) -> bool {
        x.is_finite() && y.is_finite() && z.is_finite() && !x.is_nan() && !y.is_nan() && !z.is_nan()
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
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
