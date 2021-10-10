//! Immediate-GUI functions related to rendering and interacting with radio buttons.

use super::get_hash;
use crate::{prelude::*, renderer::Rendering};

impl PixState {
    /// Draw a set of radio buttons to the current canvas.
    pub fn radio<R, S>(
        &mut self,
        rect: R,
        label: S,
        selected: &mut usize,
        index: usize,
    ) -> PixResult<bool>
    where
        R: Into<Rect<i32>>,
        S: AsRef<str>,
    {
        let rect = self.get_rect(rect);
        self._radio(rect, label.as_ref(), selected, index)
    }

    fn _radio(
        &mut self,
        rect: Rect<i32>,
        label: &str,
        selected: &mut usize,
        index: usize,
    ) -> PixResult<bool> {
        let s = self;
        let id = get_hash(&label);

        // Calculate radio rect
        let radius = 9;
        let (_, h) = s.size_of(label)?;
        let y = rect.center().y();
        let radio = circle![rect.x() + radius, y - h as i32 / 2 + radius, radius];

        // Check hover/active/keyboard focus
        let disabled = s.ui_state.disabled;
        if !disabled && radio.contains_point(s.mouse_pos()) {
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
            s.text([radio.right() + 2 * pad, y - h as i32 / 2], label)?;
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
        s.ellipse_mode(EllipseMode::Corner);
        s.fill(s.primary_color());
        s.circle(radio)?;
        if *selected == index {
            s.fill(s.highlight_color());
            s.circle(circle![radio.x(), radio.y(), radio.radius() - 3])?;
        }

        s.pop();

        // Process input
        s.ui_state.handle_input(id);
        if !disabled {
            let clicked = s.ui_state.was_clicked(id);
            if clicked {
                *selected = index;
            }
            Ok(clicked)
        } else {
            Ok(false)
        }
    }
}
