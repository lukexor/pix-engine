//! Color handling and manipulation used for draw operations.
//!
//! Each [Color] can be represented as either [Rgb], or [Hsv] and can be converted from/into other
//! representations as needed.
//!
//! Values can be provided as either integer or floating point. Floating point values will be
//! rounded to the nearest RGBA screen color for drawing.
//!
//! The number of parameters provided alter how they are interpreted. Optional values are in square
//! brackets:
//!
//! # Syntax
//!
//! ```ignore
//! // RGB values range from 0-255
//! rgb!(gray);
//! rgb!(gray, [alpha]);
//! rgb!(red, green, blue, [alpha]);
//! rgb!(hexidecimal);
//! rgb!(array_slice);
//!
//! // HSV values range from 0-360 for hue and 0.0-1.0 for all other values
//! hsv!(gray);
//! hsv!(gray, [alpha]);
//! hsv!(hue, saturation, value, [alpha]);
//! hsv!(hexidecimal);
//! hsv!(array_slice);
//!
//! NAMED_COLOR; // e.g. ALICE_BLUE
//! ```
//!
//! # Parameters
//!
//! - **gray**: Grayscale value ranging from black to white. Ranges from 0 to 255 (RGB) or 0.0 to 1.0 (HSV).
//! - **alpha**: Transparency value. Ranges from 0 to 255 (RGB) or 0.0 to 1.0 (HSV) (Optional).
//! - **v1**: Red (0 to 255) or Hue (0 to 360).
//! - **v2**: Green (0 to 255) or Saturation (0.0 to 1.0).
//! - **v3**: Blue (0 to 255) or Value (0.0 to 1.0).
//! - **NAMED_COLOR**: A Named Color constant in RGB. e.g. ALICE_BLUE.
//! - **hexidecimal**: A hexadecimal string value (in 3, 4, 6, or 8 digit formats, e.g. '#FF0000').
//! - **slice**: An array slice containing red, green, blue or hue, saturation, value channels, and
//!   optionally alpha.
//!
//! There are also methods to create randomized colors. See `Other Examples` for details.
//!
//! # Grayscale Examples
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! let c = rgb!(128); // Gray
//! assert_eq!(c.channels(), (128, 128, 128, 255));
//!
//! let c = rgb!(128); // Gray
//! assert_eq!(c.channels(), (128, 128, 128, 255));
//!
//! let c = rgb!(128, 64); // Gray with Alpha
//! assert_eq!(c.channels(), (128, 128, 128, 64));
//! ```
//!
//! # RGB Examples
//!
//! ```
//! # use pix_engine::prelude::*;
//!
//! let c = rgb!(128, 64, 0); // RGB
//! assert_eq!(c.channels(), (128, 64, 0, 255));
//!
//! let c = rgb!(128, 64, 128, 128); // RGBA
//! assert_eq!(c.channels(), (128, 64, 128, 128));
//! ```
//!
//! # HSV Example
//!
//! ```
//! # use pix_engine::prelude::*;
//!
//! let c = hsv!(337.0, 1.0, 0.8);
//! assert_eq!(c.channels(), (337.0, 1.0, 0.8, 1.0));
//! assert_eq!(c.to_rgb().channels(), (204, 0, 78, 255));
//! ```
//!
//! # Named Color Examples
//!
//! ```
//! # use pix_engine::prelude::*;
//!
//! let c = ALICE_BLUE;
//! assert_eq!(c.channels(), (240, 248, 255, 255));
//!
//! let c = DARK_ORCHID;
//! assert_eq!(c.channels(), (153, 50, 204, 255));
//! ```
//!
//! # Color String Examples
//!
//! ```
//! # use pix_engine::prelude::*;
//! use std::str::FromStr;
//!
//! let c = Rgb::from_str("#F0F").unwrap(); // 3-digit Hex string
//! assert_eq!(c.channels(), (255, 0, 255, 255));
//!
//! let c = Rgb::from_str("#F0F5").unwrap(); // 4-digit Hex string
//! assert_eq!(c.channels(), (255, 0, 255, 85));
//!
//! let c = Rgb::from_str("#F0F5BF").unwrap(); // 6-digit Hex string
//! assert_eq!(c.channels(), (240, 245, 191, 255));
//!
//! let c = Rgb::from_str("#F0F5BF5F").unwrap(); // 8-digit Hex string
//! assert_eq!(c.channels(), (240, 245, 191, 95));
//! ```
//!
//! # Vec/Slice Examples
//! ```
//! # use pix_engine::prelude::*;
//! use std::convert::TryFrom;
//!
//! let vals: Vec<u8> = vec![128, 64, 0];
//! let c = Rgb::from_slice(&vals).unwrap(); // RGB Vec
//! assert_eq!(c.channels(), (128, 64, 0, 255));
//!
//! let vals: [u8; 4] = [128, 64, 0, 128];
//! let c = Rgb::from_slice(&vals[..]).unwrap(); // RGBA slice
//! assert_eq!(c.channels(), (128, 64, 0, 128));
//! ```
//!
//! # Other Examples
//!
//! ```
//! # use pix_engine::prelude::*;
//!
//! let c = Rgb::random();
//! // `c.channels()` will return something like:
//! // (207, 12, 217, 255)
//!
//! let c = Rgb::random_alpha();
//! // `c.channels()` will return something like:
//! // (132, 159, 233, 76)
//!
//! let c = Hsv::random();
//! // `c.channels()` will return something like:
//! // (153.0565, 0.8440677, 0.7508346, 1.0)
//! assert!(c.hue() >= 0.0 && c.hue() <= 360.0);
//! assert!(c.saturation() >= 0.0 && c.saturation() <= 1.0);
//! assert!(c.value() >= 0.0 && c.value() <= 1.0);
//! assert_eq!(c.alpha(), 1.0);
//!
//! let c = Hsv::random_alpha();
//! // `c.channels()` will return something like:
//! // (268.85184, 0.8359635, 0.004390478, 0.5656874)
//! assert!(c.hue() >= 0.0 && c.hue() <= 360.0);
//! assert!(c.saturation() >= 0.0 && c.saturation() <= 1.0);
//! assert!(c.value() >= 0.0 && c.value() <= 1.0);
//! assert!(c.alpha() >= 0.0 && c.alpha() <= 1.0);
//! ```

use crate::random;
use std::{
    convert::TryFrom,
    error,
    fmt::{self, LowerHex, UpperHex},
    ops::{Index, IndexMut},
    str::FromStr,
};

/// # Create an [Rgb] [Color].
///
/// ```
/// # use pix_engine::prelude::*;
///
/// let c = rgb!(128, 64, 0); // RGB
/// assert_eq!(c.channels(), (128, 64, 0, 255));
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
        $crate::color::Rgb::rgba($r, $g, $b, $a)
    };
}

/// # Create an [Hsv] [Color].
///
/// ```
/// # use pix_engine::prelude::*;
///
/// let c = hsv!(337.0, 1.0, 0.8);
/// assert_eq!(c.channels(), (337.0, 1.0, 0.8, 1.0));
/// assert_eq!(c.to_rgb().channels(), (204, 0, 78, 255));
/// ```
#[macro_export]
macro_rules! hsv {
    ($gray:expr) => {
        hsv!($gray * 360.0, $gray, $gray)
    };
    ($gray:expr, $a:expr$(,)?) => {
        hsv!($gray * 360.0, $gray, $gray, $a)
    };
    ($h:expr, $s:expr, $v:expr$(,)?) => {
        hsv!($h, $s, $v, 1.0)
    };
    ($h:expr, $s:expr, $v:expr, $a:expr$(,)?) => {
        $crate::color::Hsv::hsva($h, $s, $v, $a)
    };
}

/// A general `Color` in a specific format like Rgb or Hsv.
///
/// # Examples
/// ```
/// # use pix_engine::prelude::*;
/// use std::convert::TryFrom;
///
/// let c = Color::Rgb(rgb!(255, 0, 0));
/// if let Color::Rgb(rgb) = c {
///     assert_eq!(rgb.channels(), (255, 0, 0, 255));
/// }
///
/// let c = Color::try_from("#fff").unwrap();
/// if let Color::Rgb(rgb) = c {
///     assert_eq!(rgb.channels(), (255, 255, 255, 255));
/// }
/// ```
#[allow(variant_size_differences)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    /// An Rgb instance of `Color`.
    Rgb(Rgb),
    /// A Hsv instance of `Color.
    Hsv(Hsv),
}

impl Color {
    /// Creates a new `Rgb` Color with random red, green, and blue with alpha of 255.
    pub fn random() -> Self {
        Color::Rgb(Rgb::random())
    }

    /// Creates a new `Rgb` Color with random red, green, blue and alpha.
    pub fn random_alpha() -> Self {
        Color::Rgb(Rgb::random_alpha())
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::Rgb(constants::TRANSPARENT)
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
        Color::from_str(s)
    }
}

impl From<u8> for Color {
    fn from(gray: u8) -> Self {
        Color::Rgb(rgb!(gray))
    }
}

impl From<(u8, u8)> for Color {
    fn from((gray, a): (u8, u8)) -> Self {
        Color::Rgb(rgb!(gray, a))
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Color::Rgb(rgb!(r, g, b))
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Color::Rgb(rgb!(r, g, b, a))
    }
}

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Self {
        Color::Rgb(rgb)
    }
}

impl From<f32> for Color {
    fn from(gray: f32) -> Self {
        Color::Hsv(hsv!(gray))
    }
}

impl From<(f32, f32)> for Color {
    fn from((gray, a): (f32, f32)) -> Self {
        Color::Hsv(hsv!(gray, a))
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from((h, s, v): (f32, f32, f32)) -> Self {
        Color::Hsv(hsv!(h, s, v))
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from((h, s, v, a): (f32, f32, f32, f32)) -> Self {
        Color::Hsv(hsv!(h, s, v, a))
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

/// An `Rgb` Color containing Red, Green, Blue, and Alpha channels.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Rgb {
    /// Creates a new `Rgb` Color.
    ///
    /// # Example
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let c = Rgb::rgb(128, 64, 0);
    /// assert_eq!(c.channels(), (128, 64, 0, 255));
    /// ```
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    /// Creates a new `Rgb` Color with Alpha.
    ///
    /// # Example
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let c = Rgb::rgba(128, 64, 128, 128);
    /// assert_eq!(c.channels(), (128, 64, 128, 128));
    /// ```
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Creates a new `Rgb` Color with random red, green, and blue with alpha of 255.
    pub fn random() -> Self {
        Self::rgb(random!(255), random!(255), random!(255))
    }

    /// Creates a new `Rgb` Color with random red, green, blue and alpha.
    pub fn random_alpha() -> Self {
        Self::rgba(random!(255), random!(255), random!(255), random!(255))
    }

    /// Create a new `Rgb` Color from an array slice.
    pub fn from_slice(slice: &[u8]) -> Result<Self, ColorError> {
        match *slice {
            [gray] => Ok(Self::rgb(gray, gray, gray)),
            [gray, a] => Ok(Self::rgba(gray, gray, gray, a)),
            [r, g, b] => Ok(Self::rgb(r, g, b)),
            [r, g, b, a] => Ok(Self::rgba(r, g, b, a)),
            _ => Err(ColorError::InvalidSlice),
        }
    }

    /// Create a new `Rgb` Color from a u32 value.
    pub fn from_hex(hex: u32) -> Self {
        let r = (hex >> 16 & 0xFF) as u8;
        let g = (hex >> 8 & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Self::rgba(r, g, b, 255)
    }

    /// Get the Red channel
    pub const fn red(self) -> u8 {
        self.r
    }

    /// Set the Red channel
    pub fn set_red(&mut self, val: u8) {
        self.r = val;
    }

    /// Get the Green channel
    pub const fn green(self) -> u8 {
        self.g
    }

    /// Set the Green channel
    pub fn set_green(&mut self, val: u8) {
        self.g = val;
    }

    /// Get the Blue channel
    pub const fn blue(self) -> u8 {
        self.b
    }

    /// Set the Blue channel
    pub fn set_blue(&mut self, val: u8) {
        self.b = val;
    }

    /// Get the Alpha channel
    pub const fn alpha(self) -> u8 {
        self.a
    }

    /// Set the Alpha channel
    pub fn set_alpha(&mut self, val: u8) {
        self.a = val;
    }

    /// Get the Red, Green, Blue, and Alpha channels as a tuple u8 values.
    pub const fn channels(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    /// Convert an `Rgb` Color into an `Hsv` Color.
    ///
    /// Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// assert_eq!(rgb!(0, 0, 255).to_hsv(), hsv!(240.0, 1.0, 1.0)); // Blue
    /// ```
    pub fn to_hsv(self) -> Hsv {
        let r1 = self.r as f32 / 255.0;
        let g1 = self.g as f32 / 255.0;
        let b1 = self.b as f32 / 255.0;
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
            hsv!(h, s, c_max)
        } else {
            hsv!(0.0, 0.0, c_max)
        }
    }

    /// Creates a new `Rgb` instance by linear interpolating between two colors by a given amount
    /// between 0.0 and 1.0.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let from = rgb!(255, 0, 0);
    /// let to = rgb!(0, 100, 255);
    /// let lerped = from.lerp(to, 0.5);
    /// assert_eq!(lerped.channels(), (128, 50, 128, 255));
    /// ```
    pub fn lerp(self, c2: Rgb, amt: f32) -> Rgb {
        let amt = amt.clamp(0.0, 1.0);
        let lerp = |start, stop, amt| amt * (stop as f32 - start as f32) + start as f32;
        let r = lerp(self.r, c2.r, amt).round() as u8;
        let g = lerp(self.g, c2.g, amt).round() as u8;
        let b = lerp(self.b, c2.b, amt).round() as u8;
        let a = lerp(self.a, c2.a, amt).round() as u8;

        rgb!(r, g, b, a)
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
            return Err(ColorError::InvalidString);
        }

        let mut channels: [u8; 4] = [0, 0, 0, 255];
        let parse_hex = |hex: &str| {
            if let Ok(value) = u8::from_str_radix(hex, 16) {
                Ok(value)
            } else {
                Err(ColorError::InvalidString)
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
            _ => return Err(ColorError::InvalidString),
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

/// A `Hsv` Color containing Hue, Saturation, Value, and Alpha channels.
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Hsv {
    h: f32,
    s: f32,
    v: f32,
    a: f32,
}

impl Hsv {
    /// Creates a new `Hsv` Color.
    pub fn hsv(h: f32, s: f32, v: f32) -> Self {
        Self::hsva(h, s, v, 1.0)
    }

    /// Creates a new `Hsv` Color with Alpha.
    pub fn hsva(h: f32, s: f32, v: f32, a: f32) -> Self {
        Self {
            h: h.clamp(0.0, 360.0),
            s: s.clamp(0.0, 1.0),
            v: v.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        }
    }

    /// Creates a new `Hsv` Color with random hue, saturation, and value with alpha of 1.0.
    pub fn random() -> Self {
        Self::hsv(random!(360.0), random!(1.0), random!(1.0))
    }

    /// Creates a new `Hsv` Color with random hue, saturation, value and alpha.
    pub fn random_alpha() -> Self {
        Self::hsva(random!(360.0), random!(1.0), random!(1.0), random!(1.0))
    }

    /// Get the Hue channel
    pub const fn hue(self) -> f32 {
        self.h
    }

    /// Set the Hue channel
    pub fn set_hue(&mut self, val: f32) {
        self.h = val;
    }

    /// Get the Saturation channel
    pub const fn saturation(self) -> f32 {
        self.s
    }

    /// Set the Saturation channel
    pub fn set_saturation(&mut self, val: f32) {
        self.s = val;
    }

    /// Get the Value channel
    pub const fn value(self) -> f32 {
        self.v
    }

    /// Set the Value channel
    pub fn set_value(&mut self, val: f32) {
        self.v = val;
    }

    /// Get the Alpha channel
    pub const fn alpha(self) -> f32 {
        self.a
    }

    /// Set the Alpha channel
    pub fn set_alpha(&mut self, val: f32) {
        self.a = val;
    }

    /// Get the Hue, Saturation, Value, and Alpha channels as a tuple of f32 values.
    pub const fn channels(&self) -> (f32, f32, f32, f32) {
        (self.h, self.s, self.v, self.a)
    }

    /// Convert an `Hsv` Color into an `Rgb` Color.
    ///
    /// Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// assert_eq!(hsv!(240.0, 1.0, 1.0).to_rgb(), rgb!(0, 0, 255)); // Blue
    /// ```
    pub fn to_rgb(&self) -> Rgb {
        if self.v == 0.0 {
            rgb!(0, 0, 0)
        } else if self.s == 0.0 {
            let gray = (self.v * 255.0).round() as u8;
            rgb!(gray, gray, gray)
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
            rgb!(r, g, b, a)
        }
    }

    /// Creates a new `Hsv` instance by linear interpolating between two colors by a given amount
    /// between 0.0 and 1.0.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let from = hsv!(255.0, 0.0, 0.0);
    /// let to = hsv!(0.0, 1.0, 1.0);
    /// let lerped = from.lerp(to, 0.5);
    /// assert_eq!(lerped.channels(), (127.5, 0.5, 0.5, 1.0));
    /// ```
    pub fn lerp(&self, c2: Hsv, amt: f32) -> Hsv {
        let amt = amt.clamp(0.0, 1.0);

        let lerp = |start, stop, amt| amt * (stop - start) + start;

        let h = lerp(self.h, c2.h, amt);
        let s = lerp(self.s, c2.s, amt);
        let v = lerp(self.v, c2.v, amt);
        let a = lerp(self.a, c2.a, amt);

        hsv!(h, s, v, a)
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

/// Types of errors creating/converting colors can return.
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum ColorError {
    /// Result when attempting to create a Rgb/Hsv color from an invalid slice of values.
    InvalidSlice,
    /// Result when attempting to create a Rgb color from an invalid hexidecimal string.
    InvalidString,
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ColorError::*;
        match self {
            InvalidSlice => write!(f, "invalid color slice"),
            InvalidString => write!(f, "invalid color string format"),
        }
    }
}

impl error::Error for ColorError {}

/// SVG 1.0 color constants: http://www.w3.org/TR/SVG/types.html#ColorKeywords
#[allow(missing_docs)]
pub mod constants {
    use super::Rgb;

    pub const ALICE_BLUE: Rgb = Rgb::rgb(0xF0, 0xF8, 0xFF);
    pub const ANTIQUE_WHITE: Rgb = Rgb::rgb(0xFA, 0xEB, 0xD7);
    pub const AQUA: Rgb = Rgb::rgb(0x0, 0xFF, 0xFF);
    pub const AQUA_MARINE: Rgb = Rgb::rgb(0x7F, 0xFF, 0xD4);
    pub const AZURE: Rgb = Rgb::rgb(0xF0, 0xFF, 0xFF);
    pub const BEIGE: Rgb = Rgb::rgb(0xF5, 0xF5, 0xDC);
    pub const BISQUE: Rgb = Rgb::rgb(0xFF, 0xE4, 0xC4);
    pub const BLACK: Rgb = Rgb::rgb(0x0, 0x0, 0x0);
    pub const BLANCHE_DALMOND: Rgb = Rgb::rgb(0xFF, 0xEB, 0xCD);
    pub const BLUE: Rgb = Rgb::rgb(0x0, 0x0, 0xFF);
    pub const BLUE_VIOLET: Rgb = Rgb::rgb(0x8A, 0x2B, 0xE2);
    pub const BROWN: Rgb = Rgb::rgb(0xA5, 0x2A, 0x2A);
    pub const BURLY_WOOD: Rgb = Rgb::rgb(0xDE, 0xB8, 0x87);
    pub const CADET_BLUE: Rgb = Rgb::rgb(0x5F, 0x9E, 0xA0);
    pub const CHARTREUSE: Rgb = Rgb::rgb(0x7F, 0xFF, 0x0);
    pub const CHOCOLATE: Rgb = Rgb::rgb(0xD2, 0x69, 0x1E);
    pub const CORAL: Rgb = Rgb::rgb(0xFF, 0x7F, 0x50);
    pub const CORNFLOWER_BLUE: Rgb = Rgb::rgb(0x64, 0x95, 0xED);
    pub const CORN_SILK: Rgb = Rgb::rgb(0xFF, 0xF8, 0xDC);
    pub const CRIMSON: Rgb = Rgb::rgb(0xDC, 0x14, 0x3C);
    pub const CYAN: Rgb = Rgb::rgb(0x0, 0xFF, 0xFF);
    pub const DARK_BLUE: Rgb = Rgb::rgb(0x0, 0x0, 0x8B);
    pub const DARK_CYAN: Rgb = Rgb::rgb(0x0, 0x8B, 0x8B);
    pub const DARK_GOLDENROD: Rgb = Rgb::rgb(0xB8, 0x86, 0xB);
    pub const DARK_GRAY: Rgb = Rgb::rgb(0xA9, 0xA9, 0xA9);
    pub const DARK_GREEN: Rgb = Rgb::rgb(0x0, 0x64, 0x0);
    pub const DARK_GREY: Rgb = Rgb::rgb(0xA9, 0xA9, 0xA9);
    pub const DARK_KHAKI: Rgb = Rgb::rgb(0xBD, 0xB7, 0x6B);
    pub const DARK_MAGENTA: Rgb = Rgb::rgb(0x8B, 0x0, 0x8B);
    pub const DARK_OLIVE_GREEN: Rgb = Rgb::rgb(0x55, 0x6B, 0x2F);
    pub const DARK_ORANGE: Rgb = Rgb::rgb(0xFF, 0x8C, 0x0);
    pub const DARK_ORCHID: Rgb = Rgb::rgb(0x99, 0x32, 0xCC);
    pub const DARK_RED: Rgb = Rgb::rgb(0x8B, 0x0, 0x0);
    pub const DARK_SALMON: Rgb = Rgb::rgb(0xE9, 0x96, 0x7A);
    pub const DARK_SEA_GREEN: Rgb = Rgb::rgb(0x8F, 0xBC, 0x8F);
    pub const DARK_SLATE_BLUE: Rgb = Rgb::rgb(0x48, 0x3D, 0x8B);
    pub const DARK_SLATE_GRAY: Rgb = Rgb::rgb(0x2F, 0x4F, 0x4F);
    pub const DARK_SLATE_GREY: Rgb = Rgb::rgb(0x2F, 0x4F, 0x4F);
    pub const DARK_TURQUOISE: Rgb = Rgb::rgb(0x0, 0xCE, 0xD1);
    pub const DARK_VIOLET: Rgb = Rgb::rgb(0x94, 0x0, 0xD3);
    pub const DEEP_PINK: Rgb = Rgb::rgb(0xFF, 0x14, 0x93);
    pub const DEEP_SKY_BLUE: Rgb = Rgb::rgb(0x0, 0xBF, 0xFF);
    pub const DIM_GRAY: Rgb = Rgb::rgb(0x69, 0x69, 0x69);
    pub const DIM_GREY: Rgb = Rgb::rgb(0x69, 0x69, 0x69);
    pub const DODGER_BLUE: Rgb = Rgb::rgb(0x1E, 0x90, 0xFF);
    pub const FIRE_BRICK: Rgb = Rgb::rgb(0xB2, 0x22, 0x22);
    pub const FLORAL_WHITE: Rgb = Rgb::rgb(0xFF, 0xFA, 0xF0);
    pub const FOREST_GREEN: Rgb = Rgb::rgb(0x22, 0x8B, 0x22);
    pub const FUCHSIA: Rgb = Rgb::rgb(0xFF, 0x0, 0xFF);
    pub const GAINSBORO: Rgb = Rgb::rgb(0xDC, 0xDC, 0xDC);
    pub const GHOST_WHITE: Rgb = Rgb::rgb(0xF8, 0xF8, 0xFF);
    pub const GOLD: Rgb = Rgb::rgb(0xFF, 0xD7, 0x0);
    pub const GOLDENROD: Rgb = Rgb::rgb(0xDA, 0xA5, 0x20);
    pub const GRAY: Rgb = Rgb::rgb(0x80, 0x80, 0x80);
    pub const GREEN: Rgb = Rgb::rgb(0x0, 0x80, 0x0);
    pub const GREEN_YELLOW: Rgb = Rgb::rgb(0xAD, 0xFF, 0x2F);
    pub const GREY: Rgb = Rgb::rgb(0x80, 0x80, 0x80);
    pub const HONEYDEW: Rgb = Rgb::rgb(0xF0, 0xFF, 0xF0);
    pub const HOTOINK: Rgb = Rgb::rgb(0xFF, 0x69, 0xB4);
    pub const INDIAN_RED: Rgb = Rgb::rgb(0xCD, 0x5C, 0x5C);
    pub const INDIGO: Rgb = Rgb::rgb(0x4B, 0x0, 0x82);
    pub const IVORY: Rgb = Rgb::rgb(0xFF, 0xFF, 0xF0);
    pub const KHAKI: Rgb = Rgb::rgb(0xF0, 0xE6, 0x8C);
    pub const LAVENDER: Rgb = Rgb::rgb(0xE6, 0xE6, 0xFA);
    pub const LAVENDER_BLUSH: Rgb = Rgb::rgb(0xFF, 0xF0, 0xF5);
    pub const LAWN_GREEN: Rgb = Rgb::rgb(0x7C, 0xFC, 0x0);
    pub const LEMON_CHIFFON: Rgb = Rgb::rgb(0xFF, 0xFA, 0xCD);
    pub const LIGHT_BLUE: Rgb = Rgb::rgb(0xAD, 0xD8, 0xE6);
    pub const LIGHT_CORAL: Rgb = Rgb::rgb(0xF0, 0x80, 0x80);
    pub const LIGHT_CYAN: Rgb = Rgb::rgb(0xE0, 0xFF, 0xFF);
    pub const LIGHT_GOLDENROD_YELLOW: Rgb = Rgb::rgb(0xFA, 0xFA, 0xD2);
    pub const LIGHT_GRAY: Rgb = Rgb::rgb(0xD3, 0xD3, 0xD3);
    pub const LIGHT_GREEN: Rgb = Rgb::rgb(0x90, 0xEE, 0x90);
    pub const LIGHT_GREY: Rgb = Rgb::rgb(0xD3, 0xD3, 0xD3);
    pub const LIGHT_PINK: Rgb = Rgb::rgb(0xFF, 0xB6, 0xC1);
    pub const LIGHT_SALMON: Rgb = Rgb::rgb(0xFF, 0xA0, 0x7A);
    pub const LIGHT_SEA_GREEN: Rgb = Rgb::rgb(0x20, 0xB2, 0xAA);
    pub const LIGHT_SKY_BLUE: Rgb = Rgb::rgb(0x87, 0xCE, 0xFA);
    pub const LIGHT_SLATE_GRAY: Rgb = Rgb::rgb(0x77, 0x88, 0x99);
    pub const LIGHT_SLATE_GREY: Rgb = Rgb::rgb(0x77, 0x88, 0x99);
    pub const LIGHT_STEEL_BLUE: Rgb = Rgb::rgb(0xB0, 0xC4, 0xDE);
    pub const LIGHT_YELLOW: Rgb = Rgb::rgb(0xFF, 0xFF, 0xE0);
    pub const LIME: Rgb = Rgb::rgb(0x0, 0xFF, 0x0);
    pub const LIME_GREEN: Rgb = Rgb::rgb(0x32, 0xCD, 0x32);
    pub const LINEN: Rgb = Rgb::rgb(0xFA, 0xF0, 0xE6);
    pub const MAGENTA: Rgb = Rgb::rgb(0xFF, 0x0, 0xFF);
    pub const MAROON: Rgb = Rgb::rgb(0x80, 0x0, 0x0);
    pub const MEDIUMAQUA_MARINE: Rgb = Rgb::rgb(0x66, 0xCD, 0xAA);
    pub const MEDIUM_BLUE: Rgb = Rgb::rgb(0x0, 0x0, 0xCD);
    pub const MEDIUM_ORCHID: Rgb = Rgb::rgb(0xBA, 0x55, 0xD3);
    pub const MEDIUM_PURPLE: Rgb = Rgb::rgb(0x93, 0x70, 0xDB);
    pub const MEDIUM_SEA_GREEN: Rgb = Rgb::rgb(0x3C, 0xB3, 0x71);
    pub const MEDIUM_SLATE_BLUE: Rgb = Rgb::rgb(0x7B, 0x68, 0xEE);
    pub const MEDIUM_SPRING_GREEN: Rgb = Rgb::rgb(0x0, 0xFA, 0x9A);
    pub const MEDIUM_TURQUOISE: Rgb = Rgb::rgb(0x48, 0xD1, 0xCC);
    pub const MEDIUM_VIOLET_RED: Rgb = Rgb::rgb(0xC7, 0x15, 0x85);
    pub const MIDNIGHT_BLUE: Rgb = Rgb::rgb(0x19, 0x19, 0x70);
    pub const MINT_CREAM: Rgb = Rgb::rgb(0xF5, 0xFF, 0xFA);
    pub const MISTY_ROSE: Rgb = Rgb::rgb(0xFF, 0xE4, 0xE1);
    pub const MOCCASIN: Rgb = Rgb::rgb(0xFF, 0xE4, 0xB5);
    pub const NAVAJO_WHITE: Rgb = Rgb::rgb(0xFF, 0xDE, 0xAD);
    pub const NAVY: Rgb = Rgb::rgb(0x0, 0x0, 0x80);
    pub const OLD_LACE: Rgb = Rgb::rgb(0xFD, 0xF5, 0xE6);
    pub const OLIVE: Rgb = Rgb::rgb(0x80, 0x80, 0x0);
    pub const OLIVE_DRAB: Rgb = Rgb::rgb(0x6B, 0x8E, 0x23);
    pub const ORANGE: Rgb = Rgb::rgb(0xFF, 0xA5, 0x0);
    pub const ORANGE_RED: Rgb = Rgb::rgb(0xFF, 0x45, 0x0);
    pub const ORCHID: Rgb = Rgb::rgb(0xDA, 0x70, 0xD6);
    pub const PALE_GOLDENROD: Rgb = Rgb::rgb(0xEE, 0xE8, 0xAA);
    pub const PALE_GREEN: Rgb = Rgb::rgb(0x98, 0xFB, 0x98);
    pub const PALE_TURQUOISE: Rgb = Rgb::rgb(0xAF, 0xEE, 0xEE);
    pub const PALE_VIOLET_RED: Rgb = Rgb::rgb(0xDB, 0x70, 0x93);
    pub const PAPAYA_WHIP: Rgb = Rgb::rgb(0xFF, 0xEF, 0xD5);
    pub const PEACH_PUFF: Rgb = Rgb::rgb(0xFF, 0xDA, 0xB9);
    pub const PERU: Rgb = Rgb::rgb(0xCD, 0x85, 0x3F);
    pub const PINK: Rgb = Rgb::rgb(0xFF, 0xC0, 0xCB);
    pub const PLUM: Rgb = Rgb::rgb(0xDD, 0xA0, 0xDD);
    pub const POWDER_BLUE: Rgb = Rgb::rgb(0xB0, 0xE0, 0xE6);
    pub const PURPLE: Rgb = Rgb::rgb(0x80, 0x0, 0x80);
    pub const REBECCA_PURPLE: Rgb = Rgb::rgb(0x66, 0x33, 0x99);
    pub const RED: Rgb = Rgb::rgb(0xFF, 0x0, 0x0);
    pub const ROSY_BROWN: Rgb = Rgb::rgb(0xBC, 0x8F, 0x8F);
    pub const ROYAL_BLUE: Rgb = Rgb::rgb(0x41, 0x69, 0xE1);
    pub const SADDLE_BROWN: Rgb = Rgb::rgb(0x8B, 0x45, 0x13);
    pub const SALMON: Rgb = Rgb::rgb(0xFA, 0x80, 0x72);
    pub const SANDY_BROWN: Rgb = Rgb::rgb(0xF4, 0xA4, 0x60);
    pub const SEA_GREEN: Rgb = Rgb::rgb(0x2E, 0x8B, 0x57);
    pub const SEA_SHELL: Rgb = Rgb::rgb(0xFF, 0xF5, 0xEE);
    pub const SIENNA: Rgb = Rgb::rgb(0xA0, 0x52, 0x2D);
    pub const SILVER: Rgb = Rgb::rgb(0xC0, 0xC0, 0xC0);
    pub const SKY_BLUE: Rgb = Rgb::rgb(0x87, 0xCE, 0xEB);
    pub const SLATE_BLUE: Rgb = Rgb::rgb(0x6A, 0x5A, 0xCD);
    pub const SLATE_GRAY: Rgb = Rgb::rgb(0x70, 0x80, 0x90);
    pub const SLATE_GREY: Rgb = Rgb::rgb(0x70, 0x80, 0x90);
    pub const SNOW: Rgb = Rgb::rgb(0xFF, 0xFA, 0xFA);
    pub const SPRING_GREEN: Rgb = Rgb::rgb(0x0, 0xFF, 0x7F);
    pub const STEEL_BLUE: Rgb = Rgb::rgb(0x46, 0x82, 0xB4);
    pub const TAN: Rgb = Rgb::rgb(0xD2, 0xB4, 0x8C);
    pub const TEAL: Rgb = Rgb::rgb(0x0, 0x80, 0x80);
    pub const THISTLE: Rgb = Rgb::rgb(0xD8, 0xBF, 0xD8);
    pub const TOMATO: Rgb = Rgb::rgb(0xFF, 0x63, 0x47);
    pub const TRANSPARENT: Rgb = Rgb::rgb(0x0, 0x0, 0x0);
    pub const TURQUOISE: Rgb = Rgb::rgb(0x40, 0xE0, 0xD0);
    pub const VIOLET: Rgb = Rgb::rgb(0xEE, 0x82, 0xEE);
    pub const WHEAT: Rgb = Rgb::rgb(0xF5, 0xDE, 0xB3);
    pub const WHITE: Rgb = Rgb::rgb(0xFF, 0xFF, 0xFF);
    pub const WHITE_SMOKE: Rgb = Rgb::rgb(0xF5, 0xF5, 0xF5);
    pub const YELLOW: Rgb = Rgb::rgb(0xFF, 0xFF, 0x0);
    pub const YELLOW_GREEN: Rgb = Rgb::rgb(0x9A, 0xCD, 0x32);
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
