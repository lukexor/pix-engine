//! [Color] functions for drawing.
//!
//! Each `Color` can be represented as [Rgb], [Hsb], or [Hsl] and can be converted from/into other
//! representations as needed. The default mode is [Rgb] with values ranging from `0..=255` for
//! red, green, blue, and alpha. [Hsb] and [Hsl] values range from `0.0..=360.0` for hue,
//! `0.0..=100.0` for saturation and brightness/lightness and `0.0..=1.0` for alpha.
//!
//! There are convience macros for easy construction: [rgb!], [hsb!] and [hsl!] that take 1-4
//! parameters. The number of parameters provided alter how they are interpreted.
//!
//! Providing a single parameter results in a grayscale color. Two parameters results in grayscale
//! with alpha transparency. Three parameters are used as RGB or HSB/HSL values with a fourth
//! parameter used to apply alpha transparency.
//!
//! The `Color` instance will remember which mode it was created in, modifying how manipulation
//! methods are interprted and can be changed to different modes as needed.
//!
//! There are also several named color [constants](constants) available in the
//! [prelude](crate::prelude) matching the [SVG 1.0 Color
//! Keywords](https://www.w3.org/TR/SVG11/types.html#ColorKeywords).
//!
//! # Examples
//!
//! `Rgb` values range from `0..=255` for red, green, blue and alpha transparency.
//!
//! ```
//! # use pix_engine::prelude::*;
//!
//! let c = rgb!(55); // Grayscale
//! assert_eq!(c.channels(), [55, 55, 55, 255]);
//!
//! let c = rgb!(55, 128); // Grayscale with alpha
//! assert_eq!(c.channels(), [55, 55, 55, 128]);
//!
//! let c = rgb!(128, 0, 55); // Red, Green, Blue
//! assert_eq!(c.channels(), [128, 0, 55, 255]);
//!
//! let c = rgb!(128, 0, 55, 128); // Red, Green, Blue, and Alpha
//! assert_eq!(c.channels(), [128, 0, 55, 128]);
//! ```
//!
//! `Hsb`/`Hsl` values range from `0.0..=360.0` for hue, `0.0..=100.0` for saturation and
//! brightness/lightness and `0.0..=1.0` for alpha transparency.
//!
//! ```
//! # use pix_engine::prelude::*;
//!
//! let c = hsb!(50.0); // Grayscale
//! assert_eq!(c.levels(), [0.0, 0.0, 0.5, 1.0]);
//!
//! let c = hsb!(50.0, 0.8); // Grayscale with alpha
//! assert_eq!(c.levels(), [0.0, 0.0, 0.5, 0.8]);
//!
//! let c = hsb!(126.0, 100.0, 50.0); // Hue, Saturation, Value
//! assert_eq!(c.levels(), [0.35, 1.0, 0.5, 1.0]);
//!
//! let c = hsb!(234.0, 80.0, 100.0, 0.8); // Hue, Saturation, Value, and Alpha
//! assert_eq!(c.levels(), [0.65, 0.8, 1.0, 0.8]);
//! ```
//!
//! SVG 1.0 Named color constants
//!
//! ```
//! # use pix_engine::prelude::*;
//!
//! let c = ALICE_BLUE;
//! assert_eq!(c.channels(), [240, 248, 255, 255]);
//!
//! let c = DARK_ORCHID;
//! assert_eq!(c.channels(), [153, 50, 204, 255]);
//! ```
//!
//! You can also create colors from hexidecimal strings using 3, 4, 6, or 8-digit formats.
//!
//! ```
//! # use pix_engine::prelude::*;
//! use std::str::FromStr;
//!
//! let c = Color::from_str("#F0F")?; // 3-digit Hex string
//! assert_eq!(c.channels(), [255, 0, 255, 255]);
//!
//! let c = Color::from_str("#F0F5")?; // 4-digit Hex string
//! assert_eq![c.channels(), [255, 0, 255, 85]];
//!
//! let c = Color::from_str("#F0F5BF")?; // 6-digit Hex string
//! assert_eq!(c.channels(), [240, 245, 191, 255]);
//!
//! let c = Color::from_str("#F0F5BF5F")?; // 8-digit Hex string
//! assert_eq!(c.channels(), [240, 245, 191, 95]);
//! # Ok::<(), ColorError>(())
//! ```

use crate::random;
use conversion::{calculate_channels, convert_levels, maxes};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    error,
    fmt::{self, LowerHex, UpperHex},
    ops::*,
};

pub mod constants;
pub mod conversion;

/// Color mode indicating level interpretation.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ColorMode {
    /// Red, Green, and Blue
    Rgb,
    /// Hue, Saturation, and Brightness
    Hsb,
    /// Hue, Saturation, and Lightness
    Hsl,
}

use ColorMode::*;

/// A color represented as [Rgb], [Hsb], or [Hsl].
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Color {
    /// Color mode.
    mode: ColorMode,
    /// RGB values ranging `0.0..=1.0`.
    levels: [f64; 4],
    /// RGB values ranging from `0..=255`.
    channels: [u8; 4],
}

impl Color {
    /// Constructs a [Rgb] `Color` and max alpha.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::new(0, 0, 128);
    /// assert_eq!(c.channels(), [0, 0, 128, 255]);
    /// ```
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self::rgb(r, g, b)
    }

    /// Constructs a [Rgb] `Color` with alpha.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::new_alpha(0, 0, 128, 50);
    /// assert_eq!(c.channels(), [0, 0, 128, 50]);
    /// ```
    pub fn new_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::rgba(r, g, b, a)
    }

    /// Constructs a `Color` with the given [ColorMode] and max alpha.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::with_mode(ColorMode::Rgb, 0.0, 0.0, 128.0);
    /// assert_eq!(c.channels(), [0, 0, 128, 255]);
    ///
    /// let c = Color::with_mode(ColorMode::Hsb, 126.0, 50.0, 100.0);
    /// assert_eq!(c.levels(), [0.35, 0.5, 1.0, 1.0]);
    /// ```
    pub fn with_mode(mode: ColorMode, v1: f64, v2: f64, v3: f64) -> Self {
        let [_, _, _, alpha_max] = maxes(mode);
        Self::with_mode_alpha(mode, v1, v2, v3, alpha_max)
    }

    /// Constructs a `Color` with the given [ColorMode] and alpha.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::with_mode_alpha(ColorMode::Rgb, 0.0, 0.0, 128.0, 50.0);
    /// assert_eq!(c.channels(), [0, 0, 128, 50]);
    ///
    /// let c = Color::with_mode_alpha(ColorMode::Hsb, 126.0, 50.0, 100.0, 0.8);
    /// assert_eq!(c.levels(), [0.35, 0.5, 1.0, 0.8]);
    /// ```
    pub fn with_mode_alpha(mode: ColorMode, v1: f64, v2: f64, v3: f64, alpha: f64) -> Self {
        // Normalize channels
        let [v1_max, v2_max, v3_max, alpha_max] = maxes(mode);
        let levels = [
            (v1 / v1_max).clamp(0.0, 1.0),
            (v2 / v2_max).clamp(0.0, 1.0),
            (v3 / v3_max).clamp(0.0, 1.0),
            (alpha / alpha_max).clamp(0.0, 1.0),
        ];

        // Convert to `Rgb`
        let levels = convert_levels(levels, mode, Rgb);
        let channels = calculate_channels(levels);

        Self {
            mode,
            levels,
            channels,
        }
    }

    /// Constructs a [Rgb] `Color` containing red, green, and blue with alpha of
    /// 255.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(128, 64, 0);
    /// assert_eq!(c.channels(), [128, 64, 0, 255]);
    /// ```
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::with_mode(Rgb, f64::from(r), f64::from(g), f64::from(b))
    }

    /// Constructs a [Rgb] `Color` containing red, green, blue, and alpha.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgba(128, 64, 128, 128);
    /// assert_eq!(c.channels(), [128, 64, 128, 128]);
    /// ```
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::with_mode_alpha(Rgb, f64::from(r), f64::from(g), f64::from(b), f64::from(a))
    }

    /// Constructs a [Hsb] `Color` containing hue, saturation, and brightness
    /// with alpha of 1.0.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::hsb(126.0, 80.0, 0.0);
    /// assert_eq!(c.levels(), [0.35, 0.8, 0.0, 1.0]);
    /// ```
    pub fn hsb(h: f64, s: f64, b: f64) -> Self {
        Self::with_mode(Hsb, h, s, b)
    }

    /// Constructs a [Hsb] `Color` containing hue, saturation, brightness and
    /// alpha.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::hsba(126.0, 80.0, 0.0, 0.5);
    /// assert_eq!(c.levels(), [0.35, 0.8, 0.0, 0.5]);
    /// ```
    pub fn hsba(h: f64, s: f64, b: f64, a: f64) -> Self {
        Self::with_mode_alpha(Hsb, h, s, b, a)
    }

    /// Constructs a [Hsl] `Color` containing hue, saturation, and lightness
    /// with alpha of 1.0.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::hsl(126.0, 80.0, 0.0);
    /// assert_eq!(c.levels(), [0.35, 0.8, 0.0, 1.0]);
    /// ```
    pub fn hsl(h: f64, s: f64, l: f64) -> Self {
        Self::with_mode(Hsl, h, s, l)
    }

    /// Constructs a [Hsl] `Color` containing hue, saturation, lightness and
    /// alpha.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::hsla(126.0, 80.0, 0.0, 0.5);
    /// assert_eq!(c.levels(), [0.35, 0.8, 0.0, 0.5]);
    /// ```
    pub fn hsla(h: f64, s: f64, l: f64, a: f64) -> Self {
        Self::with_mode_alpha(Hsl, h, s, l, a)
    }

    /// Constructs a raw `Color` with the given [ColorMode] and alpha using the levels passed in
    /// as-is without normalizing them.
    ///
    /// # Safety
    ///
    /// This may result in unexpected behavior if values are outside the range `0.0..=1.0`. It is
    /// up to the responsibility of the caller to hold this invariant.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::from_raw(Rgb, 0.4, 0.5, 1.0, 0.8);
    /// assert_eq!(c.channels(), [102, 128, 255, 204]);
    /// ```
    pub unsafe fn from_raw(mode: ColorMode, v1: f64, v2: f64, v3: f64, alpha: f64) -> Self {
        let levels = [v1, v2, v3, alpha];
        Self {
            mode,
            levels,
            channels: calculate_channels(levels),
        }
    }

    /// Constructs a random [Rgb] `Color` with max alpha.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # #[allow(unused_variables)]
    /// let c = Color::random();
    /// // `c.channels()` will return something like:
    /// // [207, 12, 217, 255]
    /// ```
    pub fn random() -> Self {
        Self::new(random!(255), random!(255), random!(255))
    }

    /// Constructs a random [Rgb] `Color` with alpha.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # #[allow(unused_variables)]
    /// let c = Color::random_alpha()
    /// // `c.channels()` will return something like:
    /// // [132, 159, 233, 76]
    /// ```
    pub fn random_alpha() -> Self {
        Self::new_alpha(random!(255), random!(255), random!(255), random!(255))
    }

    /// Constructs a `Color` from a slice of 1-4 values. The number of values
    /// provided alter how they are interpreted similar to the [rgb!], [hsb!],
    /// and [hsl!] macros.
    ///
    /// # Errors
    ///
    /// If the slice is empty or has more than 4 values, an error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let vals: Vec<f64> = vec![128.0, 64.0, 0.0];
    /// let c = Color::from_slice(ColorMode::Rgb, &vals)?; // RGB Vec
    /// assert_eq!(c.channels(), [128, 64, 0, 255]);
    ///
    /// let vals: [f64; 4] = [128.0, 64.0, 0.0, 128.0];
    /// let c = Color::from_slice(ColorMode::Rgb, &vals[..])?; // RGBA slice
    /// assert_eq!(c.channels(), [128, 64, 0, 128]);
    /// # Ok::<(), ColorError>(())
    /// ```
    pub fn from_slice(mode: ColorMode, slice: &[f64]) -> Result<Self, ColorError> {
        match *slice {
            [gray] => Ok(Self::with_mode(mode, gray, gray, gray)),
            [gray, a] => Ok(Self::with_mode_alpha(mode, gray, gray, gray, a)),
            [v1, v2, v3] => Ok(Self::with_mode(mode, v1, v2, v3)),
            [v1, v2, v3, a] => Ok(Self::with_mode_alpha(mode, v1, v2, v3, a)),
            _ => Err(ColorError::InvalidSlice(Cow::from(slice.to_owned()))),
        }
    }

    /// Constructs a [Rgb] `Color` from a [u32] RGBA hexadecimal value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::from_hex(0xF0FF00FF);
    /// assert_eq!(c.channels(), [240, 255, 0, 255]);
    ///
    /// let c = Color::from_hex(0xF0FF0080);
    /// assert_eq!(c.channels(), [240, 255, 0, 128]);
    /// ```
    pub fn from_hex(hex: u32) -> Self {
        let [r, g, b, a] = hex.to_be_bytes();
        Self::rgba(r, g, b, a)
    }

    /// Returns a list of max values for each color channel.
    pub fn maxes(&self) -> [f64; 4] {
        maxes(self.mode)
    }

    /// Returns the `Color` levels which range from [0.0, 1.0].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgba(128, 64, 128, 128);
    /// assert_eq!(c.channels(), [128, 64, 128, 128]);
    /// ```
    pub const fn levels(&self) -> [f64; 4] {
        self.levels
    }

    /// Returns the `Color` [Rgb] channels which range from [0, 255].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgba(128, 64, 128, 128);
    /// assert_eq!(c.channels(), [128, 64, 128, 128]);
    /// ```
    #[inline]
    pub fn channels(&self) -> [u8; 4] {
        self.channels
    }
}

/// # Constructs a [Rgb] [Color].
///
/// # Examples
///
/// ```
/// # use pix_engine::prelude::*;
///
/// let c = rgb!(128); // Gray
/// assert_eq!(c.channels(), [128, 128, 128, 255]);
///
/// let c = rgb!(128, 64); // Gray with alpha
/// assert_eq!(c.channels(), [128, 128, 128, 64]);
///
/// let c = rgb!(128, 64, 0); // Red, Green, Blue
/// assert_eq!(c.channels(), [128, 64, 0, 255]);
///
/// let c = rgb!(128, 64, 128, 128); // Red, Green, Blue, Alpha
/// assert_eq!(c.channels(), [128, 64, 128, 128]);
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

/// # Constructs a [Hsb] [Color].
///
/// # Examples
///
/// ```
/// # use pix_engine::prelude::*;
///
/// let c = hsb!(50.0); // Gray
/// assert_eq!(c.levels(), [0.0, 0.0, 0.5, 1.0]);
///
/// let c = hsb!(50.0, 0.5); // Gray with alpha
/// assert_eq!(c.levels(), [0.0, 0.0, 0.5, 0.5]);
///
/// let c = hsb!(342.0, 100.0, 80.0); // Hue, Saturation, Brightness
/// assert_eq!(c.levels(), [0.95, 1.0, 0.8, 1.0]);
///
/// let c = hsb!(342.0, 100.0, 80.0, 0.5); // Hue, Saturation, Brightness, Alpha
/// assert_eq!(c.levels(), [0.95, 1.0, 0.8, 0.5]);
/// ```
#[macro_export]
macro_rules! hsb {
    ($gray:expr) => {
        hsb!(0.0, 0.0, $gray)
    };
    ($gray:expr, $a:expr$(,)?) => {
        hsb!(0.0, 0.0, $gray, $a)
    };
    ($h:expr, $s:expr, $b:expr$(,)?) => {
        hsb!($h, $s, $b, 1.0)
    };
    ($h:expr, $s:expr, $b:expr, $a:expr$(,)?) => {
        $crate::color::Color::hsba($h, $s, $b, $a)
    };
}

/// # Constructs a [Hsl] [Color].
///
/// # Examples
///
/// ```
/// # use pix_engine::prelude::*;
///
/// let c = hsl!(50.0); // Gray
/// assert_eq!(c.levels(), [0.0, 0.0, 0.5, 1.0]);
///
/// let c = hsl!(50.0, 0.5); // Gray with alpha
/// assert_eq!(c.levels(), [0.0, 0.0, 0.5, 0.5]);
///
/// let c = hsl!(342.0, 100.0, 80.0); // Hue, Saturation, Lightness
/// assert_eq!(c.levels(), [0.95, 1.0, 0.8, 1.0]);
///
/// let c = hsl!(342.0, 100.0, 80.0, 0.5); // Hue, Saturation, Lightness, Alpha
/// assert_eq!(c.levels(), [0.95, 1.0, 0.8, 0.5]);
/// ```
#[macro_export]
macro_rules! hsl {
    ($gray:expr) => {
        hsl!(0.0, 0.0, $gray)
    };
    ($gray:expr, $a:expr$(,)?) => {
        hsl!(0.0, 0.0, $gray, $a)
    };
    ($h:expr, $s:expr, $l:expr$(,)?) => {
        hsl!($h, $s, $l, 1.0)
    };
    ($h:expr, $s:expr, $l:expr, $a:expr$(,)?) => {
        $crate::color::Color::hsla($h, $s, $l, $a)
    };
}

impl Default for Color {
    fn default() -> Self {
        Self::rgb(0, 0, 0)
    }
}

impl Index<usize> for Color {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            i if i < 4 => self.levels.get(i).unwrap(),
            _ => panic!("index out of bounds: the len is 4 but the index is {}", idx),
        }
    }
}

impl LowerHex for Color {
    #[allow(clippy::many_single_char_names)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [r, g, b, a] = self.channels();
        write!(f, "#{:x}{:x}{:x}{:x}", r, g, b, a)
    }
}

impl UpperHex for Color {
    #[allow(clippy::many_single_char_names)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [r, g, b, a] = self.channels();
        write!(f, "#{:X}{:X}{:X}{:X}", r, g, b, a)
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Color) -> Self::Output {
        let [v1, v2, v3, a] = self.levels();
        let [ov1, ov2, ov3, oa] = other.levels();
        Self::with_mode_alpha(self.mode, v1 + ov1, v2 + ov2, v3 + ov3, a + oa)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        for i in 0..4 {
            self.levels[i] = (self.levels[i] + other.levels[i]).clamp(0.0, 1.0);
        }
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, other: Color) -> Self::Output {
        let [v1, v2, v3, a] = self.levels();
        let [ov1, ov2, ov3, oa] = other.levels();
        Self::with_mode_alpha(self.mode, v1 - ov1, v2 - ov2, v3 - ov3, a - oa)
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, other: Color) {
        for i in 0..4 {
            self.levels[i] = (self.levels[i] - other.levels[i]).clamp(0.0, 1.0);
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, s: f32) -> Self::Output {
        let [v1, v2, v3, a] = self.levels();
        Self::with_mode_alpha(
            self.mode,
            v1 * s as f64,
            v2 * s as f64,
            v3 * s as f64,
            a * s as f64,
        )
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, s: f32) {
        for i in 0..4 {
            self.levels[i] = (self.levels[i] * s as f64).clamp(0.0, 1.0);
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, s: f64) -> Self::Output {
        let [v1, v2, v3, a] = self.levels();
        Self::with_mode_alpha(self.mode, v1 * s, v2 * s, v3 * s, a * s)
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, s: f64) {
        for i in 0..4 {
            self.levels[i] = (self.levels[i] * s).clamp(0.0, 1.0);
        }
    }
}

impl Div<f32> for Color {
    type Output = Self;
    fn div(self, s: f32) -> Self::Output {
        let [v1, v2, v3, a] = self.levels();
        Self::with_mode_alpha(
            self.mode,
            v1 / s as f64,
            v2 / s as f64,
            v3 / s as f64,
            a / s as f64,
        )
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, s: f32) {
        for i in 0..4 {
            self.levels[i] = (self.levels[i] / s as f64).clamp(0.0, 1.0);
        }
    }
}

impl Div<f64> for Color {
    type Output = Self;
    fn div(self, s: f64) -> Self::Output {
        let [v1, v2, v3, a] = self.levels();
        Self::with_mode_alpha(self.mode, v1 / s, v2 / s, v3 / s, a / s)
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, s: f64) {
        for i in 0..4 {
            self.levels[i] = (self.levels[i] / s).clamp(0.0, 1.0);
        }
    }
}

/// The error type for [Color] operations.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum ColorError {
    /// Error when creating a [Color] from an invalid slice.
    InvalidSlice(Cow<'static, [f64]>),
    /// Error when creating a [Color] from an invalid string.
    InvalidString(Cow<'static, str>),
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ColorError::*;
        match self {
            InvalidSlice(slice) => write!(f, "invalid color slice: {:?}", slice),
            InvalidString(s) => write!(f, "invalid color string format: {}", s),
        }
    }
}

impl error::Error for ColorError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructors() {
        let expected = |mode| Color {
            mode,
            levels: [0.0, 0.0, 0.0, 1.0],
            channels: [0, 0, 0, 255],
        };
        assert_eq!(Color::with_mode(Rgb, 0.0, 0.0, 0.0), expected(Rgb));
        assert_eq!(
            Color::with_mode_alpha(Rgb, 0.0, 0.0, 0.0, 255.0),
            expected(Rgb)
        );
        assert_eq!(Color::with_mode(Hsb, 0.0, 0.0, 0.0), expected(Hsb));
        assert_eq!(
            Color::with_mode_alpha(Hsb, 0.0, 0.0, 0.0, 1.0),
            expected(Hsb)
        );
        assert_eq!(Color::with_mode(Hsl, 0.0, 0.0, 0.0), expected(Hsl));
        assert_eq!(
            Color::with_mode_alpha(Hsl, 0.0, 0.0, 0.0, 1.0),
            expected(Hsl)
        );
    }
}
