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

use hsv::Hsv;
use rgb::Rgb;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    convert::TryFrom,
    error,
    fmt::{self, LowerHex, UpperHex},
    ops::*,
    str::FromStr,
};

pub mod constants;
pub mod hsv;
pub mod rgb;

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

impl LowerHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rgb(rgb) => LowerHex::fmt(rgb, f),
            Self::Hsv(hsv) => LowerHex::fmt(&hsv.to_rgb(), f),
        }
    }
}

impl UpperHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rgb(rgb) => UpperHex::fmt(rgb, f),
            Self::Hsv(hsv) => UpperHex::fmt(&hsv.to_rgb(), f),
        }
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, color: Color) -> Self::Output {
        match self {
            Self::Rgb(rgb) => {
                let (r, g, b, a) = color.rgb_channels();
                rgb!(
                    rgb.r.saturating_add(r),
                    rgb.g.saturating_add(g),
                    rgb.b.saturating_add(b),
                    rgb.a.saturating_add(a)
                )
            }
            Self::Hsv(hsv) => {
                let (_, s, _, a) = color.hsv_channels();
                hsv!(
                    hsv.h,
                    (hsv.s + s).clamp(0.0, 1.0),
                    hsv.v,
                    (hsv.a + a).clamp(0.0, 1.0)
                )
            }
        }
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, color: Color) -> Self::Output {
        match self {
            Self::Rgb(rgb) => {
                let (r, g, b, a) = color.rgb_channels();
                rgb!(
                    rgb.r.saturating_sub(r),
                    rgb.g.saturating_sub(g),
                    rgb.b.saturating_sub(b),
                    rgb.a.saturating_sub(a)
                )
            }
            Self::Hsv(hsv) => {
                let (_, s, _, a) = color.hsv_channels();
                hsv!(
                    hsv.h,
                    (hsv.s - s).clamp(0.0, 1.0),
                    hsv.v,
                    (hsv.a - a).clamp(0.0, 1.0)
                )
            }
        }
    }
}

impl Mul<u8> for Color {
    type Output = Self;
    fn mul(self, s: u8) -> Self::Output {
        match self {
            Self::Rgb(rgb) => rgb!(rgb.r * s, rgb.g * s, rgb.b * s),
            Self::Hsv(hsv) => hsv!(hsv.h, hsv.s * s as f32, hsv.v),
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, s: f64) -> Self::Output {
        match self {
            Self::Rgb(rgb) => rgb!(
                (rgb.r as f64 * s).clamp(0.0, 255.0) as u8,
                (rgb.g as f64 * s).clamp(0.0, 255.0) as u8,
                (rgb.b as f64 * s).clamp(0.0, 255.0) as u8
            ),
            Self::Hsv(hsv) => hsv!(hsv.h, hsv.s * s as f32, hsv.v),
        }
    }
}

impl Div<u8> for Color {
    type Output = Self;
    fn div(self, s: u8) -> Self::Output {
        match self {
            Self::Rgb(rgb) => rgb!(rgb.r / s, rgb.g / s, rgb.b / s),
            Self::Hsv(hsv) => hsv!(hsv.h, hsv.s / s as f32, hsv.v),
        }
    }
}

impl Div<f64> for Color {
    type Output = Self;
    fn div(self, s: f64) -> Self::Output {
        match self {
            Self::Rgb(rgb) => rgb!(
                (rgb.r as f64 / s).clamp(0.0, 255.0) as u8,
                (rgb.g as f64 / s).clamp(0.0, 255.0) as u8,
                (rgb.b as f64 / s).clamp(0.0, 255.0) as u8
            ),
            Self::Hsv(hsv) => hsv!(hsv.h, hsv.s / s as f32, hsv.v),
        }
    }
}

impl FromStr for Color {
    type Err = ColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Rgb(Rgb::from_str(s)?))
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
        match color {
            Color::Rgb(rgb) => rgb.into(),
            Color::Hsv(hsv) => hsv.to_rgb().into(),
        }
    }
}

impl From<Color> for (f32, f32, f32, f32) {
    fn from(color: Color) -> Self {
        match color {
            Color::Rgb(rgb) => rgb.to_hsv().into(),
            Color::Hsv(hsv) => hsv.into(),
        }
    }
}

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Self {
        Self::Rgb(rgb)
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
        Self::Hsv(hsv)
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
