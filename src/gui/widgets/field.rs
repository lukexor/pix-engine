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
//! # impl AppState for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
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

const TEXT_CURSOR: &str = "|";

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
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
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
    ) -> PixResult<bool>
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
        let font_size = clamp_size(s.theme.sizes.body);
        let spacing = s.theme.spacing;
        let colors = s.theme.colors;
        let fpad = spacing.frame_pad;
        let ipad = spacing.item_pad;

        // Calculate input rect
        let next_width =
            s.ui.next_width
                .take()
                .unwrap_or_else(|| s.width().unwrap_or(100) - 2 * fpad.x() as u32);
        let mut input = rect![pos, clamp_size(next_width), font_size + 2 * ipad.y()];
        let (label_width, label_height) = s.size_of(label)?;
        if !label.is_empty() {
            input.offset_x(clamp_size(label_width) + ipad.x());
        }

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, &input);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;

        s.push();
        s.ui.push_cursor();

        // Label
        s.rect_mode(RectMode::Corner);
        if hovered {
            s.frame_cursor(&Cursor::ibeam())?;
        }

        if !label.is_empty() {
            s.no_stroke();
            s.fill(colors.on_background());
            s.set_cursor_pos([
                pos.x(),
                pos.y() + input.height() / 2 - clamp_size(label_height) / 2,
            ]);
            s.text(label)?;
        }

        // Input
        let [stroke, bg, fg] = s.widget_colors(id, ColorType::Background);
        s.stroke(stroke);
        s.fill(bg);
        s.rect(input)?;

        // Text
        let (vw, vh) = s.size_of(&value)?;
        let mut x = input.x() + ipad.x();
        let y = input.center().y() - clamp_size(vh) / 2;
        let mut width = clamp_size(vw);
        if focused {
            let (cw, _) = s.size_of(TEXT_CURSOR)?;
            width += clamp_size(cw);
        }
        if width > input.width() {
            x -= width - input.width();
        }

        s.no_wrap();
        s.set_cursor_pos([x, y]);
        s.clip(input)?;
        s.no_stroke();
        s.fill(fg);
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
            s.set_cursor_pos([x + clamp_size(vw), y]);
            s.text(TEXT_CURSOR)?;
        }

        s.no_clip()?;
        s.ui.pop_cursor();
        s.pop();

        // Process input
        let changed = focused && s.handle_text_events(value)?;
        if changed {
            value.retain(|c| !c.is_control());
            if let Some(filter) = filter {
                value.retain(filter);
            }
        }
        s.ui.handle_events(id);
        s.advance_cursor(rect![pos, input.right() - pos.x(), input.height()]);

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
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
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
    ) -> PixResult<bool>
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
        let colors = s.theme.colors;
        let ipad = spacing.item_pad;

        // Calculate input rect
        let mut input = rect![pos, clamp_size(width), clamp_size(height)];
        let (label_width, label_height) = s.size_of(label)?;
        if !label.is_empty() {
            input.offset_y(clamp_size(label_height) + ipad.y());
        }

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, &input);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;

        s.push();
        s.ui.push_cursor();

        // Label
        s.rect_mode(RectMode::Corner);
        if hovered {
            s.frame_cursor(&Cursor::ibeam())?;
        }
        s.no_stroke();
        s.fill(colors.on_background());
        s.text(label)?;

        // Input
        let [stroke, bg, fg] = s.widget_colors(id, ColorType::Background);
        s.stroke(stroke);
        s.fill(bg);
        s.rect(input)?;

        // Text
        let scroll = s.ui.scroll(id);
        s.wrap((input.width() - ipad.x()) as u32);
        let mut text_pos = input.top_left();
        text_pos.offset(ipad - scroll);
        s.set_cursor_pos(text_pos);
        s.clip(input)?;
        s.no_stroke();
        s.fill(fg);
        let blink_cursor = focused && s.elapsed() as usize >> 9 & 1 > 0;
        // TODO: total width here always maxes out at wrap_width when words can't wrap
        let (_, text_height) = if value.is_empty() {
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
        let mut text_height = clamp_size(text_height);

        s.no_clip()?;
        s.ui.pop_cursor();
        s.pop();

        // Process input
        let mut changed = focused && s.handle_text_events(value)?;
        if focused {
            if let Some(Key::Return) = s.ui.key_entered() {
                value.push('\n');
                changed = true;
            }
        }
        if changed {
            value.retain(|c| c == '\n' || !c.is_control());
            if let Some(filter) = filter {
                if changed {
                    value.retain(filter);
                }
            }

            // Keep cursor within scroll region
            let mut scroll = s.ui.scroll(id);
            let (_, vh) = s.size_of(&value)?;
            let (_, ch) = s.size_of(TEXT_CURSOR)?;
            text_height = clamp_size(vh);
            // EXPL: wrapping chops off the trailing newline, so make sure to adjust height
            if value.ends_with('\n') {
                text_height += clamp_size(ch);
            }
            if text_height < input.height() {
                scroll.set_y(0);
            } else {
                scroll.set_y(text_height + 2 * ipad.y() - input.height());
            }
            s.ui.set_scroll(id, scroll);
        }

        s.ui.handle_events(id);
        // Scrollbars
        let rect = s.scroll(id, input, 0, text_height + 2 * ipad.y())?;
        s.advance_cursor(rect![
            pos,
            rect.width().max(clamp_size(label_width)),
            rect.bottom() - pos.y()
        ]);

        Ok(changed)
    }
}

impl PixState {
    /// Helper to handle text entry and text shortcuts.
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
