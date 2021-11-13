//! Select widget rendering methods.
//!
//! Provided [PixState] methods:
//!
//! - [PixState::select_box]
//! - [PixState::select_list]
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App { select_box: usize };
//! # impl AppState for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     let items = ["Item 1", "Item 2", "Item 3"];
//!     s.select_box("Select Box", &mut self.select_box, &items)?;
//!
//!     let displayed_count = 4;
//!     s.select_list("Select Box", &mut self.select_box, &items, displayed_count)?;
//!     Ok(())
//! }
//! # }
//! ```

use crate::{gui::scroll::SCROLL_SIZE, prelude::*};
use std::cmp;

impl PixState {
    /// Draw a select box the current canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { select_box: usize };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     let items = ["Item 1", "Item 2", "Item 3"];
    ///     let displayed_count = 4;
    ///     s.select_box("Select Box", &mut self.select_box, &items, displayed_count)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn select_box<S, I>(
        &mut self,
        label: S,
        selected: &mut usize,
        items: &[I],
        displayed_count: usize,
    ) -> PixResult<()>
    where
        S: AsRef<str>,
        I: AsRef<str>,
    {
        let label = label.as_ref();

        let s = self;
        let id = s.ui.get_id(&label);
        let label = label.split('#').next().unwrap_or("");
        let pos = s.cursor_pos();
        let font_size = s.theme.font_sizes.body as i32;
        let style = s.theme.style;
        let fpad = style.frame_pad;
        let ipad = style.item_pad;

        // Calculate rect
        let width =
            s.ui.next_width
                .take()
                .unwrap_or_else(|| s.width().unwrap_or(100));
        let (_, h) = s.size_of(items.get(0).map(|i| i.as_ref()).unwrap_or(""))?;
        let mut select_box = rect![pos, width as i32 - 2 * fpad.x(), h as i32 + 2 * ipad.y()];
        let (lwidth, lheight) = s.size_of(label)?;
        if !label.is_empty() {
            let offset = lwidth as i32 + ipad.x();
            select_box.offset_x(offset);
        }

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, select_box);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;

        s.push();
        s.ui.push_cursor();

        // Render
        s.rect_mode(RectMode::Corner);

        // Label
        if !label.is_empty() {
            s.set_cursor_pos([
                pos.x(),
                pos.y() + select_box.height() / 2 - lheight as i32 / 2,
            ]);
            s.text(label)?;
        }

        // Select Box
        s.push();
        if focused {
            s.stroke(s.highlight_color());
        } else {
            s.stroke(s.muted_color());
        }
        if hovered {
            s.frame_cursor(Cursor::hand())?;
            s.fill(s.secondary_color());
        } else if disabled {
            s.fill(s.primary_color() / 2);
        } else {
            s.fill(s.primary_color());
        }
        s.same_line(None);
        s.rect(select_box)?;

        // Arrow
        let [_, y, _, height] = select_box.as_array();
        let arrow_box = square![select_box.right() - height, y, height];
        s.no_stroke();
        if hovered || focused {
            s.fill(s.highlight_color());
        } else if disabled {
            s.fill(s.secondary_color() / 2);
        } else {
            s.fill(s.secondary_color());
        }
        s.rect(arrow_box)?;

        let third = arrow_box.width() / 3;
        let fourth = arrow_box.width() / 4;
        let [x, y, width, height] = arrow_box.as_array();
        s.no_stroke();
        if disabled {
            s.fill(WHITE / 2);
        } else {
            s.fill(WHITE);
        }
        s.triangle([
            point![x + fourth, y + third + 1],
            point![(x + width) - fourth, y + third + 1],
            point![x + width / 2, (y + height) - third - 2],
        ])?;
        s.pop();

        // Item
        s.no_wrap();
        s.set_cursor_pos([select_box.x() + ipad.x(), select_box.y() + ipad.y()]);
        s.text(&items[*selected])?;
        s.ui.pop_cursor();

        s.pop();
        s.advance_cursor(rect![
            pos,
            select_box.right() - pos.x(),
            select_box.height()
        ]);

        // Process input
        if focused {
            // Pop select list
            let mut width = select_box.width() + 1;
            let height = displayed_count as i32 * (font_size + 2 * ipad.y()) + 1;
            let total_height = items.len() as i32 * (font_size + 2 * ipad.y());
            let mut src = rect![0, 0, width, height];
            let dst = rect![select_box.left(), select_box.bottom() + 1, width, height];
            let texture_id = s.get_or_create_texture(id, src, dst)?;

            if total_height < height {
                width += SCROLL_SIZE + 1;
                src.offset_width(-(SCROLL_SIZE));
            }

            s.ui.set_mouse_offset(select_box.bottom_left());
            s.with_texture(texture_id, |s: &mut PixState| {
                s.set_cursor_pos([0, 0]);
                // Extend width to account for border
                s.next_width(width as u32 + 2);
                s.select_list(format!("#{}", label), selected, items, displayed_count)?;
                Ok(())
            })?;
            s.ui.clear_mouse_offset();

            if let Some(Key::Escape) = s.ui.key_entered() {
                s.ui.blur();
            }
        }
        s.ui.handle_events(id);

        Ok(())
    }

    /// Draw a select list to the current canvas with a scrollable region.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { select_box: usize };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     let items = ["Item 1", "Item 2", "Item 3"];
    ///     let displayed_count = 4;
    ///     s.select_list("Select Box", &mut self.select_box, &items, displayed_count)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn select_list<S, I>(
        &mut self,
        label: S,
        selected: &mut usize,
        items: &[I],
        displayed_count: usize,
    ) -> PixResult<()>
    where
        S: AsRef<str>,
        I: AsRef<str>,
    {
        let label = label.as_ref();

        let s = self;
        let id = s.ui.get_id(&label);
        let label = label.split('#').next().unwrap_or("");
        let pos = s.cursor_pos();
        let font_size = s.theme.font_sizes.body as i32;
        let style = s.theme.style;
        let fpad = style.frame_pad;
        let ipad = style.item_pad;

        // Calculate rect
        let width =
            s.ui.next_width
                .take()
                .unwrap_or_else(|| s.width().unwrap_or(100));
        let mut select_box = rect![
            pos,
            width as i32 - 2 * fpad.x(),
            displayed_count as i32 * (font_size + 2 * ipad.y())
        ];
        if !label.is_empty() {
            let (_, h) = s.size_of(label)?;
            let offset = h as i32 + ipad.y();
            select_box.offset_y(offset);
        }

        // Calculate displayed items
        let line_height = font_size + ipad.y() * 2;
        let scroll = s.ui.scroll(id);
        let skip_count = (scroll.y() / line_height) as usize;
        let displayed_items = items
            .iter()
            .enumerate()
            .skip(skip_count)
            .take(displayed_count + 1); // Display extra items for scrolling overflow

        // Calculate scrollbars
        let total_height = items.len() as i32 * line_height;
        let total_width = items.iter().fold(0, |max_width, item| {
            let (w, _) = s.size_of(item).unwrap_or((0, 0));
            cmp::max(w as i32, max_width)
        }) + 2 * fpad.x();

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, select_box);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;
        let active = s.ui.is_active(id);

        s.push();
        s.ui.push_cursor();

        // Render
        s.rect_mode(RectMode::Corner);

        // Label
        s.text(label)?;

        // Select List
        s.push();
        if focused {
            s.stroke(s.highlight_color());
        } else {
            s.stroke(s.muted_color());
        }
        if disabled {
            s.fill(s.primary_color() / 2);
        } else {
            s.fill(s.primary_color());
        }
        s.rect(select_box)?;
        s.pop();

        // Items
        let mpos = s.mouse_pos();

        let mut clip = select_box;
        clip.offset_size([-1, -1]);
        s.clip(clip)?;

        let x = select_box.x() - scroll.x();
        let mut y = select_box.y() - scroll.y() + (skip_count as i32 * line_height);
        for (i, item) in displayed_items {
            let item_rect = rect!(select_box.x(), y, select_box.width(), line_height);
            let clickable =
                item_rect.bottom() > select_box.y() || item_rect.top() < select_box.height();
            s.push();
            if hovered && clickable && item_rect.contains_point(mpos) {
                s.frame_cursor(Cursor::hand())?;
                s.no_stroke();
                s.fill(s.highlight_color());
                s.rect([select_box.x(), y, select_box.width(), line_height])?;
                if active && s.mouse_down(Mouse::Left) {
                    *selected = i;
                }
            }
            if *selected == i {
                s.no_stroke();
                s.fill(s.secondary_color());
                s.rect([select_box.x(), y, select_box.width(), line_height])?;
            }
            s.pop();
            s.set_cursor_pos([x + ipad.x(), y + ipad.y()]);
            s.text(item)?;
            y += line_height;
        }

        s.no_clip()?;
        s.ui.pop_cursor();
        s.pop();

        // Process input
        if focused {
            if let Some(key) = s.ui.key_entered() {
                let new_selected = match key {
                    Key::Up => selected.saturating_sub(1),
                    Key::Down => cmp::min(items.len() - 1, selected.saturating_add(1)),
                    _ => *selected,
                };
                if *selected != new_selected {
                    s.ui.clear_entered();
                    *selected = new_selected;
                    let sel_y = *selected as i32 * line_height;
                    let mut new_scroll = scroll;
                    if sel_y < scroll.y() {
                        // Snap scroll to top of the window
                        new_scroll.set_y(sel_y);
                    } else if sel_y + line_height > scroll.y() + select_box.height() {
                        // Snap scroll to bottom of the window
                        new_scroll.set_y(sel_y - (select_box.height() - line_height));
                    }
                    if new_scroll != scroll {
                        s.ui.set_scroll(id, new_scroll);
                    }
                }
            }
        }
        s.ui.handle_events(id);

        // Scrollbars
        let rect = s.scroll(id, select_box, total_width, total_height)?;
        s.advance_cursor(rect![pos, rect.width(), rect.bottom() - pos.y()]);

        Ok(())
    }
}
