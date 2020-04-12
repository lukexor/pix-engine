use super::{element::Text, Drawable, StateData};
use crate::{
    draw::Rect,
    event::{Key, PixEvent},
    pixel::Pixel,
    PixEngineResult,
};

const DEFAULT_LINE_HEIGHT: u32 = 10;

#[derive(Debug, Clone)]
pub struct ListSelection {
    rect: Rect,
    dirty: bool,
    selected: usize,
    scroll: usize,
    scroll_padding: usize,
    rows: usize,
    line_height: u32,
    bg_color: Pixel,
    font_color: Pixel,
    highlight_color: Pixel,
    items: Vec<Text>,
}

impl ListSelection {
    pub fn new() -> Self {
        Self::with_items(Rect::new(0, 0, 100, 100), &Vec::new())
    }

    pub fn with_items(rect: Rect, items: &Vec<Text>) -> Self {
        let mut items = items.to_owned();
        let rows = if items.is_empty() {
            0
        } else {
            std::cmp::min(
                items.len(),
                (rect.h / (items[0].height() + DEFAULT_LINE_HEIGHT)) as usize,
            )
        };
        for item in items.iter_mut() {
            item.set_x(rect.x + 10);
            item.set_y(rect.y);
        }
        Self {
            rect,
            dirty: true,
            selected: 0,
            scroll: 0,
            scroll_padding: 3,
            rows,
            line_height: DEFAULT_LINE_HEIGHT,
            bg_color: Pixel::dark_gray(),
            font_color: Pixel::white(),
            highlight_color: Pixel::blue(),
            items,
        }
    }

    pub fn selected(&self) -> usize {
        self.selected + self.scroll
    }
}

impl Drawable for ListSelection {
    fn update(&mut self, data: &mut StateData) {
        data.events.retain(|&event| match event {
            PixEvent::KeyPress(Key::Down, pressed, ..) => {
                if pressed && (self.selected + self.scroll) < self.items.len() - 1 {
                    if self.selected == self.rows - self.scroll_padding
                        && (self.selected + self.scroll) < self.items.len() - self.scroll_padding
                    {
                        self.scroll += 1;
                    } else if self.selected < self.items.len() - 1 {
                        self.selected += 1;
                    }
                }
                false
            }
            PixEvent::KeyPress(Key::Up, pressed, ..) => {
                if pressed && (self.selected + self.scroll) > 0 {
                    if self.selected == self.scroll_padding && self.scroll > 0 {
                        self.scroll -= 1;
                    } else {
                        self.selected -= 1;
                    }
                }
                false
            }
            _ => true,
        });
        self.draw(data);
    }

    fn draw(&mut self, data: &mut StateData) -> PixEngineResult<()> {
        let orig_color = data.get_draw_color();
        data.set_draw_color(self.bg_color)?;
        data.set_viewport(Some(self.rect))?;
        data.fill_rect(self.rect)?;
        let mut y = self.rect.y + self.line_height;
        let max = std::cmp::min(self.scroll + self.rows, self.items.len());
        for (index, item) in self.items[self.scroll..max].iter_mut().enumerate() {
            if index == self.selected {
                item.set_color(self.highlight_color);
            } else {
                item.set_color(self.font_color);
            }
            item.set_y(y);
            item.draw(data)?;
            y += item.height() + self.line_height;
        }
        data.set_viewport(None)?;
        data.set_draw_color(orig_color)?;
        Ok(())
    }
}

impl Default for ListSelection {
    fn default() -> Self {
        Self::new()
    }
}
