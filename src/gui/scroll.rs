//! UI scrollbar rendering functions.

use super::{state::ElementId, Direction};
use crate::prelude::*;

pub(crate) const THUMB_MIN: i32 = 10;
pub(crate) const SCROLL_SIZE: i32 = 12;
pub(crate) const SCROLL_SPEED: i32 = 3;

impl PixState {
    /// Draw a scrollable region to the current canvas.
    pub fn scroll_area<S, F>(&mut self, label: S, width: u32, height: u32, f: F) -> PixResult<()>
    where
        S: AsRef<str>,
        F: FnOnce(&mut PixState) -> PixResult<()>,
    {
        let label = label.as_ref();

        let s = self;
        let id = s.ui.get_id(&label);
        let pos = s.cursor_pos();
        let style = s.theme.style;
        let fpad = style.frame_pad;

        // Calculate rect
        let scroll_area = rect![pos, width as i32, height as i32];

        // Check hover/active/keyboard focus
        let _hovered = s.ui.try_hover(id, scroll_area);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;
        let active = s.ui.is_active(id);

        s.ui.reset_line_height();

        // Scroll area
        let scroll = s.ui.scroll(id);
        let texture_id = s.get_or_create_texture(id, None, scroll_area)?;
        s.ui.set_mouse_offset(scroll_area.top_left());
        s.ui.set_cursor_offset_x(-scroll.x());
        let mut max_cursor_pos = s.cursor_pos();

        let w = scroll_area.width();
        let h = scroll_area.height();
        let right = scroll_area.width() - fpad.x();
        let bottom = scroll_area.height() - fpad.y();
        s.with_texture(texture_id, |s: &mut PixState| {
            s.push();
            if disabled {
                s.background(s.primary_color() / 2)?;
            } else {
                s.background(s.primary_color())?;
            }
            s.rect_mode(RectMode::Corner);
            if focused || active {
                s.stroke(s.highlight_color());
            } else {
                s.stroke(s.muted_color());
            }
            s.no_fill();
            s.rect([0, 0, scroll_area.width(), scroll_area.height()])?;
            s.pop();

            s.set_cursor_pos(s.cursor_pos() - scroll);
            f(s)?;
            max_cursor_pos = s.cursor_pos() + scroll;

            // Since clip doesn't work texture targets, we fake it
            if disabled {
                s.fill(s.primary_color() / 2);
            } else {
                s.fill(s.primary_color());
            }
            s.rect([0, 0, fpad.x() - 1, h])?;
            s.rect([0, 0, w, fpad.y()])?;
            s.rect([right, 0, fpad.x(), h])?;
            s.rect([0, bottom, w, fpad.y()])?;
            Ok(())
        })?;
        s.ui.clear_cursor_offset();
        s.ui.clear_mouse_offset();

        s.ui.handle_events(id);

        // Scrollbars
        let total_width = max_cursor_pos.x() + s.ui.last_width() + fpad.x();
        let total_height = max_cursor_pos.y();
        let rect = s.scroll(id, scroll_area, total_width, total_height)?;
        s.advance_cursor(rect![pos, rect.width(), rect.height()]);

        Ok(())
    }
}

impl PixState {
    /// Handles mouse wheel scroll for `hovered` elements.
    pub(crate) fn scroll(
        &mut self,
        id: ElementId,
        mut rect: Rect<i32>,
        width: i32,
        height: i32,
    ) -> PixResult<Rect<i32>> {
        let s = self;

        let scroll = s.ui.scroll(id);
        let xmax = width - rect.width();
        let ymax = height - rect.height();
        let mut new_scroll = scroll;

        // Vertical scroll
        if ymax > 0 {
            if s.ui.is_hovered(id) {
                new_scroll.set_y((scroll.y() + SCROLL_SPEED * s.ui.mouse.yrel).clamp(0, ymax));
            }

            if s.ui.is_focused(id) {
                if let Some(key) = s.ui.key_entered() {
                    match key {
                        Key::Up => {
                            new_scroll.set_y((scroll.y() - SCROLL_SPEED).clamp(0, ymax));
                        }
                        Key::Down => {
                            new_scroll.set_y((scroll.y() + SCROLL_SPEED).clamp(0, ymax));
                        }
                        _ => (),
                    };
                }
            }

            let mut scroll_y = new_scroll.y();
            let scrolled = s.scrollbar(
                rect![rect.right(), rect.top(), SCROLL_SIZE, rect.height()],
                ymax,
                &mut scroll_y,
                Direction::Vertical,
            )?;
            if scrolled {
                new_scroll.set_y(scroll_y);
            }
        }

        // Horizontal scroll
        if xmax > 0 {
            if s.ui.is_hovered(id) {
                new_scroll.set_x((scroll.x() + SCROLL_SPEED * s.ui.mouse.xrel).clamp(0, xmax));
            }

            if s.ui.is_focused(id) {
                if let Some(key) = s.ui.key_entered() {
                    match key {
                        Key::Left => {
                            new_scroll.set_x((scroll.x() - SCROLL_SPEED).clamp(0, xmax));
                        }
                        Key::Right => {
                            new_scroll.set_x((scroll.x() + SCROLL_SPEED).clamp(0, xmax));
                        }
                        _ => (),
                    };
                }
            }

            let mut scroll_x = new_scroll.x();
            let scrolled = s.scrollbar(
                rect![rect.left(), rect.bottom(), rect.width(), SCROLL_SIZE],
                xmax,
                &mut scroll_x,
                Direction::Horizontal,
            )?;
            if scrolled {
                new_scroll.set_x(scroll_x);
            }
        }

        if new_scroll != scroll {
            s.ui.set_scroll(id, new_scroll);
        }

        rect.offset_width(SCROLL_SIZE);
        rect.offset_height(SCROLL_SIZE);
        Ok(rect)
    }

    fn scrollbar(
        &mut self,
        rect: Rect<i32>,
        max: i32,
        value: &mut i32,
        dir: Direction,
    ) -> PixResult<bool> {
        use Direction::*;

        let s = self;
        let id = s.ui.get_id(&rect);

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, rect);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;
        let active = s.ui.is_active(id);

        s.push();

        // Clamp value
        *value = (*value).clamp(0, max);

        // Scroll region
        s.no_stroke();
        s.fill(s.background_color());
        s.rect(rect)?;

        // Scroll thumb
        if hovered {
            s.frame_cursor(Cursor::hand())?;
        }
        if hovered || active || focused {
            s.fill(s.highlight_color());
        } else if disabled {
            s.fill(s.muted_color() / 2);
        } else {
            s.fill(s.muted_color());
        }
        let thumb_w = match dir {
            Horizontal => {
                let w = rect.width() as f32;
                let w = ((w / (max as f32 + w)) * w) as i32;
                w.max(THUMB_MIN).min(w)
            }
            Vertical => rect.width(),
        };
        let thumb_h = match dir {
            Horizontal => rect.height(),
            Vertical => {
                let h = rect.height() as f32;
                let h = ((h / (max as f32 + h)) * h) as i32;
                h.max(THUMB_MIN).min(h)
            }
        };
        match dir {
            Horizontal => {
                let thumb_x = ((rect.width() - thumb_w) * *value) / max;
                s.rect([rect.x() + thumb_x, rect.y(), thumb_w, thumb_h])?
            }
            Vertical => {
                let thumb_y = ((rect.height() - thumb_h) * *value) / max;
                s.rect([rect.x(), rect.y() + thumb_y, thumb_w, thumb_h])?
            }
        }

        s.pop();

        // Process keyboard input
        let mut new_value = *value;
        if focused {
            if let Some(key) = s.ui.key_entered() {
                match (key, dir) {
                    (Key::Up, Vertical) | (Key::Left, Horizontal) => {
                        new_value = value.saturating_sub(SCROLL_SPEED).max(0);
                    }
                    (Key::Down, Vertical) | (Key::Right, Horizontal) => {
                        new_value = value.saturating_add(SCROLL_SPEED).min(max);
                    }
                    _ => (),
                }
            }
        }

        // Process mouse wheel
        if hovered {
            let offset = match dir {
                Horizontal => s.ui.mouse.xrel,
                Vertical => s.ui.mouse.yrel,
            };
            new_value -= SCROLL_SPEED * offset;
        }
        // Process mouse input
        if active {
            new_value = match dir {
                Horizontal => {
                    let mx = (s.mouse_pos().x() - rect.x()).clamp(0, rect.width());
                    (mx * max) / rect.width()
                }
                Vertical => {
                    let my = (s.mouse_pos().y() - rect.y()).clamp(0, rect.height());
                    (my * max) / rect.height()
                }
            };
        }
        s.ui.handle_events(id);

        if new_value != *value {
            *value = new_value.clamp(0, max);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
