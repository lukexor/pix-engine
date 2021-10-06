//! Mouse state management.

use crate::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

/// Keep track of mouse position and clicks between frames.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub(crate) struct MouseState {
    pub(crate) pos: PointI2,
    pub(crate) pressed: HashSet<Mouse>,
    pub(crate) last_clicked: HashMap<Mouse, Instant>,
}

impl MouseState {
    /// Whether any [Mouse] buttons are pressed.
    #[inline]
    pub(crate) fn is_pressed(&self) -> bool {
        !self.pressed.is_empty()
    }

    /// Returns if a specific [Mouse] button is currently being held.
    #[inline]
    pub(crate) fn is_down(&self, btn: Mouse) -> bool {
        self.pressed.contains(&btn)
    }

    /// Store a pressed [Mouse] button.
    #[inline]
    pub(crate) fn press(&mut self, btn: Mouse) {
        self.pressed.insert(btn);
    }

    /// Remove a pressed [Mouse] button.
    #[inline]
    pub(crate) fn release(&mut self, btn: Mouse) {
        self.pressed.remove(&btn);
    }

    /// Store last time a [Mouse] button was clicked.
    #[inline]
    pub(crate) fn click(&mut self, btn: Mouse, time: Instant) {
        self.last_clicked.insert(btn, time);
    }

    /// Returns last time a [Mouse] button was clicked.
    #[inline]
    pub(crate) fn last_clicked(&self, btn: Mouse) -> Option<&Instant> {
        self.last_clicked.get(&btn)
    }
}
