//! Color handling and manipulation used for draw operations.
//!
//! Each color stores the color mode it was created with (e.g. Rgb or Hsl). Internally, colors are
//! stored as RGBA values ranging from 0-255. Other color representations than the initial color
//! mode are calculated and cached as needed.

use crate::math::random;
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

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
    /// Creates a new Rgb Color.
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::prelude::*;
    ///
    /// let c = Color::rgb(128, 64, 0);
    /// assert_eq!(c.values(), (128, 64, 0, 255));
    /// ```
    #[inline(always)]
    #[allow(non_snake_case)]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    /// Creates a new Rgb Color with alpha.
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::prelude::*;
    ///
    /// let c = Color::rgba(128, 64, 0, 128);
    /// assert_eq!(c.values(), (128, 64, 0, 128));
    /// ```
    #[inline(always)]
    #[allow(non_snake_case)]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
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
    /// use pix_engine::prelude::*;
    ///
    /// let magenta: u32 = (128 << 24) | (128 << 8) | 255;
    /// let c = Color::from_u32(magenta);
    /// assert_eq!(c.values(), (128, 0, 128, 255));
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
    /// use pix_engine::prelude::*;
    ///
    /// let c = color!(128, 0, 128);
    /// let magenta: u32 = (128 << 24) | (128 << 8) | 255;
    /// assert_eq!(c.to_u32(), magenta);
    /// ```
    pub const fn to_u32(self) -> u32 {
        (self.r as u32) << RED_SHIFT
            | (self.g as u32) << GREEN_SHIFT
            | (self.b as u32) << BLUE_SHIFT
            | (self.a as u32) << ALPHA_SHIFT
    }

    /// Creates a random RGB color with values ranging from 0-255.
    pub fn random_rgb() -> Self {
        Self::rgb(random(255), random(255), random(255))
    }

    /// Creates a random RGBA color with values ranging from 0-255.
    pub fn random_rgba() -> Self {
        Self::rgba(random(255), random(255), random(255), random(255))
    }

    /// Get the red value of the color ranging from 0-255.
    #[inline(always)]
    pub const fn red(self) -> u8 {
        self.r
    }
    /// Set the red value of the color ranging from 0-255.
    #[inline(always)]
    pub fn set_red(&mut self, r: u8) {
        self.r = r;
    }

    /// Get the green value of the color ranging from 0-255.
    #[inline(always)]
    pub const fn green(self) -> u8 {
        self.g
    }
    /// Set the green value of the color ranging from 0-255.
    #[inline(always)]
    pub fn set_green(&mut self, g: u8) {
        self.g = g;
    }

    /// Get the blue value of the color ranging from 0-255.
    #[inline(always)]
    pub const fn blue(self) -> u8 {
        self.b
    }
    /// Set the blue value of the color ranging from 0-255.
    #[inline(always)]
    pub fn set_blue(&mut self, b: u8) {
        self.b = b;
    }

    /// Get the alpha value of the color ranging from 0-255.
    #[inline(always)]
    pub const fn alpha(self) -> u8 {
        self.a
    }
    /// Set the alpha value of the color ranging from 0-255.
    #[inline(always)]
    pub fn set_alpha(&mut self, a: u8) {
        self.a = a;
    }

    /// Returns a representation of this color as a Vec of u8 values based on the current
    /// `State::color_mode`.
    ///
    /// - RGB: (red, green, blue, alpha)
    /// - HSB/HSL: (hue, saturation, brightness/lightness, alpha)
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::prelude::*;
    ///
    /// let mut c1 = color!(128, 0, 128);
    /// assert_eq!(c1.into_vec(), vec![128, 0, 128, 255]);
    ///
    /// let mut c2 = color!(128, 0, 128, 64);
    /// assert_eq!(c2.into_vec(), vec![128, 0, 128, 64]);
    /// ```
    pub fn into_vec(self) -> Vec<u8> {
        vec![self.r, self.g, self.b, self.a]
    }

    /// Returns a tuple representing the values based on the current `State::color_mode()`.
    ///
    /// - RGB: (red, green, blue, alpha)
    /// - HSB/HSL: (hue, saturation, brightness/lightness, alpha)
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::prelude::*;
    ///
    /// let mut c1 = color!(128, 0, 128);
    /// assert_eq!(c1.into_vec(), vec![128, 0, 128, 255]);
    ///
    /// let mut c2 = color!(128, 0, 128, 64);
    /// assert_eq!(c2.into_vec(), vec![128, 0, 128, 64]);
    /// ```
    pub fn values(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
}

/// Creates a new Color. The parameters are interpreted as either RGB, HSB, or HSL depending on the
/// current `State::color_mode()`.
///
/// The default is RGB with values ranging from 0 to 255. HSB/HSL values range from 0 to 360 for
/// Hue and Saturation, and 0 to 100 for Brightness/Lightness.
///
/// The number of parameters provided alter how they are interpreted:
///
/// # Syntax
///
/// color!(gray, [alpha]);
///
/// color!(v1, v2, v3, [alpha]);
///
/// color!(value);
///
/// color!(values);
///
/// color!(color);
///
/// # Parameters
///
/// - gray: 0 to 255 value ranging from black to white.
/// - alpha: Transparency value ranging from 0 to 255 for RGB and 0 to 100 for HSB/HSL (Optional,
///   defaults to 255)
/// - v1: Red (0 to 255) or Hue (0 to 360)
/// - v2: Green (0 to 255) or Saturation (0 to 360)
/// - v3: Blue (0 to 255) or Brightness/Lightness (0 to 100)
/// - value: A color string in either text or hexidecimal format
/// - values: A slice containing rgba or hsba/hsla values.
/// - color: A `Color` instance.
///
/// # Examples
///
/// ```
/// use pix_engine::prelude::*;
///
/// let c1 = color!(128, 64, 0); // RGB
/// assert_eq!(c1.values(), (128, 64, 0, 255));
///
/// let c2 = color!(128, 64, 128, 128); // RGBA
/// assert_eq!(c2.values(), (128, 64, 128, 128));
///
/// let c3 = color!(128); // Gray
/// assert_eq!(c3.values(), (128, 128, 128, 255));
///
/// let c4 = color!(128, 64); // Gray with Alpha
/// assert_eq!(c4.values(), (128, 128, 128, 64));
///
/// let vals: Vec<u8> = vec![128, 64, 0];
/// let c5 = color!(vals.as_slice()); // RGB from slice
/// assert_eq!(c5.values(), (128, 64, 0, 255));
///
/// let vals: [u8; 4] = [128, 64, 0, 128];
/// let c6 = color!(&vals[..]); // RGBA from slice
/// assert_eq!(c6.values(), (128, 64, 0, 128));
/// ```
#[macro_export]
macro_rules! color {
    ($value:expr) => {
        Color::from($value);
    };
    ($gray:expr, $alpha:expr) => {
        Color::rgba($gray, $gray, $gray, $alpha);
    };
    ($red:expr, $green:expr, $blue:expr) => {
        Color::rgb($red, $green, $blue);
    };
    ($red:expr, $green:expr, $blue:expr, $alpha:expr) => {
        Color::rgba($red, $green, $blue, $alpha);
    };
}

impl Deref for Color {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { ::std::slice::from_raw_parts(self as *const Self as *const u8, 4) }
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { ::std::slice::from_raw_parts_mut(self as *mut Self as *mut u8, 4) }
    }
}

impl From<u8> for Color {
    fn from(gray: u8) -> Self {
        Color::rgb(gray, gray, gray)
    }
}

impl From<&[u8]> for Color {
    fn from(slice: &[u8]) -> Self {
        match slice {
            [r, g, b] => Color::rgb(*r, *g, *b),
            [r, g, b, a] => Color::rgba(*r, *g, *b, *a),
            _ => panic!("invalid color slice"),
        }
    }
}

impl Into<(u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

impl Into<(u8, u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

// Color Constants for common colors

// WHITE/BLACK/BLANK
pub const WHITE: Color = Color::rgb(255, 255, 255);
pub const BLACK: Color = Color::rgb(0, 0, 0);
pub const TRANSPARENT: Color = Color::rgba(0, 0, 0, 0);

// GRAY
pub const BRIGHT_GRAY: Color = Color::rgb(192, 192, 192);
pub const GRAY: Color = Color::rgb(128, 128, 128);
pub const DARK_GRAY: Color = Color::rgb(64, 64, 64);

// RED
pub const BRIGHT_RED: Color = Color::rgb(255, 0, 0);
pub const RED: Color = Color::rgb(128, 0, 0);
pub const DARK_RED: Color = Color::rgb(64, 0, 0);

// ORANGE
pub const BRIGHT_ORANGE: Color = Color::rgb(255, 128, 0);
pub const ORANGE: Color = Color::rgb(128, 64, 0);
pub const DARK_ORANGE: Color = Color::rgb(64, 32, 0);

// YELLOW
pub const BRIGHT_YELLOW: Color = Color::rgb(255, 255, 0);
pub const YELLOW: Color = Color::rgb(128, 128, 0);
pub const DARK_YELLOW: Color = Color::rgb(64, 64, 0);

// GREEN
pub const BRIGHT_GREEN: Color = Color::rgb(0, 255, 0);
pub const GREEN: Color = Color::rgb(0, 128, 0);
pub const DARK_GREEN: Color = Color::rgb(0, 64, 0);

// CYAN
pub const BRIGHT_CYAN: Color = Color::rgb(0, 255, 255);
pub const CYAN: Color = Color::rgb(0, 128, 128);
pub const DARK_CYAN: Color = Color::rgb(0, 64, 64);

// BLUE
pub const BRIGHT_BLUE: Color = Color::rgb(0, 255, 255);
pub const BLUE: Color = Color::rgb(0, 0, 128);
pub const DARK_BLUE: Color = Color::rgb(0, 0, 64);

// MAGENTA
pub const BRIGHT_MAGENTA: Color = Color::rgb(255, 0, 255);
pub const MAGENTA: Color = Color::rgb(128, 0, 128);
pub const DARK_MAGENTA: Color = Color::rgb(64, 0, 64);
