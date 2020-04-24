//! Coordinate and Matrix transformation state and manipulation routines.

use crate::State;

/// Contains the current matrix transformation state of the engine
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Transform {
    // TODO Transform
}

impl Transform {
    /// Creates a new Transform instance with the identity matrix
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn translate(&mut self, x: f32, y: f32) {}
}
