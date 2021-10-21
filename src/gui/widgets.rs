//! UI widget rendering functions.

use super::Direction;
use crate::prelude::*;
use std::cmp;

mod field;
mod select;
mod text;

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
        let id = s.ui.get_hash(&label);
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

        // Button text
        s.no_stroke();
        if disabled {
            s.fill(s.text_color() / 2);
        } else {
            s.fill(s.text_color());
        }

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
        let id = s.ui.get_hash(&label);
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

        // Label
        s.same_line(None);
        s.text(label)?;

        s.pop();

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
        let id = s.ui.get_hash(&label);
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

        // Label
        s.same_line(None);
        s.text(label)?;

        s.pop();

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

    /// Draw help marker text that, when hovered, displays a help box with text to the current
    /// canvas.
    pub fn help_marker<S>(&mut self, text: S) -> PixResult<()>
    where
        S: AsRef<str>,
    {
        let text = text.as_ref();

        let s = self;
        let id = s.ui.get_hash(&text);
        let pos = s.cursor_pos();

        // Calculate hover area
        let marker = "(?)";
        let (w, h) = s.size_of(marker)?;
        let hover = rect!(pos, w as i32, h as i32);

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, hover);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;

        s.push();

        // Render
        s.rect_mode(RectMode::Corner);

        if focused {
            s.stroke(s.highlight_color());
            s.no_fill();
            s.rect(hover)?;
        }

        // Marker
        s.disable();
        s.text(marker)?;
        if !disabled {
            s.no_disable();
        }

        // Tooltip
        if hovered || focused {
            s.tooltip(text)?;
        }

        s.pop();

        // Process input
        s.ui.handle_input(id);

        Ok(())
    }

    /// Draw tooltip box at the mouse cursor with text to the current canvas.
    pub fn tooltip<S>(&mut self, text: S) -> PixResult<()>
    where
        S: AsRef<str>,
    {
        let text = text.as_ref();

        let s = self;
        let style = s.theme.style;
        let pad = style.frame_pad;

        let (w, h) = s.size_of(text)?;

        // Render
        s.ui.push_cursor();
        s.advanced_tooltip(
            w + 2 * pad.x() as u32,
            h + 2 * pad.y() as u32,
            |s: &mut PixState| {
                s.background(s.primary_color())?;

                s.rect_mode(RectMode::Corner);
                s.stroke(s.muted_color());
                s.no_fill();
                s.rect([0, 0, s.width() - 1, s.height() - 1])?;

                s.no_stroke();
                s.fill(s.text_color());
                s.text(text)?;

                Ok(())
            },
        )?;
        s.ui.pop_cursor();

        Ok(())
    }

    /// Draw an advanced tooltip box at the mouse cursor to the current canvas.
    pub fn advanced_tooltip<F>(&mut self, width: u32, height: u32, f: F) -> PixResult<()>
    where
        F: FnOnce(&mut PixState) -> PixResult<()>,
    {
        let s = self;

        // Calculate rect
        let mpos = s.mouse_pos();
        let mut rect = rect![mpos.x() + 15, mpos.y() + 15, width as i32, height as i32];
        let mut texture = s.create_texture(width, height, PixelFormat::Rgba)?;

        // Ensure rect stays inside window
        let (width, height) = s.dimensions();
        if rect.right() > width as i32 {
            rect.set_right(mpos.x() - 10);
        }
        if rect.bottom() > height as i32 {
            rect.set_bottom(mpos.y() - 5);
        }

        s.ui.set_mouse_offset(rect.top_left());
        s.with_texture(&mut texture, |s: &mut PixState| {
            s.set_cursor_pos(s.theme.style.frame_pad);
            f(s)
        })?;
        s.ui.clear_mouse_offset();
        s.ui.textures.push((texture, None, Some(rect)));

        Ok(())
    }
}

impl PixState {
    fn scroll(
        &mut self,
        rect: Rect<i32>,
        max: u32,
        value: &mut i32,
        dir: Direction,
    ) -> PixResult<bool> {
        use Direction::*;

        let s = self;
        let id = s.ui.get_hash(&rect);

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
