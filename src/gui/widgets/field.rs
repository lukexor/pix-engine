//! Input field widget rendering methods.
//!
//! Provided [PixState] methods:
//!
//! - [PixState::text_field]
//! - [PixState::text_field_filtered]
//! - [PixState::text_field_hint]
//! - [PixState::text_area]
//! - [PixState::text_area_filtered]
//! - [PixState::text_area_hint]
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App { text_field: String, text_area: String};
//! # impl AppState for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     s.text_field("Text Field", &mut self.text_field)?;
//!     s.text_field_hint("Text Field w/ hint", "placeholder", &mut self.text_field)?;
//!     s.text_field_filtered(
//!         "Filtered Text Field",
//!         &mut self.text_field,
//!         char::is_numeric
//!     )?;
//!
//!     s.text_area("Text Area", 200, 100, &mut self.text_area)?;
//!     s.text_area_hint("Text Area w/ hint", "placeholder", 200, 100, &mut self.text_area)?;
//!     s.text_area_filtered(
//!         "Filtered Text Area",
//!         200,
//!         100,
//!         &mut self.text_area,
//!         char::is_numeric
//!     )?;
//!     Ok(())
//! }
//! # }
//! ```

use crate::{gui::MOD_CTRL, prelude::*};

const TEXT_CURSOR: &str = "_";

impl PixState {
    /// Draw a text field to the current canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { text_field: String, text_area: String};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.text_field("Text Field", &mut self.text_field)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn text_field<L>(&mut self, label: L, value: &mut String) -> PixResult<bool>
    where
        L: AsRef<str>,
    {
        self.text_field_hint(label, "", value)
    }

    /// Draw a text field that filters allowed values to the current canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { text_field: String, text_area: String};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.text_field_filtered(
    ///         "Filtered Text Field",
    ///         &mut self.text_field,
    ///         char::is_numeric
    ///     )?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn text_field_filtered<L, F>(
        &mut self,
        label: L,
        value: &mut String,
        filter: F,
    ) -> PixResult<bool>
    where
        L: AsRef<str>,
        F: FnMut(char) -> bool,
    {
        let changed = self.text_field_hint(label, "", value)?;
        if changed {
            value.retain(filter);
        }
        Ok(changed)
    }

    /// Draw a text field with a placeholder hint to the current canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { text_field: String, text_area: String};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.text_field_hint("Text Field w/ hint", "placeholder", &mut self.text_field)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn text_field_hint<L, H>(
        &mut self,
        label: L,
        hint: H,
        value: &mut String,
    ) -> PixResult<bool>
    where
        L: AsRef<str>,
        H: AsRef<str>,
    {
        let label = label.as_ref();
        let hint = hint.as_ref();

        let s = self;
        let id = s.ui.get_id(&label);
        let label = label.split('#').next().unwrap_or("");
        let pos = s.cursor_pos();
        let font_size = s.theme.font_sizes.body as i32;
        let style = s.theme.style;
        let fpad = style.frame_pad;
        let ipad = style.item_pad;

        // Calculate input rect
        let width =
            s.ui.next_width
                .take()
                .unwrap_or_else(|| s.width().unwrap_or(100) - 2 * fpad.x() as u32);
        let mut input = rect![pos, width as i32, font_size + 2 * ipad.y()];
        let (lwidth, lheight) = s.size_of(label)?;
        if !label.is_empty() {
            input.offset_x(lwidth as i32 + ipad.x());
        }

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, input);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;
        let active = s.ui.is_active(id);

        s.push();
        s.ui.push_cursor();

        // Render
        s.rect_mode(RectMode::Corner);

        // Label
        if !label.is_empty() {
            s.set_cursor_pos([pos.x(), pos.y() + input.height() / 2 - lheight as i32 / 2]);
            s.text(label)?;
        }

        // Input
        s.push();
        if focused || active {
            s.stroke(s.highlight_color());
        } else {
            s.stroke(s.muted_color());
        }
        if hovered {
            s.frame_cursor(Cursor::ibeam())?;
            s.fill(s.secondary_color());
        } else if disabled {
            s.fill(s.primary_color() / 2);
        } else {
            s.fill(s.primary_color());
        }
        s.same_line(None);
        s.rect(input)?;
        s.pop();

        // Text
        let (vw, vh) = s.size_of(&value)?;
        let mut x = input.x() + ipad.x();
        let y = input.center().y() - vh as i32 / 2;
        let mut width = vw as i32;
        if focused {
            let (cw, _) = s.size_of(TEXT_CURSOR)?;
            width += cw as i32;
        }
        if width > input.width() {
            x -= width - input.width();
        }

        s.no_wrap();
        s.set_cursor_pos([x, y]);
        s.clip(input)?;
        if value.is_empty() {
            s.disable();
            s.text(&hint)?;
            if !disabled {
                s.no_disable();
            }
        } else {
            s.text(&value)?;
        }

        if focused && s.elapsed() as usize >> 9 & 1 > 0 {
            s.set_cursor_pos([x + vw as i32, y]);
            s.text(TEXT_CURSOR)?;
        }

        s.no_clip()?;
        s.ui.pop_cursor();
        s.pop();

        // Process input
        let mut changed = false;
        if focused {
            changed = s.handle_text_events(value)?;
            if changed {
                value.retain(|c| !c.is_control());
            }
        }
        s.ui.handle_events(id);
        s.advance_cursor(rect![pos, input.right() - pos.x(), input.height()]);

        Ok(changed)
    }

    /// Draw a text area field to the current canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { text_field: String, text_area: String};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
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
    ) -> PixResult<bool>
    where
        L: AsRef<str>,
    {
        self.text_area_hint(label, "", width, height, value)
    }

    /// Draw a text area field that filters allowed values to the current canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { text_field: String, text_area: String};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.text_area_filtered(
    ///         "Filtered Text Area",
    ///         200,
    ///         100,
    ///         &mut self.text_area,
    ///         char::is_numeric
    ///     )?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn text_area_filtered<L, F>(
        &mut self,
        label: L,
        width: u32,
        height: u32,
        value: &mut String,
        filter: F,
    ) -> PixResult<bool>
    where
        L: AsRef<str>,
        F: FnMut(char) -> bool,
    {
        let changed = self.text_area_hint(label, "", width, height, value)?;
        if changed {
            value.retain(filter);
        }
        Ok(changed)
    }

    /// Draw a text area field with a placeholder hint to the current canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { text_field: String, text_area: String};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.text_area_hint(
    ///         "Text Area w/ hint",
    ///         "placeholder",
    ///         200,
    ///         100,
    ///         &mut self.text_area
    ///     )?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn text_area_hint<L, H>(
        &mut self,
        label: L,
        hint: H,
        width: u32,
        height: u32,
        value: &mut String,
    ) -> PixResult<bool>
    where
        L: AsRef<str>,
        H: AsRef<str>,
    {
        let label = label.as_ref();
        let hint = hint.as_ref();

        let s = self;
        let id = s.ui.get_id(&label);
        let label = label.split('#').next().unwrap_or("");
        let pos = s.cursor_pos();
        let style = s.theme.style;
        let ipad = style.item_pad;

        // Calculate input rect
        let mut input = rect![pos, width as i32, height as i32];
        if !label.is_empty() {
            let (_, lheight) = s.size_of(label)?;
            input.offset_y(lheight as i32 + ipad.y());
        }

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, input);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;
        let active = s.ui.is_active(id);

        s.push();

        // Render
        s.rect_mode(RectMode::Corner);

        // Label
        s.text(label)?;

        // Input
        s.push();
        if focused || active {
            s.stroke(s.highlight_color());
        } else {
            s.stroke(s.muted_color());
        }
        if hovered {
            s.frame_cursor(Cursor::ibeam())?;
            s.fill(s.secondary_color());
        } else if disabled {
            s.fill(s.primary_color() / 2);
        } else {
            s.fill(s.primary_color());
        }
        s.rect(input)?;
        s.pop();

        // Text
        let scroll = s.ui.scroll(id);
        s.wrap_width((input.width() - ipad.x()) as u32);
        let mut text_pos = input.top_left();
        text_pos.offset(ipad - scroll);
        s.set_cursor_pos(text_pos);
        s.clip(input)?;
        let blink_cursor = focused && s.elapsed() as usize >> 9 & 1 > 0;
        // TODO: total width here always maxes out at wrap_width when words can't wrap
        let (_, total_height) = if value.is_empty() {
            s.disable();
            let pos = s.cursor_pos();
            let size = s.text(&hint)?;
            if !disabled {
                s.no_disable();
            }
            if blink_cursor {
                s.set_cursor_pos(pos);
                s.text(TEXT_CURSOR)?;
            }
            size
        } else if blink_cursor {
            s.text(format!("{}{}", value, TEXT_CURSOR))?
        } else {
            s.text(format!("{}  ", value))?
        };

        s.no_clip()?;
        s.pop();

        // Process input
        let mut changed = false;
        if focused {
            changed = s.handle_text_events(value)?;
            if let Some(Key::Return) = s.ui.key_entered() {
                value.push('\n');
                changed = true;
            }
            if changed {
                value.retain(|c| c == '\n' || !c.is_control());
            }

            // Keep cursor within scroll region
            let mut scroll = s.ui.scroll(id);
            let text_cursor = total_height as i32;
            if text_cursor < input.height() {
                scroll.set_y(0);
                s.ui.set_scroll(id, scroll);
            } else if text_cursor > scroll.y() + input.height() {
                let (_, line_height) = s.size_of(TEXT_CURSOR)?;
                scroll.set_y(text_cursor - (input.height() - line_height as i32));
                s.ui.set_scroll(id, scroll);
            }
        }
        s.ui.handle_events(id);

        // Scrollbars
        let total_height = total_height as i32 + 2 * ipad.y();
        s.set_cursor_pos(pos);
        s.scroll(id, input, 0, total_height)?;
        // EXPL: To preseve label pos being restored for `same_line`
        s.same_line(None);
        s.advance_cursor(rect![pos, 0, input.bottom() - pos.y()]);

        Ok(changed)
    }
}

impl PixState {
    fn handle_text_events(&mut self, value: &mut String) -> PixResult<bool> {
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
                        if let Some(true) = value.chars().last().map(char::is_whitespace) {
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
                    s.set_clipboard_text(value)?;
                    value.clear();
                    changed = true;
                }
                Key::C if s.keymod_down(MOD_CTRL) => {
                    s.set_clipboard_text(value)?;
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
