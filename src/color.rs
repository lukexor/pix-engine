use std::fmt;

const RED_SHIFT: u32 = 24;
const GREEN_SHIFT: u32 = 16;
const BLUE_SHIFT: u32 = 8;
const ALPHA_SHIFT: u32 = 0;

/// ColoreMode changes the way PixEngine interprets color data. The default is Rgb.
///
/// RGB values range from 0-255 for red, green, blue, and alpha
/// HSB instead ranges from 0-360 for hue, and 0-100 for saturation and brightness
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ColorMode {
    Rgb,
    Hsb,
}

/// Represents a color (by default stored as RGBA values ranging from 0-255).  The default is
/// black.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    color_mode: ColorMode,
}

impl Color {
    /// Creates a new Rgb Color (same as calling new)
    #[inline]
    #[allow(non_snake_case)]
    pub const fn RGB(r: u8, g: u8, b: u8) -> Self {
        Self::RGBA(r, g, b, 255)
    }

    /// Creates a new Rgb Color with alpha
    #[inline]
    #[allow(non_snake_case)]
    pub const fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r,
            g,
            b,
            a,
            color_mode: ColorMode::Rgb,
        }
    }

    /// Creates a new Rgb Color from a u32
    pub fn from_u32(color: u32) -> Self {
        Self {
            r: (color >> RED_SHIFT) as u8,
            g: (color >> GREEN_SHIFT) as u8,
            b: (color >> BLUE_SHIFT) as u8,
            a: (color >> ALPHA_SHIFT) as u8,
            color_mode: ColorMode::Rgb,
        }
    }

    /// Converts a Color to a u32 representation
    pub fn to_u32(self, color: u32) -> u32 {
        (self.r as u32) << RED_SHIFT
            | (self.g as u32) << GREEN_SHIFT
            | (self.b as u32) << BLUE_SHIFT
            | (self.a as u32) << ALPHA_SHIFT
    }

    /// Returns an rgb tuple
    #[inline]
    pub const fn rgb(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    /// Returns an rgba tuple
    #[inline]
    pub const fn rgba(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    /// Get the red value of the color ranging from 0-255
    pub const fn r(self) -> u8 {
        self.r
    }
    /// Set the red value of the color ranging from 0-255
    pub fn set_r(mut self, r: u8) {
        self.r = r;
    }

    /// Get the green value of the color ranging from 0-255
    pub const fn g(self) -> u8 {
        self.g
    }
    /// Set the green value of the color ranging from 0-255
    pub fn set_g(mut self, g: u8) {
        self.g = g;
    }

    /// Get the blue value of the color ranging from 0-255
    pub const fn b(self) -> u8 {
        self.b
    }
    /// Set the blue value of the color ranging from 0-255
    pub fn set_b(mut self, b: u8) {
        self.b = b;
    }

    /// Get the alpha value of the color ranging from 0-255
    pub const fn a(self) -> u8 {
        self.a
    }
    /// Set the alpha value of the color ranging from 0-255
    pub fn set_a(mut self, a: u8) {
        self.a = a;
    }
}

impl Default for Color {
    fn default() -> Self {
        BLACK
    }
}

impl Default for ColorMode {
    fn default() -> Self {
        Self::Rgb
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Color {
        Color::RGB(r, g, b)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Color {
        Color::RGBA(r, g, b, a)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

/// Color Constants for common colors

// WHITE/BLACK/BLANK
pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
    color_mode: ColorMode::Rgb,
};
pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
    color_mode: ColorMode::Rgb,
};
pub const TRANSPARENT: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
    color_mode: ColorMode::Rgb,
};

// GRAY
pub const GRAY: Color = Color {
    r: 192,
    g: 192,
    b: 192,
    a: 255,
    color_mode: ColorMode::Rgb,
};
pub const DARK_GRAY: Color = Color {
    r: 128,
    g: 128,
    b: 128,
    a: 255,
    color_mode: ColorMode::Rgb,
};
pub const VERY_DARK_GRAY: Color = Color {
    r: 64,
    g: 64,
    b: 64,
    a: 255,
    color_mode: ColorMode::Rgb,
};

// RED
pub const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
    color_mode: ColorMode::Rgb,
};
pub const DARK_RED: Color = Color {
    r: 128,
    g: 0,
    b: 0,
    a: 255,
    color_mode: ColorMode::Rgb,
};
pub const VERY_DARK_RED: Color = Color {
    r: 64,
    g: 0,
    b: 0,
    a: 255,
    color_mode: ColorMode::Rgb,
};

// ORANGE
pub const ORANGE: Color = Color {
    r: 255,
    g: 128,
    b: 0,
    a: 255,
    color_mode: ColorMode::Rgb,
};

// YELLOW
pub const YELLOW: Color = Color {
    r: 255,
    g: 255,
    b: 0,
    a: 255,
    color_mode: ColorMode::Rgb,
};
pub const DARK_YELLOW: Color = Color {
    r: 128,
    g: 128,
    b: 0,
    a: 255,
    color_mode: ColorMode::Rgb,
};
pub const VERY_DARK_YELLOW: Color = Color {
    r: 64,
    g: 64,
    b: 0,
    a: 255,
    color_mode: ColorMode::Rgb,
};

// GREEN
pub const GREEN: Color = Color {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
    color_mode: ColorMode::Rgb,
};
pub const DARK_GREEN: Color = Color {
    r: 0,
    g: 128,
    b: 0,
    a: 255,
    color_mode: ColorMode::Rgb,
};
pub const VERY_DARK_GREEN: Color = Color {
    r: 0,
    g: 64,
    b: 0,
    a: 255,
    color_mode: ColorMode::Rgb,
};

// CYAN
pub const CYAN: Color = Color {
    r: 0,
    g: 255,
    b: 255,
    a: 255,
    color_mode: ColorMode::Rgb,
};
pub const DARK_CYAN: Color = Color {
    r: 0,
    g: 128,
    b: 128,
    a: 255,
    color_mode: ColorMode::Rgb,
};
pub const VERY_DARK_CYAN: Color = Color {
    r: 0,
    g: 64,
    b: 64,
    a: 255,
    color_mode: ColorMode::Rgb,
};

// BLUE
pub const BLUE: Color = Color {
    r: 0,
    g: 255,
    b: 255,
    a: 255,
    color_mode: ColorMode::Rgb,
};
pub const DARK_BLUE: Color = Color {
    r: 0,
    g: 0,
    b: 128,
    a: 255,
    color_mode: ColorMode::Rgb,
};
pub const VERY_DARK_BLUE: Color = Color {
    r: 0,
    g: 0,
    b: 64,
    a: 255,
    color_mode: ColorMode::Rgb,
};

// MAGENTA
pub const MAGENTA: Color = Color {
    r: 255,
    g: 0,
    b: 255,
    a: 255,
    color_mode: ColorMode::Rgb,
};
pub const DARK_MAGENTA: Color = Color {
    r: 128,
    g: 0,
    b: 128,
    a: 255,
    color_mode: ColorMode::Rgb,
};
pub const VERY_DARK_MAGENTA: Color = Color {
    r: 64,
    g: 0,
    b: 64,
    a: 255,
    color_mode: ColorMode::Rgb,
};
