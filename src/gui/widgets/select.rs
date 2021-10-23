//! Select UI widgets.

use crate::{
    gui::{state::Texture, Direction},
    prelude::*,
};
use std::cmp;

const SCROLL_SIZE: i32 = 12;

impl PixState {
    /// Draw a select box the current canvas.
    pub fn select_box<S, I>(&mut self, label: S, selected: &mut usize, items: &[I]) -> PixResult<()>
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
        if focused {
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
        s.same_line(None);
        s.rect(select_box)?;

        // Arrow
        let [_, y, _, height] = select_box.values();
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
        let [x, y, width, height] = arrow_box.values();
        s.no_stroke();
        s.fill(WHITE);
        s.triangle([
            [x + fourth, y + third + 1],
            [(x + width) - fourth, y + third + 1],
            [x + width / 2, (y + height) - third - 2],
        ])?;

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
            if !s.ui.textures.contains_key(&id) {
                let height = 4 * (font_size + 2 * ipad.y()) + 1;
                let texture_id = s.create_texture(
                    select_box.width() as u32 + 2 * fpad.x() as u32,
                    height as u32,
                    PixelFormat::Rgba,
                )?;
                let src = Some(rect![0, 0, select_box.width(), height]);
                let dst = Some(rect![select_box.bottom_left(), select_box.width(), height]);
                s.ui.textures.insert(id, Texture::new(texture_id, src, dst));
            }
            let texture_id = {
                // SAFETY: We just checked or inserted a texture.
                let texture = s.ui.textures.get_mut(&id).expect("valid texture target");
                texture.visible = true;
                texture.id
            };

            s.ui.set_mouse_offset(select_box.bottom_left());
            s.with_texture(texture_id, |s: &mut PixState| {
                s.select_list(format!("#{}", label), selected, items, 4)?;
                Ok(())
            })?;
            s.ui.clear_mouse_offset();
        }
        s.ui.handle_input(id);

        Ok(())
    }

    /// Draw a select list to the current canvas with a scrollable region.
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
        let mut scroll = s.ui.scroll(id);
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
        });

        let vertical_scroll = total_height > select_box.height();
        if vertical_scroll {
            select_box.offset_width(-SCROLL_SIZE);
        }
        let horizontal_scroll = total_width > select_box.width();

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, select_box);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;
        let active = s.ui.is_active(id);

        s.push();

        // Render
        s.rect_mode(RectMode::Corner);

        // Label
        s.text(label)?;

        // Select List
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

        // Items
        let mpos = s.mouse_pos();

        s.ui.push_cursor();
        let mut clip = select_box;
        clip.offset_size([-1, -1]);
        s.clip(clip)?;

        let x = select_box.x() - scroll.x();
        let mut y = select_box.y() - scroll.y() + (skip_count as i32 * line_height);
        for (i, item) in displayed_items {
            let item_rect = rect!(select_box.x(), y, select_box.width(), line_height);
            let clickable =
                item_rect.bottom() > select_box.y() || item_rect.top() < select_box.height();
            if hovered && clickable && item_rect.contains_point(mpos) {
                s.frame_cursor(&Cursor::hand())?;
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
            if disabled {
                s.fill(s.text_color() / 2);
            } else {
                s.fill(s.text_color());
            }
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
                    *selected = new_selected;
                    let sel_y = *selected as i32 * line_height;
                    // Snap scroll to top of the window
                    if sel_y < scroll.y() {
                        scroll.set_y(sel_y);
                        s.ui.set_scroll(id, scroll);
                    } else if sel_y + line_height > scroll.y() + select_box.height() {
                        // Snap scroll to bottom of the window
                        scroll.set_y(sel_y - (select_box.height() - line_height));
                        s.ui.set_scroll(id, scroll);
                    }
                }
            }
        }
        s.ui.handle_input(id);

        // Process mouse wheel
        let ymax = total_height - select_box.height();
        let xmax = total_width - select_box.width() - SCROLL_SIZE;
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
        if vertical_scroll {
            let mut scroll_y = scroll.y();
            let scrolled = s.scroll(
                rect![
                    select_box.right() + 1,
                    select_box.top(),
                    SCROLL_SIZE,
                    select_box.height(),
                ],
                ymax as u32,
                &mut scroll_y,
                Direction::Vertical,
            )?;
            if scrolled {
                scroll.set_y(scroll_y);
                s.ui.set_scroll(id, scroll);
            }
            select_box.offset_width(SCROLL_SIZE);
        }
        s.advance_cursor(select_box);

        if horizontal_scroll {
            let mut scroll_x = scroll.x();
            let scrolled = s.scroll(
                rect![
                    select_box.left(),
                    select_box.bottom() + 1,
                    select_box.width(),
                    SCROLL_SIZE,
                ],
                xmax as u32,
                &mut scroll_x,
                Direction::Horizontal,
            )?;
            if scrolled {
                scroll.set_x(scroll_x);
                s.ui.set_scroll(id, scroll);
            }

            s.advance_cursor(rect![s.cursor_pos(), select_box.width(), SCROLL_SIZE]);
        }

        Ok(())
    }
}
