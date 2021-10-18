//! UI widget rendering functions.

use crate::{prelude::*, renderer::Rendering};
use num_traits::AsPrimitive;
use std::cmp;

const TEXT_CURSOR: &str = "│";
const SCROLL_SPEED: i32 = 2;

/// Scroll direction.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    /// Horizontal.
    Horizontal,
    /// Vertical.
    Vertical,
}

impl PixState {
    /// Draw text to the current canvas.
    pub fn text<S>(&mut self, text: S) -> PixResult<()>
    where
        S: AsRef<str>,
    {
        self.text_transformed(text, 0.0, None, None)
    }

    /// Draw transformed text to the current canvas, optionally rotated about a `center` by `angle`
    /// or `flipped`. `angle` can be in radians or degrees depending on [AngleMode].
    pub fn text_transformed<S, T, C, F>(
        &mut self,
        text: S,
        angle: T,
        center: C,
        flipped: F,
    ) -> PixResult<()>
    where
        S: AsRef<str>,
        C: Into<Option<PointI2>>,
        F: Into<Option<Flipped>>,
        T: AsPrimitive<Scalar>,
    {
        self._text_transformed(
            text.as_ref(),
            angle.as_().into(),
            center.into(),
            flipped.into(),
        )
    }

    /// Draw a text field to the current canvas.
    pub fn text_field<L>(&mut self, label: L, value: &mut String) -> PixResult<bool>
    where
        L: AsRef<str>,
    {
        self._text_field(label.as_ref(), value)
    }

    /// Draw a button to the current canvas that returns `true` when clicked.
    pub fn button<L>(&mut self, label: L) -> PixResult<bool>
    where
        L: AsRef<str>,
    {
        self._button(label.as_ref())
    }

    /// Draw a select list to the current canvas with a scrollable region.
    pub fn select_list<S, I, T>(
        &mut self,
        label: S,
        items: &[I],
        item_height: T,
        selected: &mut Option<usize>,
    ) -> PixResult<()>
    where
        S: AsRef<str>,
        I: AsRef<str>,
        T: AsPrimitive<u32>,
    {
        self._select_list(label.as_ref(), items, item_height.as_(), selected)
    }

    /// Draw a checkbox to the current canvas.
    pub fn checkbox<S>(&mut self, label: S, checked: &mut bool) -> PixResult<bool>
    where
        S: AsRef<str>,
    {
        self._checkbox(label.as_ref(), checked)
    }

    /// Draw a set of radio buttons to the current canvas.
    pub fn radio<S>(&mut self, label: S, selected: &mut usize, index: usize) -> PixResult<bool>
    where
        S: AsRef<str>,
    {
        self._radio(label.as_ref(), selected, index)
    }

    /// Draw help marker text that, when hovered, displays a help box with text to the current
    /// canvas.
    pub fn help_marker<S>(&mut self, text: S) -> PixResult<()>
    where
        S: AsRef<str>,
    {
        self._help_marker(text.as_ref())
    }

    /// Draw tooltip box with text to the current canvas.
    pub fn tooltip<S>(&mut self, text: S) -> PixResult<()>
    where
        S: AsRef<str>,
    {
        self._tooltip(text.as_ref())
    }
}

impl PixState {
    fn _text_transformed(
        &mut self,
        text: &str,
        mut angle: Option<Scalar>,
        center: Option<PointI2>,
        flipped: Option<Flipped>,
    ) -> PixResult<()> {
        let s = self;
        let mut pos = s.cursor_pos();
        if let RectMode::Center = s.settings.rect_mode {
            let (width, height) = s.renderer.size_of(text)?;
            pos.offset([-(width as i32 / 2), -(height as i32 / 2)]);
        };
        if let AngleMode::Radians = s.settings.angle_mode {
            angle = angle.map(|a| a.to_degrees());
        };
        let mut size = point!();
        for line in text.split('\n') {
            s.renderer.text(
                pos,
                line,
                s.settings.wrap_width,
                angle,
                center,
                flipped,
                s.settings.fill,
            )?;
            let (w, h) = s.size_of(line)?;
            pos.offset_y(h as i32);
            size.set_x(cmp::max(w as i32, size.x()));
            size.offset_y(h as i32);
        }
        s.advance_cursor(size);
        Ok(())
    }

    fn _text_field(&mut self, label: &str, value: &mut String) -> PixResult<bool> {
        let s = self;
        let id = s.ui.get_hash(&label);
        let pos = s.cursor_pos();
        let font_size = s.theme.font_sizes.body as i32;
        let style = s.theme.style;
        let pad = style.item_pad;

        // Calculate input rect
        let mut input = rect![pos, 15 * font_size, font_size + 2 * pad.y()];
        let label = label.split('#').next().unwrap_or("");
        if !label.is_empty() {
            let (w, _) = s.size_of(label)?;
            input.offset_x(w as i32 + pad.x());
        }

        // Check hover/active/keyboard focus
        let disabled = s.ui.disabled;
        if !disabled && input.contains_point(s.mouse_pos()) {
            s.ui.hover(id);
        }
        s.ui.try_capture(id);
        let focused = !disabled && s.ui.is_focused(id);
        let hovered = s.ui.is_hovered(id);
        let active = !disabled && s.ui.is_active(id);

        s.push();
        let mut changed = false;

        // Render
        s.rect_mode(RectMode::Corner);

        // Label
        if !label.is_empty() {
            s.fill(s.text_color());
            s.text(label)?;
        }

        // Input
        if focused || active {
            s.stroke(s.highlight_color());
        } else {
            s.stroke(s.muted_color());
        }
        if hovered {
            s.frame_cursor(&Cursor::ibeam())?;
        }
        if disabled {
            s.fill(s.background_color());
        } else {
            s.fill(s.primary_color());
        }
        s.same_line(None);
        s.rect(input)?;

        // Text
        if disabled {
            s.fill(s.muted_color());
        } else {
            s.fill(s.text_color());
        }
        s.clip(input)?;
        let (vw, vh) = s.size_of(&value)?;
        let (cw, _) = s.size_of(TEXT_CURSOR)?;
        let mut x = input.x() + pad.x();
        let y = input.center().y() - vh as i32 / 2;
        let width = (vw + cw) as i32;
        if width > input.width() {
            x -= width - input.width();
        }
        s.ui.push_cursor([x, y]);
        s.text(&value)?;
        s.ui.pop_cursor();
        if focused && !s.ui.disabled && s.elapsed() as usize >> 8 & 1 > 0 {
            let offset = 2; // Remove some left space of the text cursor
            s.ui.push_cursor([x + vw as i32 - offset, y]);
            s.text(TEXT_CURSOR)?;
            s.ui.pop_cursor();
        }
        s.no_clip()?;

        s.pop();

        // Process input
        if focused {
            if let Some(key) = s.ui.key_entered() {
                match key {
                    Key::Backspace if !value.is_empty() => {
                        value.pop();
                        changed = true;
                    }
                    _ => (),
                }
            }
            if let Some(text) = s.ui.keys.typed.take() {
                value.push_str(&text);
                changed = true;
            }
        }
        s.ui.handle_input(id);
        s.advance_cursor(input.size());

        Ok(changed)
    }

    fn scroll(
        &mut self,
        rect: Rect<i32>,
        label: &str,
        max: u32,
        value: &mut i32,
        dir: Direction,
    ) -> PixResult<bool> {
        use Direction::*;

        let s = self;
        let mut id = s.ui.get_hash(&label);
        match dir {
            Horizontal => id += rect.x() as u64,
            Vertical => id += rect.y() as u64,
        }

        // Check hover/active/keyboard focus
        let disabled = s.ui.disabled;
        if !disabled && rect.contains_point(s.mouse_pos()) {
            s.ui.hover(id);
        }
        s.ui.try_capture(id);
        let focused = !disabled && s.ui.is_focused(id);
        let hovered = s.ui.is_hovered(id);
        let active = !disabled && s.ui.is_active(id);

        s.push();
        let mut changed = false;

        // Clamp value
        let max = max as i32;
        *value = cmp::max(0, cmp::min(max, *value));

        // Render
        let radius = 3;

        // Scroll region
        s.stroke(s.background_color());
        s.fill(s.background_color());
        s.rect(rect)?;

        // Thumb scroll
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
        }
        if hovered || active || focused {
            s.fill(s.highlight_color());
        } else if disabled {
            s.fill(s.muted_color());
        } else {
            s.fill(s.secondary_color());
        }
        let thumb_w = match dir {
            Horizontal => {
                let w = rect.width() as f32;
                ((w / (max as f32 + w)) * w) as i32
            }
            Vertical => rect.width() - 6,
        };
        let thumb_h = match dir {
            Horizontal => rect.height() - 6,
            Vertical => {
                let h = rect.height() as f32;
                ((h / (max as f32 + h)) * h) as i32
            }
        };
        match dir {
            Horizontal => {
                let thumb_x = ((rect.width() - thumb_w) * *value) / max;
                s.rounded_rect(
                    [rect.x() + thumb_x, rect.y() + 3, thumb_w, thumb_h - 1],
                    radius,
                )?
            }
            Vertical => {
                let thumb_y = ((rect.height() - thumb_h) * *value) / max;
                s.rounded_rect(
                    [rect.x() + 3, rect.y() + thumb_y, thumb_w - 1, thumb_h],
                    radius,
                )?
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

    fn _button(&mut self, label: &str) -> PixResult<bool> {
        let s = self;
        let id = s.ui.get_hash(&label);
        let pos = s.cursor_pos();
        let style = s.theme.style;
        let pad = style.item_pad;

        // Calculate button size
        let label = label.split('#').next().unwrap_or("");
        let (width, height) = s.size_of(label)?;
        let mut button = rect![
            pos.x(),
            pos.y(),
            width as i32 + 2 * pad.x(),
            height as i32 + 2 * pad.y()
        ];

        // Check hover/active/keyboard focus
        let disabled = s.ui.disabled;
        if !disabled && button.contains_point(s.mouse_pos()) {
            s.ui.hover(id);
        }
        s.ui.try_capture(id);
        let focused = !disabled && s.ui.is_focused(id);
        let hovered = s.ui.is_hovered(id);
        let active = s.ui.is_active(id);

        s.push();

        // Render

        // Button
        s.rect_mode(RectMode::Corner);
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
            s.fill(s.background_color());
        } else {
            s.fill(s.primary_color());
        }
        s.rect(button)?;

        // Button text
        if disabled {
            s.fill(s.muted_color());
        } else {
            s.fill(s.text_color());
        }
        s.clip(button)?;
        s.ui.push_cursor(button.center());
        s.rect_mode(RectMode::Center);
        s.text(label)?;
        s.ui.pop_cursor();
        s.no_clip()?;

        s.pop();

        // Process input
        s.ui.handle_input(id);
        s.advance_cursor(button.size());
        if !disabled {
            Ok(s.ui.was_clicked(id))
        } else {
            Ok(false)
        }
    }

    fn _select_list<S>(
        &mut self,
        label: &str,
        items: &[S],
        item_height: u32,
        selected: &mut Option<usize>,
    ) -> PixResult<()>
    where
        S: AsRef<str>,
    {
        let s = self;
        let id = s.ui.get_hash(&label);
        let font_size = s.theme.font_sizes.body as i32;
        let style = s.theme.style;
        let pad = style.item_pad;

        s.push();

        // Render label
        s.rect_mode(RectMode::Corner);
        let label = label.split('#').next().unwrap_or("");
        if !label.is_empty() {
            s.fill(s.text_color());
            s.text(label)?;
        }

        // Calculate list content rect
        let pos = s.cursor_pos();
        let border = rect![pos, 15 * font_size, 5 * (font_size + 2 * pad.y())];
        let mut content = border;
        content.offset_x(pad.x());
        content.offset_width(-2 * pad.x());

        // Calculate displayed items
        let pad = style.item_pad;
        let line_height = item_height as i32 + pad.y() * 2;
        let mut scroll = s.ui.scroll(id);
        let skip_count = (scroll.y() / line_height) as usize;
        let displayed_count = (content.height() / line_height) as usize;
        let displayed_items = items
            .iter()
            .enumerate()
            .skip(skip_count)
            .take(displayed_count + 2);

        // Calculate total height and whether a vertical scrollbar is needed
        let total_height = items.len() as i32 * line_height;
        let mut scroll_width = 0;
        if total_height > content.height() {
            scroll_width = 16;
            content.set_width(content.width() - scroll_width);
        }

        // Calcualte total width and whether a horizontal scrollbar is needed
        let mut total_width = 0;
        let mut scroll_height = 0;
        for (_, item) in displayed_items.clone() {
            let (w, _) = s.size_of(item).unwrap_or((0, 0));
            total_width = cmp::max(w as i32, total_width);
            if scroll_height == 0 && w as i32 > content.width() {
                scroll_height = 16;
                content.set_height(content.height() - scroll_height);
            }
        }
        total_width += scroll_width;

        // Check hover/active/keyboard focus
        let disabled = s.ui.disabled;
        if !disabled && content.contains_point(s.mouse_pos()) {
            s.ui.hover(id);
        }
        s.ui.try_capture(id);
        let focused = !disabled && s.ui.is_focused(id);
        let hovered = s.ui.is_hovered(id);
        let active = s.ui.is_active(id);

        // Render content

        // Border
        if focused {
            s.stroke(s.highlight_color());
        } else {
            s.stroke(s.muted_color());
        }
        if disabled {
            s.fill(s.background_color());
        } else {
            s.fill(s.primary_color());
        }
        s.rect(border)?;

        // Contents
        let mouse = s.mouse_pos();

        let mut clip = border;
        clip.offset_size([-1, -1]);
        s.clip(clip)?;
        let x = content.x() - scroll.x();
        let mut y = content.y() - scroll.y() + (skip_count as i32 * line_height);
        for (i, item) in displayed_items {
            let item_rect = rect!(content.x(), y, content.width(), line_height);
            let clickable = item_rect.bottom() > content.y() || item_rect.top() < content.height();
            if clickable && hovered && item_rect.contains_point(mouse) {
                s.frame_cursor(&Cursor::hand())?;
                s.no_stroke();
                s.fill(s.highlight_color());
                s.rect([border.x(), y, border.width(), line_height])?;
                if active && s.mouse_down(Mouse::Left) {
                    *selected = Some(i);
                }
            }
            if matches!(*selected, Some(el) if el == i) {
                s.no_stroke();
                s.fill(s.secondary_color());
                s.rect([border.x(), y, border.width(), line_height])?;
            }
            if disabled {
                s.fill(s.muted_color());
            } else {
                s.fill(s.text_color());
            }
            s.ui.push_cursor([x, y + pad.y()]);
            s.text(item)?;
            s.ui.pop_cursor();
            y += line_height;
        }

        // Process input
        if focused {
            if let Some(key) = s.ui.key_entered() {
                let changed_selection = match key {
                    Key::Up => {
                        *selected = selected.map(|s| s.saturating_sub(1)).or(Some(0));
                        true
                    }
                    Key::Down => {
                        *selected = selected
                            .map(|s| {
                                let mut s = s.saturating_add(1);
                                if s >= items.len() {
                                    s = items.len() - 1;
                                }
                                s
                            })
                            .or(Some(0));
                        true
                    }
                    _ => false,
                };
                if changed_selection {
                    let sel_y = selected.unwrap_or(0) as i32 * line_height;
                    // Snap scroll to top of the window
                    if sel_y < scroll.y() {
                        scroll.set_y(sel_y);
                        s.ui.set_scroll(id, scroll);
                    } else if sel_y + line_height > scroll.y() + content.height() {
                        // Snap scroll to bottom of the window
                        scroll.set_y(sel_y - (content.height() - line_height));
                        s.ui.set_scroll(id, scroll);
                    }
                }
            }
        }
        s.ui.handle_input(id);
        s.advance_cursor(border.size());

        // Process mouse wheel
        let ymax = total_height - content.height();
        let xmax = total_width - content.width() - scroll_width;
        if hovered {
            if s.ui.mouse.yrel != 0 {
                scroll.set_y(cmp::max(
                    0,
                    cmp::min(ymax, scroll.y() - 3 * s.ui.mouse.yrel),
                ));
                s.ui.set_scroll(id, scroll);
            }
            if s.ui.mouse.xrel != 0 {
                scroll.set_x(cmp::max(
                    0,
                    cmp::min(xmax, scroll.x() - 3 * s.ui.mouse.xrel),
                ));
                s.ui.set_scroll(id, scroll);
            }
        }

        // Scrollbar
        let mut scroll_y = scroll.y();
        if scroll_width > 0
            && s.scroll(
                rect![
                    border.right() - scroll_width,
                    border.top() + 1,
                    scroll_width,
                    border.height(),
                ],
                label,
                ymax as u32,
                &mut scroll_y,
                Direction::Vertical,
            )?
        {
            scroll.set_y(scroll_y);
            s.ui.set_scroll(id, scroll);
        }
        let mut scroll_x = scroll.x();
        if scroll_height > 0
            && s.scroll(
                rect![
                    border.left() + 1,
                    border.bottom() - scroll_height,
                    border.width() - scroll_width,
                    scroll_height,
                ],
                label,
                xmax as u32,
                &mut scroll_x,
                Direction::Horizontal,
            )?
        {
            scroll.set_x(scroll_x);
            s.ui.set_scroll(id, scroll);
        }

        s.no_clip()?;
        s.pop();

        Ok(())
    }

    fn _checkbox(&mut self, label: &str, checked: &mut bool) -> PixResult<bool> {
        let s = self;
        let id = s.ui.get_hash(&label);
        let pos = s.cursor_pos();

        // Calculate checkbox rect
        let checkbox = square![pos, 16];

        // Check hover/active/keyboard focus
        let disabled = s.ui.disabled;
        if !disabled && checkbox.contains_point(s.mouse_pos()) {
            s.ui.hover(id);
        }
        s.ui.try_capture(id);
        let focused = !disabled && s.ui.is_focused(id);
        let hovered = s.ui.is_hovered(id);
        let active = !disabled && s.ui.is_active(id);

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
        }
        if disabled {
            s.fill(s.background_color());
        } else {
            s.fill(s.primary_color());
        }
        s.rect(checkbox)?;
        if *checked {
            if disabled {
                s.stroke(s.muted_color());
            } else {
                s.stroke(s.highlight_color());
            }
            s.stroke_weight(2);
            let third = 16 / 3;
            let x = checkbox.x() + 3 + third;
            let y = checkbox.bottom() - 1 - third / 2;
            let start = [x - third + 1, y - third + 1];
            let mid = [x, y];
            let end = [x + third, y - third * 2 + 1];
            s.line([start, mid])?;
            s.line([mid, end])?;
        }
        s.advance_cursor(checkbox.size());

        // Label
        let label = label.split('#').next().unwrap_or("");
        if !label.is_empty() {
            if disabled {
                s.fill(s.muted_color());
            } else {
                s.fill(s.text_color());
            }
            s.same_line(None);
            s.text(label)?;
        }

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

    fn _radio(&mut self, label: &str, selected: &mut usize, index: usize) -> PixResult<bool> {
        let s = self;
        let id = s.ui.get_hash(&label);
        let pos = s.cursor_pos();

        // Calculate radio rect
        let radius = 8;
        let radio = circle![pos + radius, radius];

        // Check hover/active/keyboard focus
        let disabled = s.ui.disabled;
        if !disabled && radio.contains_point(s.mouse_pos()) {
            s.ui.hover(id);
        }
        s.ui.try_capture(id);
        let focused = !disabled && s.ui.is_focused(id);
        let hovered = s.ui.is_hovered(id);
        let active = !disabled && s.ui.is_active(id);

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
        }
        s.ellipse_mode(EllipseMode::Corner);
        if disabled {
            s.fill(s.background_color());
        } else {
            s.fill(s.primary_color());
        }
        s.circle(radio)?;
        if *selected == index {
            if disabled {
                s.fill(s.muted_color());
            } else {
                s.fill(s.highlight_color());
            }
            s.circle([radio.x(), radio.y(), radio.radius() - 3])?;
        }
        s.advance_cursor(radio.size());

        // Label
        let label = label.split('#').next().unwrap_or("");
        if !label.is_empty() {
            if disabled {
                s.fill(s.muted_color());
            } else {
                s.fill(s.text_color());
            }
            s.same_line(None);
            s.text(label)?;
        }

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

    fn _help_marker(&mut self, text: &str) -> PixResult<()> {
        let s = self;
        let id = s.ui.get_hash(&text);
        let pos = s.cursor_pos();

        // Calculate hover area
        let marker = "(?)";
        let (w, h) = s.size_of(marker)?;
        let hover = rect!(pos, w, h);

        // Check hover/active/keyboard focus
        let disabled = s.ui.disabled;
        if !disabled && hover.contains_point(s.mouse_pos()) {
            s.ui.hover(id);
        }
        s.ui.try_capture(id);
        let hovered = !disabled && s.ui.is_hovered(id);

        s.push();

        // Render

        // Marker
        s.rect_mode(RectMode::Corner);
        s.fill(s.muted_color());
        s.text(marker)?;

        // Tooltip
        if hovered {
            s._tooltip(text)?;
        }

        s.pop();

        // Process input
        s.ui.handle_input(id);

        Ok(())
    }

    fn _tooltip(&mut self, text: &str) -> PixResult<()> {
        let s = self;
        let style = s.theme.style;
        let pad = style.frame_pad;

        s.push();

        // Render
        s.rect_mode(RectMode::Corner);

        // Calculate rect
        let pos = s.mouse_pos();
        let (w, h) = s.size_of(text)?;
        let mut rect = rect![pos.x() + 30, pos.y() + 30, w as i32, h as i32];
        rect.offset_size([2 * pad.x(), 2 * pad.y()]);

        // Ensure rect stays inside window
        let (width, height) = s.dimensions();
        if rect.right() > width as i32 {
            rect.set_right(pos.x() - 10);
        }
        if rect.bottom() > height as i32 {
            rect.set_bottom(pos.y() - 5);
        }

        s.stroke(s.muted_color());
        s.fill(s.primary_color());
        s.rect(rect)?;

        s.wrap_width(rect.width() - 2 * pad.x());
        s.clip(rect)?;

        s.ui.push_cursor(rect.top_left() + point!(pad.x(), pad.y()));
        s.fill(s.text_color());
        s.text(text)?;
        s.ui.pop_cursor();

        s.no_clip()?;

        s.pop();

        Ok(())
    }
}
