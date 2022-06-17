//! UI scrollbar rendering functions.

use super::state::ElementId;
use crate::{ops::clamp_size, prelude::*};

pub(crate) const THUMB_MIN: i32 = 10;
pub(crate) const SCROLL_SPEED: i32 = 3;

/// Scroll direction.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ScrollDirection {
    /// Horizontal.
    Horizontal,
    /// Vertical.
    Vertical,
}

impl PixState {
    /// Draw a scrollable region to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    pub fn scroll_area<S, F>(&mut self, label: S, width: u32, height: u32, f: F) -> PixResult<()>
    where
        S: AsRef<str>,
        F: FnOnce(&mut PixState) -> PixResult<()>,
    {
        let label = label.as_ref();

        let s = self;
        let id = s.ui.get_id(&label);
        let label = s.ui.get_label(label);
        let pos = s.cursor_pos();
        let spacing = s.theme.spacing;
        let colors = s.theme.colors;
        let fpad = spacing.frame_pad;
        let ipad = spacing.item_pad;

        // Calculate rect
        let [x, mut y] = pos.coords();
        let (label_width, label_height) = s.text_size(label)?;
        if !label.is_empty() {
            y += label_height + ipad.y();
        }
        let scroll_area = rect![x, y, clamp_size(width), clamp_size(height)];

        // Check hover/active/keyboard focus
        s.ui.try_hover(id, &scroll_area);
        s.ui.try_focus(id);

        s.push();
        s.ui.push_cursor();

        // Label
        if !label.is_empty() {
            s.text(label)?;
        }

        // Scroll area
        s.rect_mode(RectMode::Corner);
        let [stroke, _, fg] = s.widget_colors(id, ColorType::Background);
        let scroll = s.ui.scroll(id);
        let texture_id = s.get_or_create_texture(id, None, scroll_area)?;
        s.ui.offset_mouse(scroll_area.top_left());
        s.ui.set_column_offset(-scroll.x());
        let mut max_cursor_pos = s.cursor_pos();

        let scroll_width = scroll_area.width();
        let scroll_height = scroll_area.height();
        let right = scroll_area.width() - fpad.x();
        let bottom = scroll_area.height() - fpad.y();
        s.with_texture(texture_id, |s: &mut PixState| {
            s.background(colors.background);

            s.set_cursor_pos(s.cursor_pos() - scroll);
            s.stroke(None);
            s.fill(fg);
            f(s)?;
            max_cursor_pos = s.cursor_pos() + scroll;

            // Since clip doesn't work texture targets, we fake it
            s.fill(colors.background);
            s.rect([0, 0, scroll_width, fpad.y()])?; // Top
            s.rect([0, 0, fpad.x(), scroll_height])?; // Left
            s.rect([right, 0, fpad.x(), scroll_height])?; // Right
            s.rect([0, bottom, scroll_width, fpad.y()])?; // Bottom

            s.stroke(stroke);
            s.fill(None);
            s.rect([0, 0, scroll_width, scroll_height])?;
            Ok(())
        })?;
        s.ui.reset_column_offset();
        s.ui.clear_mouse_offset();

        s.ui.pop_cursor();
        s.pop();

        s.ui.handle_events(id);

        // Scrollbars
        let total_width = max_cursor_pos.x() + s.ui.last_width() + fpad.x();
        let total_height = max_cursor_pos.y() + fpad.y();
        let rect = s.scroll(id, scroll_area, total_width, total_height)?;
        s.advance_cursor([rect.width().max(label_width), rect.bottom() - pos.y()]);

        Ok(())
    }
}

impl PixState {
    /// Handles mouse wheel scroll for `hovered` elements.
    pub(crate) fn scroll(
        &mut self,
        id: ElementId,
        rect: Rect<i32>,
        width: i32,
        height: i32,
    ) -> PixResult<Rect<i32>> {
        let s = self;
        let scroll_size = s.theme.spacing.scroll_size;

        let scroll = s.ui.scroll(id);
        let xmax = width - rect.width();
        let ymax = height - rect.height();
        let mut new_scroll = scroll;

        // Vertical scroll
        if ymax > 0 {
            if s.ui.is_hovered(id) {
                new_scroll.set_y((scroll.y() + SCROLL_SPEED * -s.ui.mouse.yrel).clamp(0, ymax));
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
            s.push_id(1);
            let scrolled = s.scrollbar(
                id,
                rect![rect.right(), rect.top(), scroll_size, rect.height()],
                ymax,
                &mut scroll_y,
                ScrollDirection::Vertical,
            )?;
            s.pop_id();
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
            s.push_id(2);
            let scrolled = s.scrollbar(
                id,
                rect![rect.left(), rect.bottom(), rect.width(), scroll_size],
                xmax,
                &mut scroll_x,
                ScrollDirection::Horizontal,
            )?;
            s.pop_id();
            if scrolled {
                new_scroll.set_x(scroll_x);
            }
        }

        if new_scroll != scroll {
            s.ui.set_scroll(id, new_scroll);
        }

        Ok(rect.offset_size([scroll_size, scroll_size]))
    }

    /// Helper to render either a vertical or a horizontal scroll bar.
    fn scrollbar(
        &mut self,
        id: ElementId,
        rect: Rect<i32>,
        max: i32,
        value: &mut i32,
        dir: ScrollDirection,
    ) -> PixResult<bool> {
        use ScrollDirection::{Horizontal, Vertical};

        let s = self;
        let id = s.ui.get_id(&id);
        let colors = s.theme.colors;

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, &rect);
        let focused = s.ui.try_focus(id);
        let active = s.ui.is_active(id);

        s.push();

        // Clamp value
        *value = (*value).clamp(0, max);

        // Scroll region
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
        }

        let [stroke, bg, _] = s.widget_colors(id, ColorType::Secondary);
        if active || focused {
            s.stroke(stroke);
        } else {
            s.stroke(None);
        }
        s.fill(colors.on_secondary);
        s.rect(rect)?;

        // Scroll thumb
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
        s.fill(bg);
        match dir {
            Horizontal => {
                let thumb_x = ((rect.width() - thumb_w) * *value) / max;
                s.rect([rect.x() + thumb_x, rect.y(), thumb_w, thumb_h])?;
            }
            Vertical => {
                let thumb_y = ((rect.height() - thumb_h) * *value) / max;
                s.rect([rect.x(), rect.y() + thumb_y, thumb_w, thumb_h])?;
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
                Vertical => -s.ui.mouse.yrel,
            };
            new_value += SCROLL_SPEED * offset;
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

        if new_value == *value {
            Ok(false)
        } else {
            *value = new_value.clamp(0, max);
            Ok(true)
        }
    }
}
