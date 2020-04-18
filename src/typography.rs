//! Typography creation, manipulation, and drawing routines.

use bitflags::bitflags;
use std::path::PathBuf;

/// The default leading for `Font`.
pub const DEFAULT_TEXT_LEADING: u32 = 15;
/// The default size for `Font`.
pub const DEFAULT_TEXT_SIZE: u32 = 12;

/// Represents a given Font setting which includes font-family, style, size, etc.
#[derive(Default, Debug, Clone)]
pub struct Font {
    pub(crate) size: u32,
    pub(crate) leading: u32,
    pub(crate) align: TextAlign,
    pub(crate) style: TextStyle,
    pub(crate) family: FontFamily,
}

/// Represents a specfic font-family such as Arial, "Times New Roman", or a custom
/// family loaded from a `.ttf` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FontFamily {
    Arial,
    Custom(String, PathBuf), // Family name, `.ttf` path
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

impl Default for FontFamily {
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
