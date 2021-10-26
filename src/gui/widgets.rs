//! UI widget rendering functions.

use super::{state::ElementId, Direction};
use crate::prelude::*;
use std::cmp;

mod field;
mod select;
mod text;
mod tooltip;

const SCROLL_SIZE: i32 = 12;
const SCROLL_SPEED: i32 = 2;
const CHECKBOX_SIZE: i32 = 16;
const RADIO_SIZE: i32 = 8;

impl PixState {
    /// Draw a button to the current canvas that returns `true` when clicked.
    pub fn button<L>(&mut self, label: L) -> PixResult<bool>
    where
        L: AsRef<str>,
    {
        let label = label.as_ref();

        let s = self;
        let id = s.ui.get_id(&label);
        let label = label.split('#').next().unwrap_or("");
        let pos = s.cursor_pos();
        let style = s.theme.style;
        let pad = style.item_pad;

        // Calculate button size
        let (width, height) = s.size_of(label)?;
        let mut button = rect![
            pos.x(),
            pos.y(),
            width as i32 + 2 * pad.x(),
            height as i32 + 2 * pad.y()
        ];

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, button);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;
        let active = s.ui.is_active(id);

        s.push();
        s.ui.push_cursor();

        // Render
        s.rect_mode(RectMode::Corner);

        // Button
        s.push();
        if focused {
            s.stroke(s.highlight_color());
        } else {
            s.stroke(s.muted_color());
        }
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
            s.fill(s.highlight_color());
            if active {
                button.offset([1, 1]);
            }
        } else if disabled {
            s.fill(s.primary_color() / 2);
        } else {
            s.fill(s.primary_color());
        }
        s.rect(button)?;
        s.pop();

        // Button text
        s.rect_mode(RectMode::Center);
        s.clip(button)?;
        s.set_cursor_pos(button.center());
        s.text(label)?;
        s.no_clip()?;

        s.ui.pop_cursor();
        s.pop();

        // Process input
        s.ui.handle_input(id);
        s.advance_cursor(button);
        if !disabled {
            Ok(s.ui.was_clicked(id))
        } else {
            Ok(false)
        }
    }

    /// Draw a checkbox to the current canvas.
    pub fn checkbox<S>(&mut self, label: S, checked: &mut bool) -> PixResult<bool>
    where
        S: AsRef<str>,
    {
        let label = label.as_ref();

        let s = self;
        let id = s.ui.get_id(&label);
        let label = label.split('#').next().unwrap_or("");
        let pos = s.cursor_pos();

        // Calculate checkbox rect
        let checkbox = square![pos, CHECKBOX_SIZE];

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, checkbox);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;
        let active = s.ui.is_active(id);

        s.push();

        // Render
        s.rect_mode(RectMode::Corner);

        // Checkbox
        s.push();
        if focused || active {
            s.stroke(s.highlight_color());
        } else {
            s.stroke(s.muted_color());
        }
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
            s.fill(s.secondary_color());
        } else if disabled {
            s.fill(s.primary_color() / 2);
        } else {
            s.fill(s.primary_color());
        }
        s.rect(checkbox)?;

        if *checked {
            if disabled {
                s.stroke(s.highlight_color() / 2);
            } else {
                s.stroke(s.highlight_color());
            }
            s.stroke_weight(2);
            let half = CHECKBOX_SIZE / 2;
            let third = CHECKBOX_SIZE / 3;
            let x = checkbox.left() + half - 1;
            let y = checkbox.bottom() - third;
            let start = [x - third + 2, y - third + 2];
            let mid = [x, y];
            let end = [x + third + 1, y - half + 2];
            s.line([start, mid])?;
            s.line([mid, end])?;
        }
        s.advance_cursor(checkbox);
        s.pop();

        // Label
        s.same_line(None);
        s.text(label)?;

        // Process input
        s.ui.handle_input(id);
        if !disabled {
            let clicked = s.ui.was_clicked(id);
            if clicked {
                *checked = !(*checked);
            }
            Ok(clicked)
        } else {
            Ok(false)
        }
    }

    /// Draw a set of radio buttons to the current canvas.
    pub fn radio<S>(&mut self, label: S, selected: &mut usize, index: usize) -> PixResult<bool>
    where
        S: AsRef<str>,
    {
        let label = label.as_ref();

        let s = self;
        let id = s.ui.get_id(&label);
        let label = label.split('#').next().unwrap_or("");
        let pos = s.cursor_pos();

        // Calculate radio rect
        let radio = circle![pos + RADIO_SIZE, RADIO_SIZE];

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, radio);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;
        let active = s.ui.is_active(id);

        s.push();

        // Render
        s.rect_mode(RectMode::Corner);
        s.ellipse_mode(EllipseMode::Corner);

        // Checkbox
        if focused || active {
            s.stroke(s.highlight_color());
        } else {
            s.stroke(s.muted_color());
        }
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
            s.fill(s.secondary_color());
        } else if disabled {
            s.fill(s.primary_color() / 2);
        } else {
            s.fill(s.primary_color());
        }
        s.circle(radio)?;
        if *selected == index {
            s.no_stroke();
            if disabled {
                s.fill(s.highlight_color() / 2);
            } else {
                s.fill(s.highlight_color());
            }
            s.circle([radio.x(), radio.y(), radio.radius() - 2])?;
        }
        s.advance_cursor(radio.bounding_rect());
        s.pop();

        // Label
        s.same_line(None);
        s.text(label)?;

        // Process input
        s.ui.handle_input(id);
        if !disabled {
            let clicked = s.ui.was_clicked(id);
            if clicked {
                *selected = index;
            }
            Ok(clicked)
        } else {
            Ok(false)
        }
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
    ) -> PixResult<()> {
        use cmp::{max, min};
        let s = self;

        let mut scroll = s.ui.scroll(id);
        let xmax = width - rect.width();
        let ymax = height - rect.height();
        if s.ui.is_hovered(id) {
            let speed = 3;
            if s.ui.mouse.xrel != 0 {
                scroll.set_x(max(0, min(xmax, scroll.x() - speed * s.ui.mouse.xrel)));
                s.ui.set_scroll(id, scroll);
            }
            if s.ui.mouse.yrel != 0 {
                scroll.set_y(max(0, min(ymax, scroll.y() - speed * s.ui.mouse.yrel)));
                s.ui.set_scroll(id, scroll);
            }
        }

        // Vertical scroll
        if height > rect.height() {
            let mut scroll_y = scroll.y();
            let scrolled = s.scrollbar(
                rect![rect.right() + 1, rect.top(), SCROLL_SIZE, rect.height()],
                ymax as u32,
                &mut scroll_y,
                Direction::Vertical,
            )?;
            if scrolled {
                scroll.set_y(scroll_y);
                s.ui.set_scroll(id, scroll);
            }
            rect.offset_width(SCROLL_SIZE);
        }

        // Horizontal scroll
        if width > rect.width() {
            let mut scroll_x = scroll.x();
            let scrolled = s.scrollbar(
                rect![
                    rect.left(),
                    rect.bottom() + 1,
                    rect.width() - SCROLL_SIZE,
                    SCROLL_SIZE
                ],
                xmax as u32,
                &mut scroll_x,
                Direction::Horizontal,
            )?;
            if scrolled {
                scroll.set_x(scroll_x);
                s.ui.set_scroll(id, scroll);
            }
            rect.offset_height(SCROLL_SIZE);
        }

        s.advance_cursor(rect);

        Ok(())
    }

    fn scrollbar(
        &mut self,
        rect: Rect<i32>,
        max: u32,
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
        let mut changed = false;

        // Clamp value
        let max = max as i32;
        *value = cmp::max(0, cmp::min(max, *value));

        // Scroll region
        s.no_stroke();
        s.fill(s.background_color());
        s.rect(rect)?;

        // Thumb scroll
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
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
                ((w / (max as f32 + w)) * w) as i32
            }
            Vertical => rect.width(),
        };
        let thumb_h = match dir {
            Horizontal => rect.height(),
            Vertical => {
                let h = rect.height() as f32;
                ((h / (max as f32 + h)) * h) as i32
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
        if focused {
            if let Some(key) = s.ui.key_entered() {
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
        let mut new_value = *value;
        // Process mouse wheel
        if hovered {
            match dir {
                Vertical => {
                    new_value -= 3 * s.ui.mouse.yrel;
                }
                Horizontal => {
                    new_value -= 3 * s.ui.mouse.xrel;
                }
            };
        }
        // Process mouse input
        if active {
            new_value = match dir {
                Vertical => {
                    let my = (s.mouse_pos().y() - rect.y()).clamp(0, rect.height());
                    (my * max) / rect.height()
                }
                Horizontal => {
                    let mx = (s.mouse_pos().x() - rect.x()).clamp(0, rect.width());
                    (mx * max) / rect.width()
                }
            };
        }
        if new_value != *value {
            *value = new_value;
            changed = true;
        }
        s.ui.handle_input(id);

        Ok(changed)
    }
}
