use std::{borrow::Cow, path::PathBuf};

pub const DEFAULT_TEXT_LEADING: u32 = 15;
pub const DEFAULT_TEXT_SIZE: u32 = 12;

/// Represents a specfic font-family
pub enum Font {
    Arial,
    Custom(Cow<'static, str>, PathBuf),
}

/// The horizontal alignment for drawing text. Default is Left.
pub enum TextAlignHori {
    Left,
    Center,
    Right,
}

/// The vertical alignment for drawing text. Default is Top.
pub enum TextAlignVert {
    Top,
    Center,
    Bottom,
    Baseline,
}

/// The text style for drawing text. The default is Normal.
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

impl Default for TextAlignHori {
    fn default() -> Self {
        Self::Left
    }
}

impl Default for TextAlignVert {
    fn default() -> Self {
        Self::Top
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self::Normal
    }
}
