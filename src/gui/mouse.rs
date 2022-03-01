//! Mouse state management.

use crate::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};

/// Keep track of mouse position and clicks between frames.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub(crate) struct MouseState {
    pub(crate) pos: Point<i32>,
    pub(crate) xrel: i32,
    pub(crate) yrel: i32,
    pub(crate) pressed: HashSet<Mouse>,
    pub(crate) clicked: HashSet<Mouse>,
    pub(crate) last_clicked: HashMap<Mouse, Instant>,
    pub(crate) last_dbl_clicked: HashMap<Mouse, Instant>,
}

impl MouseState {
    /// Whether any [Mouse] buttons are pressed.
    #[inline]
    #[must_use]
    pub(crate) fn is_pressed(&self) -> bool {
        !self.pressed.is_empty()
    }

    /// Whether a [Mouse] buttons was clicked.
    #[inline]
    #[must_use]
    pub(crate) fn was_clicked(&self, btn: Mouse) -> bool {
        self.clicked.contains(&btn)
    }

    /// Whether a [Mouse] buttons was double clicked.
    #[inline]
    #[must_use]
    pub(crate) fn was_dbl_clicked(&self, btn: Mouse) -> bool {
        match (self.last_dbl_clicked(btn), self.last_clicked(btn)) {
            (Some(dbl), Some(clicked)) => {
                dbl >= clicked && (*dbl - *clicked) < Duration::from_millis(500)
            }
            _ => false,
        }
    }

    /// Returns if a specific [Mouse] button is currently being held.
    #[inline]
    #[must_use]
    pub(crate) fn is_down(&self, btn: Mouse) -> bool {
        self.pressed.contains(&btn)
    }

    /// Store a pressed [Mouse] button.
    #[inline]
    pub(crate) fn press(&mut self, btn: Mouse) {
        self.pressed.insert(btn);
    }

    /// Store mouse wheel motion.
    #[inline]
    pub(crate) fn wheel(&mut self, x: i32, y: i32) {
        self.xrel = x;
        self.yrel = y;
    }

    /// Remove a pressed [Mouse] button.
    #[inline]
    pub(crate) fn release(&mut self, btn: Mouse) {
        self.pressed.remove(&btn);
    }

    /// Store last time a [Mouse] button was clicked.
    #[inline]
    pub(crate) fn click(&mut self, btn: Mouse, time: Instant) {
        self.clicked.insert(btn);
        self.last_clicked.insert(btn, time);
    }

    /// Store last time a [Mouse] button was double clicked.
    #[inline]
    pub(crate) fn dbl_click(&mut self, btn: Mouse, time: Instant) {
        self.last_dbl_clicked.insert(btn, time);
    }

    /// Returns last time a [Mouse] button was clicked.
    #[inline]
    pub(crate) fn last_clicked(&self, btn: Mouse) -> Option<&Instant> {
        self.last_clicked.get(&btn)
    }

    /// Returns last time a [Mouse] button was double-clicked.
    #[inline]
    pub(crate) fn last_dbl_clicked(&self, btn: Mouse) -> Option<&Instant> {
        self.last_dbl_clicked.get(&btn)
    }
}
