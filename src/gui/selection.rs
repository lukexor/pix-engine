use super::{
    element::{Text, TEXT_DEFAULT_HEIGHT},
    Drawable, StateData,
};
use crate::{
    event::{Key, PixEvent},
    image::{Image, ImageRef},
    pixel::{self, Pixel},
};

const DEFAULT_LINE_HEIGHT: u32 = 10;

pub struct ListSelection {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    dirty: bool,
    selected: usize,
    rows: usize,
    scroll: usize,
    line_height: u32,
    image: ImageRef,
    color: Pixel,
    items: Vec<Text>,
}

impl ListSelection {
    pub fn new() -> Self {
        Self::with_items(0, 0, 100, 100, Vec::new())
    }

    pub fn with_items(x: u32, y: u32, width: u32, height: u32, mut items: Vec<Text>) -> Self {
        let rows = if items.is_empty() {
            0
        } else {
            std::cmp::min(
                items.len(),
                (height / (items[0].height() + DEFAULT_LINE_HEIGHT)) as usize,
            )
        };
        for item in items.iter_mut() {
            item.set_x(x);
            item.set_y(y);
        }
        Self {
            x,
            y,
            width,
            height,
            dirty: true,
            selected: 0,
            rows,
            scroll: 0,
            line_height: DEFAULT_LINE_HEIGHT,
            image: Image::new_ref(width, height),
            color: pixel::DARK_GRAY,
            items,
        }
    }
}

impl Drawable for ListSelection {
    fn update(&mut self, data: &mut StateData) {
        data.events.retain(|&event| match event {
            PixEvent::KeyPress(Key::Down, pressed, ..) => {
                if pressed && self.selected < self.items.len() - 1 {
                    self.selected += 1;
                    self.dirty = true;
                    if self.selected >= self.scroll && self.selected - self.scroll >= self.rows {
                        self.scroll += 1;
                    }
                }
                false
            }
            PixEvent::KeyPress(Key::Up, pressed, ..) => {
                if pressed && self.selected > 0 {
                    self.selected -= 1;
                    self.dirty = true;
                }
                false
            }
            _ => true,
        });
    }

    fn draw(&mut self, data: &mut StateData) {
        if self.dirty {
            let mut y = self.y;
            for (index, mut item) in self.items[self.scroll..self.rows].iter_mut().enumerate() {
                if index == self.selected {
                    item.set_color(pixel::BLUE);
                } else {
                    item.set_color(pixel::WHITE);
                }
                item.set_y(y);
                item.draw(data);
                y += item.height() + self.line_height;
            }
            self.dirty = false;
        }
    }
}

impl Default for ListSelection {
    fn default() -> Self {
        Self::new()
    }
}
