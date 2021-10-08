//! Immediate-GUI functions related to rendering and interacting with check boxes.

use super::get_hash;
use crate::{prelude::*, renderer::Rendering};

impl PixState {
    /// Draw a select list to the current canvas with a scrollable region.
    pub fn checkbox<R, S>(&mut self, rect: R, label: S, checked: &mut bool) -> PixResult<bool>
    where
        R: Into<Rect<i32>>,
        S: AsRef<str>,
    {
        let rect = self.get_rect(rect);
        self._checkbox(rect, label.as_ref(), checked)
    }

    fn _checkbox(&mut self, rect: Rect<i32>, label: &str, checked: &mut bool) -> PixResult<bool> {
        let s = self;
        let id = get_hash(&rect);

        // Calculate checkbox rect
        let (_, h) = s.size_of(label)?;
        let y = rect.center().y();
        let checkbox = square![rect.x(), y - h as i32 / 2, 16];

        // Check hover/active/keyboard focus
        let disabled = s.ui_state.disabled;
        if !disabled && checkbox.contains_point(s.mouse_pos()) {
            s.ui_state.hover(id);
        }
        s.ui_state.try_capture(id);
        let focused = !disabled && s.ui_state.is_focused(id);
        let hovered = s.ui_state.is_hovered(id);
        let active = s.ui_state.is_active(id);

        s.push();

        // Render
        let pad = s.theme.padding;
        s.rect_mode(RectMode::Corner);
        s.renderer.font_family(&s.theme.fonts.body)?;

        // Label
        if !label.is_empty() {
            s.fill(s.text_color());
            s.text([rect.x() + checkbox.width() + pad, y - h as i32 / 2], label)?;
        }

        // Checkbox
        if focused || active {
            s.stroke(s.secondary_color());
        } else {
            s.stroke(s.muted_color());
        }
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
        }
        s.fill(s.primary_color());
        s.rect(checkbox)?;
        if *checked {
            s.fill(s.accent_color());
            s.rect(checkbox)?;
        }

        s.pop();

        // Process input
        s.ui_state.handle_input(id);
        if !disabled {
            let clicked = s.ui_state.was_clicked(id);
            if clicked {
                *checked = !(*checked);
            }
            Ok(clicked)
        } else {
            Ok(false)
        }
    }
}
