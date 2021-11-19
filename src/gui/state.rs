//! GUI State.

use super::theme::FontId;
use crate::{
    gui::{keys::KeyState, mouse::MouseState},
    prelude::*,
};
use lru::LruCache;
use std::{
    cmp,
    collections::hash_map::DefaultHasher,
    error::Error,
    hash::{Hash, Hasher},
    mem,
    str::FromStr,
};

/// A hashed element identifier for internal state management.
pub(crate) type ElementId = u64;

const ELEMENT_CACHE_SIZE: usize = 128;

/// UI Texture with source and destination.
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Texture {
    pub(crate) id: TextureId,
    pub(crate) element_id: ElementId,
    pub(crate) src: Option<Rect<i32>>,
    pub(crate) dst: Option<Rect<i32>>,
    pub(crate) visible: bool,
    pub(crate) font_id: FontId,
    pub(crate) font_size: u32,
}

impl Texture {
    pub(crate) fn new(
        id: TextureId,
        element_id: ElementId,
        src: Option<Rect<i32>>,
        dst: Option<Rect<i32>>,
        font_id: FontId,
        font_size: u32,
    ) -> Self {
        Self {
            id,
            element_id,
            src,
            dst,
            visible: true,
            font_id,
            font_size,
        }
    }
}

/// Internal tracked UI state.
#[derive(Debug)]
pub(crate) struct UiState {
    /// Current global render position, in window coordinates.
    cursor: PointI2,
    /// Previous global render position, in window coordinates.
    pub(crate) pcursor: PointI2,
    /// X offset for global render position, in window coordinates.
    offset_x: i32,
    /// Current line height.
    pub(crate) line_height: i32,
    /// Previous line height.
    pub(crate) pline_height: i32,
    /// Temporary stack of cursor positions.
    cursor_stack: Vec<(PointI2, PointI2, i32, i32)>,
    /// ID stack to assist with generating unique element IDs.
    id_stack: Vec<ElementId>,
    /// Override for max-width elements.
    pub(crate) next_width: Option<u32>,
    /// UI texture to be drawn over rendered frame, in rendered order.
    pub(crate) textures: Vec<Texture>,
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
    pub(crate) elements: LruCache<ElementId, ElementState>,
    /// Which element is active.
    active: Option<ElementId>,
    /// Which element is hovered.
    hovered: Option<ElementId>,
    /// Which element is focused.
    focused: Option<ElementId>,
    /// Which element is being edited.
    editing: Option<ElementId>,
    /// Last focusable element rendered.
    last_focusable: Option<ElementId>,
    /// Last bounding box rendered.
    last_size: Option<Rect<i32>>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            cursor: point!(),
            pcursor: point!(),
            offset_x: 0,
            line_height: 0,
            pline_height: 0,
            cursor_stack: vec![],
            id_stack: vec![],
            next_width: None,
            textures: vec![],
            disabled: false,
            mouse: MouseState::default(),
            mouse_offset: None,
            pmouse: MouseState::default(),
            keys: KeyState::default(),
            elements: LruCache::new(ELEMENT_CACHE_SIZE),
            active: None,
            hovered: None,
            focused: None,
            editing: None,
            last_focusable: None,
            last_size: None,
        }
    }
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
        for texture in &mut self.textures {
            texture.visible = false;
        }

        if !self.mouse.is_down(Mouse::Left) {
            self.clear_active();
        } else if !self.has_active() {
            // Disable focused state while mouse is down from previous frame
            self.set_active(0);
        }
        self.clear_entered();
    }

    /// Helper function to hash element labels.
    #[inline]
    pub(crate) fn get_id<T: Hash>(&self, t: &T) -> ElementId {
        let mut hasher = DefaultHasher::new();
        t.hash(&mut hasher);
        if let Some(id) = self.id_stack.last() {
            id.hash(&mut hasher);
        }
        hasher.finish()
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

    /// Set the x offset for the current UI rendering position.
    #[inline]
    pub(crate) fn set_cursor_offset_x(&mut self, x: i32) {
        self.offset_x = x;
    }

    /// Clears the x offset for the current UI rendering position.
    #[inline]
    pub(crate) fn clear_cursor_offset(&mut self) {
        self.offset_x = 0;
    }

    /// Push a new UI rendering position to the stack.
    #[inline]
    pub(crate) fn push_cursor(&mut self) {
        self.cursor_stack.push((
            self.pcursor,
            self.cursor,
            self.pline_height,
            self.line_height,
        ));
    }

    /// Pop a new UI rendering position from the stack.
    #[inline]
    pub(crate) fn pop_cursor(&mut self) {
        if let Some((pcursor, cursor, pline_height, line_height)) = self.cursor_stack.pop() {
            self.pcursor = pcursor;
            self.cursor = cursor;
            self.pline_height = pline_height;
            self.line_height = line_height;
        }
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

    /// Whether an element is being edited or not.
    #[inline]
    pub(crate) fn is_editing(&self, id: ElementId) -> bool {
        !self.disabled && matches!(self.editing, Some(el) if el == id)
    }

    /// Start edit mode for a given element.
    #[inline]
    pub(crate) fn begin_edit(&mut self, id: ElementId) {
        self.editing = Some(id);
    }

    /// End edit mode for a given element.
    #[inline]
    pub(crate) fn end_edit(&mut self) {
        self.editing = None;
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
            !self.mouse.is_down(Mouse::Left) && self.is_hovered(id) && self.is_active(id)
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
        self.mouse.clicked.clear();
        self.mouse.xrel = 0;
        self.mouse.yrel = 0;
    }

    /// Returns the current `scroll` state for this element.
    #[inline]
    pub(crate) fn scroll(&mut self, id: ElementId) -> VectorI2 {
        self.elements
            .get(&id)
            .map(|s| s.scroll)
            .unwrap_or_else(VectorI2::default)
    }

    /// Set the current `scroll` state for this element.
    #[inline]
    pub(crate) fn set_scroll(&mut self, id: ElementId, scroll: VectorI2) {
        if let Some(state) = self.elements.get_mut(&id) {
            state.scroll = scroll;
        } else {
            self.elements.put(
                id,
                ElementState {
                    scroll,
                    ..Default::default()
                },
            );
        }
    }

    /// Returns the current `text_edit` state for this element.
    #[inline]
    pub(crate) fn text_edit<S>(&mut self, id: ElementId, initial_text: S) -> String
    where
        S: Into<String>,
    {
        if let Some(state) = self.elements.get_mut(&id) {
            mem::take(&mut state.text_edit)
        } else {
            initial_text.into()
        }
    }

    /// Updates the current `text_edit` state for this element.
    #[inline]
    pub(crate) fn set_text_edit(&mut self, id: ElementId, text_edit: String) {
        if let Some(state) = self.elements.get_mut(&id) {
            state.text_edit = text_edit;
        } else {
            self.elements.put(
                id,
                ElementState {
                    text_edit,
                    ..Default::default()
                },
            );
        }
    }

    /// Returns the currently selected tab state for this element.
    #[inline]
    pub(crate) fn current_tab(&mut self, id: ElementId) -> usize {
        self.elements
            .get(&id)
            .map(|s| s.current_tab)
            .unwrap_or_default()
    }

    /// Updates the currently selected tab state for this element.
    #[inline]
    pub(crate) fn set_current_tab(&mut self, id: ElementId, tab: usize) {
        if let Some(state) = self.elements.get_mut(&id) {
            state.current_tab = tab;
        } else {
            self.elements.put(
                id,
                ElementState {
                    current_tab: tab,
                    ..Default::default()
                },
            );
        }
    }

    /// Parses the current `text_edit` state for this element into a given type.
    #[inline]
    pub(crate) fn parse_text_edit<T>(&mut self, id: ElementId, default: T) -> PixResult<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Error + Sync + Send + 'static,
    {
        if let Some(state) = self.elements.pop(&id) {
            Ok(state.text_edit.parse().unwrap_or(default))
        } else {
            Ok(default)
        }
    }

    /// Returns whether the current element is expanded or not.
    #[inline]
    pub(crate) fn expanded(&mut self, id: ElementId) -> bool {
        if let Some(state) = self.elements.get_mut(&id) {
            state.expanded
        } else {
            false
        }
    }

    /// Set whether the current element is expanded or not.
    #[inline]
    pub(crate) fn set_expanded(&mut self, id: ElementId, expanded: bool) {
        if let Some(state) = self.elements.get_mut(&id) {
            state.expanded = expanded;
        } else {
            self.elements.put(
                id,
                ElementState {
                    expanded,
                    ..Default::default()
                },
            );
        }
    }

    /// Returns the width of the last rendered UI element, or 0 if there is no last rendered
    /// element.
    #[inline]
    pub(crate) fn last_width(&self) -> i32 {
        self.last_size.map(|s| s.width()).unwrap_or_default()
    }
}

impl PixState {
    /// Push a new seed to the UI ID stack. Helps in generating unique widget identifiers that have
    /// the same text label. Pushing a unique ID to the stack will seed the hash of the label.
    #[inline]
    pub fn push_id(&mut self, id: ElementId) {
        self.ui.id_stack.push(id);
    }

    /// Pop a seed from the UI ID stack.
    #[inline]
    pub fn pop_id(&mut self) {
        self.ui.id_stack.pop();
    }

    /// Disables any UI elements drawn after this is called.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { checkbox: bool };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     if s.button("Disable UI")? {
    ///         s.disable();
    ///     }
    ///     s.checkbox("Disabled checkbox", &mut self.checkbox)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn disable(&mut self) {
        self.ui.disabled = true;
    }

    /// Enables any UI elements drawn after this is called.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { checkbox: bool };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.disable();
    ///     if s.button("Enable UI")? {
    ///         s.no_disable();
    ///     }
    ///     s.checkbox("Enabled checkbox", &mut self.checkbox)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn no_disable(&mut self) {
        self.ui.disabled = false;
    }

    /// Returns the current UI rendering position.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     let mut pos = s.cursor_pos();
    ///     pos.offset_y(20);
    ///     s.set_cursor_pos(pos);
    ///     s.text("Some text, offset down by 20 pixels")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn cursor_pos(&self) -> PointI2 {
        self.ui.cursor()
    }

    /// Set the current UI rendering position.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.set_cursor_pos(s.center()?);
    ///     s.rect_mode(RectMode::Center);
    ///     s.text("Centered text")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn set_cursor_pos<P: Into<PointI2>>(&mut self, cursor: P) {
        self.ui.set_cursor(cursor.into());
    }

    /// Returns whether the last item drawn is hovered with the mouse.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.text("Hover me")?;
    ///     if s.hovered() {
    ///         s.tooltip("I'm a tooltip!");
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn hovered(&self) -> bool {
        if let Some(rect) = self.ui.last_size {
            !self.ui.disabled && rect.contains_point(self.mouse_pos())
        } else {
            false
        }
    }

    /// Returns whether the last item drawn was clicked with the left mouse button.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.text("Hover me")?;
    ///     if s.clicked() {
    ///         println!("I was clicked!");
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn clicked(&self) -> bool {
        if let Some(rect) = self.ui.last_size {
            !self.ui.disabled
                && self.mouse_clicked(Mouse::Left)
                && rect.contains_point(self.mouse_pos())
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
        let padx = style.frame_pad.x();
        let pady = style.item_pad.y();
        let offset_x = self.ui.offset_x;

        // Previous cursor ends at the right of this item
        self.ui.pcursor = point![pos.x() + rect.width(), pos.y()];

        // Move cursor to the next line with padding, choosing the maximum of the next line or the
        // previous y value to account for variable line heights when using `same_line`.
        let line_height = cmp::max(self.ui.line_height, rect.height());
        self.ui.cursor = point![padx + offset_x, pos.y() + line_height + pady];
        self.ui.pline_height = line_height;
        self.ui.line_height = 0;
        self.ui.last_size = Some(rect);
    }

    /// Get or create a UI texture to render to
    #[inline]
    pub(crate) fn get_or_create_texture<R>(
        &mut self,
        id: ElementId,
        src: R,
        dst: Rect<i32>,
    ) -> PixResult<TextureId>
    where
        R: Into<Option<Rect<i32>>>,
    {
        let font_id = self.theme.fonts.body.id();
        let font_size = self.theme.font_sizes.body;
        if let Some(texture) = self
            .ui
            .textures
            .iter_mut()
            .find(|t| t.element_id == id && t.font_id == font_id && t.font_size == font_size)
        {
            texture.visible = true;
            texture.dst = Some(dst);
            Ok(texture.id)
        } else {
            let texture_id =
                self.create_texture(dst.width() as u32, dst.height() as u32, PixelFormat::Rgba)?;
            self.ui.textures.push(Texture::new(
                texture_id,
                id,
                src.into(),
                Some(dst),
                font_id,
                font_size,
            ));
            if self.ui.textures.len() > 2 * ELEMENT_CACHE_SIZE {
                self.ui.textures.truncate(ELEMENT_CACHE_SIZE);
            }
            Ok(texture_id)
        }
    }
}

/// Internal tracked UI element state.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ElementState {
    scroll: VectorI2,
    text_edit: String,
    current_tab: usize,
    expanded: bool,
}
