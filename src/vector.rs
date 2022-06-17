//! A [Euclidean] `Vector` in N-dimensional space.
//!
//! Each [Vector] represents a N-dimensional [Euclidean] (or geometric) vector with a magnitude
//! and a direction. The [Vector] `struct`, however, contains N values for each dimensional
//! coordinate. The magnitude and direction are retrieved with the [`Vector::mag`] and
//! [`Vector::heading`] methods.
//!
//! Some example uses of a [Vector] include modeling a position, velocity, or acceleration of an
//! object or particle in 2D or 3D space.
//!
//! # Examples
//!
//! You can create a [Vector] using [`Vector::new`]:
//!
//! ```
//! # use pix_engine::prelude::*;
//! let v = Vector::new([10.0, 20.0, 15.0]);
//! ```
//! ...or by using the [vector!] macro:
//!
//! ```
//! # use pix_engine::prelude::*;
//! let v: Vector<f64, 3> = vector!(); // vector at the origin (0, 0, 0) with no direction or magnitude
//! assert_eq!(v.coords(), [0.0, 0.0, 0.0]);
//!
//! let v = vector!(5.0); // 1D vector on the x-axis with magnitude 5
//! assert_eq!(v.coords(), [5.0]);
//!
//! let v = vector!(5.0, 10.0); // 2D vector in the x/y-plane
//! assert_eq!(v.coords(), [5.0, 10.0]);
//!
//! let v = vector!(-1.5, 3.0, 2.2); // 3D vector
//! assert_eq!(v.coords(), [-1.5, 3.0, 2.2]);
//! ```
//!
//! You can also create random `Vector`s using [`Vector::random`] which create unit vectors with
//! magnitudes in the range `-1.0..=1.0`.
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! let v: Vector<f64, 1> = Vector::random();
//! // `v.coords()` will return something like:
//! // [-0.9993116191591512, 0.03709835324533284, 0.0]
//! assert!(v.x() >= -1.0 && v.x() <= 1.0);
//!
//! let v: Vector<f64, 2> = Vector::random();
//! // `v.coords()` will return something like:
//! // [-0.9993116191591512, 0.03709835324533284, 0.0]
//! assert!(v.x() >= -1.0 && v.x() <= 1.0);
//! assert!(v.y() >= -1.0 && v.y() <= 1.0);
//!
//! let v: Vector<f64, 3> = Vector::random();
//! // `v.coords()` will return something like:
//! // [-0.40038099206441835, 0.8985763512414204, 0.17959844705110184]
//! assert!(v.x() >= -1.0 && v.x() <= 1.0);
//! assert!(v.y() >= -1.0 && v.y() <= 1.0);
//! assert!(v.z() >= -1.0 && v.z() <= 1.0);
//! ```
//!
//! [Euclidean]: https://en.wikipedia.org/wiki/Euclidean_vector

use crate::prelude::*;
#[cfg(feature = "serde")]
use crate::serialize::arrays;
use num_traits::Signed;
use rand::distributions::uniform::SampleUniform;
#[cfg(feature = "serde")]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fmt, ops::MulAssign};

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
/// [module-level documentation]: mod@crate::vector
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[must_use]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound = "T: Serialize + DeserializeOwned"))]
pub struct Vector<T = f64, const N: usize = 2>(
    #[cfg_attr(feature = "serde", serde(with = "arrays"))] pub(crate) [T; N],
);

/// Constructs a [Vector].
///
/// # Examples
///
/// ```
/// # use pix_engine::prelude::*;
/// let v: Vector<f64, 3> = vector!();
/// assert_eq!(v.coords(), [0.0, 0.0, 0.0]);
///
/// let v = vector!(1.0);
/// assert_eq!(v.coords(), [1.0]);
///
/// let v = vector!(1.0, 2.0);
/// assert_eq!(v.coords(), [1.0, 2.0]);
///
/// let v = vector!(1.0, -2.0, 1.0);
/// assert_eq!(v.coords(), [1.0, -2.0, 1.0]);
/// ```
#[macro_export]
macro_rules! vector {
    () => {
        $crate::prelude::Vector::origin()
    };
    ($x:expr) => {
        $crate::prelude::Vector::from_x($x)
    };
    ($x:expr, $y:expr$(,)?) => {
        $crate::prelude::Vector::from_xy($x, $y)
    };
    ($x:expr, $y:expr, $z:expr$(,)?) => {
        $crate::prelude::Vector::from_xyz($x, $y, $z)
    };
}

impl<T, const N: usize> Vector<T, N> {
    /// Constructs a `Vector` from `[T; N]` coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = Vector::new([2.1]);
    /// assert_eq!(v.coords(), [2.1]);
    ///
    /// let v = Vector::new([2.1, 3.5]);
    /// assert_eq!(v.coords(), [2.1, 3.5]);
    ///
    /// let v = Vector::new([2.1, 3.5, 1.0]);
    /// assert_eq!(v.coords(), [2.1, 3.5, 1.0]);
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
    /// # use pix_engine::prelude::*;
    /// let v: Vector<f64, 3> = Vector::origin();
    /// assert_eq!(v.coords(), [0.0, 0.0, 0.0]);
    /// ```
    #[inline]
    pub fn origin() -> Self
    where
        T: Default,
    {
        Self::new([(); N].map(|_| T::default()))
    }
}

impl<T> Vector<T, 1> {
    /// Constructs a `Vector` from an individual x coordinate.
    #[inline]
    pub const fn from_x(x: T) -> Self {
        Self([x])
    }
}

impl<T> Vector<T> {
    /// Constructs a `Vector` from individual x/y coordinates.
    #[inline]
    pub const fn from_xy(x: T, y: T) -> Self {
        Self([x, y])
    }
}

impl<T> Vector<T, 3> {
    /// Constructs a `Vector` from individual x/y/z coordinates.
    #[inline]
    pub const fn from_xyz(x: T, y: T, z: T) -> Self {
        Self([x, y, z])
    }
}

impl<T: Num + Float> Vector<T> {
    /// Constructs a `Vector` from another `Vector`, rotated by an `angle`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// use pix_engine::math::FRAC_PI_2;
    /// let v1 = Vector::new([10.0, 20.0]);
    /// let v2 = Vector::rotated(v1, FRAC_PI_2);
    /// assert!(v2.approx_eq(vector![-20.0, 10.0], 1e-4));
    /// ```
    pub fn rotated<V>(v: V, angle: T) -> Self
    where
        V: Into<Vector<T>>,
    {
        let mut v = v.into();
        v.rotate(angle);
        v
    }

    /// Constructs a 2D unit `Vector` in the XY plane from a given angle. Angle is given as
    /// radians and is unaffected by [`AngleMode`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// use pix_engine::math::FRAC_PI_4;
    /// let v = Vector::from_angle(FRAC_PI_4, 15.0);
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
    /// let v = vector!(10.0, 10.0);
    /// let heading: f64 = v.heading();
    /// assert_eq!(heading.to_degrees(), 45.0);
    /// ```
    pub fn heading(&self) -> T {
        self.y().atan2(self.x())
    }

    /// Rotate a 2D `Vector` by an angle in radians, magnitude remains the same. Unaffected by
    /// [`AngleMode`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// use pix_engine::math::FRAC_PI_2;
    /// let mut v = vector!(10.0, 20.0);
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

impl<T: Num + Float> Vector<T, 3> {
    /// Returns the [cross product](https://en.wikipedia.org/wiki/Cross_product) between two
    /// `Vector`s. Only defined for 3D `Vector`s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v1 = vector!(1.0, 2.0, 3.0);
    /// let v2 = vector!(1.0, 2.0, 3.0);
    /// let cross = v1.cross(v2);
    /// assert_eq!(cross.coords(), [0.0, 0.0, 0.0]);
    /// ```
    pub fn cross<V>(&self, v: V) -> Self
    where
        V: Into<Vector<T, 3>>,
    {
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
    /// # use pix_engine::prelude::*;
    /// let v1 = vector!(1.0, 0.0, 0.0);
    /// let v2 = vector!(0.0, 1.0, 0.0);
    /// let angle = v1.angle_between(v2);
    /// assert_eq!(angle, std::f64::consts::FRAC_PI_2);
    /// ```
    pub fn angle_between<V>(&self, v: V) -> T
    where
        V: Into<Vector<T, 3>>,
    {
        let v = v.into();
        // This should range from -1.0 to 1.0, inclusive but could possibly land outside this range
        // due to floating-point rounding, so we'll need to clamp it to the correct range.
        let dot_mag_product =
            num_traits::clamp(self.dot(v) / (self.mag() * v.mag()), -T::one(), T::one());
        dot_mag_product.acos() * self.cross(v).z().signum()
    }
}

impl<T: Copy, const N: usize> Vector<T, N> {
    /// Constructs a `Vector` from a [Point].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let p = point!(1.0, 2.0);
    /// let v = Vector::from_point(p);
    /// assert_eq!(v.coords(), [1.0, 2.0]);
    /// ```
    #[inline]
    pub fn from_point(p: Point<T, N>) -> Self {
        Self::new(p.coords())
    }

    /// Returns the `x-coordinate`.
    ///
    /// # Panics
    ///
    /// If `Vector` has zero dimensions.
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
        self.0[0]
    }

    /// Sets the `x-magnitude`.
    ///
    /// # Panics
    ///
    /// If `Vector` has zero dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v = vector!(1.0, 2.0);
    /// v.set_x(3.0);
    /// assert_eq!(v.coords(), [3.0, 2.0]);
    /// ```
    #[inline]
    pub fn set_x(&mut self, x: T) {
        self.0[0] = x;
    }

    /// Returns the `y-magnitude`.
    ///
    /// # Panics
    ///
    /// If `Vector` has less than 2 dimensions.
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
        self.0[1]
    }

    /// Sets the `y-magnitude`.
    ///
    /// # Panics
    ///
    /// If `Vector` has less than 2 dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v = vector!(1.0, 2.0);
    /// v.set_y(3.0);
    /// assert_eq!(v.coords(), [1.0, 3.0]);
    /// ```
    #[inline]
    pub fn set_y(&mut self, y: T) {
        self.0[1] = y;
    }

    /// Returns the `z-magnitude`.
    ///
    /// # Panics
    ///
    /// If `Vector` has less than 3 dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = vector!(1.0, 2.0, 2.5);
    /// assert_eq!(v.z(), 2.5);
    /// ```
    #[inline]
    pub fn z(&self) -> T {
        self.0[2]
    }

    /// Sets the `z-magnitude`.
    ///
    /// # Panics
    ///
    /// If `Vector` has less than 3 dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v = vector!(1.0, 2.0, 1.0);
    /// v.set_z(3.0);
    /// assert_eq!(v.coords(), [1.0, 2.0, 3.0]);
    /// ```
    #[inline]
    pub fn set_z(&mut self, z: T) {
        self.0[2] = z;
    }

    /// Get `Vector` coordinates as `[T; N]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v = vector!(2.0, 1.0, 3.0);
    /// assert_eq!(v.coords(), [2.0, 1.0, 3.0]);
    /// ```
    #[inline]
    pub fn coords(&self) -> [T; N] {
        self.0
    }

    /// Get `Vector` coordinates as a mutable slice `&[T; N]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut vector = vector!(2.0, 1.0, 3.0);
    /// for v in vector.coords_mut() {
    ///     *v *= 2.0;
    /// }
    /// assert_eq!(vector.coords(), [4.0, 2.0, 6.0]);
    /// ```
    #[inline]
    pub fn coords_mut(&mut self) -> &mut [T; N] {
        &mut self.0
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
    #[inline]
    pub fn to_vec(self) -> Vec<T> {
        self.0.to_vec()
    }
}

impl<T: Num, const N: usize> Vector<T, N> {
    /// Constructs a `Vector` by shifting coordinates by given amount.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v = vector!(2.0, 3.0, 1.5);
    /// v.offset([2.0, -4.0]);
    /// assert_eq!(v.coords(), [4.0, -1.0, 1.5]);
    /// ```
    #[inline]
    pub fn offset<V, const M: usize>(&mut self, offsets: V)
    where
        V: Into<Vector<T, M>>,
    {
        let offsets = offsets.into();
        for (v, o) in self.iter_mut().zip(offsets) {
            *v += o;
        }
    }

    /// Offsets the `x-coordinate` of the point by a given amount.
    ///
    /// # Panics
    ///
    /// If `Point` has zero dimensions.
    #[inline]
    pub fn offset_x(&mut self, offset: T) {
        self.0[0] += offset;
    }

    /// Offsets the `y-coordinate` of the point by a given amount.
    ///
    /// # Panics
    ///
    /// If `Vector` has less than 2 dimensions.
    #[inline]
    pub fn offset_y(&mut self, offset: T) {
        self.0[1] += offset;
    }

    /// Offsets the `z-coordinate` of the point by a given amount.
    ///
    /// # Panics
    ///
    /// If `Vector` has less than 3 dimensions.
    #[inline]
    pub fn offset_z(&mut self, offset: T) {
        self.0[2] += offset;
    }

    /// Constructs a `Vector` by multiplying it by the given scale factor.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut v = vector!(2.0, 3.0, 1.5);
    /// v.scale(2.0);
    /// assert_eq!(v.coords(), [4.0, 6.0, 3.0]);
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
    /// let mut v = vector!(200.0, 300.0);
    /// v.wrap([150.0, 400.0], 10.0);
    /// assert_eq!(v.coords(), [-10.0, 300.0]);
    ///
    /// let mut v = vector!(-100.0, 300.0);
    /// v.wrap([150.0, 400.0], 10.0);
    /// assert_eq!(v.coords(), [160.0, 300.0]);
    /// ```
    pub fn wrap(&mut self, wrap: [T; N], size: T)
    where
        T: Signed,
    {
        for (v, w) in self.iter_mut().zip(wrap) {
            let w = w + size;
            if *v > w {
                *v = -size;
            } else if *v < -size {
                *v = w;
            }
        }
    }

    /// Constructs a random unit `Vector` in 1D space.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v: Vector<f64, 3> = Vector::random();
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
        Self::new(coords)
    }
}

impl<T: Num + Float, const N: usize> Vector<T, N> {
    /// Constructs a `Vector` from a reflection about a normal to a line in 2D space or a plane in 3D
    /// space.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v1 = Vector::new([1.0, 1.0, 0.0]);
    /// let normal = Vector::new([0.0, 1.0, 0.0]);
    /// let v2 = Vector::reflection(v1, normal);
    /// assert_eq!(v2.coords(), [-1.0, 1.0, 0.0]);
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
    /// # use pix_engine::prelude::*;
    /// let v1 = Vector::new([0.0, 5.0, 0.0]);
    /// let v2 = Vector::normalized(v1);
    /// assert_eq!(v2.coords(), [0.0, 1.0, 0.0]);
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
    /// # use pix_engine::prelude::*;
    /// let v = vector!(1.0, 2.0, 3.0);
    /// let abs_difference = (v.mag() as f64 - 3.7416).abs();
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
    /// # use pix_engine::prelude::*;
    /// let v = vector!(1.0, 2.0, 3.0);
    /// assert_eq!(v.mag_sq(), 14.0);
    /// ```
    pub fn mag_sq(&self) -> T {
        let mut sum = T::zero();
        for &v in self.iter() {
            sum += v * v;
        }
        sum
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
    pub fn dot<V>(&self, o: V) -> T
    where
        V: Into<Vector<T, N>>,
    {
        let o = o.into();
        let mut sum = T::zero();
        for (&v, o) in self.iter().zip(o) {
            sum += v * o;
        }
        sum
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
    /// assert_eq!(v.x(), -4.0);
    /// assert_eq!(v.y(), 6.0);
    /// ```
    pub fn reflect<V>(&mut self, normal: V)
    where
        V: Into<Vector<T, N>>,
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
    /// # use pix_engine::prelude::*;
    /// let v1 = vector!(1.0, 0.0, 0.0);
    /// let v2 = vector!(0.0, 1.0, 0.0);
    /// let dist = v1.dist(v2);
    /// let abs_difference: f64 = (dist - std::f64::consts::SQRT_2).abs();
    /// assert!(abs_difference <= 1e-4);
    /// ```
    pub fn dist<V>(&self, v: V) -> T
    where
        V: Into<Vector<T, N>>,
    {
        (*self - v.into()).mag()
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
    /// # use pix_engine::prelude::*;
    /// let mut v = vector!(10.0, 20.0, 2.0);
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
    /// # use pix_engine::prelude::*;
    /// let v1 = vector!(1.0, 1.0, 0.0);
    /// let v2 = vector!(3.0, 3.0, 0.0);
    /// let v3 = v1.lerp(v2, 0.5);
    /// assert_eq!(v3.coords(), [2.0, 2.0, 0.0]);
    /// ```
    pub fn lerp<V>(&self, o: V, amt: T) -> Self
    where
        V: Into<Vector<T, N>>,
    {
        let o = o.into();
        let lerp = |start, stop, amt| amt * (stop - start) + start;
        let amt = num_traits::clamp(amt, T::zero(), T::one());
        let mut coords = [T::zero(); N];
        for ((c, &v), o) in coords.iter_mut().zip(self.iter()).zip(o) {
            *c = lerp(v, o, amt);
        }
        Self::new(coords)
    }

    /// Returns whether two `Vector`s are approximately equal.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let v1 = vector!(10.0, 20.0, 2.0);
    /// let v2 = vector!(10.0001, 20.0, 2.0);
    /// assert!(v1.approx_eq(v2, 1e-3));
    /// ```
    pub fn approx_eq<V>(&self, other: V, epsilon: T) -> bool
    where
        V: Into<Vector<T, N>>,
    {
        let other = other.into();
        let mut approx_eq = true;
        for (&v, o) in self.iter().zip(other) {
            approx_eq &= (v - o).abs() < epsilon;
        }
        approx_eq
    }
}

impl<T: Default, const N: usize> Default for Vector<T, N> {
    /// Return default `Vector` as origin.
    fn default() -> Self {
        Self::origin()
    }
}

impl<T, const N: usize> fmt::Display for Vector<T, N>
where
    [T; N]: fmt::Debug,
{
    /// Display [Vector] as a string of coordinates.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T: Copy, const N: usize> From<Point<T, N>> for Vector<T, N> {
    fn from(p: Point<T, N>) -> Self {
        Self::from_point(p)
    }
}

impl<T: Copy, const N: usize> From<&Point<T, N>> for Vector<T, N> {
    fn from(p: &Point<T, N>) -> Self {
        Self::from_point(*p)
    }
}

impl<T: Copy, const N: usize> From<Vector<T, N>> for Point<T, N> {
    fn from(v: Vector<T, N>) -> Self {
        Self::from_vector(v)
    }
}

impl<T: Copy, const N: usize> From<&Vector<T, N>> for Point<T, N> {
    fn from(v: &Vector<T, N>) -> Self {
        Self::from_vector(*v)
    }
}
