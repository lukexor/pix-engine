//! [Light] source functions.

use crate::prelude::{Point, Scalar, Vector};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Source of [Light].
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LightSource<T> {
    /// Ambient light.
    Ambient,
    /// Light from a specific point.
    Point(Point<T>),
    /// Light from a specific direction.
    Direction(Vector<T>),
}

/// `Light` representation including `source` and `intensity`.
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Light<T = Scalar> {
    /// Source of light.
    pub source: LightSource<T>,
    /// Light intensity.
    pub intensity: T,
}

impl<T> Light<T> {
    /// Constructs a new `Light`.
    pub fn new(source: LightSource<T>, intensity: T) -> Self {
        Self { source, intensity }
    }

    /// Constructs a `Light` with `source` as [LightSource::Ambient].
    pub fn ambient(intensity: T) -> Self {
        Self::new(LightSource::Ambient, intensity)
    }

    /// Constructs a `Light` with `source` as [LightSource::Point].
    pub fn point<P>(intensity: T, position: P) -> Self
    where
        P: Into<Point<T>>,
    {
        Self::new(LightSource::Point(position.into()), intensity)
    }

    /// Constructs a `Light` with source as [LightSource::Direction].
    pub fn direction<V>(intensity: T, direction: V) -> Self
    where
        V: Into<Vector<T>>,
    {
        Self::new(LightSource::Direction(direction.into()), intensity)
    }
}

impl Default for Light {
    fn default() -> Self {
        Self::new(LightSource::Ambient, 1.0)
    }
}