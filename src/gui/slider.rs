//! Immediate-GUI functions related to rendering and interacting with slider controls.

use crate::prelude::*;

use super::get_hash;

const SCROLL_SPEED: i32 = 2;

impl PixState {
    pub(crate) fn slider<R>(&mut self, rect: R, max: i32, value: &mut i32) -> PixResult<bool>
    where
        R: Into<Rect<i32>>,
    {
        let s = self;
        let rect = s.get_rect(rect);
        let id = get_hash(&rect);

        s.push();

        // Check hover/active/keyboard focus
        if rect.contains_point(s.mouse_pos()) {
            s.ui_state.hover(id);
        }
        s.ui_state.try_capture(id);

        // Render

        // Scroll region
        let focused = s.ui_state.is_focused(id);
        if focused {
            s.stroke(s.secondary_color());
        } else {
            s.stroke(s.muted_color());
        }
        s.fill(s.primary_color());
        s.rounded_rect(rect, 3.0)?;

        // Thumb slider
        s.no_stroke();
        let hovered = s.ui_state.is_hovered(id);
        let active = s.ui_state.is_active(id);
        if hovered || active || focused {
            s.frame_cursor(&Cursor::hand())?;
            s.fill(s.secondary_color());
        } else {
            s.fill(s.muted_color());
        }
        let thumb_y = ((rect.height() - 16) * *value) / max;
        s.rounded_rect([rect.x(), rect.y() + thumb_y, 16, 16], 3.0)?;

        s.pop();

        // Process keyboard input
        s.ui_state.handle_tab(id);
        if s.ui_state.is_focused(id) {
            if let Some(key) = s.ui_state.key_entered() {
                match key {
                    Key::Up => {
                        *value = value.saturating_sub(SCROLL_SPEED);
                        if *value < 0 {
                            *value = 0;
                        }
                        return Ok(true);
                    }
                    Key::Down => {
                        *value = value.saturating_add(SCROLL_SPEED);
                        if *value > max {
                            *value = max;
                        }
                        return Ok(true);
                    }
                    _ => (),
                }
            }
        }
        s.ui_state.set_last(id);

        // Process mouse input
        if active {
            let my = (s.mouse_pos().y() - rect.y()).clamp(0, rect.height());
            let new_value = (my * max) / rect.height();
            if new_value != *value {
                *value = new_value;
                return Ok(true);
            }
        }

        Ok(false)
    }
}
