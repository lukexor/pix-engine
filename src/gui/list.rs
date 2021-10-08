//! Immediate-GUI functions related to rendering and interacting with lists and select boxes.

use std::cmp::max;

use num_traits::AsPrimitive;

use super::{get_hash, slider::Direction};
use crate::{prelude::*, renderer::Rendering};

impl PixState {
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
        let id = get_hash(&rect);

        // Calculate list content rect
        let pad = s.theme.padding;
        let mut border = rect;
        if !label.is_empty() {
            // Resize content area to fit label
            let (_, h) = s.size_of(&label)?;
            let offset = h as i32 + pad;
            border.set_y(border.y() + offset);
            border.set_height(border.height() - offset);
        }
        let mut content = border;
        content.set_x(content.x() + pad);

        let line_height = item_height as i32 + pad * 2;
        let mut scroll = s.ui_state.scroll(id);
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
            total_width = max(w as i32, total_width);
            if scroll_height == 0 && w as i32 > content.width() {
                scroll_height = 16;
                content.set_height(content.height() - scroll_height);
            }
        }
        total_width += scroll_width;

        // Check hover/active/keyboard focus
        let disabled = s.ui_state.disabled;
        if !disabled && content.contains_point(s.mouse_pos()) {
            s.ui_state.hover(id);
        }
        s.ui_state.try_capture(id);
        let focused = !disabled && s.ui_state.is_focused(id);
        let hovered = s.ui_state.is_hovered(id);
        let active = s.ui_state.is_active(id);

        s.push();

        // Render
        s.rect_mode(RectMode::Corner);
        s.renderer.font_family(&s.theme.fonts.body)?;

        // Label
        if !label.is_empty() {
            s.fill(s.text_color());
            s.text([rect.x(), rect.y()], label)?;
        }

        // Background
        s.fill(s.primary_color());
        s.rect(border)?;

        // Contents
        let mouse = s.mouse_pos();

        s.clip(border)?;
        let x = content.x() - scroll.x();
        let mut y = content.y() - scroll.y() + (skip_count as i32 * line_height);
        for (i, item) in displayed_items {
            let item_rect = rect!(content.x(), y, content.width(), line_height);
            let clickable = item_rect.bottom() > content.y() || item_rect.top() < content.height();
            if clickable {
                let click_area = item_rect;
                if hovered && click_area.contains_point(mouse) {
                    s.frame_cursor(&Cursor::hand())?;
                    if active && s.mouse_down(Mouse::Left) {
                        *selected = Some(i);
                    }
                }
            }
            if matches!(*selected, Some(el) if el == i) {
                s.no_stroke();
                s.fill(s.highlight_color());
                s.rect([border.x(), y, border.width(), line_height])?;
                s.fill(s.text_color());
            } else {
                s.fill(WHITE);
            }
            s.text([x, y + pad], item)?;
            y += line_height;
        }
        s.no_clip()?;

        // Process input
        if focused {
            if let Some(key) = s.ui_state.key_entered() {
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
                        s.ui_state.set_scroll(id, scroll);
                    } else if sel_y + line_height > scroll.y() + content.height() {
                        // Snap scroll to bottom of the window
                        scroll.set_y(sel_y - (content.height() - line_height));
                        s.ui_state.set_scroll(id, scroll);
                    }
                }
            }
        }
        s.ui_state.handle_input(id);

        // Scrollbar
        if scroll_width > 0
            && s.slider(
                [
                    border.right() - scroll_width,
                    border.top(),
                    scroll_width,
                    border.height(),
                ],
                total_height - content.height(),
                &mut scroll.y_mut(),
                Direction::Vertical,
            )?
        {
            s.ui_state.set_scroll(id, scroll);
        }
        if scroll_height > 0
            && s.slider(
                [
                    border.left(),
                    border.bottom() - scroll_height,
                    border.width() - scroll_width,
                    scroll_height,
                ],
                total_width - content.width() - scroll_width,
                &mut scroll.x_mut(),
                Direction::Horizontal,
            )?
        {
            s.ui_state.set_scroll(id, scroll);
        }

        // Border
        s.no_fill();
        if focused {
            s.stroke(s.secondary_color());
        } else {
            s.stroke(s.muted_color());
        }
        s.rect(border)?;

        s.pop();

        Ok(())
    }
}
