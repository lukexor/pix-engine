//! UI widget rendering functions.

use super::get_hash;
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
    pub fn text<P, S>(&mut self, p: P, text: S) -> PixResult<()>
    where
        P: Into<PointI2>,
        S: AsRef<str>,
    {
        self.text_transformed(p, text, 0.0, None, None)
    }

    /// Draw transformed text to the current canvas, optionally rotated about a `center` by `angle`
    /// or `flipped`. `angle` can be in radians or degrees depending on [AngleMode].
    pub fn text_transformed<P, S, T, C, F>(
        &mut self,
        p: P,
        text: S,
        angle: T,
        center: C,
        flipped: F,
    ) -> PixResult<()>
    where
        P: Into<PointI2>,
        S: AsRef<str>,
        C: Into<Option<PointI2>>,
        F: Into<Option<Flipped>>,
        T: AsPrimitive<Scalar>,
    {
        self._text_transformed(
            p.into(),
            text.as_ref(),
            angle.as_().into(),
            center.into(),
            flipped.into(),
        )
    }

    /// Draw a text field to the current canvas.
    ///
    /// Affected by `PixState::same_line`.
    pub fn text_field<R, L>(&mut self, rect: R, label: L, value: &mut String) -> PixResult<bool>
    where
        R: Into<Rect<i32>>,
        L: AsRef<str>,
    {
        let rect = self.get_rect(rect);
        self._text_field(rect, label.as_ref(), value)
    }

    /// Draw a button to the current canvas that returns `true` when clicked.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     if s.button([0, 0, 100, 50], "Click Me")? {
    ///       println!("I was clicked!");
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn button<R, L>(&mut self, rect: R, label: L) -> PixResult<bool>
    where
        R: Into<Rect<i32>>,
        L: AsRef<str>,
    {
        let rect = self.get_rect(rect);
        self._button(rect, label.as_ref())
    }

    /// Draw a select list to the current canvas with a scrollable region.
    pub fn select_list<R, S, I, T>(
        &mut self,
        rect: R,
        label: S,
        items: &[I],
        item_height: T,
        selected: &mut Option<usize>,
    ) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
        S: AsRef<str>,
        I: AsRef<str>,
        T: AsPrimitive<u32>,
    {
        let rect = self.get_rect(rect);
        self._select_list(rect, label.as_ref(), items, item_height.as_(), selected)
    }

    /// Draw a checkbox to the current canvas.
    pub fn checkbox<R, S>(&mut self, rect: R, label: S, checked: &mut bool) -> PixResult<bool>
    where
        R: Into<Rect<i32>>,
        S: AsRef<str>,
    {
        let rect = self.get_rect(rect);
        self._checkbox(rect, label.as_ref(), checked)
    }

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

    /// Draw a tooltip when hovered to the current canvas.
    pub fn tooltip<R1, R2, S>(&mut self, rect: R1, text: S, hover: R2) -> PixResult<()>
    where
        R1: Into<Rect<i32>>,
        R2: Into<Rect<i32>>,
        S: AsRef<str>,
    {
        let rect = self.get_rect(rect);
        self._tooltip(rect, text.as_ref(), hover.into())
    }
}

impl PixState {
    fn _text_transformed(
        &mut self,
        mut p: PointI2,
        text: &str,
        mut angle: Option<Scalar>,
        center: Option<PointI2>,
        flipped: Option<Flipped>,
    ) -> PixResult<()> {
        let s = self;
        if let RectMode::Center = s.settings.rect_mode {
            let (width, height) = s.renderer.size_of(text)?;
            p.offset([-(width as i32 / 2), -(height as i32 / 2)]);
        };
        if let AngleMode::Radians = s.settings.angle_mode {
            angle = angle.map(|a| a.to_degrees());
        };
        if text.contains('\n') {
            for line in text.split('\n') {
                s.renderer.text(
                    p,
                    line,
                    s.settings.wrap_width,
                    angle,
                    center,
                    flipped,
                    s.settings.fill,
                )?;
                let (_, h) = s.size_of(line)?;
                p.set_y(p.y() + h as i32);
            }
            Ok(())
        } else {
            Ok(s.renderer.text(
                p,
                text,
                s.settings.wrap_width,
                angle,
                center,
                flipped,
                s.settings.fill,
            )?)
        }
    }

    fn _text_field(&mut self, rect: Rect<i32>, label: &str, value: &mut String) -> PixResult<bool> {
        let s = self;
        let id = get_hash(&label);
        let style = s.theme.style;

        // Calculate input rect
        let mut input = rect;
        let (pad_x, pad_y) = style.frame_pad;
        if !label.is_empty() {
            // Resize input area to fit label
            let (w, h) = s.size_of(label)?;
            if s.ui.same_line {
                let offset = (w + pad_x) as i32;
                input.set_x(input.x() + offset);
                input.set_width(input.width() - offset);
            } else {
                let offset = (h + pad_y) as i32;
                input.set_y(input.y() + offset);
                input.set_height(input.height() - offset);
            }
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
            if s.ui.same_line {
                let (_, h) = s.size_of(label)?;
                let y = rect.center().y();
                s.text([rect.x(), y - h as i32 / 2], label)?;
            } else {
                s.text([rect.x(), rect.y()], label)?;
            }
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
        s.rect(input)?;

        // Text
        if disabled {
            s.fill(s.muted_color());
        } else {
            s.fill(s.text_color());
        }
        s.clip(input)?;
        let (vw, vh) = s.size_of(&value)?;
        let (cw, _) = s.size_of(&TEXT_CURSOR)?;
        let mut x = input.x() + pad_x as i32;
        let y = input.center().y() - vh as i32 / 2;
        let width = (vw + cw) as i32;
        if width > input.width() {
            x -= width - input.width();
        }
        s.text([x, y], &value)?;
        if focused && !s.ui.disabled && s.frame_count() >> 8 & 1 > 0 {
            let offset = 2; // Remove some left space of the text cursor
            s.text([x + vw as i32 - offset, y], &TEXT_CURSOR)?;
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

        Ok(changed)
    }

    /// Draw a scroll control that returns `true` when changed.
    fn scroll<R>(
        &mut self,
        rect: R,
        label: &str,
        max: u32,
        value: &mut i32,
        dir: Direction,
    ) -> PixResult<bool>
    where
        R: Into<Rect<i32>>,
    {
        let rect = self.get_rect(rect);
        self._scroll(rect, label, max, value, dir)
    }

    fn _scroll(
        &mut self,
        rect: Rect<i32>,
        label: &str,
        max: u32,
        value: &mut i32,
        dir: Direction,
    ) -> PixResult<bool> {
        use Direction::*;

        let s = self;
        let mut id = get_hash(&label);
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

    fn _button(&mut self, rect: Rect<i32>, label: &str) -> PixResult<bool> {
        let s = self;
        let id = get_hash(&label);

        // Check hover/active/keyboard focus
        let disabled = s.ui.disabled;
        if !disabled && rect.contains_point(s.mouse_pos()) {
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
                let [x, y, width, height] = rect.values();
                s.rect([x + 1, y + 1, width, height])?;
            } else {
                s.rect(rect)?;
            }
        } else if disabled {
            s.fill(s.background_color());
            s.rect(rect)?;
        } else {
            s.fill(s.primary_color());
            s.rect(rect)?;
        }

        // Button text
        s.rect_mode(RectMode::Center);
        if disabled {
            s.fill(s.muted_color());
        } else {
            s.fill(s.text_color());
        }
        s.clip(rect)?;
        s.text(rect.center(), label)?;
        s.no_clip()?;

        s.pop();

        // Process input
        s.ui.handle_input(id);
        if !disabled {
            Ok(s.ui.was_clicked(id))
        } else {
            Ok(false)
        }
    }

    fn _select_list<S>(
        &mut self,
        rect: Rect<i32>,
        label: &str,
        items: &[S],
        item_height: u32,
        selected: &mut Option<usize>,
    ) -> PixResult<()>
    where
        S: AsRef<str>,
    {
        let s = self;
        let id = get_hash(&label);
        let style = s.theme.style;

        // Calculate list content rect
        let (pad_x, pad_y) = style.frame_pad;
        let mut border = rect;
        if !label.is_empty() {
            // Resize content area to fit label
            let (_, h) = s.size_of(&label)?;
            let offset = (h + pad_y) as i32;
            border.set_y(border.y() + offset);
            border.set_height(border.height() - offset);
        }
        let mut content = border;
        content.set_x(content.x() + pad_x as i32);

        // Calculate displayed items
        let line_height = item_height as i32 + pad_y as i32 * 2;
        let mut scroll = s.ui.scroll(id);
        let skip_count = (scroll.y / line_height) as usize;
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

        s.push();

        // Render
        s.rect_mode(RectMode::Corner);

        // Label
        if !label.is_empty() {
            s.fill(s.text_color());
            s.text([rect.x(), rect.y()], label)?;
        }

        // Background
        if disabled {
            s.fill(s.background_color());
        } else {
            s.fill(s.primary_color());
        }
        s.rect(border)?;

        // Contents
        let mouse = s.mouse_pos();

        s.clip(border)?;
        let x = content.x() - scroll.x;
        let mut y = content.y() - scroll.y + (skip_count as i32 * line_height);
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
            s.text([x, y + pad_y as i32], item)?;
            y += line_height;
        }
        s.no_clip()?;

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
                    if sel_y < scroll.y {
                        scroll.y = sel_y;
                        s.ui.set_scroll(id, scroll);
                    } else if sel_y + line_height > scroll.y + content.height() {
                        // Snap scroll to bottom of the window
                        scroll.y = sel_y - (content.height() - line_height);
                        s.ui.set_scroll(id, scroll);
                    }
                }
            }
        }
        s.ui.handle_input(id);

        // Process mouse wheel
        let ymax = total_height - content.height();
        let xmax = total_width - content.width() - scroll_width;
        if hovered {
            if s.ui.mouse.yrel != 0 {
                scroll.y = cmp::max(0, cmp::min(ymax, scroll.y - 3 * s.ui.mouse.yrel));
                s.ui.set_scroll(id, scroll);
            }
            if s.ui.mouse.xrel != 0 {
                scroll.x = cmp::max(0, cmp::min(xmax, scroll.x - 3 * s.ui.mouse.xrel));
                s.ui.set_scroll(id, scroll);
            }
        }

        // Scrollbar
        if scroll_width > 0
            && s.scroll(
                [
                    border.right() - scroll_width,
                    border.top(),
                    scroll_width,
                    border.height(),
                ],
                label,
                ymax as u32,
                &mut scroll.y,
                Direction::Vertical,
            )?
        {
            s.ui.set_scroll(id, scroll);
        }
        if scroll_height > 0
            && s.scroll(
                [
                    border.left(),
                    border.bottom() - scroll_height,
                    border.width() - scroll_width,
                    scroll_height,
                ],
                label,
                xmax as u32,
                &mut scroll.x,
                Direction::Horizontal,
            )?
        {
            s.ui.set_scroll(id, scroll);
        }

        // Border
        s.no_fill();
        if focused {
            s.stroke(s.highlight_color());
        } else {
            s.stroke(s.muted_color());
        }
        s.rect(border)?;

        s.pop();

        Ok(())
    }

    fn _checkbox(&mut self, rect: Rect<i32>, label: &str, checked: &mut bool) -> PixResult<bool> {
        let s = self;
        let id = get_hash(&label);
        let style = s.theme.style;

        // Calculate checkbox rect
        let (_, h) = s.size_of(label)?;
        let y = rect.center().y();
        let checkbox = square![rect.x(), y - h as i32 / 2, 16];

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
        let (pad_x, _) = style.frame_pad;
        s.rect_mode(RectMode::Corner);

        // Label
        if !label.is_empty() {
            if disabled {
                s.fill(s.muted_color());
            } else {
                s.fill(s.text_color());
            }
            s.text(
                [checkbox.right() + 2 * pad_x as i32, y - h as i32 / 2],
                label,
            )?;
        }

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

    fn _radio(
        &mut self,
        rect: Rect<i32>,
        label: &str,
        selected: &mut usize,
        index: usize,
    ) -> PixResult<bool> {
        let s = self;
        let id = get_hash(&label);
        let style = s.theme.style;

        // Calculate radio rect
        let radius = 9;
        let (_, h) = s.size_of(label)?;
        let y = rect.center().y();
        let radio = circle![rect.x() + radius, y - h as i32 / 2 + radius, radius];

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
        let (pad_x, _) = style.frame_pad;
        s.rect_mode(RectMode::Corner);

        // Label
        if !label.is_empty() {
            if disabled {
                s.fill(s.muted_color());
            } else {
                s.fill(s.text_color());
            }
            s.text([radio.right() + 2 * pad_x as i32, y - h as i32 / 2], label)?;
        }

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
            s.circle(circle![radio.x(), radio.y(), radio.radius() - 3])?;
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

    fn _tooltip(&mut self, rect: Rect<i32>, text: &str, hover: Rect<i32>) -> PixResult<()> {
        let s = self;
        let id = get_hash(&text);
        let style = s.theme.style;

        // Check hover/active/keyboard focus
        let (pad_x, pad_y) = style.frame_pad;
        let disabled = s.ui.disabled;
        if !disabled && hover.contains_point(s.mouse_pos()) {
            s.ui.hover(id);
        }
        s.ui.try_capture(id);
        let hovered = !disabled && s.ui.is_hovered(id);

        s.push();

        // Render

        // Tooltip
        s.rect_mode(RectMode::Corner);
        if hovered {
            s.stroke(s.muted_color());
            s.fill(s.primary_color());
            let m = s.mouse_pos();
            let pad_x2 = pad_x as i32 * 2;
            let pad_y2 = pad_y as i32 * 2;
            let mut rect = rect![
                m.x() + rect.x() + pad_x2,
                m.y() + rect.y() + pad_y2,
                rect.width() + pad_x2,
                rect.height() + pad_y2
            ];
            let (w, h) = s.dimensions();
            if rect.right() > w as i32 {
                rect.set_x(rect.x() - rect.width() - pad_x2);
            }
            if rect.bottom() > h as i32 {
                rect.set_y(rect.y() - rect.height() - pad_y2);
            }
            s.rect(rect)?;
            s.clip(rect)?;
            s.fill(s.text_color());
            s.text(rect.top_left() + point!(pad_x as i32, pad_y as i32), text)?;
            s.no_clip()?;
        }

        s.pop();

        // Process input
        s.ui.handle_input(id);

        Ok(())
    }
}
