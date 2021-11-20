//! [Light] source functions.

use crate::prelude::*;
#[cfg(feature = "serde")]
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Source of [Light].
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound = "T: Serialize + DeserializeOwned"))]
pub enum LightSource<T, const N: usize> {
    /// Ambient light.
    Ambient,
    /// Light from a specific point.
    Point(Point<T, N>),
    /// Light from a specific direction.
    Direction(Vector<T, N>),
}

/// `Light` representation including `source` and `intensity`.
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound = "T: Serialize + DeserializeOwned"))]
pub struct Light<T, const N: usize> {
    /// Source of light.
    pub source: LightSource<T, N>,
    /// Light intensity.
    pub intensity: T,
}

/// A 2D `Light` represented by floating point numbers.
pub type LightF2 = Light<Scalar, 2>;

/// A 3D `Light` represented by floating point numbers.
pub type LightF3 = Light<Scalar, 3>;

impl<T, const N: usize> Light<T, N> {
    /// Constructs a new `Light`.
    pub const fn new(source: LightSource<T, N>, intensity: T) -> Self {
        Self { source, intensity }
    }

    /// Constructs a `Light` with `source` as [`LightSource::Ambient`].
    pub const fn ambient(intensity: T) -> Self {
        Self::new(LightSource::Ambient, intensity)
    }

    /// Constructs a `Light` with `source` as [`LightSource::Point`].
    pub fn point<P>(intensity: T, position: P) -> Self
    where
        P: Into<Point<T, N>>,
    {
        Self::new(LightSource::Point(position.into()), intensity)
    }

    /// Constructs a `Light` with source as [`LightSource::Direction`].
    pub fn direction<V>(intensity: T, direction: V) -> Self
    where
        V: Into<Vector<T, N>>,
    {
        Self::new(LightSource::Direction(direction.into()), intensity)
    }
}

impl<T, const N: usize> Default for Light<T, N>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(LightSource::Ambient, T::default())
    }
}
