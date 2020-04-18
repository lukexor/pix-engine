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
    Hsb, // TODO ColorMode::HSB
}

impl Default for ColorMode {
    fn default() -> Self {
        Self::Rgb
    }
}

/// Represents a color (by default stored as RGBA values ranging from 0-255).  The default is
/// black.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    color_mode: ColorMode,
}

impl<'a> Color {
    /// Creates a new Rgb/Rgba Color. Shortcut for `Color::RGB()` or `Color::RGBA()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::Color;
    ///
    /// let c1 = Color::new((128, 64, 0)); // RGB
    /// assert_eq!(c1.rgba(), (128, 64, 0, 255));
    ///
    /// let c2 = Color::new((128, 64, 128, 128)); // RGBA
    /// assert_eq!(c2.rgba(), (128, 64, 128, 128));
    ///
    /// let c3 = Color::new(128); // Gray
    /// assert_eq!(c3.rgba(), (128, 128, 128, 255));
    ///
    /// let c4 = Color::new((128, 64)); // Gray with Alpha
    /// assert_eq!(c4.rgba(), (128, 128, 128, 64));
    ///
    /// let c5 = Color::new(&[128u8, 64, 0][..]); // RGB from slice
    /// assert_eq!(c5.rgba(), (128, 64, 0, 255));
    ///
    /// let c6 = Color::new(&[128u8, 64, 0, 128][..]); // RGBA from slice
    /// assert_eq!(c6.rgba(), (128, 64, 0, 128));
    /// ```
    #[inline(always)]
    pub fn new<C: Into<Color>>(c: C) -> Self {
        let c = c.into();
        Color::RGBA(c.r, c.g, c.b, c.a)
    }

    /// Creates a new Rgb Color.
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::Color;
    ///
    /// let c = Color::RGB(128, 64, 0);
    /// assert_eq!(c.rgba(), (128, 64, 0, 255));
    /// ```
    #[inline(always)]
    #[allow(non_snake_case)]
    pub const fn RGB(r: u8, g: u8, b: u8) -> Self {
        Self::RGBA(r, g, b, 255)
    }

    /// Creates a new Rgb Color with alpha.
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::Color;
    ///
    /// let c = Color::RGBA(128, 64, 0, 128);
    /// assert_eq!(c.rgba(), (128, 64, 0, 128));
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::Color;
    ///
    /// let magenta: u32 = (128 << 24) | (128 << 8) | 255;
    /// let c = Color::from_u32(magenta);
    /// assert_eq!(c.rgba(), (128, 0, 128, 255));
    /// ```
    pub const fn from_u32(val: u32) -> Self {
        Self {
            r: (val >> RED_SHIFT) as u8,
            g: (val >> GREEN_SHIFT) as u8,
            b: (val >> BLUE_SHIFT) as u8,
            a: (val >> ALPHA_SHIFT) as u8,
            color_mode: ColorMode::Rgb,
        }
    }

    /// Converts a Color to a u32 representation.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::Color;
    ///
    /// let c = Color::new((128, 0, 128));
    /// let magenta: u32 = (128 << 24) | (128 << 8) | 255;
    /// assert_eq!(c.to_u32(), magenta);
    /// ```
    pub const fn to_u32(self) -> u32 {
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

    /// Set the rgb values. Alpha is unaffected.
    #[inline(always)]
    pub fn set_rgb<C: Into<Color>>(&mut self, c: C) {
        let c = c.into();
        self.r = c.r;
        self.g = c.g;
        self.b = c.b;
    }

    /// Set the rgba values.
    #[inline(always)]
    pub fn set_rgba<C: Into<Color>>(&mut self, c: C) {
        let c = c.into();
        self.r = c.r;
        self.g = c.g;
        self.b = c.b;
        self.a = c.a;
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

    /// Returns a representation of this color as a Vec of u8 values. Useful for temporary use.
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::Color;
    ///
    /// let mut c1 = Color::new((128, 0, 128));
    /// assert_eq!(c1.to_vec(), vec![128, 0, 128, 255]);
    ///
    /// let mut c2 = Color::new((128, 0, 128, 64));
    /// assert_eq!(c2.to_vec(), vec![128, 0, 128, 64]);
    /// ```
    pub fn to_vec(self) -> Vec<u8> {
        vec![self.r, self.g, self.b, self.a]
    }

    /// Gets a Color as a slice of rgba u8 values.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::Color;
    ///
    /// let mut c = Color::new((128, 0, 128));
    /// assert_eq!(c.as_slice(), &[128, 0, 128, 255]);
    /// ```
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn as_slice(&self) -> &'a [u8] {
        unsafe { core::slice::from_raw_parts(self as *const Self as *const u8, 4) }
    }

    /// Gets a Color as a mutable slice of rgba u8 values.
    pub fn as_slice_mut(&mut self) -> &'a mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self as *mut Self as *mut u8, 4) }
    }
}

/// From gray u8 value to Color.
impl From<u8> for Color {
    fn from(gray: u8) -> Self {
        Color::RGB(gray, gray, gray)
    }
}

/// From gray u8 value with alpha u8 to Color.
impl From<(u8, u8)> for Color {
    fn from((gray, a): (u8, u8)) -> Self {
        Color::RGBA(gray, gray, gray, a)
    }
}

/// From a u8 tuple of (r, g, b) to Color.
impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Color::RGB(r, g, b)
    }
}

/// From a u8 tuple of (r, g, b, a) to Color.
impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Color::RGBA(r, g, b, a)
    }
}

/// Convert to a u8 tuple of (r, g, b).
impl Into<(u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

/// Convert to a u8 tuple of (r, g, b, a).
impl Into<(u8, u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
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
