use super::{selection::ListSelection, Drawable, StateData};
use crate::{pixel::Pixel, PixEngineResult};
use std::fmt;

pub const TEXT_DEFAULT_SIZE: u32 = 2;
pub const TEXT_DEFAULT_HEIGHT: u32 = TEXT_DEFAULT_SIZE * 8;

#[derive(Debug, Clone)]
pub enum Element {
    ListSelection(ListSelection),
    Button(Button),
    Checkbox(Checkbox),
    RadioButton(RadioButton),
    Text(Text),
    TextInput(TextInput),
}

#[derive(Debug, Clone)]
pub struct Button {
    id: u32,
    text: String,
}

#[derive(Debug, Clone)]
pub struct Checkbox {
    id: u32,
    label: String,
    checked: bool,
}

#[derive(Debug, Clone)]
pub struct RadioButton {
    id: u32,
    selected: usize,
    options: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Text {
    x: u32,
    y: u32,
    text: String,
    size: u32,
    color: Pixel,
}

#[derive(Debug, Clone)]
pub struct TextInput {
    value: String,
    width: u32,
    has_focus: bool,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self {
            x: 0,
            y: 0,
            text: text.to_owned(),
            size: TEXT_DEFAULT_SIZE,
            color: Pixel::white(),
        }
    }

    pub fn x(&self) -> u32 {
        self.x
    }
    pub fn y(&self) -> u32 {
        self.y
    }
    pub fn width(&self) -> u32 {
        self.size * self.text.len() as u32 * 8
    }
    pub fn height(&self) -> u32 {
        self.size * 8
    }

    pub fn set_x(&mut self, x: u32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: u32) {
        self.y = y;
    }
    pub fn set_color(&mut self, color: Pixel) {
        self.color = color;
    }
}

impl Drawable for Text {
    fn draw(&mut self, data: &mut StateData) -> PixEngineResult<()> {
        let orig_scale = data.get_draw_scale();
        data.set_draw_scale(self.size);
        data.set_draw_color(self.color)?;
        data.draw_string(self.x, self.y, &self.text);
        data.set_draw_scale(orig_scale);
        Ok(())
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}
