//! Light sources.

use crate::prelude::{Point, Vector};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Source of light.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LightSource<T> {
    /// Ambient light.
    Ambient,
    /// Light from a specific point.
    Point(Point<T>),
    /// Light from a specific direction.
    Direction(Vector<T>),
}

/// `Light` source.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Light<T> {
    /// Source of light.
    pub source: LightSource<T>,
    /// Light intensity.
    pub intensity: T,
}

impl<T> Light<T> {
    /// Create `Light` with source as `LightSource::Ambient`.
    pub fn ambient(intensity: T) -> Self {
        Self {
            source: LightSource::Ambient,
            intensity,
        }
    }

    /// Create `Light` with source as `LightSource::Point`.
    pub fn point<P>(intensity: T, position: P) -> Self
    where
        P: Into<Point<T>>,
    {
        Self {
            source: LightSource::Point(position.into()),
            intensity,
        }
    }

    /// Create `Light` with source as `LightSource::Direction`.
    pub fn direction<V>(intensity: T, direction: V) -> Self
    where
        V: Into<Vector<T>>,
    {
        Self {
            source: LightSource::Direction(direction.into()),
            intensity,
        }
    }
}
