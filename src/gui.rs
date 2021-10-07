//! Graphical User Interface

use self::{keys::KeyState, mouse::MouseState};
use crate::prelude::*;
use std::{
    collections::{
        hash_map::{DefaultHasher, Entry},
        HashMap,
    },
    hash::{Hash, Hasher},
};

pub use theme::*;

pub mod button;
pub mod keys;
pub mod list;
pub mod mouse;
pub mod slider;
pub mod text;
pub mod theme;

/// A hashed element identifier for internal state management.
pub(crate) type ElementId = u64;

/// Internal tracked UI state.
#[derive(Default, Debug, PartialEq, Eq)]
pub(crate) struct UiState {
    pub(crate) mouse: MouseState,
    pub(crate) pmouse: MouseState,
    pub(crate) keys: KeyState,
    pub(crate) elements: HashMap<ElementId, ElementState>,
    pub(crate) active: Option<ElementId>,
    pub(crate) hovered: Option<ElementId>,
    pub(crate) focused: Option<ElementId>,
    pub(crate) last_el: Option<ElementId>,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Scroll(i32, i32);

// TODO: Add horizontal scrolling
#[allow(dead_code)]
impl Scroll {
    #[inline]
    pub(crate) fn x(&self) -> i32 {
        self.0
    }

    #[inline]
    pub(crate) fn x_mut(&mut self) -> &mut i32 {
        &mut self.0
    }

    #[inline]
    pub(crate) fn y(&self) -> i32 {
        self.1
    }

    #[inline]
    pub(crate) fn set_y(&mut self, y: i32) {
        self.1 = y;
    }

    #[inline]
    pub(crate) fn y_mut(&mut self) -> &mut i32 {
        &mut self.1
    }
}

impl UiState {
    #[inline]
    pub(crate) fn is_active(&self, id: ElementId) -> bool {
        matches!(self.active, Some(el) if el == id)
    }

    #[inline]
    pub(crate) fn has_active(&self) -> bool {
        self.active.is_some()
    }

    #[inline]
    pub(crate) fn set_active(&mut self, id: ElementId) {
        self.active = Some(id);
    }

    #[inline]
    pub(crate) fn clear_active(&mut self) {
        self.active = None;
    }

    #[inline]
    pub(crate) fn is_hovered(&self, id: ElementId) -> bool {
        matches!(self.hovered, Some(el) if el == id)
    }

    #[inline]
    pub(crate) fn hover(&mut self, id: ElementId) {
        self.hovered = Some(id);
        // If mouse is down this frame while hovered, focus element
        if !self.has_active() && self.mouse.is_down(Mouse::Left) {
            self.active = Some(id);
        }
    }

    #[inline]
    pub(crate) fn try_capture(&mut self, id: ElementId) {
        if !self.has_focused() {
            self.focus(id);
        }
    }

    #[inline]
    pub(crate) fn is_focused(&self, id: ElementId) -> bool {
        matches!(self.focused, Some(el) if el == id)
    }

    #[inline]
    pub(crate) fn has_focused(&self) -> bool {
        self.focused.is_some()
    }

    #[inline]
    pub(crate) fn focus(&mut self, id: ElementId) {
        self.focused = Some(id);
    }

    #[inline]
    pub(crate) fn blur(&mut self) {
        self.focused = None;
    }

    #[inline]
    pub(crate) fn handle_tab(&mut self, id: ElementId) {
        if self.is_focused(id) && self.keys.was_entered(Key::Tab) {
            self.blur();
            if self.keys.mod_down(KeyMod::SHIFT) {
                self.focused = self.last_el;
            }
            self.clear_entered();
        }
    }

    #[inline]
    pub(crate) fn set_last(&mut self, id: ElementId) {
        self.last_el = Some(id);
    }

    #[inline]
    pub(crate) fn was_clicked(&mut self, id: ElementId) -> bool {
        // Enter simulates a click
        if self.is_focused(id) && self.keys.was_entered(Key::Return) {
            self.clear_entered();
            true
        } else {
            // Mouse is up, but we're hovered and active so user must have clicked
            let clicked =
                !self.mouse.is_down(Mouse::Left) && self.is_hovered(id) && self.is_active(id);
            if clicked {
                self.focus(id);
            }
            clicked
        }
    }

    #[inline]
    pub(crate) fn key_entered(&self) -> Option<Key> {
        self.keys.entered
    }

    #[inline]
    pub(crate) fn clear_entered(&mut self) {
        self.keys.typed = None;
        self.keys.entered = None;
    }

    #[inline]
    pub(crate) fn scroll(&self, id: ElementId) -> Scroll {
        self.elements
            .get(&id)
            .map(|s| s.scroll)
            .unwrap_or_else(Scroll::default)
    }

    #[inline]
    pub(crate) fn set_scroll(&mut self, id: ElementId, scroll: Scroll) {
        let state = match self.elements.entry(id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(ElementState::default()),
        };
        state.scroll = scroll;
    }
}

/// Internal tracked UI element state.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ElementState {
    scroll: Scroll,
}

/// Helper function to hash element labels.
pub(crate) fn get_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
