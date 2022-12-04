//! Input field widget rendering methods.
//!
//! Provided [`PixState`] methods:
//!
//! - [`PixState::text_field`]
//! - [`PixState::advanced_text_field`]
//! - [`PixState::text_area`]
//! - [`PixState::advanced_text_area`]
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App { text_field: String, text_area: String};
//! # impl PixEngine for App {
//! fn on_update(&mut self, s: &mut PixState) -> Result<()> {
//!     s.text_field("Text Field", &mut self.text_field)?;
//!     s.advanced_text_field(
//!         "Filtered Text Field w/ hint",
//!         "placeholder",
//!         &mut self.text_field,
//!         Some(char::is_numeric),
//!     )?;
//!
//!     s.text_area("Text Area", 200, 100, &mut self.text_area)?;
//!     s.advanced_text_area(
//!         "Filtered Text Area w/ hint",
//!         "placeholder",
//!         200,
//!         100,
//!         &mut self.text_area,
//!         None,
//!         )?;
//!     Ok(())
//! }
//! # }
//! ```

use crate::{gui::MOD_CTRL, ops::clamp_size, prelude::*};

const TEXT_CURSOR: &str = "_";

impl PixState {
    /// Draw a text field to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { text_field: String, text_area: String};
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.text_field("Text Field", &mut self.text_field)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn text_field<L>(&mut self, label: L, value: &mut String) -> Result<bool>
    where
        L: AsRef<str>,
    {
        self.advanced_text_field(label, "", value, None)
    }

    /// Draw a text field with a placeholder hint to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { text_field: String, text_area: String};
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.advanced_text_field(
    ///         "Filtered Text Field w/ hint",
    ///         "placeholder",
    ///         &mut self.text_field,
    ///         Some(char::is_numeric),
    ///     )?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn advanced_text_field<L, H>(
        &mut self,
        label: L,
        hint: H,
        value: &mut String,
        filter: Option<fn(char) -> bool>,
    ) -> Result<bool>
    where
        L: AsRef<str>,
        H: AsRef<str>,
    {
        let label = label.as_ref();
        let hint = hint.as_ref();

        let s = self;
        let id = s.ui.get_id(&label);
        let label = s.ui.get_label(label);
        let pos = s.cursor_pos();
        let spacing = s.theme.spacing;
        let ipad = spacing.item_pad;

        // Calculate input rect
        let width =
            s.ui.next_width
                .take()
                .unwrap_or_else(|| s.ui_width().unwrap_or(100));
        let (label_width, label_height) = s.text_size(label)?;
        let [mut x, y] = pos.coords();
        if !label.is_empty() {
            x += label_width + ipad.x();
        }
        let input = rect![x, y, width, label_height + 2 * ipad.y()];

        // Check hover/active/keyboard focus
        let hovered = s.focused() && s.ui.try_hover(id, &input);
        let focused = s.focused() && s.ui.try_focus(id);
        let disabled = s.ui.disabled;

        s.push();
        s.ui.push_cursor();

        // Label
        if !label.is_empty() {
            s.set_cursor_pos([pos.x(), pos.y() + input.height() / 2 - label_height / 2]);
            s.text(label)?;
        }

        // Input
        s.rect_mode(RectMode::Corner);
        if hovered {
            s.frame_cursor(&Cursor::ibeam())?;
        }
        let [stroke, bg, fg] = s.widget_colors(id, ColorType::Background);
        s.stroke(stroke);
        s.fill(bg);
        s.rect(input)?;

        // Text
        let clip = input.shrink(ipad);
        let (text_width, text_height) = s.text_size(value)?;
        let (cursor_width, _) = s.text_size(TEXT_CURSOR)?;
        let width = text_width + cursor_width;
        let (mut x, y) = (clip.x(), input.center().y() - text_height / 2);
        if width > clip.width() {
            x -= width - clip.width();
        }

        s.wrap(None);
        s.set_cursor_pos([x, y]);
        s.clip(clip)?;
        s.stroke(None);
        s.fill(fg);
        if value.is_empty() {
            // FIXME: push and pop disabled state instead
            s.ui.push_cursor();
            s.disable(true);
            s.text(hint)?;
            if !disabled {
                s.disable(false);
            }
            s.ui.pop_cursor();
            if focused {
                s.text(TEXT_CURSOR)?;
            }
        } else if focused {
            s.text(format!("{}{}", value, TEXT_CURSOR))?;
        } else {
            s.text(&value)?;
        }

        s.clip(None)?;
        s.ui.pop_cursor();
        s.pop();

        // Process input
        let changed = focused && {
            if let Some(Key::Return | Key::Escape) = s.ui.key_entered() {
                s.ui.blur();
            }
            s.handle_text_events(value)?
        };
        if changed {
            value.retain(|c| !c.is_control());
            if let Some(filter) = filter {
                value.retain(filter);
            }
        }
        s.ui.handle_focus(id);
        s.advance_cursor([input.right() - pos.x(), input.height()]);

        Ok(changed)
    }

    /// Draw a text area field to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { text_field: String, text_area: String};
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.text_area("Text Area", 200, 100, &mut self.text_area)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn text_area<L>(
        &mut self,
        label: L,
        width: u32,
        height: u32,
        value: &mut String,
    ) -> Result<bool>
    where
        L: AsRef<str>,
    {
        self.advanced_text_area(label, "", width, height, value, None)
    }

    /// Draw a text area field with a placeholder hint to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { text_field: String, text_area: String};
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.advanced_text_area(
    ///         "Filtered Text Area w/ hint",
    ///         "placeholder",
    ///         200,
    ///         100,
    ///         &mut self.text_area,
    ///         Some(char::is_alphabetic),
    ///     )?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn advanced_text_area<L, H>(
        &mut self,
        label: L,
        hint: H,
        width: u32,
        height: u32,
        value: &mut String,
        filter: Option<fn(char) -> bool>,
    ) -> Result<bool>
    where
        L: AsRef<str>,
        H: AsRef<str>,
    {
        let label = label.as_ref();
        let hint = hint.as_ref();

        let s = self;
        let id = s.ui.get_id(&label);
        let label = s.ui.get_label(label);
        let pos = s.cursor_pos();
        let spacing = s.theme.spacing;
        let ipad = spacing.item_pad;

        // Calculate input rect
        let (label_width, label_height) = s.text_size(label)?;
        let [x, mut y] = pos.coords();
        if !label.is_empty() {
            y += label_height + 2 * ipad.y();
        }
        let input = rect![x, y, clamp_size(width), clamp_size(height)];

        // Check hover/active/keyboard focus
        let hovered = s.focused() && s.ui.try_hover(id, &input);
        let focused = s.focused() && s.ui.try_focus(id);
        let disabled = s.ui.disabled;

        s.push();
        s.ui.push_cursor();

        // Label
        if !label.is_empty() {
            s.set_cursor_pos([pos.x(), pos.y() + ipad.y()]);
            s.text(label)?;
        }

        // Input
        s.rect_mode(RectMode::Corner);
        if hovered {
            s.frame_cursor(&Cursor::ibeam())?;
        }
        let [stroke, bg, fg] = s.widget_colors(id, ColorType::Background);
        s.stroke(stroke);
        s.fill(bg);
        s.rect(input)?;

        // Text
        let clip = input.shrink(ipad);
        let scroll = s.ui.scroll(id);
        s.wrap(clip.width() as u32);
        let mut text_pos = input.top_left();
        text_pos.offset(ipad - scroll);

        s.set_cursor_pos(text_pos);
        s.clip(clip)?;
        s.stroke(None);
        s.fill(fg);
        let (_, text_height) = if value.is_empty() {
            // FIXME: push and pop disabled state instead
            s.ui.push_cursor();
            s.disable(true);
            let size = s.text(hint)?;
            if !disabled {
                s.disable(false);
            }
            s.ui.pop_cursor();
            if focused {
                s.text(TEXT_CURSOR)?;
            }
            size
        } else if focused {
            s.text(format!("{}{}", value, TEXT_CURSOR))?
        } else {
            s.text(&value)?
        };

        // Process input
        let mut text_height = clamp_size(text_height) + 2 * ipad.y();
        let changed = focused && {
            match s.ui.key_entered() {
                Some(Key::Return) => {
                    value.push('\n');
                    true
                }
                Some(Key::Escape) => {
                    s.ui.blur();
                    false
                }
                _ => s.handle_text_events(value)?,
            }
        };

        if changed {
            value.retain(|c| c == '\n' || !c.is_control());
            if let Some(filter) = filter {
                value.retain(filter);
            }
            let (_, height) = s.text_size(&format!("{}{}", value, TEXT_CURSOR))?;
            text_height = height + 2 * ipad.y();

            // Keep cursor within scroll region
            let mut scroll = s.ui.scroll(id);
            if text_height < input.height() {
                scroll.set_y(0);
            } else {
                scroll.set_y(text_height - input.height());
            }
            s.ui.set_scroll(id, scroll);
        }

        s.clip(None)?;
        s.ui.pop_cursor();
        s.pop();

        s.ui.handle_focus(id);
        // Scrollbars
        let rect = s.scroll(id, input, 0, text_height)?;
        s.advance_cursor([rect.width().max(label_width), rect.bottom() - pos.y()]);

        Ok(changed)
    }
}

impl PixState {
    /// Helper to handle text entry and text shortcuts.
    fn handle_text_events(&mut self, value: &mut String) -> Result<bool> {
        let s = self;
        let mut changed = false;
        if let Some(key) = s.ui.key_entered() {
            match key {
                Key::Backspace if !value.is_empty() => {
                    if s.keymod_down(MOD_CTRL) {
                        value.clear();
                    } else if s.keymod_down(KeyMod::ALT) {
                        // If last char is whitespace, remove it so we find the next previous
                        // word
                        if value.chars().last().map(char::is_whitespace) == Some(true) {
                            value.pop();
                        }
                        if let Some(idx) = value.rfind(char::is_whitespace) {
                            value.truncate(idx + 1);
                        } else {
                            value.clear();
                        }
                    } else {
                        value.pop();
                    }
                    changed = true;
                }
                Key::X if s.keymod_down(MOD_CTRL) => {
                    s.set_clipboard_text(&value)?;
                    value.clear();
                    changed = true;
                }
                Key::C if s.keymod_down(MOD_CTRL) => {
                    s.set_clipboard_text(&value)?;
                }
                Key::V if s.keymod_down(MOD_CTRL) => {
                    value.push_str(&s.clipboard_text());
                    changed = true;
                }
                _ => (),
            }
        }
        if let Some(text) = s.ui.keys.typed.take() {
            value.push_str(&text);
            changed = true;
        }
        Ok(changed)
    }
}
