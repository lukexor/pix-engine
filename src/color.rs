//! [Color] functions for drawing.
//!
//! Each `Color` can be represented as either [Rgb] or [Hsv] and can be converted from/into other
//! representations as needed.
//!
//! There are two convience macros for easy construction: [rgb!] and [hsv!] that take 1-4
//! parameters. The number of parameters provided alter how they are interpreted.
//!
//! Providing a single parameter results in a grayscale color. Two parameters is used for grayscale
//! with alpha transparency. Three is interpreted as RGB or HSV values with a fourth parameter used
//! to apply alpha transparency.
//!
//! There are also several named color [constants](constants) available in the
//! [prelude](crate::prelude) matching the [SVG 1.0 Color
//! Keywords](https://www.w3.org/TR/SVG11/types.html#ColorKeywords).
//!
//! # Examples
//!
//! ```
//! # use pix_engine::prelude::*;
//! // RGB values range from 0-255
//! let c = rgb!(55); // Grayscale
//! assert_eq!(c.rgb_channels(), (55, 55, 55, 255));
//!
//! let c = rgb!(55, 128); // Grayscale with alpha
//! assert_eq!(c.rgb_channels(), (55, 55, 55, 128));
//!
//! let c = rgb!(128, 0, 55); // Red, Green, Blue
//! assert_eq!(c.rgb_channels(), (128, 0, 55, 255));
//!
//! let c = rgb!(128, 0, 55, 128); // Red, Green, Blue, and Alpha
//! assert_eq!(c.rgb_channels(), (128, 0, 55, 128));
//!
//! // HSV values range from 0.0-360.0 for hue and 0.0-1.0 for all other values
//! let c = hsv!(0.5); // Grayscale
//! assert_eq!(c.hsv_channels(), (0.0, 0.0, 0.5, 1.0));
//!
//! let c = hsv!(0.5, 0.8); // Grayscale with alpha
//! assert_eq!(c.hsv_channels(), (0.0, 0.0, 0.5, 0.8));
//!
//! let c = hsv!(128.0, 1.0, 0.5); // Hue, Saturation, Value
//! assert_eq!(c.hsv_channels(), (128.0, 1.0, 0.5, 1.0));
//!
//! let c = hsv!(228.0, 0.8, 1.0, 0.8); // Hue, Saturation, Value, and Alpha
//! assert_eq!(c.hsv_channels(), (228.0, 0.8, 1.0, 0.8));
//!
//! // Named color constants
//! let c = ALICE_BLUE;
//! assert_eq!(c.rgb_channels(), (240, 248, 255, 255));
//! ```
//!
//! You can also create colors from hexidecimal strings using 3, 4, 6, or 8-digit formats.
//!
//! # Examples
//!
//! ```
//! # use pix_engine::prelude::*;
//! use std::str::FromStr;
//!
//! let c = Color::from_str("#F0F")?; // 3-digit Hex string
//! assert_eq!(c.rgb_channels(), (255, 0, 255, 255));
//!
//! let c = Color::from_str("#F0F5")?; // 4-digit Hex string
//! assert_eq!(c.rgb_channels(), (255, 0, 255, 85));
//!
//! let c = Color::from_str("#F0F5BF")?; // 6-digit Hex string
//! assert_eq!(c.rgb_channels(), (240, 245, 191, 255));
//!
//! let c = Color::from_str("#F0F5BF5F")?; // 8-digit Hex string
//! assert_eq!(c.rgb_channels(), (240, 245, 191, 95));
//! # Ok::<(), ColorError>(())
//! ```

use crate::random;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    convert::TryFrom,
    error,
    fmt::{self, LowerHex, UpperHex},
    ops::{Index, IndexMut},
    str::FromStr,
};

/// # Create an [Rgb] [Color].
///
/// # Examples
///
/// ```
/// # use pix_engine::prelude::*;
///
/// let c = rgb!(128); // Gray
/// assert_eq!(c.rgb_channels(), (128, 128, 128, 255));
///
/// let c = rgb!(128, 64); // Gray with alpha
/// assert_eq!(c.rgb_channels(), (128, 128, 128, 64));
///
/// let c = rgb!(128, 64, 0); // Red, Green, Blue
/// assert_eq!(c.rgb_channels(), (128, 64, 0, 255));
///
/// let c = rgb!(128, 64, 128, 128); // Red, Green, Blue, Alpha
/// assert_eq!(c.rgb_channels(), (128, 64, 128, 128));
/// ```
#[macro_export]
macro_rules! rgb {
    ($gray:expr) => {
        rgb!($gray, $gray, $gray)
    };
    ($gray:expr, $a:expr$(,)?) => {
        rgb!($gray, $gray, $gray, $a)
    };
    ($r:expr, $g:expr, $b:expr$(,)?) => {
        rgb!($r, $g, $b, 255)
    };
    ($r:expr, $g:expr, $b:expr, $a:expr$(,)?) => {
        $crate::color::Color::rgba($r, $g, $b, $a)
    };
}

/// # Create a [Hsv] [Color].
///
/// # Examples
///
/// ```
/// # use pix_engine::prelude::*;
///
/// let c = hsv!(0.5); // Gray
/// assert_eq!(c.hsv_channels(), (0.0, 0.0, 0.5, 1.0));
///
/// let c = hsv!(0.5, 0.5); // Gray with alpha
/// assert_eq!(c.hsv_channels(), (0.0, 0.0, 0.5, 0.5));
///
/// let c = hsv!(337.0, 1.0, 0.8); // Hue, Saturation, Value
/// assert_eq!(c.hsv_channels(), (337.0, 1.0, 0.8, 1.0));
///
/// let c = hsv!(337.0, 1.0, 0.8, 0.5); // Hue, Saturation, Value, Alpha
/// assert_eq!(c.hsv_channels(), (337.0, 1.0, 0.8, 0.5));
/// ```
#[macro_export]
macro_rules! hsv {
    ($gray:expr) => {
        hsv!(0.0, 0.0, $gray)
    };
    ($gray:expr, $a:expr$(,)?) => {
        hsv!(0.0, 0.0, $gray, $a)
    };
    ($h:expr, $s:expr, $v:expr$(,)?) => {
        hsv!($h, $s, $v, 1.0)
    };
    ($h:expr, $s:expr, $v:expr, $a:expr$(,)?) => {
        $crate::color::Color::hsva($h, $s, $v, $a)
    };
}

/// A color represented as [Rgb] or [Hsv].
#[allow(variant_size_differences)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Color {
    /// Red, Green, Blue and Alpha values.
    Rgb(Rgb),
    /// Hue, Saturation, Value and Alpha values.
    Hsv(Hsv),
}

impl Color {
    /// Create a new `Color`, defaulting to black.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::new();
    /// assert_eq!(c.rgb_channels(), (0, 0, 0, 255));
    /// ```
    pub fn new() -> Self {
        Self::Rgb(Rgb::new())
    }

    /// Create a new `Color` with red, green, and blue with alpha of 255.
    ///
    /// # Example
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(128, 64, 0);
    /// assert_eq!(c.rgb_channels(), (128, 64, 0, 255));
    /// ```
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    /// Create a new `Color` with red, green, blue, and alpha of 255.
    ///
    /// # Example
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgba(128, 64, 128, 128);
    /// assert_eq!(c.rgb_channels(), (128, 64, 128, 128));
    /// ```
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::Rgb(Rgb::rgba(r, g, b, a))
    }

    /// Create a new `Color` with hue, saturation, and value with alpha of 255.
    ///
    /// # Example
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::hsv(128.0, 0.8, 0.0);
    /// assert_eq!(c.hsv_channels(), (128.0, 0.8, 0.0, 1.0));
    /// ```
    pub fn hsv(h: f32, s: f32, v: f32) -> Self {
        Self::hsva(h, s, v, 1.0)
    }

    /// Create a new `Color` with hue, saturation, value and alpha.
    ///
    /// # Example
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::hsva(128.0, 0.8, 0.0, 0.5);
    /// assert_eq!(c.hsv_channels(), (128.0, 0.8, 0.0, 0.5));
    /// ```
    pub fn hsva(h: f32, s: f32, v: f32, a: f32) -> Self {
        Self::Hsv(Hsv::hsva(h, s, v, a))
    }

    /// Create a new `Color` with random red, green, and blue with alpha of 255.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # #[allow(unused_variables)]
    /// let c = Color::random();
    /// // `c.rgb_channels()` will return something like:
    /// // (207, 12, 217, 255)
    /// ```
    pub fn random() -> Self {
        Self::Rgb(Rgb::random())
    }

    /// Create a new `Color` with random red, green, blue and alpha.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # #[allow(unused_variables)]
    /// let c = Color::random_alpha();
    /// // `c.rgb_channels()` will return something like:
    /// // (132, 159, 233, 76)
    /// ```
    pub fn random_alpha() -> Self {
        Self::Rgb(Rgb::random_alpha())
    }

    /// Create a new `Color` from a slice of 1-4 [u8] RGBA values. The number of values provided
    /// alter how they are interpreted similar to the [rgb!] macro.
    ///
    /// # Errors
    ///
    /// If the slice is empty or has more than 4 values, an error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let vals: Vec<u8> = vec![128, 64, 0];
    /// let c = Color::from_rgb_slice(&vals)?; // RGB Vec
    /// assert_eq!(c.rgb_channels(), (128, 64, 0, 255));
    ///
    /// let vals: [u8; 4] = [128, 64, 0, 128];
    /// let c = Color::from_rgb_slice(&vals[..])?; // RGBA slice
    /// assert_eq!(c.rgb_channels(), (128, 64, 0, 128));
    /// # Ok::<(), ColorError>(())
    /// ```
    pub fn from_rgb_slice(slice: &[u8]) -> Result<Self, ColorError> {
        match *slice {
            [gray] => Ok(Self::rgb(gray, gray, gray)),
            [gray, a] => Ok(Self::rgba(gray, gray, gray, a)),
            [r, g, b] => Ok(Self::rgb(r, g, b)),
            [r, g, b, a] => Ok(Self::rgba(r, g, b, a)),
            _ => Err(ColorError::InvalidRgbSlice(Cow::from(slice.to_owned()))),
        }
    }

    /// Create a new `Color` from a slice of 1-4 [u32] HSVA values. The number of values provided
    /// alter how they are interpreted similar to the [hsv!] macro.
    ///
    /// # Errors
    ///
    /// If the slice is empty or has more than 4 values, an error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let vals: Vec<f32> = vec![128.0, 0.8, 0.0];
    /// let c = Color::from_hsv_slice(&vals)?; // HSV Vec
    /// assert_eq!(c.hsv_channels(), (128.0, 0.8, 0.0, 1.0));
    ///
    /// let vals: [f32; 4] = [128.0, 0.8, 0.0, 0.5];
    /// let c = Color::from_hsv_slice(&vals[..])?; // HSVA slice
    /// assert_eq!(c.hsv_channels(), (128.0, 0.8, 0.0, 0.5));
    /// # Ok::<(), ColorError>(())
    /// ```
    pub fn from_hsv_slice(slice: &[f32]) -> Result<Self, ColorError> {
        match *slice {
            [gray] => Ok(Self::hsv(gray, gray, gray)),
            [gray, a] => Ok(Self::hsva(gray, gray, gray, a)),
            [h, s, v] => Ok(Self::hsv(h, s, v)),
            [h, s, v, a] => Ok(Self::hsva(h, s, v, a)),
            _ => Err(ColorError::InvalidHsvSlice(Cow::from(slice.to_owned()))),
        }
    }

    /// Create a new `Color` from a [u32] RGBA hexadecimal value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::from_hex(0xF0FF00FF);
    /// assert_eq!(c.rgb_channels(), (240, 255, 0, 255));
    ///
    /// let c = Color::from_hex(0xF0FF0080);
    /// assert_eq!(c.rgb_channels(), (240, 255, 0, 128));
    /// ```
    pub fn from_hex(hex: u32) -> Self {
        let [r, g, b, a] = hex.to_be_bytes();
        Self::rgba(r, g, b, a)
    }

    /// Get the red, green, blue, and alpha channels as a tuple of 4 [u8] values.
    ///
    /// # Example
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgba(128, 64, 128, 128);
    /// assert_eq!(c.rgb_channels(), (128, 64, 128, 128));
    /// ```
    pub fn rgb_channels(self) -> (u8, u8, u8, u8) {
        match self {
            Self::Rgb(rgb) => (rgb.r, rgb.g, rgb.b, rgb.a),
            Self::Hsv(hsv) => {
                let rgb = hsv.to_rgb();
                (rgb.r, rgb.g, rgb.b, rgb.a)
            }
        }
    }

    /// Get the hue, saturation, value, and alpha channels as a tuple of 4 [f32] values.
    ///
    /// # Example
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::hsva(128.0, 0.8, 1.0, 0.5);
    /// assert_eq!(c.hsv_channels(), (128.0, 0.8, 1.0, 0.5));
    /// ```
    pub fn hsv_channels(self) -> (f32, f32, f32, f32) {
        match self {
            Self::Rgb(rgb) => {
                let hsv = rgb.to_hsv();
                (hsv.h, hsv.s, hsv.v, hsv.a)
            }
            Self::Hsv(hsv) => (hsv.h, hsv.s, hsv.v, hsv.a),
        }
    }

    /// Convert `Color` from [Rgb] into [Hsv].
    ///
    /// Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// assert_eq!(rgb!(0, 0, 255).to_hsv(), hsv!(240.0, 1.0, 1.0)); // blue
    /// ```
    pub fn to_hsv(self) -> Self {
        match self {
            Self::Rgb(rgb) => Self::Hsv(rgb.to_hsv()),
            Self::Hsv(_) => self,
        }
    }

    /// Convert `Color` from [Hsv] into [Rgb].
    ///
    /// Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// assert_eq!(hsv!(240.0, 1.0, 1.0).to_rgb(), rgb!(0, 0, 255)); // blue
    /// ```
    pub fn to_rgb(self) -> Self {
        match self {
            Self::Rgb(_) => self,
            Self::Hsv(hsv) => Self::Rgb(hsv.to_rgb()),
        }
    }

    /// Creates a new `Color` by linear interpolating between two colors by a given amount between
    /// 0.0 and 1.0.
    ///
    /// # Note
    ///
    /// You can lerp between any mix of [Rgb] and [Hsv] Colors, but an implicit conversion is
    /// performed based on the format of `self`. See examples for reference.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let from = rgb!(255, 0, 0);
    /// let to = rgb!(0, 100, 255);
    /// let lerped = from.lerp(to, 0.5);
    /// assert_eq!(lerped.rgb_channels(), (128, 50, 128, 255));
    ///
    /// let from = rgb!(255, 0, 0);
    /// let to = hsv!(120.0, 0.8, 1.0, 0.5);
    /// let lerped = from.lerp(to, 0.25); // `to` is implicity converted to RGB
    /// assert_eq!(lerped.rgb_channels(), (204, 64, 13, 223));
    /// ```
    pub fn lerp(self, c2: Color, amt: f32) -> Self {
        let amt = amt.clamp(0.0, 1.0);
        let lerp = |start, stop, amt| amt * (stop - start) + start;
        match self {
            Self::Rgb(rgb) => {
                let (r, g, b, a) = rgb.channels();
                let (or, og, ob, oa) = c2.rgb_channels();
                let r = lerp(r as f32, or as f32, amt).round() as u8;
                let g = lerp(g as f32, og as f32, amt).round() as u8;
                let b = lerp(b as f32, ob as f32, amt).round() as u8;
                let a = lerp(a as f32, oa as f32, amt).round() as u8;
                Self::rgba(r, g, b, a)
            }
            Self::Hsv(hsv) => {
                let (h, s, v, a) = hsv.channels();
                let (oh, os, ov, oa) = c2.hsv_channels();
                let h = lerp(h, oh, amt);
                let s = lerp(s, os, amt);
                let v = lerp(v, ov, amt);
                let a = lerp(a, oa, amt);
                Self::hsva(h, s, v, a)
            }
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Color {
    type Err = ColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Color::Rgb(Rgb::from_str(s)?))
    }
}

impl TryFrom<&str> for Color {
    type Error = ColorError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

impl From<u8> for Color {
    fn from(gray: u8) -> Self {
        rgb!(gray)
    }
}

impl From<(u8, u8)> for Color {
    fn from((gray, a): (u8, u8)) -> Self {
        rgb!(gray, a)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        rgb!(r, g, b)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        rgb!(r, g, b, a)
    }
}

impl From<Color> for (u8, u8, u8, u8) {
    fn from(color: Color) -> Self {
        use Color::*;
        match color {
            Rgb(rgb) => rgb.into(),
            Hsv(hsv) => hsv.to_rgb().into(),
        }
    }
}

impl From<Color> for (f32, f32, f32, f32) {
    fn from(color: Color) -> Self {
        use Color::*;
        match color {
            Rgb(rgb) => rgb.to_hsv().into(),
            Hsv(hsv) => hsv.into(),
        }
    }
}

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Self {
        Color::Rgb(rgb)
    }
}

impl From<f32> for Color {
    fn from(gray: f32) -> Self {
        hsv!(gray)
    }
}

impl From<(f32, f32)> for Color {
    fn from((gray, a): (f32, f32)) -> Self {
        hsv!(gray, a)
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from((h, s, v): (f32, f32, f32)) -> Self {
        hsv!(h, s, v)
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from((h, s, v, a): (f32, f32, f32, f32)) -> Self {
        hsv!(h, s, v, a)
    }
}

impl From<Hsv> for Color {
    fn from(hsv: Hsv) -> Self {
        Color::Hsv(hsv)
    }
}

impl From<Color> for Rgb {
    fn from(color: Color) -> Self {
        match color {
            Color::Rgb(rgb) => rgb,
            Color::Hsv(hsv) => hsv.to_rgb(),
        }
    }
}

impl From<Color> for Hsv {
    fn from(color: Color) -> Self {
        match color {
            Color::Rgb(rgb) => rgb.to_hsv(),
            Color::Hsv(hsv) => hsv,
        }
    }
}

/// An `Rgb` value containing red, green, blue, and alpha channels.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Rgb {
    /// Create a new `Rgb` instance, defaulting to black.
    const fn new() -> Self {
        Self::rgb(0, 0, 0)
    }

    /// Create a new `Rgb` instance.
    const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    /// Create a new `Rgb` instance with alpha.
    const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create a new `Rgb` instance with random red, green, and blue with alpha of 255.
    fn random() -> Self {
        Self::rgb(random!(255), random!(255), random!(255))
    }

    /// Create a new `Rgb` instance with random red, green, blue and alpha.
    fn random_alpha() -> Self {
        Self::rgba(random!(255), random!(255), random!(255), random!(255))
    }

    /// Get the red, green, blue, and alpha channels as a tuple u8 values.
    pub fn channels(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    /// Convert to [Hsv] format.
    fn to_hsv(self) -> Hsv {
        let r1 = f32::from(self.r) / 255.0;
        let g1 = f32::from(self.g) / 255.0;
        let b1 = f32::from(self.b) / 255.0;
        let c_max = r1.max(g1).max(b1);
        let c_min = r1.min(g1).min(b1);
        let chr = c_max - c_min;
        if chr != 0.0 {
            let h = if (r1 - c_max).abs() < f32::EPSILON {
                ((g1 - b1) / chr) % 6.0
            } else if (g1 - c_max).abs() < f32::EPSILON {
                ((b1 - r1) / chr) + 2.0
            } else {
                ((r1 - g1) / chr) + 4.0 // b1 == c_max
            };
            let mut h = h * 60.0;
            if h < 0.0 {
                h += 360.0;
            }
            let s = chr / c_max;
            Hsv::hsv(h, s, c_max)
        } else {
            Hsv::hsv(0.0, 0.0, c_max)
        }
    }
}

impl Index<usize> for Rgb {
    type Output = u8;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            3 => &self.a,
            _ => panic!("index out of bounds: the len is 4 but the index is {}", idx),
        }
    }
}

impl IndexMut<usize> for Rgb {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            3 => &mut self.a,
            _ => panic!("index out of bounds: the len is 4 but the index is {}", idx),
        }
    }
}

impl LowerHex for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:x}{:x}{:x}{:x}", self.r, self.g, self.b, self.a)
    }
}

impl UpperHex for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:X}{:X}{:X}{:X}", self.r, self.g, self.b, self.a)
    }
}

impl FromStr for Rgb {
    type Err = ColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('#') {
            return Err(ColorError::InvalidString(Cow::from(s.to_owned())));
        }

        let mut channels: [u8; 4] = [0, 0, 0, 255];
        let parse_hex = |hex: &str| {
            if let Ok(value) = u8::from_str_radix(hex, 16) {
                Ok(value)
            } else {
                Err(ColorError::InvalidString(Cow::from(hex.to_owned())))
            }
        };

        let s = s.trim().to_lowercase();
        match s.len() - 1 {
            3 | 4 => {
                for (i, _) in s[1..].char_indices() {
                    let hex = parse_hex(&s[i + 1..i + 2])?;
                    channels[i] = (hex << 4) | hex;
                }
            }
            6 | 8 => {
                for (i, _) in s[1..].char_indices().step_by(2) {
                    channels[i / 2] = parse_hex(&s[i + 1..i + 3])?;
                }
            }
            _ => return Err(ColorError::InvalidString(Cow::from(s))),
        }

        Ok(Self::rgba(
            channels[0],
            channels[1],
            channels[2],
            channels[3],
        ))
    }
}

impl TryFrom<&str> for Rgb {
    type Error = ColorError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Rgb::from_str(s)
    }
}

impl From<u8> for Rgb {
    fn from(gray: u8) -> Self {
        Self::rgb(gray, gray, gray)
    }
}

impl From<(u8, u8)> for Rgb {
    fn from((gray, alpha): (u8, u8)) -> Self {
        Self::rgba(gray, gray, gray, alpha)
    }
}

impl From<(u8, u8, u8)> for Rgb {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::rgb(r, g, b)
    }
}

impl From<(u8, u8, u8, u8)> for Rgb {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self::rgba(r, g, b, a)
    }
}

impl From<Rgb> for (u8, u8, u8, u8) {
    fn from(rgb: Rgb) -> Self {
        rgb.channels()
    }
}

/// A `Hsv` value containing hue, saturation, value, and alpha channels.
#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Hsv {
    h: f32,
    s: f32,
    v: f32,
    a: f32,
}

impl Hsv {
    /// Create a new `Hsv` instance.
    fn hsv(h: f32, s: f32, v: f32) -> Self {
        Self::hsva(h, s, v, 1.0)
    }

    /// Create a new `Hsv` instance with alpha.
    fn hsva(h: f32, s: f32, v: f32, a: f32) -> Self {
        Self {
            h: h.clamp(0.0, 360.0),
            s: s.clamp(0.0, 1.0),
            v: v.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        }
    }

    /// Get the hue, saturation, value, and alpha channels as a tuple f32 values.
    pub fn channels(self) -> (f32, f32, f32, f32) {
        (self.h, self.s, self.v, self.a)
    }

    /// Convert to [Rgb] format.
    fn to_rgb(self) -> Rgb {
        if self.v == 0.0 {
            Rgb::rgb(0, 0, 0)
        } else if self.s == 0.0 {
            let gray = (self.v * 255.0).round() as u8;
            Rgb::rgb(gray, gray, gray)
        } else {
            let chroma = self.v * self.s;
            let hue_six = self.h / 60.0;
            let value = chroma * (1.0 - (hue_six % 2.0 - 1.0).abs());
            let (r1, g1, b1) = match hue_six.floor() as usize {
                0 | 6 => (chroma, value, 0.0),
                1 => (value, chroma, 0.0),
                2 => (0.0, chroma, value),
                3 => (0.0, value, chroma),
                4 => (value, 0.0, chroma),
                5 => (chroma, 0.0, value),
                _ => unreachable!(),
            };
            let add = self.v - chroma;
            let r = ((r1 + add) * 255.0).round() as u8;
            let g = ((g1 + add) * 255.0).round() as u8;
            let b = ((b1 + add) * 255.0).round() as u8;
            let a = (self.a * 255.0).round() as u8;
            Rgb::rgba(r, g, b, a)
        }
    }
}

impl Index<usize> for Hsv {
    type Output = f32;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.h,
            1 => &self.s,
            2 => &self.v,
            3 => &self.a,
            _ => panic!("index out of bounds: the len is 4 but the index is {}", idx),
        }
    }
}

impl IndexMut<usize> for Hsv {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.h,
            1 => &mut self.s,
            2 => &mut self.v,
            3 => &mut self.a,
            _ => panic!("index out of bounds: the len is 4 but the index is {}", idx),
        }
    }
}

impl From<f32> for Hsv {
    fn from(gray: f32) -> Self {
        Self::hsv(0.0, 0.0, gray)
    }
}

impl From<(f32, f32)> for Hsv {
    fn from((gray, alpha): (f32, f32)) -> Self {
        Self::hsva(0.0, 0.0, gray, alpha)
    }
}

impl From<(f32, f32, f32)> for Hsv {
    fn from((h, s, v): (f32, f32, f32)) -> Self {
        Self::hsv(h, s, v)
    }
}

impl From<(f32, f32, f32, f32)> for Hsv {
    fn from((h, s, v, a): (f32, f32, f32, f32)) -> Self {
        Self::hsva(h, s, v, a)
    }
}

impl From<Hsv> for (f32, f32, f32, f32) {
    fn from(hsv: Hsv) -> Self {
        hsv.channels()
    }
}

/// The error type for [Color] operations.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum ColorError {
    /// Error when creating a [Color] from an invalid [Rgb] slice.
    InvalidRgbSlice(Cow<'static, [u8]>),
    /// Error when creating a [Color] from an invalid [Hsv] slice.
    InvalidHsvSlice(Cow<'static, [f32]>),
    /// Error when creating a [Color] from an invalid string.
    InvalidString(Cow<'static, str>),
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ColorError::*;
        match self {
            InvalidRgbSlice(slice) => write!(f, "invalid Rgb slice: {:?}", slice),
            InvalidHsvSlice(slice) => write!(f, "invalid Hsv slice: {:?}", slice),
            InvalidString(s) => write!(f, "invalid color string format: {}", s),
        }
    }
}

impl error::Error for ColorError {}

/// [SVG 1.0 Color Keywords](https://www.w3.org/TR/SVG11/types.html#ColorKeywords).
#[allow(missing_docs)]
pub mod constants {
    use super::Color;

    pub const ALICE_BLUE: Color = Color::rgb(0xF0, 0xF8, 0xFF);
    pub const ANTIQUE_WHITE: Color = Color::rgb(0xFA, 0xEB, 0xD7);
    pub const AQUA: Color = Color::rgb(0x0, 0xFF, 0xFF);
    pub const AQUA_MARINE: Color = Color::rgb(0x7F, 0xFF, 0xD4);
    pub const AZURE: Color = Color::rgb(0xF0, 0xFF, 0xFF);
    pub const BEIGE: Color = Color::rgb(0xF5, 0xF5, 0xDC);
    pub const BISQUE: Color = Color::rgb(0xFF, 0xE4, 0xC4);
    pub const BLACK: Color = Color::rgb(0x0, 0x0, 0x0);
    pub const BLANCHE_DALMOND: Color = Color::rgb(0xFF, 0xEB, 0xCD);
    pub const BLUE: Color = Color::rgb(0x0, 0x0, 0xFF);
    pub const BLUE_VIOLET: Color = Color::rgb(0x8A, 0x2B, 0xE2);
    pub const BROWN: Color = Color::rgb(0xA5, 0x2A, 0x2A);
    pub const BURLY_WOOD: Color = Color::rgb(0xDE, 0xB8, 0x87);
    pub const CADET_BLUE: Color = Color::rgb(0x5F, 0x9E, 0xA0);
    pub const CHARTREUSE: Color = Color::rgb(0x7F, 0xFF, 0x0);
    pub const CHOCOLATE: Color = Color::rgb(0xD2, 0x69, 0x1E);
    pub const CORAL: Color = Color::rgb(0xFF, 0x7F, 0x50);
    pub const CORNFLOWER_BLUE: Color = Color::rgb(0x64, 0x95, 0xED);
    pub const CORN_SILK: Color = Color::rgb(0xFF, 0xF8, 0xDC);
    pub const CRIMSON: Color = Color::rgb(0xDC, 0x14, 0x3C);
    pub const CYAN: Color = Color::rgb(0x0, 0xFF, 0xFF);
    pub const DARK_BLUE: Color = Color::rgb(0x0, 0x0, 0x8B);
    pub const DARK_CYAN: Color = Color::rgb(0x0, 0x8B, 0x8B);
    pub const DARK_GOLDENROD: Color = Color::rgb(0xB8, 0x86, 0xB);
    pub const DARK_GRAY: Color = Color::rgb(0xA9, 0xA9, 0xA9);
    pub const DARK_GREEN: Color = Color::rgb(0x0, 0x64, 0x0);
    pub const DARK_GREY: Color = Color::rgb(0xA9, 0xA9, 0xA9);
    pub const DARK_KHAKI: Color = Color::rgb(0xBD, 0xB7, 0x6B);
    pub const DARK_MAGENTA: Color = Color::rgb(0x8B, 0x0, 0x8B);
    pub const DARK_OLIVE_GREEN: Color = Color::rgb(0x55, 0x6B, 0x2F);
    pub const DARK_ORANGE: Color = Color::rgb(0xFF, 0x8C, 0x0);
    pub const DARK_ORCHID: Color = Color::rgb(0x99, 0x32, 0xCC);
    pub const DARK_RED: Color = Color::rgb(0x8B, 0x0, 0x0);
    pub const DARK_SALMON: Color = Color::rgb(0xE9, 0x96, 0x7A);
    pub const DARK_SEA_GREEN: Color = Color::rgb(0x8F, 0xBC, 0x8F);
    pub const DARK_SLATE_BLUE: Color = Color::rgb(0x48, 0x3D, 0x8B);
    pub const DARK_SLATE_GRAY: Color = Color::rgb(0x2F, 0x4F, 0x4F);
    pub const DARK_SLATE_GREY: Color = Color::rgb(0x2F, 0x4F, 0x4F);
    pub const DARK_TURQUOISE: Color = Color::rgb(0x0, 0xCE, 0xD1);
    pub const DARK_VIOLET: Color = Color::rgb(0x94, 0x0, 0xD3);
    pub const DEEP_PINK: Color = Color::rgb(0xFF, 0x14, 0x93);
    pub const DEEP_SKY_BLUE: Color = Color::rgb(0x0, 0xBF, 0xFF);
    pub const DIM_GRAY: Color = Color::rgb(0x69, 0x69, 0x69);
    pub const DIM_GREY: Color = Color::rgb(0x69, 0x69, 0x69);
    pub const DODGER_BLUE: Color = Color::rgb(0x1E, 0x90, 0xFF);
    pub const FIRE_BRICK: Color = Color::rgb(0xB2, 0x22, 0x22);
    pub const FLORAL_WHITE: Color = Color::rgb(0xFF, 0xFA, 0xF0);
    pub const FOREST_GREEN: Color = Color::rgb(0x22, 0x8B, 0x22);
    pub const FUCHSIA: Color = Color::rgb(0xFF, 0x0, 0xFF);
    pub const GAINSBORO: Color = Color::rgb(0xDC, 0xDC, 0xDC);
    pub const GHOST_WHITE: Color = Color::rgb(0xF8, 0xF8, 0xFF);
    pub const GOLD: Color = Color::rgb(0xFF, 0xD7, 0x0);
    pub const GOLDENROD: Color = Color::rgb(0xDA, 0xA5, 0x20);
    pub const GRAY: Color = Color::rgb(0x80, 0x80, 0x80);
    pub const GREEN: Color = Color::rgb(0x0, 0x80, 0x0);
    pub const GREEN_YELLOW: Color = Color::rgb(0xAD, 0xFF, 0x2F);
    pub const GREY: Color = Color::rgb(0x80, 0x80, 0x80);
    pub const HONEYDEW: Color = Color::rgb(0xF0, 0xFF, 0xF0);
    pub const HOTOINK: Color = Color::rgb(0xFF, 0x69, 0xB4);
    pub const INDIAN_RED: Color = Color::rgb(0xCD, 0x5C, 0x5C);
    pub const INDIGO: Color = Color::rgb(0x4B, 0x0, 0x82);
    pub const IVORY: Color = Color::rgb(0xFF, 0xFF, 0xF0);
    pub const KHAKI: Color = Color::rgb(0xF0, 0xE6, 0x8C);
    pub const LAVENDER: Color = Color::rgb(0xE6, 0xE6, 0xFA);
    pub const LAVENDER_BLUSH: Color = Color::rgb(0xFF, 0xF0, 0xF5);
    pub const LAWN_GREEN: Color = Color::rgb(0x7C, 0xFC, 0x0);
    pub const LEMON_CHIFFON: Color = Color::rgb(0xFF, 0xFA, 0xCD);
    pub const LIGHT_BLUE: Color = Color::rgb(0xAD, 0xD8, 0xE6);
    pub const LIGHT_CORAL: Color = Color::rgb(0xF0, 0x80, 0x80);
    pub const LIGHT_CYAN: Color = Color::rgb(0xE0, 0xFF, 0xFF);
    pub const LIGHT_GOLDENROD_YELLOW: Color = Color::rgb(0xFA, 0xFA, 0xD2);
    pub const LIGHT_GRAY: Color = Color::rgb(0xD3, 0xD3, 0xD3);
    pub const LIGHT_GREEN: Color = Color::rgb(0x90, 0xEE, 0x90);
    pub const LIGHT_GREY: Color = Color::rgb(0xD3, 0xD3, 0xD3);
    pub const LIGHT_PINK: Color = Color::rgb(0xFF, 0xB6, 0xC1);
    pub const LIGHT_SALMON: Color = Color::rgb(0xFF, 0xA0, 0x7A);
    pub const LIGHT_SEA_GREEN: Color = Color::rgb(0x20, 0xB2, 0xAA);
    pub const LIGHT_SKY_BLUE: Color = Color::rgb(0x87, 0xCE, 0xFA);
    pub const LIGHT_SLATE_GRAY: Color = Color::rgb(0x77, 0x88, 0x99);
    pub const LIGHT_SLATE_GREY: Color = Color::rgb(0x77, 0x88, 0x99);
    pub const LIGHT_STEEL_BLUE: Color = Color::rgb(0xB0, 0xC4, 0xDE);
    pub const LIGHT_YELLOW: Color = Color::rgb(0xFF, 0xFF, 0xE0);
    pub const LIME: Color = Color::rgb(0x0, 0xFF, 0x0);
    pub const LIME_GREEN: Color = Color::rgb(0x32, 0xCD, 0x32);
    pub const LINEN: Color = Color::rgb(0xFA, 0xF0, 0xE6);
    pub const MAGENTA: Color = Color::rgb(0xFF, 0x0, 0xFF);
    pub const MAROON: Color = Color::rgb(0x80, 0x0, 0x0);
    pub const MEDIUMAQUA_MARINE: Color = Color::rgb(0x66, 0xCD, 0xAA);
    pub const MEDIUM_BLUE: Color = Color::rgb(0x0, 0x0, 0xCD);
    pub const MEDIUM_ORCHID: Color = Color::rgb(0xBA, 0x55, 0xD3);
    pub const MEDIUM_PURPLE: Color = Color::rgb(0x93, 0x70, 0xDB);
    pub const MEDIUM_SEA_GREEN: Color = Color::rgb(0x3C, 0xB3, 0x71);
    pub const MEDIUM_SLATE_BLUE: Color = Color::rgb(0x7B, 0x68, 0xEE);
    pub const MEDIUM_SPRING_GREEN: Color = Color::rgb(0x0, 0xFA, 0x9A);
    pub const MEDIUM_TURQUOISE: Color = Color::rgb(0x48, 0xD1, 0xCC);
    pub const MEDIUM_VIOLET_RED: Color = Color::rgb(0xC7, 0x15, 0x85);
    pub const MIDNIGHT_BLUE: Color = Color::rgb(0x19, 0x19, 0x70);
    pub const MINT_CREAM: Color = Color::rgb(0xF5, 0xFF, 0xFA);
    pub const MISTY_ROSE: Color = Color::rgb(0xFF, 0xE4, 0xE1);
    pub const MOCCASIN: Color = Color::rgb(0xFF, 0xE4, 0xB5);
    pub const NAVAJO_WHITE: Color = Color::rgb(0xFF, 0xDE, 0xAD);
    pub const NAVY: Color = Color::rgb(0x0, 0x0, 0x80);
    pub const OLD_LACE: Color = Color::rgb(0xFD, 0xF5, 0xE6);
    pub const OLIVE: Color = Color::rgb(0x80, 0x80, 0x0);
    pub const OLIVE_DRAB: Color = Color::rgb(0x6B, 0x8E, 0x23);
    pub const ORANGE: Color = Color::rgb(0xFF, 0xA5, 0x0);
    pub const ORANGE_RED: Color = Color::rgb(0xFF, 0x45, 0x0);
    pub const ORCHID: Color = Color::rgb(0xDA, 0x70, 0xD6);
    pub const PALE_GOLDENROD: Color = Color::rgb(0xEE, 0xE8, 0xAA);
    pub const PALE_GREEN: Color = Color::rgb(0x98, 0xFB, 0x98);
    pub const PALE_TURQUOISE: Color = Color::rgb(0xAF, 0xEE, 0xEE);
    pub const PALE_VIOLET_RED: Color = Color::rgb(0xDB, 0x70, 0x93);
    pub const PAPAYA_WHIP: Color = Color::rgb(0xFF, 0xEF, 0xD5);
    pub const PEACH_PUFF: Color = Color::rgb(0xFF, 0xDA, 0xB9);
    pub const PERU: Color = Color::rgb(0xCD, 0x85, 0x3F);
    pub const PINK: Color = Color::rgb(0xFF, 0xC0, 0xCB);
    pub const PLUM: Color = Color::rgb(0xDD, 0xA0, 0xDD);
    pub const POWDER_BLUE: Color = Color::rgb(0xB0, 0xE0, 0xE6);
    pub const PURPLE: Color = Color::rgb(0x80, 0x0, 0x80);
    pub const REBECCA_PURPLE: Color = Color::rgb(0x66, 0x33, 0x99);
    pub const RED: Color = Color::rgb(0xFF, 0x0, 0x0);
    pub const ROSY_BROWN: Color = Color::rgb(0xBC, 0x8F, 0x8F);
    pub const ROYAL_BLUE: Color = Color::rgb(0x41, 0x69, 0xE1);
    pub const SADDLE_BROWN: Color = Color::rgb(0x8B, 0x45, 0x13);
    pub const SALMON: Color = Color::rgb(0xFA, 0x80, 0x72);
    pub const SANDY_BROWN: Color = Color::rgb(0xF4, 0xA4, 0x60);
    pub const SEA_GREEN: Color = Color::rgb(0x2E, 0x8B, 0x57);
    pub const SEA_SHELL: Color = Color::rgb(0xFF, 0xF5, 0xEE);
    pub const SIENNA: Color = Color::rgb(0xA0, 0x52, 0x2D);
    pub const SILVER: Color = Color::rgb(0xC0, 0xC0, 0xC0);
    pub const SKY_BLUE: Color = Color::rgb(0x87, 0xCE, 0xEB);
    pub const SLATE_BLUE: Color = Color::rgb(0x6A, 0x5A, 0xCD);
    pub const SLATE_GRAY: Color = Color::rgb(0x70, 0x80, 0x90);
    pub const SLATE_GREY: Color = Color::rgb(0x70, 0x80, 0x90);
    pub const SNOW: Color = Color::rgb(0xFF, 0xFA, 0xFA);
    pub const SPRING_GREEN: Color = Color::rgb(0x0, 0xFF, 0x7F);
    pub const STEEL_BLUE: Color = Color::rgb(0x46, 0x82, 0xB4);
    pub const TAN: Color = Color::rgb(0xD2, 0xB4, 0x8C);
    pub const TEAL: Color = Color::rgb(0x0, 0x80, 0x80);
    pub const THISTLE: Color = Color::rgb(0xD8, 0xBF, 0xD8);
    pub const TOMATO: Color = Color::rgb(0xFF, 0x63, 0x47);
    pub const TRANSPARENT: Color = Color::rgb(0x0, 0x0, 0x0);
    pub const TURQUOISE: Color = Color::rgb(0x40, 0xE0, 0xD0);
    pub const VIOLET: Color = Color::rgb(0xEE, 0x82, 0xEE);
    pub const WHEAT: Color = Color::rgb(0xF5, 0xDE, 0xB3);
    pub const WHITE: Color = Color::rgb(0xFF, 0xFF, 0xFF);
    pub const WHITE_SMOKE: Color = Color::rgb(0xF5, 0xF5, 0xF5);
    pub const YELLOW: Color = Color::rgb(0xFF, 0xFF, 0x0);
    pub const YELLOW_GREEN: Color = Color::rgb(0x9A, 0xCD, 0x32);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hsv_to_rgb() {
        assert_eq!(hsv!(0.0, 0.0, 0.0).to_rgb(), rgb!(0, 0, 0)); // Black
        assert_eq!(hsv!(0.0, 0.0, 1.0).to_rgb(), rgb!(255, 255, 255)); // White
        assert_eq!(hsv!(0.0, 1.0, 1.0).to_rgb(), rgb!(255, 0, 0)); // Red
        assert_eq!(hsv!(120.0, 1.0, 1.0).to_rgb(), rgb!(0, 255, 0)); // Lime
        assert_eq!(hsv!(240.0, 1.0, 1.0).to_rgb(), rgb!(0, 0, 255)); // Blue
        assert_eq!(hsv!(60.0, 1.0, 1.0).to_rgb(), rgb!(255, 255, 0)); // Yellow
        assert_eq!(hsv!(180.0, 1.0, 1.0).to_rgb(), rgb!(0, 255, 255)); // Cyan
        assert_eq!(hsv!(300.0, 1.0, 1.0).to_rgb(), rgb!(255, 0, 255)); // Magenta
        assert_eq!(hsv!(0.0, 0.0, 0.75).to_rgb(), rgb!(191, 191, 191)); // Silver
        assert_eq!(hsv!(0.0, 0.0, 0.5).to_rgb(), rgb!(128, 128, 128)); // Gray
        assert_eq!(hsv!(0.0, 1.0, 0.5).to_rgb(), rgb!(128, 0, 0)); // Maroon
        assert_eq!(hsv!(60.0, 1.0, 0.5).to_rgb(), rgb!(128, 128, 0)); // Olive
        assert_eq!(hsv!(120.0, 1.0, 0.5).to_rgb(), rgb!(0, 128, 0)); // Green
        assert_eq!(hsv!(300.0, 1.0, 0.5).to_rgb(), rgb!(128, 0, 128)); // Purple
        assert_eq!(hsv!(180.0, 1.0, 0.5).to_rgb(), rgb!(0, 128, 128)); // Teal
        assert_eq!(hsv!(240.0, 1.0, 0.5).to_rgb(), rgb!(0, 0, 128)); // Navy
    }

    #[test]
    fn test_rgb_to_hsv() {
        assert_eq!(rgb!(0, 0, 0).to_hsv(), hsv!(0.0, 0.0, 0.0)); // Black
        assert_eq!(rgb!(255, 255, 255).to_hsv(), hsv!(0.0, 0.0, 1.0)); // White
        assert_eq!(rgb!(255, 0, 0).to_hsv(), hsv!(0.0, 1.0, 1.0)); // Red
        assert_eq!(rgb!(0, 255, 0).to_hsv(), hsv!(120.0, 1.0, 1.0)); // Lime
        assert_eq!(rgb!(0, 0, 255).to_hsv(), hsv!(240.0, 1.0, 1.0)); // Blue
        assert_eq!(rgb!(255, 255, 0).to_hsv(), hsv!(60.0, 1.0, 1.0)); // Yellow
        assert_eq!(rgb!(0, 255, 255).to_hsv(), hsv!(180.0, 1.0, 1.0)); // Cyan
        assert_eq!(rgb!(255, 0, 255).to_hsv(), hsv!(300.0, 1.0, 1.0)); // Magenta

        assert_eq!(rgb!(191, 191, 191).to_hsv(), hsv!(0.0, 0.0, 0.7490196)); // Silver
        assert_eq!(rgb!(128, 128, 128).to_hsv(), hsv!(0.0, 0.0, 0.5019608)); // Gray
        assert_eq!(rgb!(128, 0, 0).to_hsv(), hsv!(0.0, 1.0, 0.5019608)); // Maroon
        assert_eq!(rgb!(128, 128, 0).to_hsv(), hsv!(60.0, 1.0, 0.5019608)); // Olive
        assert_eq!(rgb!(0, 128, 0).to_hsv(), hsv!(120.0, 1.0, 0.5019608)); // Green
        assert_eq!(rgb!(128, 0, 128).to_hsv(), hsv!(300.0, 1.0, 0.5019608)); // Purple
        assert_eq!(rgb!(0, 128, 128).to_hsv(), hsv!(180.0, 1.0, 0.5019608)); // Teal
        assert_eq!(rgb!(0, 0, 128).to_hsv(), hsv!(240.0, 1.0, 0.5019608)); // Navy
    }
}
