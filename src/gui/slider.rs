//! Immediate-GUI functions related to rendering and interacting with slider controls.

use super::get_hash;
use crate::prelude::*;
use std::cmp;

const SCROLL_SPEED: i32 = 2;

/// Slider direction.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Horizontal.
    Horizontal,
    /// Vertical.
    Vertical,
}

impl PixState {
    /// Draw a slider control that returns `true` when changed.
    pub fn slider<R>(
        &mut self,
        rect: R,
        label: &str,
        max: i32,
        value: &mut i32,
        dir: Direction,
    ) -> PixResult<bool>
    where
        R: Into<Rect<i32>>,
    {
        let rect = self.get_rect(rect);
        self._slider(rect, label, max, value, dir)
    }

    fn _slider(
        &mut self,
        rect: Rect<i32>,
        label: &str,
        max: i32,
        value: &mut i32,
        dir: Direction,
    ) -> PixResult<bool> {
        if max <= 0 {
            return Err(PixError::Other("invalid `max` value".into()));
        }

        use Direction::*;

        let s = self;
        let mut id = get_hash(&label);
        match dir {
            Horizontal => id += rect.x() as u64,
            Vertical => id += rect.y() as u64,
        }

        // Check hover/active/keyboard focus
        let disabled = s.ui_state.disabled;
        if !disabled && rect.contains_point(s.mouse_pos()) {
            s.ui_state.hover(id);
        }
        s.ui_state.try_capture(id);
        let focused = !disabled && s.ui_state.is_focused(id);
        let hovered = s.ui_state.is_hovered(id);
        let active = !disabled && s.ui_state.is_active(id);

        s.push();
        let mut changed = false;

        // Clamp value
        *value = cmp::max(0, cmp::min(max, *value));

        // Render
        let radius = 6;

        // Scroll region
        if focused {
            s.stroke(s.secondary_color());
        } else {
            s.stroke(s.muted_color());
        }
        s.fill(s.background_color());
        s.rect(rect)?;

        // Thumb slider
        s.no_stroke();
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
        }
        if hovered || active || focused {
            s.fill(s.highlight_color());
        } else {
            s.fill(s.muted_color());
        }
        let thumb_w = match dir {
            Horizontal => {
                let w = rect.width() as f32;
                ((w / (max as f32 + w)) * w) as i32
            }
            Vertical => 16,
        };
        let thumb_h = match dir {
            Horizontal => 16,
            Vertical => {
                let h = rect.height() as f32;
                ((h / (max as f32 + h)) * h) as i32
            }
        };
        match dir {
            Horizontal => {
                let thumb_x = ((rect.width() - thumb_w) * *value) / max;
                s.rounded_rect([rect.x() + thumb_x, rect.y(), thumb_w, thumb_h], radius)?
            }
            Vertical => {
                let thumb_y = ((rect.height() - thumb_h) * *value) / max;
                s.rounded_rect([rect.x(), rect.y() + thumb_y, thumb_w, thumb_h], radius)?
            }
        }

        s.pop();

        // Process keyboard input
        if focused {
            if let Some(key) = s.ui_state.key_entered() {
                match key {
                    Key::Up if dir == Vertical => {
                        *value = value.saturating_sub(SCROLL_SPEED);
                        if *value < 0 {
                            *value = 0;
                        }
                        changed = true;
                    }
                    Key::Down if dir == Vertical => {
                        *value = value.saturating_add(SCROLL_SPEED);
                        if *value > max {
                            *value = max;
                        }
                        changed = true;
                    }
                    Key::Left if dir == Horizontal => {
                        *value = value.saturating_sub(SCROLL_SPEED);
                        if *value < 0 {
                            *value = 0;
                        }
                        changed = true;
                    }
                    Key::Right if dir == Horizontal => {
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
            let new_value = match dir {
                Vertical => {
                    let my = (s.mouse_pos().y() - rect.y()).clamp(0, rect.height());
                    (my * max) / rect.height()
                }
                Horizontal => {
                    let mx = (s.mouse_pos().x() - rect.x()).clamp(0, rect.width());
                    (mx * max) / rect.width()
                }
            };
            if new_value != *value {
                *value = new_value;
                changed = true;
            }
        }
        s.ui_state.handle_input(id);

        Ok(changed)
    }
}
