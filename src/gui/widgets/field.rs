//! Input field UI widgets.

use crate::{gui::MOD_CTRL, prelude::*};

const TEXT_CURSOR: &str = "_";

impl PixState {
    /// Draw a text field to the current canvas.
    pub fn text_field<L>(&mut self, label: L, value: &mut String) -> PixResult<bool>
    where
        L: AsRef<str>,
    {
        self.text_field_hint(label, "", value)
    }

    /// Draw a text field that filters allowed values to the current canvas.
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
        value.retain(filter);
        Ok(changed)
    }

    /// Draw a text field with a placeholder hint to the current canvas.
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
        let mut changed = false;

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
        if focused {
            if let Some(key) = s.ui.key_entered() {
                match key {
                    Key::Backspace if !value.is_empty() => {
                        value.pop();
                        changed = true;
                    }
                    Key::C if s.keymod_down(MOD_CTRL) => {
                        s.set_clipboard_text(value)?;
                    }
                    Key::V if s.keymod_down(MOD_CTRL) => {
                        *value += &s.clipboard_text().replace("\n", "");
                        changed = true;
                    }
                    _ => (),
                }
            }
            if let Some(text) = s.ui.keys.typed.take() {
                value.push_str(&text.replace("\n", ""));
                changed = true;
            }
        }
        s.ui.handle_input(id);
        s.advance_cursor(rect![pos, input.right() - pos.x(), input.height()]);

        Ok(changed)
    }

    /// Draw a text area field to the current canvas.
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
        value.retain(filter);
        Ok(changed)
    }

    /// Draw a text area field with a placeholder hint to the current canvas.
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
        let mut changed = false;

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
        s.wrap_width(input.width() - 2 * ipad.x());
        s.set_cursor_pos(input.top_left() + ipad - scroll);
        s.clip(input)?;
        // TODO: width here always maxes out at wrap_width when words can't wrap
        let blink_cursor = focused && s.elapsed() as usize >> 9 & 1 > 0;
        let (total_width, total_height) = if value.is_empty() {
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
            s.text(&value)?
        };

        s.no_clip()?;
        s.pop();

        // Process input
        if focused {
            if let Some(key) = s.ui.key_entered() {
                match key {
                    Key::Return => value.push('\n'),
                    Key::Backspace if !value.is_empty() => {
                        value.pop();
                        changed = true;
                    }
                    Key::C if s.keymod_down(MOD_CTRL) => {
                        s.set_clipboard_text(value)?;
                    }
                    Key::V if s.keymod_down(MOD_CTRL) => {
                        *value += &s.clipboard_text();
                        changed = true;
                    }
                    _ => (),
                }
            }
            if let Some(text) = s.ui.keys.typed.take() {
                value.push_str(&text);
                changed = true;
            }
        }
        s.ui.handle_input(id);

        // Scrollbars
        let total_width = total_width as i32 + 2 * ipad.x();
        let total_height = total_height as i32 + 2 * ipad.y();
        s.set_cursor_pos(pos);
        s.scroll(id, input, total_width, total_height)?;
        // EXPL: To preseve label pos being restored for `same_line`
        s.same_line(None);
        s.advance_cursor(rect![s.cursor_pos(), 0, input.bottom() - pos.y()]);

        Ok(changed)
    }
}
