//! Graphical User Interface

use self::{keys::KeyState, mouse::MouseState};
use crate::prelude::*;
#[cfg(target_pointer_width = "32")]
use hash32::{FnvHasher, Hash, Hasher};
#[cfg(target_pointer_width = "64")]
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use std::{
    collections::{hash_map::Entry, HashMap},
    ops::{Deref, DerefMut},
};
pub use theme::*;

pub mod keys;
pub mod mouse;
pub mod theme;
pub mod widgets;
pub mod widgets2;

/// A hashed element identifier for internal state management.
#[cfg(target_pointer_width = "32")]
pub(crate) type ElementId = u32;
/// A hashed element identifier for internal state management.
#[cfg(target_pointer_width = "64")]
pub(crate) type ElementId = u64;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Size(pub(crate) (u32, u32));

#[allow(unused)]
impl Size {
    pub(crate) fn width(&self) -> u32 {
        (self.0).0
    }
    pub(crate) fn height(&self) -> u32 {
        (self.0).1
    }
}

impl From<(u32, u32)> for Size {
    fn from(size: (u32, u32)) -> Self {
        Self(size)
    }
}

impl Deref for Size {
    type Target = (u32, u32);
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Size {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Scroll {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

/// Internal tracked UI state.
#[derive(Default, Debug)]
pub(crate) struct UiState {
    pub(crate) same_line: bool,
    pub(crate) disabled: bool,
    pub(crate) mouse: MouseState,
    pub(crate) pmouse: MouseState,
    pub(crate) keys: KeyState,
    pub(crate) elements: HashMap<ElementId, ElementState>,
    pub(crate) active: Option<ElementId>,
    pub(crate) hovered: Option<ElementId>,
    pub(crate) focused: Option<ElementId>,
    pub(crate) last_el: Option<ElementId>,
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
        // If mouse is down this frame while hovered, make element active, if something else isn't
        // already
        if !self.has_active() && self.mouse.is_down(Mouse::Left) {
            self.set_active(id);
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
    pub(crate) fn handle_input(&mut self, id: ElementId) {
        // Tab-focus cycling
        if self.is_focused(id) && self.keys.was_entered(Key::Tab) {
            self.blur();
            if self.keys.mod_down(KeyMod::SHIFT) {
                self.focused = self.last_el;
            }
            self.clear_entered();
        }
        // Click focusing
        let clicked = !self.mouse.is_down(Mouse::Left) && self.is_hovered(id) && self.is_active(id);
        if clicked {
            self.focus(id);
        }
        self.same_line = false; // Reset same_line
        self.set_last(id);
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
        self.mouse.xrel = 0;
        self.mouse.yrel = 0;
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
pub(crate) fn get_hash<T: Hash>(t: &T) -> ElementId {
    #[cfg(target_pointer_width = "32")]
    let mut s = FnvHasher::default();
    #[cfg(target_pointer_width = "64")]
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    line!().hash(&mut s);
    s.finish()
}

impl PixState {
    /// Alters how UI methods lay out new lines for the next drawn element.
    pub fn same_line(&mut self) {
        self.ui.same_line = true;
    }

    /// Enables or sisables any UI elements drawn after this is called.
    pub fn ui_disable(&mut self, value: bool) {
        self.ui.disabled = value;
    }
}
