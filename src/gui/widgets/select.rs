//! Select widget rendering methods.
//!
//! Provided [`PixState`] methods:
//!
//! - [`PixState::select_box`]
//! - [`PixState::select_list`]
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App { selected_box: usize, selected_list: usize };
//! # impl PixEngine for App {
//! fn on_update(&mut self, s: &mut PixState) -> Result<()> {
//!     let items = ["Item 1", "Item 2", "Item 3"];
//!     let displayed_count = 4;
//!     s.select_box("Select Box", &mut self.selected_box, &items, displayed_count)?;
//!     s.select_list("Select List", &mut self.selected_list, &items, displayed_count)?;
//!     Ok(())
//! }
//! # }
//! ```

use crate::{
    error::Result,
    gui::{state::ElementId, Direction},
    ops::clamp_size,
    prelude::*,
};
use std::cmp;

/// The maximum number of select elements that can be displayed at once.
pub const MAX_DISPLAYED: usize = 100;
const SELECT_POP_LABEL: &str = "##select_pop";

impl PixState {
    /// Draw a select box the current canvas that returns `true` when selection is changed.
    ///
    /// Maximum displayed count of 100.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, or if `displayed_count` is
    /// greater than [`MAX_DISPLAYED`] then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { select_box: usize };
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     let items = ["Item 1", "Item 2", "Item 3"];
    ///     let displayed_count = 4;
    ///     if s.select_box("Select Box", &mut self.select_box, &items, displayed_count)? {
    ///         // selection changed
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn select_box<S, I>(
        &mut self,
        label: S,
        selected: &mut usize,
        items: &[I],
        mut displayed_count: usize,
    ) -> Result<bool>
    where
        S: AsRef<str>,
        I: AsRef<str>,
    {
        let label = label.as_ref();

        if displayed_count > MAX_DISPLAYED {
            displayed_count = MAX_DISPLAYED;
        }
        if *selected >= items.len() {
            *selected = items.len() - 1;
        }

        let s = self;
        let id = s.ui.get_id(&label);
        let label = s.ui.get_label(label);
        let pos = s.cursor_pos();
        let font_size = clamp_size(s.theme.font_size);
        let spacing = s.theme.spacing;
        let fpad = spacing.frame_pad;
        let ipad = spacing.item_pad;

        // Calculate rect
        let (item_width, item_height) = s.text_size(items.get(0).map_or("", AsRef::as_ref))?;
        let width = s.ui.next_width.take().unwrap_or(item_width);
        let (label_width, label_height) = s.text_size(label)?;
        let [mut x, y] = pos.coords();
        if !label.is_empty() {
            x += label_width + ipad.x();
        }
        let select_box = rect![x, y, width, item_height].offset_size(2 * fpad);

        // Check hover/active/keyboard focus
        let hovered = s.focused() && s.ui.try_hover(id, &select_box);
        let focused = s.focused() && s.ui.try_focus(id);

        s.push();
        s.ui.push_cursor();

        // Label
        if !label.is_empty() {
            s.set_cursor_pos([
                pos.x(),
                pos.y() + select_box.height() / 2 - label_height / 2,
            ]);
            s.text(label)?;
        }

        // Select Box
        s.rect_mode(RectMode::Corner);
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
        }
        let [stroke, bg, fg] = s.widget_colors(id, ColorType::Background);
        s.stroke(stroke);
        s.fill(bg);
        s.rect(select_box)?;

        // Arrow
        let arrow_width = font_size + 2 * fpad.y();
        let arrow_x = cmp::max(select_box.left(), select_box.right() - arrow_width);

        let [_, select_y, _, select_height] = select_box.coords();
        let arrow_box = rect![arrow_x, select_y, arrow_width, select_height];
        s.rect(arrow_box)?;

        if arrow_x + arrow_width - fpad.x() <= select_box.right() {
            s.stroke(None);
            s.fill(fg);
            s.clip(arrow_box)?;
            s.arrow(
                [
                    arrow_x + fpad.y(),
                    select_y + arrow_box.height() / 2 - arrow_width / 4,
                ],
                Direction::Down,
                f64::from(fpad.y()) / 8.0,
            )?;
        }

        // Item
        s.clip(rect![
            select_box.top_left(),
            select_box.width() - arrow_box.width(),
            select_box.height()
        ])?;

        s.wrap(None);
        s.set_cursor_pos(select_box.top_left() + fpad);
        s.stroke(None);
        s.fill(fg);
        s.text(&items[*selected])?;

        s.clip(None)?;
        s.ui.pop_cursor();
        s.pop();
        s.advance_cursor([select_box.right() - pos.x(), select_box.height()]);

        let line_height = font_size + 2 * ipad.y();
        let expanded_list = rect![
            select_box.left(),
            select_box.bottom() + 1,
            select_box.width(),
            displayed_count as i32 * line_height + 2 * fpad.y(),
        ];
        let original_selected = *selected;
        s.select_list_popup(id, selected, items, displayed_count, expanded_list)?;

        // Process input
        s.push_id(id);
        let list_id = s.ui.get_id(&SELECT_POP_LABEL);
        let scroll = s.ui.scroll(list_id);
        s.pop_id();
        let expanded = s.ui.expanded(id);
        if focused {
            s.ui.set_expanded(id, true);
            if let Some(key) = s.ui.key_entered() {
                if let Key::Escape | Key::Return = key {
                    s.ui.set_expanded(id, !expanded);
                    s.ui.clear_entered();
                } else {
                    let new_selected = match key {
                        Key::Up => Some(selected.saturating_sub(1)),
                        Key::Down => Some(cmp::min(items.len() - 1, selected.saturating_add(1))),
                        _ => None,
                    };
                    if let Some(selection) = new_selected {
                        *selected = selection;
                        s.ui.clear_entered();
                        let sel_y = *selected as i32 * line_height;
                        let mut new_scroll = scroll;
                        let height = expanded_list.height();
                        if sel_y < scroll.y() {
                            // Snap scroll to top of the window
                            new_scroll.set_y(sel_y);
                        } else if sel_y + line_height > scroll.y() + height {
                            // Snap scroll to bottom of the window
                            new_scroll
                                .set_y((sel_y + line_height) - (height - font_size - ipad.y()));
                        }
                        if new_scroll != scroll {
                            s.ui.set_scroll(list_id, new_scroll);
                        }
                    }
                }
            }
        }
        let clicked_outside = s.mouse_down(Mouse::Left)
            && !select_box.contains(s.mouse_pos())
            && !expanded_list.contains(s.mouse_pos());
        if (expanded && clicked_outside) || (!focused && !s.mouse_down(Mouse::Left)) {
            s.ui.set_expanded(id, false);
        }

        s.ui.handle_focus(id);

        Ok(original_selected != *selected)
    }

    /// Draw a select list to the current canvas with a scrollable region that returns `true` when
    /// selection is changed.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, or if `displayed_count` is
    /// greater than [`MAX_DISPLAYED`] then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { select_list: usize };
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     let items = ["Item 1", "Item 2", "Item 3"];
    ///     let displayed_count = 4;
    ///     if s.select_list("Select List", &mut self.select_list, &items, displayed_count)? {
    ///         // Selection  changed
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn select_list<S, I>(
        &mut self,
        label: S,
        selected: &mut usize,
        items: &[I],
        mut displayed_count: usize,
    ) -> Result<bool>
    where
        S: AsRef<str>,
        I: AsRef<str>,
    {
        let label = label.as_ref();

        if displayed_count > MAX_DISPLAYED {
            displayed_count = MAX_DISPLAYED;
        }
        if *selected >= items.len() {
            *selected = items.len() - 1;
        }

        let s = self;
        let id = s.ui.get_id(&label);
        let label = s.ui.get_label(label);
        let pos = s.cursor_pos();
        let font_size = clamp_size(s.theme.font_size);
        let spacing = s.theme.spacing;
        let fpad = spacing.frame_pad;
        let ipad = spacing.item_pad;

        // Calculate rect
        let (label_width, label_height) = s.text_size(label)?;
        let width = s.ui.next_width.take().unwrap_or(label_width);
        let [x, mut y] = pos.coords();
        if !label.is_empty() {
            y += label_height + ipad.y();
        }
        let line_height = font_size + 2 * ipad.y();
        let select_list = rect![
            x,
            y,
            width,
            displayed_count as i32 * line_height + 2 * fpad.y() + 2
        ];

        // Check hover/active/keyboard focus
        let focused = s.focused() && s.ui.try_focus(id);

        s.push();
        s.ui.push_cursor();

        // Select List
        s.rect_mode(RectMode::Corner);
        s.text(label)?;

        let original_selected = *selected;
        s.select_list_items(id, selected, items, displayed_count, select_list)?;

        s.ui.pop_cursor();
        s.pop();

        // Process input
        let scroll = s.ui.scroll(id);
        let line_height = font_size + 2 * ipad.y();
        if focused {
            if let Some(key) = s.ui.key_entered() {
                let new_selected = match key {
                    Key::Up => Some(selected.saturating_sub(1)),
                    Key::Down => Some(cmp::min(items.len() - 1, selected.saturating_add(1))),
                    _ => None,
                };
                if let Some(selection) = new_selected {
                    *selected = selection;
                    s.ui.clear_entered();
                    let sel_y = *selected as i32 * line_height;
                    let mut new_scroll = scroll;
                    let height = select_list.height();
                    if sel_y < scroll.y() {
                        // Snap scroll to top of the window
                        new_scroll.set_y(sel_y);
                    } else if sel_y + line_height > scroll.y() + height {
                        // Snap scroll to bottom of the window
                        new_scroll.set_y((sel_y + line_height) - (height - font_size - ipad.y()));
                    }
                    if new_scroll != scroll {
                        s.ui.set_scroll(id, new_scroll);
                    }
                }
            }
        }
        s.ui.handle_focus(id);

        // Scrollbars
        let total_height = items.len() as i32 * line_height + 2;
        let total_width = items.iter().fold(0, |max_width, item| {
            let (w, _) = s.text_size(item.as_ref()).unwrap_or((0, 0));
            cmp::max(w, max_width)
        });

        let rect = s.scroll(
            id,
            select_list,
            total_width + 2 * fpad.x(),
            total_height + 2 * fpad.y(),
        )?;
        s.advance_cursor([rect.width().max(label_width), rect.bottom() - pos.y()]);

        Ok(original_selected != *selected)
    }
}

impl PixState {
    #[inline]
    fn select_list_popup<I>(
        &mut self,
        id: ElementId,
        selected: &mut usize,
        items: &[I],
        displayed_count: usize,
        size: Rect<i32>,
    ) -> Result<bool>
    where
        I: AsRef<str>,
    {
        let s = self;
        let font_size = clamp_size(s.theme.font_size);
        let spacing = s.theme.spacing;
        let fpad = spacing.frame_pad;
        let ipad = spacing.item_pad;

        let line_height = font_size + 2 * ipad.y();
        let height = displayed_count as i32 * line_height + 2 * fpad.y();

        let expanded = s.ui.expanded(id);
        if expanded {
            // Pop select list
            let total_height = items.len() as i32 * line_height + 2 * fpad.y();
            let texture_id = s.get_or_create_texture(id, None, size)?;

            s.ui.offset_mouse(size.top_left());

            s.set_texture_target(texture_id)?;
            s.clear()?;
            s.set_cursor_pos([0, 0]);
            if total_height > height {
                s.next_width((size.width() - spacing.scroll_size) as u32);
            } else {
                s.next_width(size.width() as u32);
            }
            s.ui.disable_focus();
            s.push_id(id);
            let changed = s.select_list(SELECT_POP_LABEL, selected, items, displayed_count)?;
            s.pop_id();
            s.ui.enable_focus();
            s.clear_texture_target();

            s.ui.clear_mouse_offset();
            if changed {
                s.ui.set_expanded(id, false);
            }
            Ok(changed)
        } else {
            Ok(false)
        }
    }

    #[inline]
    fn select_list_items<I>(
        &mut self,
        id: ElementId,
        selected: &mut usize,
        items: &[I],
        displayed_count: usize,
        select_list: Rect<i32>,
    ) -> Result<()>
    where
        I: AsRef<str>,
    {
        let s = self;
        let font_size = clamp_size(s.theme.font_size);
        let spacing = s.theme.spacing;
        let colors = s.theme.colors;
        let fpad = spacing.frame_pad;
        let ipad = spacing.item_pad;

        // Check hover/active/keyboard focus
        let hovered = s.focused() && s.ui.try_hover(id, &select_list);
        let active = s.ui.is_active(id);
        let disabled = s.ui.disabled;

        let [stroke, bg, fg] = s.widget_colors(id, ColorType::Background);
        s.stroke(stroke);
        s.fill(colors.background);
        s.rect(select_list)?;

        // Items
        let mpos = s.mouse_pos();

        let border_clip = select_list.shrink([1, 1]);
        s.clip(border_clip)?;
        let content_clip = border_clip.shrink(fpad);
        let item_clip = rect![
            select_list.x() + 1,
            content_clip.y(),
            select_list.width() - 2,
            content_clip.height(),
        ];

        let scroll = s.ui.scroll(id);
        let line_height = font_size + ipad.y() * 2;
        let skip_count = (scroll.y() / line_height) as usize;
        let displayed_items = items
            .iter()
            .enumerate()
            .skip(skip_count)
            .take(displayed_count + 1); // Display extra items for scrolling overflow

        let x = select_list.x() + fpad.x() - scroll.x();
        let mut y = content_clip.y() - scroll.y() + (skip_count as i32 * line_height);
        for (i, item) in displayed_items {
            let item_rect = rect!(select_list.x(), y, select_list.width(), line_height);
            let clickable =
                item_rect.bottom() > content_clip.y() || item_rect.top() < select_list.height();
            s.push();
            s.clip(item_clip)?;
            if hovered && clickable && item_rect.contains(mpos) {
                s.frame_cursor(&Cursor::hand())?;
                s.stroke(None);
                s.fill(bg);
                s.rect([item_clip.x(), y, item_clip.width(), line_height])?;
                if active && s.mouse_clicked(Mouse::Left) {
                    *selected = i;
                }
            }
            if *selected == i {
                s.stroke(None);
                if disabled {
                    s.fill(colors.primary.blended(colors.background, 0.38));
                } else {
                    s.fill(colors.primary);
                }
                s.rect([item_clip.x(), y, item_clip.width(), line_height])?;
            }
            s.pop();
            s.clip(content_clip)?;
            s.set_cursor_pos([x, y + ipad.y()]);
            s.stroke(None);
            if *selected == i {
                s.fill(colors.on_primary);
            } else {
                s.fill(fg);
            }
            s.text(item)?;
            s.clip(border_clip)?;
            y += line_height;
        }

        s.clip(None)?;

        Ok(())
    }
}
