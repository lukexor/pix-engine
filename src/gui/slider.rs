//! Immediate-GUI functions related to rendering and interacting with slider controls.

use crate::prelude::*;

use super::get_hash;

const SCROLL_SPEED: i32 = 2;

impl PixState {
    /// Draw a slider control that returns `true` when changed.
    pub fn slider<R>(&mut self, rect: R, max: i32, value: &mut i32) -> PixResult<bool>
    where
        R: Into<Rect<i32>>,
    {
        let rect = self.get_rect(rect);
        self._slider(rect, max, value)
    }

    fn _slider(&mut self, rect: Rect<i32>, max: i32, value: &mut i32) -> PixResult<bool> {
        if max <= 0 {
            return Err(PixError::Other("invalid `max` value".into()));
        }

        let mut changed = false;
        if *value < 0 {
            *value = 0;
            changed = true;
        } else if *value > max {
            *value = max;
            changed = true;
        }

        let s = self;
        let id = get_hash(&rect);

        s.push();

        // Check hover/active/keyboard focus
        if rect.contains_point(s.mouse_pos()) {
            s.ui_state.hover(id);
        }
        s.ui_state.try_capture(id);

        // Render
        let radius = 3;

        // Scroll region
        let focused = s.ui_state.is_focused(id);
        if focused {
            s.stroke(s.secondary_color());
        } else {
            s.stroke(s.muted_color());
        }
        s.fill(s.primary_color());
        s.rounded_rect(rect, radius)?;

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
        let thumb_w = 16;
        let h = rect.height() as f32;
        let thumb_h = ((h / (max as f32 + h)) * h) as i32;
        let thumb_y = ((rect.height() - thumb_h) * *value) / max;
        s.rounded_rect([rect.x(), rect.y() + thumb_y, thumb_w, thumb_h], radius)?;

        s.pop();

        // Process keyboard input
        if s.ui_state.is_focused(id) {
            if let Some(key) = s.ui_state.key_entered() {
                match key {
                    Key::Up => {
                        *value = value.saturating_sub(SCROLL_SPEED);
                        if *value < 0 {
                            *value = 0;
                        }
                        changed = true;
                    }
                    Key::Down => {
                        *value = value.saturating_add(SCROLL_SPEED);
                        if *value > max {
                            *value = max;
                        }
                        changed = true;
                    }
                    _ => (),
                }
            }
        }
        // Process mouse input
        if active {
            let my = (s.mouse_pos().y() - rect.y()).clamp(0, rect.height());
            let new_value = (my * max) / rect.height();
            if new_value != *value {
                *value = new_value;
                changed = true;
            }
        }
        s.ui_state.handle_input(id);

        Ok(changed)
    }
}
