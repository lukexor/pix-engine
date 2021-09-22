//! A [Euclidean] `Vector` in N-dimensional space.
//!
//! Each [Vector] represents a N-dimensional [Euclidean] (or geometric) vector with a magnitude
//! and a direction. The [Vector] `struct`, however, contains N values for each dimensional
//! coordinate. The magnitude and direction are retrieved with the [Vector::mag] and
//! [Vector::heading] methods.
//!
//! Some example uses of a [Vector] include modeling a position, velocity, or acceleration of an
//! object or particle in 2D or 3D space.
//!
//! # Examples
//!
//! You can create a [Vector] using [Vector::new]:
//!
//! ```
//! # use pix_engine::prelude_3d::*;
//! let v: VectorF3 = Vector::new([10.0, 20.0, 15.0]);
//! ```
//! ...or by using the [vector!] macro:
//!
//! ```
//! # use pix_engine::prelude_3d::*;
//! let v: VectorF3 = vector!(); // vector at the origin (0, 0, 0) with no direction or magnitude
//! assert_eq!(v.values(), [0.0, 0.0, 0.0]);
//!
//! let v = vector!(5.0); // 1D vector on the x-axis with magnitude 5
//! assert_eq!(v.values(), [5.0]);
//!
//! let v = vector!(5.0, 10.0); // 2D vector in the x/y-plane
//! assert_eq!(v.values(), [5.0, 10.0]);
//!
//! let v = vector!(-1.5, 3.0, 2.2); // 3D vector
//! assert_eq!(v.values(), [-1.5, 3.0, 2.2]);
//! ```
//!
//! You can also create random `Vector`s using [Vector::random] which create unit vectors with
//! magnitudes in the range `-1.0..=1.0`.
//!
//! ```
//! use pix_engine::prelude_3d::*;
//! use pix_engine::math::vector::VectorF1;
//!
//! let v: VectorF1 = Vector::random();
//! // `v.values()` will return something like:
//! // [-0.9993116191591512, 0.03709835324533284, 0.0]
//! assert!(v.x() >= -1.0 && v.x() <= 1.0);
//!
//! let v: VectorF2 = Vector::random();
//! // `v.values()` will return something like:
//! // [-0.9993116191591512, 0.03709835324533284, 0.0]
//! assert!(v.x() >= -1.0 && v.x() <= 1.0);
//! assert!(v.y() >= -1.0 && v.y() <= 1.0);
//!
//! let v: VectorF3 = Vector::random();
//! // `v.values()` will return something like:
//! // [-0.40038099206441835, 0.8985763512414204, 0.17959844705110184]
//! assert!(v.x() >= -1.0 && v.x() <= 1.0);
//! assert!(v.y() >= -1.0 && v.y() <= 1.0);
//! assert!(v.z() >= -1.0 && v.z() <= 1.0);
//! ```
//!
//! [Euclidean]: https://en.wikipedia.org/wiki/Euclidean_vector

use crate::prelude::*;
use num_traits::{AsPrimitive, Float, Signed};
use rand::distributions::uniform::SampleUniform;
// #[cfg(feature = "serde")]
// use serde::{Deserialize, Serialize};
use std::{
    array::IntoIter,
    fmt,
    iter::{Product, Sum},
    ops::*,
};

/// A [Euclidean] `Vector` in N-dimensional space.
///
/// Also known as a geometric vector. A `Vector` has both a magnitude and a direction. The [Vector]
/// struct, however, contains N values for each dimensional coordinate.
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
/// Please see the [module-level documentation] for examples.
///
/// [Euclidean]: https://en.wikipedia.org/wiki/Euclidean_vector
/// [mag]: Vector::mag
/// [heading]: Vector::heading
/// [vecmath]: https://en.wikipedia.org/wiki/Vector_(mathematics_and_p.y()sics)
/// [module-level documentation]: crate::math::vector
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Hash)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector<T, const N: usize>([T; N]);

/// A 1D `Vector` represented by integers.
pub type VectorI1 = Vector<i32, 1>;

/// A 2D `Vector` represented by integers.
pub type VectorI2 = Vector<i32, 2>;

/// A 3D `Vector` represented by integers.
pub type VectorI3 = Vector<i32, 3>;

/// A 1D `Vector` represented by floating point numbers.
pub type VectorF1 = Vector<Scalar, 1>;

/// A 2D `Vector` represented by floating point numbers.
pub type VectorF2 = Vector<Scalar, 2>;

/// A 3D `Vector` represented by floating point numbers.
pub type VectorF3 = Vector<Scalar, 3>;

/// # Constructs a [Vector].
///
/// # Examples
///
/// ```
/// # use pix_engine::prelude_3d::*;
/// let v: VectorF3 = vector!();
/// assert_eq!(v.values(), [0.0, 0.0, 0.0]);
///
/// let v = vector!(1.0);
/// assert_eq!(v.values(), [1.0]);
///
/// let v = vector!(1.0, 2.0);
/// assert_eq!(v.values(), [1.0, 2.0]);
///
/// let v = vector!(1.0, -2.0, 1.0);
/// assert_eq!(v.values(), [1.0, -2.0, 1.0]);
/// ```
#[macro_export]
macro_rules! vector {
    () => {
        $crate::prelude::Vector::origin()
    };
    ($x:expr) => {
        $crate::prelude::Vector::new([$x])
    };
    ($x:expr, $y:expr$(,)?) => {
        $crate::prelude::Vector::new([$x, $y])
    };
    ($x:expr, $y:expr, $z:expr$(,)?) => {
        $crate::prelude::Vector::new([$x, $y, $z])
    };
}

impl<T, const N: usize> Vector<T, N> {
    /// Constructs a `Vector` from `[T; N]` coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let v = Vector::new([2.1]);
    /// assert_eq!(v.values(), [2.1]);
    ///
    /// let v = Vector::new([2.1, 3.5]);
    /// assert_eq!(v.values(), [2.1, 3.5]);
    ///
    /// let v = Vector::new([2.1, 3.5, 1.0]);
    /// assert_eq!(v.values(), [2.1, 3.5, 1.0]);
    /// ```
    #[inline]
    pub const fn new(coords: [T; N]) -> Self {
        Self(coords)
    }

    /// Constructs a `Vector` at the origin.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let v: VectorF3 = Vector::origin();
    /// assert_eq!(v.values(), [0.0, 0.0, 0.0]);
    /// ```
    #[inline]
    pub fn origin() -> Self
    where
        T: Default + Copy,
    {
        Self::new([T::default(); N])
    }
}

impl<T> Vector<T, 2>
where
    T: Num + Float,
{
    /// Constructs a `Vector` from another `Vector`, rotated by an `angle`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// use pix_engine::math::constants::FRAC_PI_2;
    /// let v1: VectorF2 = Vector::new([10.0, 20.0]);
    /// let v2 = Vector::rotated(v1, FRAC_PI_2);
    /// assert!(v2.approx_eq(vector![-20.0, 10.0], 1e-4));
    /// ```
    pub fn rotated<V>(v: V, angle: T) -> Self
    where
        V: Into<Vector<T, 2>>,
    {
        let mut v = v.into();
        v.rotate(angle);
        v
    }

    /// Constructs a 2D unit `Vector` in the XY plane from a given angle. Angle is given as
    /// radians and is unaffected by [AngleMode](crate::prelude::AngleMode).
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v: VectorF2 = Vector::from_angle(FRAC_PI_4, 15.0);
    /// assert!(v.approx_eq(vector!(10.6066, 10.6066), 1e-4));
    /// ```
    pub fn from_angle(angle: T, length: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new([length * cos, length * sin])
    }

    /// Returns the 2D angular direction of the `Vector`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v: VectorF2 = vector!(10.0, 10.0);
    /// let heading = v.heading();
    /// assert_eq!(heading.to_degrees(), 45.0);
    /// ```
    pub fn heading(&self) -> T {
        self.y().atan2(self.x())
    }

    /// Rotate a 2D `Vector` by an angle in radians, magnitude remains the same. Unaffected by
    /// [AngleMode](crate::prelude::AngleMode).
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// use pix_engine::math::constants::FRAC_PI_2;
    /// let mut v: VectorF2 = vector!(10.0, 20.0);
    /// v.rotate(FRAC_PI_2);
    /// assert!(v.approx_eq(vector![-20.0, 10.0], 1e-4));
    /// ```
    pub fn rotate(&mut self, angle: T) {
        let new_heading = self.heading() + angle;
        let mag = self.mag();
        let (sin, cos) = new_heading.sin_cos();
        self.set_x(cos * mag);
        self.set_y(sin * mag);
    }
}

impl<T> Vector<T, 3>
where
    T: Num + Float,
{
    /// Returns the [cross product](https://en.wikipedia.org/wiki/Cross_product) between two
    /// `Vector`s. Only defined for 3D `Vector`s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let v1: VectorF3 = vector!(1.0, 2.0, 3.0);
    /// let v2: VectorF3 = vector!(1.0, 2.0, 3.0);
    /// let cross = v1.cross(v2);
    /// assert_eq!(cross.values(), [0.0, 0.0, 0.0]);
    /// ```
    pub fn cross<V: Into<Vector<T, 3>>>(&self, v: V) -> Self {
        let v = v.into();
        Self::new([
            self.y() * v.z() - self.z() * v.y(),
            self.z() * v.x() - self.x() * v.z(),
            self.x() * v.y() - self.y() * v.x(),
        ])
    }

    /// Returns the angle between two 3D `Vector`s in radians.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let v1: VectorF3 = vector!(1.0, 0.0, 0.0);
    /// let v2: VectorF3 = vector!(0.0, 1.0, 0.0);
    /// let angle = v1.angle_between(v2);
    /// assert_eq!(angle, std::f64::consts::FRAC_PI_2);
    /// ```
    pub fn angle_between<V: Into<Vector<T, 3>>>(&self, v: V) -> T {
        let v = v.into();
        // This should range from -1.0 to 1.0, inclusive but could possibly land outside this range
        // due to floating-point rounding, so we'll need to clamp it to the correct range.
        let dot_mag_product =
            num_traits::clamp(self.dot(v) / (self.mag() * v.mag()), -T::one(), T::one());
        dot_mag_product.acos() * self.cross(v).z().signum()
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Copy + Default,
{
    /// Constructs a `Vector` from a [Point].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = point!(1.0, 2.0);
    /// let v: VectorF2 = Vector::from_point(p);
    /// assert_eq!(v.values(), [1.0, 2.0]);
    /// ```
    #[inline]
    pub fn from_point(p: Point<T, N>) -> Self {
        Self::new(p.values())
    }

    /// Returns the `x-coordinate`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = vector!(1.0, 2.0);
    /// assert_eq!(v.x(), 1.0);
    /// ```
    #[inline]
    pub fn x(&self) -> T {
        match self.0.get(0) {
            Some(z) => *z,
            None => T::default(),
        }
    }

    /// Sets the `x-magnitude`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v = vector!(1.0, 2.0);
    /// v.set_x(3.0);
    /// assert_eq!(v.values(), [3.0, 2.0]);
    /// ```
    #[inline]
    pub fn set_x(&mut self, x: T) {
        if !self.0.is_empty() {
            self.0[0] = x;
        }
    }

    /// Returns the `y-magnitude`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = vector!(1.0, 2.0);
    /// assert_eq!(v.y(), 2.0);
    /// ```
    #[inline]
    pub fn y(&self) -> T {
        match self.0.get(1) {
            Some(z) => *z,
            None => T::default(),
        }
    }

    /// Sets the `y-magnitude`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v = vector!(1.0, 2.0);
    /// v.set_y(3.0);
    /// assert_eq!(v.values(), [1.0, 3.0]);
    /// ```
    #[inline]
    pub fn set_y(&mut self, y: T) {
        if self.0.len() > 1 {
            self.0[1] = y;
        }
    }

    /// Returns the `z-magnitude`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let v = vector!(1.0, 2.0, 2.5);
    /// assert_eq!(v.z(), 2.5);
    /// ```
    #[inline]
    pub fn z(&self) -> T {
        match self.0.get(2) {
            Some(z) => *z,
            None => T::default(),
        }
    }

    /// Sets the `z-magnitude`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let mut v = vector!(1.0, 2.0, 1.0);
    /// v.set_z(3.0);
    /// assert_eq!(v.values(), [1.0, 2.0, 3.0]);
    /// ```
    #[inline]
    pub fn set_z(&mut self, z: T) {
        if self.0.len() > 2 {
            self.0[2] = z;
        }
    }

    /// Get `Vector` coordinates as `[T; N]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let v: VectorF3 = vector!(2.0, 1.0, 3.0);
    /// assert_eq!(v.values(), [2.0, 1.0, 3.0]);
    /// ```
    #[inline]
    pub fn values(&self) -> [T; N] {
        self.0
    }

    /// Set `Vector` coordinates from `[x, y, z]`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let mut v: VectorF3 = Vector::new([2.0, 1.0, 3.0]);
    /// assert_eq!(v.values(), [2.0, 1.0, 3.0]);
    /// v.set_values([1.0, 2.0, 4.0]);
    /// assert_eq!(v.values(), [1.0, 2.0, 4.0]);
    /// ```
    #[inline]
    pub fn set_values(&mut self, coords: [T; N]) {
        self.0 = coords;
    }

    /// Returns `Vector` as a [Vec].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let v: VectorF3 = vector!(1.0, 1.0, 0.0);
    /// assert_eq!(v.to_vec(), vec![1.0, 1.0, 0.0]);
    /// ```
    #[inline]
    pub fn to_vec(self) -> Vec<T> {
        self.0.to_vec()
    }

    /// Convert `Vector<T, N>` to `Vector<U, N>` using the `as` operator.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v1: VectorF2 = vector!(1.5, 2.0);
    /// let v2: VectorI2 = v1.as_();
    /// assert_eq!(v2.values(), [1, 2]);
    /// ```
    #[inline]
    pub fn as_<U>(self) -> Vector<U, N>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy + Default,
    {
        let mut coords = [U::default(); N];
        for i in 0..N {
            coords[i] = self[i].as_();
        }
        Vector::new(coords)
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Num,
{
    /// Constructs a `Vector` by shifting coordinates by given amount.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let mut v: VectorF3 = vector!(2.0, 3.0, 1.5);
    /// v.offset([2.0, -4.0]);
    /// assert_eq!(v.values(), [4.0, -1.0, 1.5]);
    /// ```
    pub fn offset<U, const M: usize>(&mut self, offsets: [U; M])
    where
        T: AddAssign<U>,
        U: Copy,
    {
        assert!(N >= M);
        for i in 0..M {
            self[i] += offsets[i]
        }
    }

    /// Constructs a `Vector` by multiplying it by the given scale factor.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let mut v: VectorF3 = vector!(2.0, 3.0, 1.5);
    /// v.scale(2.0);
    /// assert_eq!(v.values(), [4.0, 6.0, 3.0]);
    /// ```
    pub fn scale<U>(&mut self, s: U)
    where
        T: MulAssign<U>,
        U: Num,
    {
        *self *= s;
    }

    /// Wraps `Vector` around the given `[T; N]`, and size (radius).
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v: VectorF2 = vector!(200.0, 300.0);
    /// v.wrap([150.0, 400.0], 10.0);
    /// assert_eq!(v.values(), [-10.0, 300.0]);
    ///
    /// let mut v: VectorF2 = vector!(-100.0, 300.0);
    /// v.wrap([150.0, 400.0], 10.0);
    /// assert_eq!(v.values(), [160.0, 300.0]);
    /// ```
    pub fn wrap(&mut self, wrap: [T; N], size: T)
    where
        T: Signed,
    {
        for i in 0..N {
            if self[i] > wrap[i] + size {
                self[i] = -size;
            } else if self[i] < -size {
                self[i] = wrap[i] + size;
            }
        }
    }

    /// Constructs a random unit `Vector` in 1D space.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let v: VectorF3 = Vector::random();
    /// assert!(v.x() > -1.0 && v.x() < 1.0);
    /// assert!(v.y() > -1.0 && v.y() < 1.0);
    /// assert!(v.z() > -1.0 && v.z() < 1.0);
    ///
    /// // May make v's (x, y, z) values something like:
    /// // (0.61554617, 0.0, 0.0) or
    /// // (-0.4695841, 0.0, 0.0) or
    /// // (0.6091097, 0.0, 0.0)
    /// ```
    pub fn random() -> Self
    where
        T: SampleUniform,
    {
        let mut coords = [T::zero(); N];
        for coord in &mut coords {
            *coord = random!(T::one());
        }
        Vector::new(coords)
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Num + Float,
{
    /// Constructs a `Vector` from a reflection about a normal to a line in 2D space or a plane in 3D
    /// space.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let v1: VectorF3 = Vector::new([1.0, 1.0, 0.0]);
    /// let normal = Vector::new([0.0, 1.0, 0.0]);
    /// let v2: VectorF3 = Vector::reflection(v1, normal);
    /// assert_eq!(v2.values(), [-1.0, 1.0, 0.0]);
    /// ```
    pub fn reflection<V>(v: V, normal: V) -> Self
    where
        V: Into<Vector<T, N>>,
    {
        let mut v = v.into();
        v.reflect(normal);
        v
    }

    /// Constructs a unit `Vector` of length `1` from another `Vector`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let v1: VectorF3 = Vector::new([0.0, 5.0, 0.0]);
    /// let v2: VectorF3 = Vector::normalized(v1);
    /// assert_eq!(v2.values(), [0.0, 1.0, 0.0]);
    /// ```
    pub fn normalized<V>(v: V) -> Self
    where
        V: Into<Vector<T, N>>,
    {
        let mut v = v.into();
        v.normalize();
        v
    }

    /// Returns the magnitude (length) of the `Vector`.
    ///
    /// The formula used for 2D is `sqrt(x*x + y*y)`.
    /// The formula used for 3D is `sqrt(x*x + y*y + z*z)`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let v: VectorF3 = vector!(1.0, 2.0, 3.0);
    /// let abs_difference = (v.mag() - 3.7416).abs();
    /// assert!(abs_difference <= 1e-4);
    /// ```
    pub fn mag(&self) -> T {
        self.mag_sq().sqrt()
    }

    /// Returns the squared magnitude (length) of the `Vector`. This is faster if the real length
    /// is not required in the case of comparing vectors.
    ///
    /// The formula used for 2D is `x*x + y*y`.
    /// The formula used for 3D is `x*x + y*y + z*z`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let v: VectorF3 = vector!(1.0, 2.0, 3.0);
    /// assert_eq!(v.mag_sq(), 14.0);
    /// ```
    pub fn mag_sq(&self) -> T {
        let mut sum = T::zero();
        for i in 0..N {
            sum += self[i] * self[i]
        }
        sum
    }

    /// Returns the [dot product](https://en.wikipedia.org/wiki/Dot_product) betwen two `Vector`s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let v1: VectorF3 = vector!(1.0, 2.0, 3.0);
    /// let v2: VectorF3 = vector!(2.0, 3.0, 4.0);
    /// let dot_product = v1.dot(v2);
    /// assert_eq!(dot_product, 20.0);
    /// ```
    pub fn dot<V: Into<Vector<T, N>>>(&self, v: V) -> T {
        let v = v.into();
        let mut sum = T::zero();
        for i in 0..N {
            sum += self[i] * v[i]
        }
        sum
    }

    /// Reflect `Vector` about a normal to a line in 2D space or a plane in 3D space.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v: VectorF2 = vector!(4.0, 6.0); // Vector heading right and down
    /// let n: VectorF2 = vector!(0.0, 1.0); // Surface normal facing up
    /// v.reflect(n); // Reflect about the surface normal (e.g. the x-axis)
    /// assert_eq!(v.x(), -4.0);
    /// assert_eq!(v.y(), 6.0);
    /// ```
    pub fn reflect<V: Into<Vector<T, N>>>(&mut self, normal: V) {
        let normal = Self::normalized(normal);
        *self = normal * ((T::one() + T::one()) * self.dot(normal)) - *self;
    }

    /// Set the magnitude (length) of the `Vector`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let mut v: VectorF3 = vector!(10.0, 20.0, 2.0);
    /// v.set_mag(10.0);
    /// assert!(v.approx_eq(vector![4.4543, 8.9087, 0.8908], 1e-4));
    /// ```
    pub fn set_mag(&mut self, mag: T) {
        self.normalize();
        *self *= mag;
    }

    /// Returns the Euclidean distance between two `Vector`s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let v1: VectorF3 = vector!(1.0, 0.0, 0.0);
    /// let v2: VectorF3 = vector!(0.0, 1.0, 0.0);
    /// let dist = v1.dist(v2);
    /// let abs_difference: f64 = (dist - std::f64::consts::SQRT_2).abs();
    /// assert!(abs_difference <= 1e-4);
    /// ```
    pub fn dist<V: Into<Vector<T, N>>>(&self, v: V) -> T {
        let v = v.into();
        (*self - v).mag()
    }

    /// Normalize the `Vector` to length `1` making it a unit vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let mut v: VectorF3 = vector!(10.0, 20.0, 2.0);
    /// v.normalize();
    /// assert!(v.approx_eq(vector!(0.4454, 0.8908, 0.0890), 1e-4));
    /// ```
    pub fn normalize(&mut self) {
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
    /// # use pix_engine::prelude_3d::*;
    /// let mut v: VectorF3 = vector!(10.0, 20.0, 2.0);
    /// v.limit(5.0);
    /// assert!(v.approx_eq(vector!(2.2271, 4.4543,  0.4454), 1e-4));
    /// ```
    pub fn limit(&mut self, max: T) {
        let mag_sq = self.mag_sq();
        if mag_sq > max * max {
            *self /= mag_sq.sqrt();
            *self *= max;
        }
    }

    /// Constructs a `Vector` by linear interpolating between two `Vector`s by a given amount
    /// between `0.0` and `1.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let v1: VectorF3 = vector!(1.0, 1.0, 0.0);
    /// let v2: VectorF3 = vector!(3.0, 3.0, 0.0);
    /// let v3 = v1.lerp(v2, 0.5);
    /// assert_eq!(v3.values(), [2.0, 2.0, 0.0]);
    /// ```
    pub fn lerp<V: Into<Vector<T, N>>>(&self, v: V, amt: T) -> Self {
        let v = v.into();
        let lerp = |start, stop, amt| amt * (stop - start) + start;
        let amt = num_traits::clamp(amt, T::zero(), T::one());
        let mut coords = [T::zero(); N];
        for i in 0..N {
            coords[i] = lerp(self[i], v[i], amt);
        }
        Self::new(coords)
    }

    /// Returns whether two `Vector`s are approximately equal.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude_3d::*;
    /// let v1: VectorF3 = vector!(10.0, 20.0, 2.0);
    /// let v2: VectorF3 = vector!(10.0001, 20.0, 2.0);
    /// assert!(v1.approx_eq(v2, 1e-3));
    /// ```
    pub fn approx_eq<V: Into<Vector<T, N>>>(&self, other: V, epsilon: T) -> bool {
        let other = other.into();
        let mut approx_eq = true;
        for i in 0..N {
            approx_eq &= (self[i] - other[i]).abs() < epsilon;
        }
        approx_eq
    }
}

impl<T, const N: usize> Default for Vector<T, N>
where
    T: Default + Copy,
{
    /// Return default `Vector` as origin.
    fn default() -> Self {
        Self::origin()
    }
}

impl<T, const N: usize> Deref for Vector<T, N> {
    type Target = [T; N];
    /// Deref `Vector` to `&[T; N]`.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for Vector<T, N> {
    /// Deref `Vector` to `&mut [T; N]`.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N>
where
    T: Copy,
{
    type Output = T;
    /// Return `&T` by indexing `Point` with `usize`.
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N>
where
    T: Copy,
{
    /// Return `&mut T` by indexing `Point` with `usize`.
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl<T, const N: usize> From<&Vector<T, N>> for Vector<T, N>
where
    T: Copy,
{
    /// Convert `&Vector` to `Vector`.
    fn from(v: &Vector<T, N>) -> Self {
        *v
    }
}

impl<T, const N: usize> From<&mut Vector<T, N>> for Vector<T, N>
where
    T: Copy,
{
    /// Convert `&mut Vector` to `Vector`.
    fn from(v: &mut Vector<T, N>) -> Self {
        *v
    }
}

// Operations

impl<T, const N: usize> Add for Point<T, N>
where
    T: Num,
{
    type Output = Vector<T, N>;
    /// [Point] + [Point] yields a [Vector].
    fn add(self, p: Point<T, N>) -> Self::Output {
        let mut arr = [T::zero(); N];
        for i in 0..N {
            arr[i] = self[i] + p[i];
        }
        Vector::new(arr)
    }
}

impl<T, const N: usize> Add<Vector<T, N>> for Point<T, N>
where
    T: Num,
{
    type Output = Point<T, N>;
    /// [Point] + [Vector] yields a [Point].
    fn add(mut self, v: Vector<T, N>) -> Self::Output {
        for i in 0..N {
            self[i] += v[i];
        }
        self
    }
}

impl<T, const N: usize> Add<Point<T, N>> for Vector<T, N>
where
    T: Num,
{
    type Output = Point<T, N>;
    /// [Vector] + [Point] yields a [Point].
    fn add(self, p: Point<T, N>) -> Self::Output {
        let mut arr = [T::zero(); N];
        for i in 0..N {
            arr[i] = self[i] + p[i];
        }
        Point::new(arr)
    }
}

impl<T, const N: usize> Add for Vector<T, N>
where
    T: Num,
{
    type Output = Self;
    /// [Vector] + [Vector] yields a [Vector].
    fn add(mut self, v: Vector<T, N>) -> Self::Output {
        for i in 0..N {
            self[i] += v[i];
        }
        Vector::new(self.values())
    }
}

impl<T, U, const N: usize> Add<U> for Vector<T, N>
where
    T: Num + Add<U, Output = T>,
    U: Num,
{
    type Output = Self;
    /// [Vector] + U.
    fn add(mut self, val: U) -> Self::Output {
        for i in 0..N {
            self[i] = self[i] + val;
        }
        self
    }
}

impl<T, const N: usize> AddAssign for Vector<T, N>
where
    T: Num,
{
    /// [Vector] += [Vector].
    fn add_assign(&mut self, v: Vector<T, N>) {
        for i in 0..N {
            self[i] += v[i];
        }
    }
}

impl<T, const N: usize> AddAssign<Vector<T, N>> for Point<T, N>
where
    T: Num,
{
    /// [Point] += [Vector].
    fn add_assign(&mut self, v: Vector<T, N>) {
        for i in 0..N {
            self[i] += v[i];
        }
    }
}

impl<T, U, const N: usize> AddAssign<U> for Vector<T, N>
where
    T: Num + AddAssign<U>,
    U: Num,
{
    /// [Vector] += U.
    fn add_assign(&mut self, val: U) {
        for i in 0..N {
            self[i] += val;
        }
    }
}

impl<T, const N: usize> Sub for Vector<T, N>
where
    T: Num,
{
    type Output = Self;
    /// [Vector] - [Vector] yields a [Vector].
    fn sub(mut self, v: Vector<T, N>) -> Self::Output {
        for i in 0..N {
            self[i] -= v[i];
        }
        Vector::new(self.values())
    }
}

impl<T, const N: usize> Sub<Point<T, N>> for Vector<T, N>
where
    T: Num,
{
    type Output = Point<T, N>;
    /// [Vector] - [Point] yields a [Point].
    fn sub(self, p: Point<T, N>) -> Self::Output {
        let mut arr = [T::zero(); N];
        for i in 0..N {
            arr[i] = self[i] - p[i];
        }
        Point::new(arr)
    }
}

impl<T, const N: usize> Sub for Point<T, N>
where
    T: Num,
{
    type Output = Vector<T, N>;
    /// [Point] - [Point] yields a [Vector].
    fn sub(self, p: Point<T, N>) -> Self::Output {
        let mut arr = [T::zero(); N];
        for i in 0..N {
            arr[i] = self[i] - p[i];
        }
        Vector::new(arr)
    }
}

impl<T, const N: usize> Sub<Vector<T, N>> for Point<T, N>
where
    T: Num,
{
    type Output = Point<T, N>;
    /// [Point] - [Vector] yields a [Point].
    fn sub(mut self, v: Vector<T, N>) -> Self::Output {
        for i in 0..N {
            self[i] -= v[i];
        }
        self
    }
}

impl<T, U, const N: usize> Sub<U> for Vector<T, N>
where
    T: Num + Sub<U, Output = T>,
    U: Num,
{
    type Output = Self;
    /// [Vector] - U.
    fn sub(mut self, val: U) -> Self::Output {
        for i in 0..N {
            self[i] = self[i] - val;
        }
        self
    }
}

impl<T, const N: usize> SubAssign for Vector<T, N>
where
    T: Num,
{
    /// [Vector] -= [Vector].
    fn sub_assign(&mut self, v: Vector<T, N>) {
        for i in 0..N {
            self[i] -= v[i];
        }
    }
}

impl<T, const N: usize> SubAssign<Vector<T, N>> for Point<T, N>
where
    T: Num,
{
    /// [Point] -= [Vector].
    fn sub_assign(&mut self, v: Vector<T, N>) {
        for i in 0..N {
            self[i] -= v[i];
        }
    }
}

impl<T, U, const N: usize> SubAssign<U> for Vector<T, N>
where
    T: Num + SubAssign<U>,
    U: Num,
{
    /// [Vector] -= U.
    fn sub_assign(&mut self, val: U) {
        for i in 0..N {
            self[i] -= val;
        }
    }
}

impl<T, const N: usize> Neg for Vector<T, N>
where
    T: Num + Neg<Output = T>,
{
    type Output = Self;
    /// ![Vector].
    fn neg(mut self) -> Self::Output {
        for i in 0..N {
            self[i] = self[i].neg();
        }
        self
    }
}

impl<T, U, const N: usize> Mul<U> for Vector<T, N>
where
    T: Num + Mul<U, Output = T>,
    U: Num,
{
    type Output = Self;
    /// [Vector] * U.
    fn mul(mut self, s: U) -> Self::Output {
        for i in 0..N {
            self[i] = self[i] * s;
        }
        self
    }
}

impl<T, U, const N: usize> MulAssign<U> for Vector<T, N>
where
    T: Num + MulAssign<U>,
    U: Num,
{
    /// [Vector] *= U.
    fn mul_assign(&mut self, s: U) {
        for i in 0..N {
            self[i] *= s;
        }
    }
}

impl<T, U, const N: usize> Div<U> for Vector<T, N>
where
    T: Num + Div<U, Output = T>,
    U: Num,
{
    type Output = Self;
    /// [Vector] / U.
    fn div(mut self, s: U) -> Self::Output {
        for i in 0..N {
            self[i] = self[i] / s;
        }
        self
    }
}

impl<T, U, const N: usize> DivAssign<U> for Vector<T, N>
where
    T: Num + DivAssign<U>,
    U: Num,
{
    /// [Vector] /= U.
    fn div_assign(&mut self, s: U) {
        for i in 0..N {
            self[i] /= s;
        }
    }
}

// Required because of orphan rules.
macro_rules! impl_primitive_mul {
    ($($target:ty),*) => {
        $(
            impl<const N: usize> Mul<Vector<$target, N>> for $target {
                type Output = Vector<$target, N>;
                /// T * [Vector].
                fn mul(self, mut v: Vector<$target, N>) -> Self::Output {
                    for i in 0..N {
                        v[i] *= self;
                    }
                    v
                }
            }
        )*
    };
}

impl_primitive_mul!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);

impl<T, const N: usize> IntoIterator for Vector<T, N> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item, N>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.0)
    }
}

impl<T, const N: usize> Sum for Vector<T, N>
where
    T: Default + Copy + Add,
    Self: Add<Output = Self>,
{
    /// Sum a list of `Vector`s.
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let v = Vector::origin();
        iter.fold(v, |a, b| a + b)
    }
}

impl<'a, T, const N: usize> Sum<&'a Vector<T, N>> for Vector<T, N>
where
    T: Default + Copy + Add,
    Self: Add<Output = Self>,
{
    /// Sum a list of `&Vector`s.
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        let v = Vector::origin();
        iter.fold(v, |a, b| a + *b)
    }
}

impl<T, const N: usize> Product for Vector<T, N>
where
    T: Default + Copy + Mul,
    Self: Mul<Output = Self>,
{
    /// Multiply a list of `Vector`s.
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let v = Vector::origin();
        iter.fold(v, |a, b| a * b)
    }
}

impl<'a, T, const N: usize> Product<&'a Vector<T, N>> for Vector<T, N>
where
    T: Default + Copy + Mul,
    Self: Mul<Output = Self>,
{
    /// Multiply a list of `&Vector`s.
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        let v = Vector::origin();
        iter.fold(v, |a, b| a * *b)
    }
}

macro_rules! impl_from_as {
    ($($from:ty),* => $to:ty, $zero:expr) => {
        $(
            impl<const N: usize> From<Vector<$from, N>> for Vector<$to, N> {
                /// Convert `Vector<U, N>` to `Vector<T, N>`.
                fn from(v: Vector<$from, N>) -> Self {
                    let mut coords = [$zero; N];
                    for i in 0..N {
                        coords[i] = v[i] as $to;
                    }
                    Self::new(coords)
                }
            }

            impl<const N: usize> From<[$from; N]> for Vector<$to, N> {
                /// Convert `[T; N]` to [Vector].
                fn from(arr: [$from; N]) -> Self {
                    let mut coords = [$zero; N];
                    for i in 0..N {
                        coords[i] = arr[i] as $to;
                    }
                    Self::new(coords)
                }
            }

            impl<const N: usize> From<&[$from; 3]> for Vector<$to, N> {
                /// Convert `&[x, y, z]` to [Vector].
                fn from(&arr: &[$from; 3]) -> Self {
                    let mut coords = [$zero; N];
                    for i in 0..N {
                        coords[i] = arr[i] as $to;
                    }
                    Self::new(coords)
                }
            }

            impl<const N: usize> From<Point<$from, N>> for Vector<$to, N> {
                /// Converts [Point] to [Vector].
                fn from(p: Point<$from, N>) -> Self {
                    let mut coords = [$zero; N];
                    for i in 0..N {
                        coords[i] = p[i] as $to;
                    }
                    Self::new(coords)
                }
            }

            impl<const N: usize> From<&Point<$from, N>> for Vector<$to, N> {
                /// Converts &[Point] to [Vector].
                fn from(p: &Point<$from, N>) -> Self {
                    let mut coords = [$zero; N];
                    for i in 0..N {
                        coords[i] = p[i] as $to;
                    }
                    Self::new(coords)
                }
            }

            impl<const N: usize> From<Vector<$from, N>> for Point<$to, N> {
                /// Converts [Vector] to [Point].
                fn from(v: Vector<$from, N>) -> Self {
                    let mut coords = [$zero; N];
                    for i in 0..N {
                        coords[i] = v[i] as $to;
                    }
                    Self::new(coords)
                }
            }

            impl<const N: usize> From<&Vector<$from, N>> for Point<$to, N> {
                /// Converts &[Vector] to [Point].
                fn from(v: &Vector<$from, N>) -> Self {
                    let mut coords = [$zero; N];
                    for i in 0..N {
                        coords[i] = v[i] as $to;
                    }
                    Self::new(coords)
                }
            }
        )*
    };
}

impl_from_as!(i8, u8, i16, u16, u32, i64, u64, isize, usize, f32, f64 => i32, 0);
impl_from_as!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32 => f64, 0.0);

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    /// Convert `[T; N]` to `Vector`.
    fn from(arr: [T; N]) -> Self {
        Self::new(arr)
    }
}

impl<T, const N: usize> From<&[T; N]> for Vector<T, N>
where
    T: Copy,
{
    /// Convert `&[T; N]` to `Vector`.
    fn from(&arr: &[T; N]) -> Self {
        Self::new(arr)
    }
}

impl<T, const N: usize> From<Vector<T, N>> for [T; N]
where
    T: Copy + Default,
{
    /// Convert [Vector] to `[T; N]`.
    fn from(v: Vector<T, N>) -> Self {
        v.values()
    }
}

impl<T, const N: usize> From<&Vector<T, N>> for [T; N]
where
    T: Copy + Default,
{
    /// Convert &[Vector] to `[T; N]`.
    fn from(v: &Vector<T, N>) -> Self {
        v.values()
    }
}

impl<T, const N: usize> From<Vector<T, N>> for Point<T, N>
where
    T: Copy + Default,
{
    /// Convert [Vector] to [Point].
    fn from(v: Vector<T, N>) -> Self {
        Self::new(v.values())
    }
}

impl<T, const N: usize> From<Point<T, N>> for Vector<T, N>
where
    T: Copy + Default,
{
    /// Convert [Point] to [Vector].
    fn from(p: Point<T, N>) -> Self {
        Self::new(p.values())
    }
}

impl<T, const N: usize> fmt::Display for Vector<T, N>
where
    T: Copy + Default + fmt::Debug,
{
    /// Display [Vector] as a string of coordinates.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.values())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_ops {
        ($val:expr, $e:expr) => {
            // Mul<T> for Vector
            let v = vector!(2.0, -5.0, 0.0) * $val;
            assert!(v.approx_eq([4.0, -10.0, 0.0], $e));

            // Mul<Vector> for T
            let v = $val * vector!(2.0, -5.0, 0.0);
            assert!(v.approx_eq([4.0, -10.0, 0.0], $e));

            // MulAssign<T> for Vector
            let mut v = vector!(2.0, -5.0, 0.0);
            v *= $val;
            assert!(v.approx_eq([4.0, -10.0, 0.0], $e));

            // Div<T> for Vector
            let v = vector!(1.0, -5.0, 0.0) / $val;
            assert!(v.approx_eq([0.5, -2.5, 0.0], $e));

            // DivAssign<T> for Vector
            let mut v = vector!(2.0, -5.0, 0.0);
            v /= $val;
            assert!(v.approx_eq([1.0, -2.5, 0.0], $e));
        };
    }

    #[test]
    fn test_ops() {
        // Add
        let v1 = vector!(2.0, 5.0, 1.0);
        let v2 = vector!(1.0, 5.0, -1.0);
        let v3 = v1 + v2;
        assert!(v3.approx_eq([3.0, 10.0, 0.0], 1e-4));

        // AddAssign
        let mut v1 = vector!(2.0, 5.0, 1.0);
        let v2 = vector!(1.0, 5.0, -1.0);
        v1 += v2;
        assert!(v1.approx_eq([3.0, 10.0, 0.0], 1e-4));

        // Sub
        let v1 = vector!(2.0, 1.0, 2.0);
        let v2 = vector!(1.0, 5.0, 3.0);
        let v3 = v1 - v2;
        assert!(v3.approx_eq([1.0, -4.0, -1.0], 1e-4));

        // SubAssign
        let mut v1 = vector!(2.0, 1.0, 2.0);
        let v2 = vector!(1.0, 5.0, 3.0);
        v1 -= v2;
        assert!(v1.approx_eq([1.0, -4.0, -1.0], 1e-4));

        test_ops!(2.0f32, f32::EPSILON);
        test_ops!(2.0f64, f64::EPSILON);
    }

    #[test]
    fn test_slice_conversions() {
        let _: Vector<u8, 1> = [50u8].into();
        let _: Vector<i8, 1> = [50i8].into();
        let _: Vector<u16, 1> = [50u16].into();
        let _: Vector<i16, 1> = [50i16].into();
        let _: Vector<u32, 1> = [50u32].into();
        let _: Vector<i32, 1> = [50i32].into();
        let _: Vector<f32, 1> = [50.0f32].into();
        let _: Vector<f64, 1> = [50.0f64].into();

        let _: Vector<u8, 2> = [50u8, 100].into();
        let _: Vector<i8, 2> = [50i8, 100].into();
        let _: Vector<u16, 2> = [50u16, 100].into();
        let _: Vector<i16, 2> = [50i16, 100].into();
        let _: Vector<u32, 2> = [50u32, 100].into();
        let _: Vector<i32, 2> = [50i32, 100].into();
        let _: Vector<f32, 2> = [50.0f32, 100.0].into();
        let _: Vector<f64, 2> = [50.0f64, 100.0].into();

        let _: Vector<u8, 3> = [50u8, 100, 55].into();
        let _: Vector<i8, 3> = [50i8, 100, 55].into();
        let _: Vector<u16, 3> = [50u16, 100, 55].into();
        let _: Vector<i16, 3> = [50i16, 100, 55].into();
        let _: Vector<u32, 3> = [50u32, 100, 55].into();
        let _: Vector<i32, 3> = [50i32, 100, 55].into();
        let _: Vector<f32, 3> = [50.0f32, 100.0, 55.0].into();
        let _: Vector<f64, 3> = [50.0f64, 100.0, 55.0].into();
    }

    #[test]
    fn test_member_methods() {
        let epsilon = f64::EPSILON;
        let arr: [f64; 0] = [];
        let mut v: Vector<f64, 0> = arr.into();
        assert!(v.x() < epsilon);
        assert!(v.y() < epsilon);
        assert!(v.z() < epsilon);
        v.set_x(1.0);
        v.set_y(1.0);
        v.set_z(1.0);
        assert!(v.x() < epsilon);
        assert!(v.y() < epsilon);
        assert!(v.z() < epsilon);

        let mut v: Vector<f64, 1> = [1.0].into();
        assert!((v.x() - 1.0).abs() < epsilon);
        assert!(v.y() < epsilon);
        assert!(v.z() < epsilon);
        v.set_x(2.0);
        v.set_y(1.0);
        v.set_z(1.0);
        assert!((v.x() - 2.0).abs() < epsilon);
        assert!(v.y() < epsilon);
        assert!(v.z() < epsilon);

        let mut v: Vector<f64, 2> = [1.0, 2.0].into();
        assert!((v.x() - 1.0).abs() < epsilon);
        assert!((v.y() - 2.0).abs() < epsilon);
        assert!(v.z() < epsilon);
        v.set_x(2.0);
        v.set_y(4.0);
        v.set_z(1.0);
        assert!((v.x() - 2.0).abs() < epsilon);
        assert!((v.y() - 4.0).abs() < epsilon);
        assert!(v.z() < epsilon);

        let mut v: Vector<f64, 3> = [1.0, 2.0, 3.0].into();
        assert!((v.x() - 1.0).abs() < epsilon);
        assert!((v.y() - 2.0).abs() < epsilon);
        assert!((v.z() - 3.0).abs() < epsilon);
        v.set_x(2.0);
        v.set_y(4.0);
        v.set_z(6.0);
        assert!((v.x() - 2.0).abs() < epsilon);
        assert!((v.y() - 4.0).abs() < epsilon);
        assert!((v.z() - 6.0).abs() < epsilon);
    }

    #[test]
    fn test_deref() {
        let v = vector!(1.0, 2.0, 3.0);
        let mut iter = v.into_iter();
        assert_eq!(iter.next(), Some(1.0));
        assert_eq!(iter.next(), Some(2.0));
        assert_eq!(iter.next(), Some(3.0));
        assert_eq!(iter.next(), None);
    }
}
