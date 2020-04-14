// use std::path::PathBuf;
use bitflags::bitflags;

pub const DEFAULT_TEXT_LEADING: u32 = 15;
pub const DEFAULT_TEXT_SIZE: u32 = 12;

/// Represents a specfic font-family
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Font {
    Arial,
    // Custom(String, PathBuf),
}

bitflags! {
    /// The horizontal and vertical alignment for drawing text. Default is Left/Top.
    pub struct TextAlign: u16 {
        const LEFT = 0x0000;
        const CENTER = 0x0001;
        const RIGHT = 0x0002;
        const TOP = 0x0010;
        const MIDDLE = 0x0020;
        const BOTTOM = 0x0040;
        const BASELINE = 0x0080;
    }
}

/// The text style for drawing text. The default is Normal.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TextStyle {
    Normal,
    Italic,
    Bold,
    BoldItalic,
}

impl Default for Font {
    fn default() -> Self {
        Self::Arial
    }
}

impl Default for TextAlign {
    fn default() -> Self {
        Self::LEFT | Self::TOP
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self::Normal
    }
}
