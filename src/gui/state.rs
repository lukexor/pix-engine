//! GUI State.

use crate::{
    gui::{keys::KeyState, mouse::MouseState},
    prelude::*,
};
#[cfg(target_pointer_width = "32")]
use hash32::{FnvHasher, Hash, Hasher};
use indexmap::IndexMap;
use std::{
    cmp,
    collections::{hash_map::Entry, HashMap},
};
#[cfg(target_pointer_width = "64")]
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

/// A hashed element identifier for internal state management.
#[cfg(target_pointer_width = "32")]
pub(crate) type ElementId = u32;
/// A hashed element identifier for internal state management.
#[cfg(target_pointer_width = "64")]
pub(crate) type ElementId = u64;

/// UI Texture with source and destination.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Texture {
    pub(crate) id: TextureId,
    pub(crate) src: Option<Rect<i32>>,
    pub(crate) dst: Option<Rect<i32>>,
    pub(crate) visible: bool,
}

impl Texture {
    pub(crate) fn new(id: TextureId, src: Option<Rect<i32>>, dst: Option<Rect<i32>>) -> Self {
        Self {
            id,
            src,
            dst,
            visible: true,
        }
    }
}

/// Internal tracked UI state.
#[derive(Default, Debug)]
pub(crate) struct UiState {
    /// Current global render position, in window coordinates.
    cursor: PointI2,
    /// Previous global render position, in window coordinates.
    pub(crate) pcursor: PointI2,
    /// Current line height.
    pub(crate) line_height: i32,
    /// Previous line height.
    pub(crate) pline_height: i32,
    /// Temporary stack of cursor positions.
    cursor_stack: Vec<(PointI2, PointI2)>,
    /// ID stack to assist with generating unique element IDs.
    id_stack: Vec<ElementId>,
    /// Override for max-width elements.
    pub(crate) next_width: Option<u32>,
    /// UI texture to be drawn over rendered frame, in rendered order.
    pub(crate) textures: IndexMap<ElementId, Texture>,
    /// Whether UI elements are disabled.
    pub(crate) disabled: bool,
    /// Mouse state for the current frame.
    pub(crate) mouse: MouseState,
    /// Mouse position offset for rendering within textures and viewports.
    pub(crate) mouse_offset: Option<PointI2>,
    /// Mouse state for the previous frame.
    pub(crate) pmouse: MouseState,
    /// Keyboard state for the current frame.
    pub(crate) keys: KeyState,
    /// Element state for the current frame,
    pub(crate) elements: HashMap<ElementId, ElementState>,
    /// Which element is active.
    active: Option<ElementId>,
    /// Which element is hovered.
    hovered: Option<ElementId>,
    /// Which element is focused.
    focused: Option<ElementId>,
    /// Last focusable element rendered.
    last_focusable: Option<ElementId>,
    /// Last bounding box rendered.
    last_size: Option<Rect<i32>>,
}

impl UiState {
    /// Handle state changes this frame prior to calling [AppState::on_update].
    #[inline]
    pub(crate) fn pre_update(&mut self, theme: &Theme) {
        self.clear_hovered();
        self.pcursor = point!();
        self.cursor = theme.style.frame_pad;
    }

    /// Handle state changes this frame after calling [AppState::on_update].
    #[inline]
    pub(crate) fn post_update(&mut self) {
        for texture in &mut self.textures.values_mut() {
            texture.visible = false;
        }

        if !self.mouse.is_down(Mouse::Left) {
            self.clear_active();
        } else if !self.has_active() {
            // Disable focused state while mouse is down from previous frame
            self.set_active(0);
        }
        if self.keys.was_entered(Key::Tab) {
            self.blur();
        }
        self.clear_entered();
    }

    /// Helper function to hash element labels.
    #[inline]
    pub(crate) fn get_id<T: Hash>(&self, t: &T) -> ElementId {
        #[cfg(target_pointer_width = "32")]
        let mut s = FnvHasher::default();
        #[cfg(target_pointer_width = "64")]
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        if let Some(id) = self.id_stack.last() {
            id.hash(&mut s);
        }
        s.finish()
    }

    /// Returns the current UI rendering position.
    #[inline]
    pub(crate) fn cursor(&self) -> PointI2 {
        self.cursor
    }

    /// Set the current UI rendering position.
    #[inline]
    pub(crate) fn set_cursor<P: Into<PointI2>>(&mut self, cursor: P) {
        self.cursor = cursor.into();
    }

    /// Push a new UI rendering position to the stack.
    #[inline]
    pub(crate) fn push_cursor(&mut self) {
        self.cursor_stack.push((self.pcursor, self.cursor));
    }

    /// Pop a new UI rendering position from the stack.
    #[inline]
    pub(crate) fn pop_cursor(&mut self) {
        if let Some((pcursor, cursor)) = self.cursor_stack.pop() {
            self.pcursor = pcursor;
            self.cursor = cursor;
        }
    }

    /// Push a new seed to the ID stack.
    #[inline]
    pub(crate) fn push_id(&mut self, id: ElementId) {
        self.id_stack.push(id);
    }

    /// Pop a seed from the ID stack.
    #[inline]
    pub(crate) fn pop_id(&mut self) {
        self.id_stack.pop();
    }

    /// Returns the current mouse position coordinates as `(x, y)`.
    #[inline]
    pub(crate) fn mouse_pos(&self) -> PointI2 {
        let mut pos = self.mouse.pos;
        if let Some(offset) = self.mouse_offset {
            pos.offset(-offset);
        }
        pos
    }

    /// Returns the previous mouse position coordinates last frame as `(x, y)`.
    #[inline]
    pub(crate) fn pmouse_pos(&self) -> PointI2 {
        let mut pos = self.pmouse.pos;
        if let Some(offset) = self.mouse_offset {
            pos.offset(-offset);
        }
        pos
    }

    /// Set the current mouse position.
    #[inline]
    pub(crate) fn set_mouse_pos<P: Into<PointI2>>(&mut self, pos: P) {
        self.pmouse.pos = self.mouse.pos;
        self.mouse.pos = pos.into();
    }

    /// Set a mouse offset for rendering within textures or viewports.
    #[inline]
    pub(crate) fn set_mouse_offset<P: Into<PointI2>>(&mut self, offset: P) {
        self.mouse_offset = Some(offset.into());
    }

    /// Clear mouse offset for rendering within textures or viewports.
    #[inline]
    pub(crate) fn clear_mouse_offset(&mut self) {
        self.mouse_offset = None;
    }

    /// Whether an element is `active` or not. An element is marked `active` when there is no other
    /// `active elements, it is marked `hovered` and receives a mouse down event for the
    /// [Mouse::Left] button. `active` is cleared after every frame.
    #[inline]
    pub(crate) fn is_active(&self, id: ElementId) -> bool {
        !self.disabled && matches!(self.active, Some(el) if el == id)
    }

    /// Whether any element is currently `active`.
    #[inline]
    pub(crate) fn has_active(&self) -> bool {
        self.active.is_some()
    }

    /// Set a given element as `active`.
    #[inline]
    pub(crate) fn set_active(&mut self, id: ElementId) {
        self.active = Some(id);
    }

    /// Clears the current `active` element.
    #[inline]
    pub(crate) fn clear_active(&mut self) {
        self.active = None;
    }

    /// Whether an element is `hovered` or not. When an element is considered `hovered` depends on
    /// the widget, but generally involves checking if the [PixState::mouse_pos] is within the
    /// elements bounding area.
    #[inline]
    pub(crate) fn is_hovered(&self, id: ElementId) -> bool {
        matches!(self.hovered, Some(el) if el == id)
    }

    /// Whether any element currently is `hovered`.
    #[inline]
    pub(crate) fn has_hover(&self) -> bool {
        self.hovered.is_some()
    }

    /// Set a given element as `hovered` and check for [Mouse::Left] being down to set `active`
    /// only if there are no other `active` elements. `active` is cleared after every frame.
    #[inline]
    pub(crate) fn hover(&mut self, id: ElementId) {
        self.hovered = Some(id);
        // If mouse is down this frame while hovered, make element active, if something else isn't
        // already
        if !self.has_active() && self.mouse.is_down(Mouse::Left) {
            self.set_active(id);
        }
    }

    /// Clears the current `hovered` element.
    #[inline]
    pub(crate) fn clear_hovered(&mut self) {
        self.hovered = None;
    }

    /// Try to capture `hover` if no other element is currently `hovered`.
    #[inline]
    pub(crate) fn try_hover<S: Contains<i32, 2>>(&mut self, id: ElementId, shape: S) -> bool {
        if !self.has_hover() && !self.disabled && shape.contains_point(self.mouse_pos()) {
            self.hover(id);
        }
        self.is_hovered(id)
    }

    /// Whether an element is `focused` or not. An element is `focused` when it captures it via
    /// tab-cycling, or if it is clicked.
    #[inline]
    pub(crate) fn is_focused(&self, id: ElementId) -> bool {
        !self.disabled && matches!(self.focused, Some(el) if el == id)
    }

    /// Whether any element currently has `focus`.
    #[inline]
    pub(crate) fn has_focused(&self) -> bool {
        self.focused.is_some()
    }

    /// Set a given element as `focused`.
    #[inline]
    pub(crate) fn focus(&mut self, id: ElementId) {
        self.focused = Some(id);
    }

    /// Try to capture `focus` if no other element is currently `focued`. This supports tab-cycling
    /// through elements with the keyboard.
    #[inline]
    pub(crate) fn try_focus(&mut self, id: ElementId) -> bool {
        if !self.has_focused() {
            self.focus(id);
        }
        self.is_focused(id)
    }

    /// Clears the current `focused` element.
    #[inline]
    pub(crate) fn blur(&mut self) {
        self.focused = None;
    }

    /// Handles global element inputs for `focused` checks.
    #[inline]
    pub(crate) fn handle_events(&mut self, id: ElementId) {
        // Tab-focus cycling
        // If element is focused when Tab pressed, clear it so the next element can capture focus.
        // If SHIFT was held, re-focus the last element rendered
        // Clear keys, so next element doesn't trigger tab logic
        if self.is_focused(id) && self.keys.was_entered(Key::Tab) {
            self.blur();
            if self.keys.mod_down(KeyMod::SHIFT) {
                self.focused = self.last_focusable;
            }
            self.clear_entered();
        }
        // Click focusing
        let clicked = !self.mouse.is_down(Mouse::Left) && self.is_hovered(id) && self.is_active(id);
        if clicked {
            self.focus(id);
        }
        self.last_focusable = Some(id);
    }

    /// Whether this element was `clicked` this frame. Treats [Key::Return] being pressed while
    /// focused a a `click`.
    ///
    /// If element is hovered and active, it means it was clicked last frame
    /// If mouse isn't down this frame, it means mouse was both clicked and released above an
    /// element
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

    /// Return what, if any, [Key] was entered this frame. This is cleared at the end of each
    /// frame.
    #[inline]
    pub(crate) fn key_entered(&self) -> Option<Key> {
        self.keys.entered
    }

    /// Clear all per-frame events.
    #[inline]
    pub(crate) fn clear_entered(&mut self) {
        self.keys.typed = None;
        self.keys.entered = None;
        self.mouse.xrel = 0;
        self.mouse.yrel = 0;
    }

    /// Returns the current `scroll` state for this element.
    #[inline]
    pub(crate) fn scroll(&self, id: ElementId) -> VectorI2 {
        self.elements
            .get(&id)
            .map(|s| s.scroll)
            .unwrap_or_else(VectorI2::default)
    }

    /// Set the current `scroll` state for this element.
    #[inline]
    pub(crate) fn set_scroll(&mut self, id: ElementId, scroll: VectorI2) {
        let state = match self.elements.entry(id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(ElementState::default()),
        };
        state.scroll = scroll;
    }
}

impl PixState {
    /// Disables any UI elements drawn after this is called.
    pub fn disable(&mut self) {
        self.ui.disabled = true;
    }

    /// Enables any UI elements drawn after this is called.
    pub fn no_disable(&mut self) {
        self.ui.disabled = false;
    }

    /// Returns the current UI rendering position.
    #[inline]
    pub fn cursor_pos(&self) -> PointI2 {
        self.ui.cursor()
    }

    /// Set the current UI rendering position.
    #[inline]
    pub fn set_cursor_pos<P: Into<PointI2>>(&mut self, cursor: P) {
        self.ui.set_cursor(cursor.into());
    }

    /// Returns whether the last item drawn is hovered with the mouse.
    #[inline]
    pub fn hovered(&self) -> bool {
        if let Some(rect) = self.ui.last_size {
            !self.ui.disabled && rect.contains_point(self.mouse_pos())
        } else {
            false
        }
    }

    /// Returns whether the last item drawn is hovered with the mouse.
    #[inline]
    pub fn clicked(&self) -> bool {
        if let Some(rect) = self.ui.last_size {
            !self.ui.disabled
                && rect.contains_point(self.mouse_pos())
                && self.ui.mouse.last_clicked(Mouse::Left).is_some()
        } else {
            false
        }
    }
}

impl PixState {
    /// Advance the current UI cursor position for an element.
    #[inline]
    pub(crate) fn advance_cursor<R: Into<Rect<i32>>>(&mut self, rect: R) {
        let rect = rect.into();
        let pos = self.cursor_pos();
        let style = self.theme.style;
        let padx = style.frame_pad.y();
        let pady = style.item_pad.y();

        // Previous cursor ends at the right of this item
        self.ui.pcursor = point![pos.x() + rect.width(), pos.y()];

        // Move cursor to the next line with padding, choosing the maximum of the next line or the
        // previous y value to account for variable line heights when using `same_line`.
        let line_height = cmp::max(self.ui.line_height, rect.height());
        self.ui.cursor = point![padx, pos.y() + line_height + pady];
        self.ui.pline_height = line_height;
        self.ui.line_height = 0;
        self.ui.last_size = Some(rect);
    }
}

/// Internal tracked UI element state.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ElementState {
    scroll: VectorI2,
}
