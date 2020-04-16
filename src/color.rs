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
#[repr(C)]
pub enum ColorMode {
    Rgb,
    Hsb,
}

/// Represents a color (by default stored as RGBA values ranging from 0-255).  The default is
/// black.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    color_mode: ColorMode,
}

impl<'a> Color {
    /// Creates a new Rgb Color.
    #[inline(always)]
    #[allow(non_snake_case)]
    pub const fn RGB(r: u8, g: u8, b: u8) -> Self {
        Self::RGBA(r, g, b, 255)
    }

    /// Creates a new Rgb Color with alpha.
    #[inline(always)]
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

    /// Creates a new Color from a u32.
    pub const fn from_u32(color: u32) -> Self {
        Self {
            r: (color >> RED_SHIFT) as u8,
            g: (color >> GREEN_SHIFT) as u8,
            b: (color >> BLUE_SHIFT) as u8,
            a: (color >> ALPHA_SHIFT) as u8,
            color_mode: ColorMode::Rgb,
        }
    }

    /// Converts a Color to a u32 representation.
    pub const fn to_u32(self, color: u32) -> u32 {
        (self.r as u32) << RED_SHIFT
            | (self.g as u32) << GREEN_SHIFT
            | (self.b as u32) << BLUE_SHIFT
            | (self.a as u32) << ALPHA_SHIFT
    }

    /// Get the rgb value as a tuple.
    #[inline(always)]
    pub const fn rgb(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    /// Get the rgba values as a tuple.
    #[inline(always)]
    pub const fn rgba(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    /// Set the rgb value.
    #[inline(always)]
    pub fn set_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
    }

    /// Set the rgba value.
    #[inline(always)]
    pub fn set_rgba(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
        self.a = a;
    }

    /// Get the red value of the color ranging from 0-255.
    #[inline(always)]
    pub const fn r(self) -> u8 {
        self.r
    }
    /// Set the red value of the color ranging from 0-255.
    #[inline(always)]
    pub fn set_r(&mut self, r: u8) {
        self.r = r;
    }

    /// Get the green value of the color ranging from 0-255.
    #[inline(always)]
    pub const fn g(self) -> u8 {
        self.g
    }
    /// Set the green value of the color ranging from 0-255.
    #[inline(always)]
    pub fn set_g(&mut self, g: u8) {
        self.g = g;
    }

    /// Get the blue value of the color ranging from 0-255.
    #[inline(always)]
    pub const fn b(self) -> u8 {
        self.b
    }
    /// Set the blue value of the color ranging from 0-255.
    #[inline(always)]
    pub fn set_b(&mut self, b: u8) {
        self.b = b;
    }

    /// Get the alpha value of the color ranging from 0-255.
    #[inline(always)]
    pub const fn a(self) -> u8 {
        self.a
    }
    /// Set the alpha value of the color ranging from 0-255.
    #[inline(always)]
    pub fn set_a(&mut self, a: u8) {
        self.a = a;
    }

    /// Gets a Color as a slice of rgba values.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn as_slice(&self) -> &'a [u8] {
        unsafe { core::slice::from_raw_parts(self as *const Self as *const u8, 4) }
    }

    /// Gets a Color as a mutable slice of rgba values.
    pub fn as_slice_mut(&mut self) -> &'a mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self as *mut Self as *mut u8, 4) }
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

/// From gray value to Color
impl From<u8> for Color {
    fn from(gray: u8) -> Self {
        Color::RGB(gray, gray, gray)
    }
}

/// From gray value with alpha to Color
impl From<(u8, u8)> for Color {
    fn from((gray, a): (u8, u8)) -> Self {
        Color::RGBA(gray, gray, gray, a)
    }
}

/// From tuple of (r, g, b) to Color
impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Color::RGB(r, g, b)
    }
}

/// From tuple of (r, g, b, a) to Color
impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Color::RGBA(r, g, b, a)
    }
}

/// From &[u8] to Color
impl From<&[u8]> for Color {
    fn from(slice: &[u8]) -> Self {
        match slice {
            [r, g, b] => Color::RGB(*r, *g, *b),
            [r, g, b, a] => Color::RGBA(*r, *g, *b, *a),
            _ => panic!("invalid color slice"),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

/// Color Constants for common colors

// WHITE/BLACK/BLANK
pub const WHITE: Color = Color::RGB(255, 255, 255);
pub const BLACK: Color = Color::RGB(0, 0, 0);
pub const TRANSPARENT: Color = Color::RGBA(0, 0, 0, 0);

// GRAY
pub const BRIGHT_GRAY: Color = Color::RGB(192, 192, 192);
pub const GRAY: Color = Color::RGB(128, 128, 128);
pub const DARK_GRAY: Color = Color::RGB(64, 64, 64);

// RED
pub const BRIGHT_RED: Color = Color::RGB(255, 0, 0);
pub const RED: Color = Color::RGB(128, 0, 0);
pub const DARK_RED: Color = Color::RGB(64, 0, 0);

// ORANGE
pub const BRIGHT_ORANGE: Color = Color::RGB(255, 128, 0);
pub const ORANGE: Color = Color::RGB(128, 64, 0);
pub const DARK_ORANGE: Color = Color::RGB(64, 32, 0);

// YELLOW
pub const BRIGHT_YELLOW: Color = Color::RGB(255, 255, 0);
pub const YELLOW: Color = Color::RGB(128, 128, 0);
pub const DARK_YELLOW: Color = Color::RGB(64, 64, 0);

// GREEN
pub const BRIGHT_GREEN: Color = Color::RGB(0, 255, 0);
pub const GREEN: Color = Color::RGB(0, 128, 0);
pub const DARK_GREEN: Color = Color::RGB(0, 64, 0);

// CYAN
pub const BRIGHT_CYAN: Color = Color::RGB(0, 255, 255);
pub const CYAN: Color = Color::RGB(0, 128, 128);
pub const DARK_CYAN: Color = Color::RGB(0, 64, 64);

// BLUE
pub const BRIGHT_BLUE: Color = Color::RGB(0, 255, 255);
pub const BLUE: Color = Color::RGB(0, 0, 128);
pub const DARK_BLUE: Color = Color::RGB(0, 0, 64);

// MAGENTA
pub const BRIGHT_MAGENTA: Color = Color::RGB(255, 0, 255);
pub const MAGENTA: Color = Color::RGB(128, 0, 128);
pub const DARK_MAGENTA: Color = Color::RGB(64, 0, 64);
