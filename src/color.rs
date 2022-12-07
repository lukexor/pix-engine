//! [Color] functions for drawing.
//!
//! Each [Color] can be constructed with a [Mode]. The default mode and internal
//! representation is [Rgb] with values ranging from `0..=255` for red, green, blue, and alpha
//! transparency. [Hsb] and [Hsl] values range from `0.0..=360.0` for hue, `0.0..=100.0` for
//! saturation and brightness/lightness and `0.0..=1.0` for alpha transparency.
//!
//! There are convience macros for flexible construction: [color!], [rgb!], [hsb!] and [hsl!] that
//! take 1-4 parameters. The number of parameters provided alter how they are interpreted:
//!
//! - Providing a single parameter constructs a grayscale color.
//! - Two parameters constructs a grayscale color with alpha transparency.
//! - Three parameters are used as `RGB` or `HSB`/`HSL` values.
//! - Four parameters are used as `RGBB` or `HSb`/`HSL` values with alpha transparency.
//!
//! If you're not picky about color, there are the [random](Color::random) and
//! [`random_alpha`](Color::random_alpha) methods.
//!
//! [Color] also implements [`FromStr`](std::str::FromStr) allowing conversion from a 3, 4, 6, or
//! 8-digit [hexadecimal](https://en.wikipedia.org/wiki/Web_colors) string.
//!
//! The [Color] instance stores which [Mode] it was created with, modifying how manipulation
//! methods are interprted such as [`set_alpha`](Color::set_alpha) taking a range of `0.0..=255.0` or
//! `0.0..=1.0`. The [Mode] can be changed any time to alter this behavior using
//! [`set_mode`](Color::set_mode).
//!
//! There are also several named color [constants] available in the
//! [prelude](crate::prelude) matching the [SVG 1.0 Color
//! Keywords](https://www.w3.org/TR/SVG11/types.html#ColorKeywords).
//!
//! [color!]: crate::prelude::color
//! [rgb!]: crate::prelude::rgb
//! [hsb!]: crate::prelude::hsb
//! [hsl!]: crate::prelude::hsl
//!
//! # Examples
//!
//! Rgb values range from `0..=255` for red, green, blue and alpha transparency.
//!
//! ```
//! use pix_engine::prelude::*;
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
//! use pix_engine::prelude::*;
//!
//! let c = hsb!(50.0); // Gray
//! assert_eq!(c.channels(), [128, 128, 128, 255]);
//!
//! let c = hsb!(50.0, 0.5); // Gray with alpha
//! assert_eq!(c.channels(), [128, 128, 128, 128]);
//!
//! let c = hsb!(342.0, 100.0, 80.0); // Hue, Saturation, Brightness
//! assert_eq!(c.channels(), [204, 0, 61, 255]);
//!
//! let c = hsb!(342.0, 100.0, 80.0, 0.5); // Hue, Saturation, Brightness, Alpha
//! assert_eq!(c.channels(), [204, 0, 61, 128]);
//! ```
//!
//! # SVG 1.0 Named color constants
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! let c = Color::ALICE_BLUE;
//! assert_eq!(c.channels(), [240, 248, 255, 255]);
//!
//! let c = Color::DARK_ORCHID;
//! assert_eq!(c.channels(), [153, 50, 204, 255]);
//! ```
//!
//! # From a hexadecimal string
//!
//! ```
//! use pix_engine::prelude::*;
//! use std::str::FromStr;
//!
//! let c = Color::from_str("#F0F")?; // 3-digit Hex string
//! assert_eq!(c.channels(), [255, 0, 255, 255]);
//!
//! let c = Color::from_str("#F0F5")?; // 4-digit Hex string
//! assert_eq!(c.channels(), [255, 0, 255, 85]);
//!
//! let c = Color::from_str("#F0F5BF")?; // 6-digit Hex string
//! assert_eq!(c.channels(), [240, 245, 191, 255]);
//!
//! let c = Color::from_str("#F0F5BF5F")?; // 8-digit Hex string
//! assert_eq!(c.channels(), [240, 245, 191, 95]);
//! # Ok::<(), PixError>(())
//! ```

use crate::random;
use conversion::{calculate_channels, clamp_levels, convert_levels, maxes};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

pub mod constants;
pub mod conversion;
pub mod ops;

/// [Color] mode indicating level interpretation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[must_use]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Mode {
    /// Red, Green, Blue, and Alpha
    Rgb,
    /// Hue, Saturation, Brightness, and Alpha
    Hsb,
    /// Hue, Saturation, Lightness, and Alpha
    Hsl,
}

use Mode::{Hsb, Hsl, Rgb};

/// A color represented with a [Mode].
#[derive(Debug, Copy, Clone)]
#[must_use]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Color {
    /// `Color` mode.
    mode: Mode,
    /// RGB values ranging from `0..=255`.
    channels: [u8; 4],
}

/// Constructs a [Color] with `red`, `green`, `blue` and optional `alpha`.
///
/// # Examples
///
/// ```
/// # use pix_engine::prelude::*;
/// let c = color!(128); // Gray
/// assert_eq!(c.channels(), [128, 128, 128, 255]);
///
/// let c = color!(128, 64); // Gray with alpha
/// assert_eq!(c.channels(), [128, 128, 128, 64]);
///
/// let c = color!(128, 64, 0); // Red, Green, Blue
/// assert_eq!(c.channels(), [128, 64, 0, 255]);
///
/// let c = color!(128, 64, 128, 128); // Red, Green, Blue, Alpha
/// assert_eq!(c.channels(), [128, 64, 128, 128]);
/// ```
#[macro_export]
macro_rules! color {
    ($gray:expr) => {
        rgb!($gray)
    };
    ($gray:expr, $a:expr$(,)?) => {
        rgb!($gray, $a)
    };
    ($r:expr, $g:expr, $b:expr$(,)?) => {
        rgb!($r, $g, $b)
    };
    ($r:expr, $g:expr, $b:expr, $a:expr$(,)?) => {
        rgb!($r, $g, $b, $a)
    };
}

/// Constructs a [Color] with `red`, `green`, `blue` and optional `alpha`.
///
/// Alias for [color!].
///
/// [color!]: crate::prelude::color
///
/// # Examples
///
/// ```
/// # use pix_engine::prelude::*;
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
#[doc(alias = "color")]
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
        $crate::prelude::Color::rgba($r, $g, $b, $a)
    };
}

/// Constructs a [Color] with `hue`, `saturation`, `brightness` and optional `alpha`.
///
/// # Examples
///
/// ```
/// # use pix_engine::prelude::*;
/// let c = hsb!(50.0); // Gray
/// assert_eq!(c.channels(), [128, 128, 128, 255]);
///
/// let c = hsb!(50.0, 0.5); // Gray with alpha
/// assert_eq!(c.channels(), [128, 128, 128, 128]);
///
/// let c = hsb!(342.0, 100.0, 80.0); // Hue, Saturation, Brightness
/// assert_eq!(c.channels(), [204, 0, 61, 255]);
///
/// let c = hsb!(342.0, 100.0, 80.0, 0.5); // Hue, Saturation, Brightness, Alpha
/// assert_eq!(c.channels(), [204, 0, 61, 128]);
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
        $crate::prelude::Color::hsba($h, $s, $b, $a)
    };
}

/// Constructs a [Color] with `hue`, `saturation`, `lightness` and optional `alpha`.
///
/// # Examples
///
/// ```
/// # use pix_engine::prelude::*;
/// let c = hsl!(50.0); // Gray
/// assert_eq!(c.channels(), [128, 128, 128, 255]);
///
/// let c = hsl!(50.0, 0.5); // Gray with alpha
/// assert_eq!(c.channels(), [128, 128, 128, 128]);
///
/// let c = hsl!(342.0, 100.0, 80.0); // Hue, Saturation, Lightness
/// assert_eq!(c.channels(), [255, 153, 184, 255]);
///
/// let c = hsl!(342.0, 100.0, 80.0, 0.5); // Hue, Saturation, Lightness, Alpha
/// assert_eq!(c.channels(), [255, 153, 184, 128]);
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
        $crate::prelude::Color::hsla($h, $s, $l, $a)
    };
}

impl Color {
    /// Constructs a `Color` with `red`, `green`, `blue` and max `alpha`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::new(0, 0, 128);
    /// assert_eq!(c.channels(), [0, 0, 128, 255]);
    /// ```
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self::rgb(r, g, b)
    }

    /// Constructs a `Color` with `red`, `green`, `blue` and `alpha`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::new_alpha(0, 0, 128, 50);
    /// assert_eq!(c.channels(), [0, 0, 128, 50]);
    /// ```
    #[inline]
    pub const fn new_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::rgba(r, g, b, a)
    }

    /// Constructs a `Color` with the given [Mode] and max alpha.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::with_mode(ColorMode::Rgb, 0, 0, 128);
    /// assert_eq!(c.channels(), [0, 0, 128, 255]);
    ///
    /// let c = Color::with_mode(ColorMode::Hsb, 126.0, 50.0, 100.0);
    /// assert_eq!(c.channels(), [128, 255, 140, 255]);
    /// ```
    pub fn with_mode<T: Into<f64>>(mode: Mode, v1: T, v2: T, v3: T) -> Self {
        let [_, _, _, alpha_max] = maxes(mode);
        Self::with_mode_alpha(mode, v1.into(), v2.into(), v3.into(), alpha_max)
    }

    /// Constructs a `Color` with the given [Mode] and alpha.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::with_mode_alpha(ColorMode::Rgb, 0.0, 0.0, 128.0, 50.0);
    /// assert_eq!(c.channels(), [0, 0, 128, 50]);
    ///
    /// let c = Color::with_mode_alpha(ColorMode::Hsb, 126.0, 50.0, 100.0, 0.8);
    /// assert_eq!(c.channels(), [128, 255, 140, 204]);
    /// ```
    pub fn with_mode_alpha<T: Into<f64>>(mode: Mode, v1: T, v2: T, v3: T, alpha: T) -> Self {
        let [v1, v2, v3, alpha] = [v1.into(), v2.into(), v3.into(), alpha.into()];
        let channels = if mode == Rgb {
            [v1 as u8, v2 as u8, v3 as u8, alpha as u8]
        } else {
            // Normalize channels
            let [v1_max, v2_max, v3_max, alpha_max] = maxes(mode);
            let levels = clamp_levels([v1 / v1_max, v2 / v2_max, v3 / v3_max, alpha / alpha_max]);
            // Convert to Rgb
            let levels = convert_levels(levels, mode, Rgb);
            calculate_channels(levels)
        };

        Self { mode, channels }
    }

    /// Constructs a `Color` with `red`, `green`, `blue` and max `alpha`.
    ///
    /// Alias for [Color::new].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(128, 64, 0);
    /// assert_eq!(c.channels(), [128, 64, 0, 255]);
    /// ```
    #[doc(alias = "new")]
    #[inline]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            mode: Rgb,
            channels: [r, g, b, 255],
        }
    }

    /// Constructs a `Color` with `red`, `green`, `blue` and `alpha`.
    ///
    /// Alias for [Color::new_alpha].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgba(128, 64, 128, 128);
    /// assert_eq!(c.channels(), [128, 64, 128, 128]);
    /// ```
    #[doc(alias = "new_alpha")]
    #[inline]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            mode: Rgb,
            channels: [r, g, b, a],
        }
    }

    /// Constructs a `Color` with `hue`, `saturation`, `brightness` and max `alpha`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::hsb(126.0, 80.0, 50.0);
    /// assert_eq!(c.channels(), [25, 128, 36, 255]);
    /// ```
    #[inline]
    pub fn hsb<T: Into<f64>>(h: T, s: T, b: T) -> Self {
        Self::with_mode(Hsb, h, s, b)
    }

    /// Constructs a `Color` with `hue`, `saturation`, `brightness` and `alpha`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::hsba(126.0, 80.0, 50.0, 0.5);
    /// assert_eq!(c.channels(), [25, 128, 36, 128]);
    /// ```
    #[inline]
    pub fn hsba<T: Into<f64>>(h: T, s: T, b: T, a: T) -> Self {
        Self::with_mode_alpha(Hsb, h, s, b, a)
    }

    /// Constructs a `Color` with `hue`, `saturation`, `lightness` and max `alpha`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::hsl(126.0, 80.0, 50.0);
    /// assert_eq!(c.channels(), [25, 230, 46, 255]);
    /// ```
    #[inline]
    pub fn hsl<T: Into<f64>>(h: T, s: T, l: T) -> Self {
        Self::with_mode(Hsl, h, s, l)
    }

    /// Constructs a `Color` with `hue`, `saturation`, `lightness` and `alpha`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::hsla(126.0, 80.0, 50.0, 0.5);
    /// assert_eq!(c.channels(), [25, 230, 46, 128]);
    /// ```
    #[inline]
    pub fn hsla<T: Into<f64>>(h: T, s: T, l: T, a: T) -> Self {
        Self::with_mode_alpha(Hsl, h, s, l, a)
    }

    /// Constructs a `Color` with the given [Mode] and alpha using levels ranging from `0.0..=1.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::from_levels(ColorMode::Rgb, 0.4, 0.5, 1.0, 0.8);
    /// assert_eq!(c.channels(), [102, 128, 255, 204]);
    /// ```
    pub fn from_levels<T: Into<f64>>(mode: Mode, v1: T, v2: T, v3: T, alpha: T) -> Self {
        let mut levels = [v1.into(), v2.into(), v3.into(), alpha.into()];
        for v in &mut levels {
            *v = (*v).clamp(0.0, 1.0);
        }
        Self {
            mode,
            channels: calculate_channels(levels),
        }
    }

    /// Constructs a random `Color` with `red`, `green`, `blue` and max alpha.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::random();
    /// // `c.channels()` will return something like:
    /// // [207, 12, 217, 255]
    /// ```
    #[inline]
    pub fn random() -> Self {
        Self::rgb(random!(255), random!(255), random!(255))
    }

    /// Constructs a random `Color` with `red`, `green`, `blue` and alpha.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::random_alpha();
    /// // `c.channels()` will return something like:
    /// // [132, 159, 233, 76]
    /// ```
    #[inline]
    pub fn random_alpha() -> Self {
        Self::rgba(random!(255), random!(255), random!(255), random!(255))
    }

    /// Returns the [u32] RGB hexadecimal value of a `Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(240, 255, 0);
    /// assert_eq!(c.as_hex(), 0xF0FF00);
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_hex(&self) -> u32 {
        let [r, g, b, _] = self.channels();
        u32::from_be_bytes([0, r, g, b])
    }

    /// Returns the [u32] RGBA hexadecimal value of a `Color`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(240, 255, 0);
    /// assert_eq!(c.as_hex_alpha(), 0xF0FF00FF);
    ///
    /// let c = Color::rgba(240, 255, 0, 128);
    /// assert_eq!(c.as_hex_alpha(), 0xF0FF0080);
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_hex_alpha(&self) -> u32 {
        u32::from_be_bytes(self.channels())
    }

    /// Returns a list of max values for each color channel based on [Mode].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(0, 0, 0);
    /// assert_eq!(c.maxes(), [255.0, 255.0, 255.0, 255.0]);
    ///
    /// let c = Color::hsb(0.0, 0.0, 0.0);
    /// assert_eq!(c.maxes(), [360.0, 100.0, 100.0, 1.0]);
    ///
    /// let c = Color::hsl(0.0, 0.0, 0.0);
    /// assert_eq!(c.maxes(), [360.0, 100.0, 100.0, 1.0]);
    /// ```
    #[inline]
    #[must_use]
    pub const fn maxes(&self) -> [f64; 4] {
        maxes(self.mode)
    }

    /// Returns the `Color` levels for the given [Mode] which range from `0.0..=1.0`.
    #[inline]
    #[must_use]
    pub fn levels(&self) -> [f64; 4] {
        let [r, g, b, a] = self.channels;
        let [r_max, g_max, b_max, a_max] = maxes(Rgb);
        let levels = clamp_levels([
            f64::from(r) / r_max,
            f64::from(g) / g_max,
            f64::from(b) / b_max,
            f64::from(a) / a_max,
        ]);
        // Convert to current mode
        convert_levels(levels, Rgb, self.mode)
    }

    /// Set the `Color` levels ranging from `0.0..=1.0` using the current [Mode].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::rgba(128, 64, 128, 128);
    /// c.set_levels([1.0, 0.5, 0.4, 1.0]);
    /// assert_eq!(c.channels(), [255, 128, 102, 255]);
    /// ```
    #[inline]
    pub fn set_levels(&mut self, levels: [f64; 4]) {
        let levels = clamp_levels(levels);
        self.update_channels(levels, self.mode);
    }

    /// Returns the `Color` channels as `[red, green, blue, alpha]` which range from `0..=255`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgba(128, 64, 128, 128);
    /// assert_eq!(c.channels(), [128, 64, 128, 128]);
    /// ```
    #[inline]
    #[must_use]
    pub const fn channels(&self) -> [u8; 4] {
        self.channels
    }

    /// Returns the current color [Mode].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(100, 0, 0);
    /// assert_eq!(c.mode(), ColorMode::Rgb);
    ///
    /// let c = Color::hsb(100.0, 0.0, 0.0);
    /// assert_eq!(c.mode(), ColorMode::Hsb);
    /// ```
    #[inline]
    pub const fn mode(&self) -> Mode {
        self.mode
    }

    /// Set the color [Mode].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::rgb(100, 0, 0);
    /// c.set_mode(ColorMode::Hsb);
    /// assert_eq!(c.mode(), ColorMode::Hsb);
    /// ```
    #[inline]
    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    /// Returns the red `Color` channel ranging from `0..=255`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(100, 0, 0);
    /// assert_eq!(c.red(), 100);
    /// ```
    #[inline]
    #[must_use]
    pub const fn red(&self) -> u8 {
        self.channels[0]
    }

    /// Set the red `Color` channel ranging from `0..=255`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::default();
    /// assert_eq!(c.channels(), [0, 0, 0, 255]);
    /// c.set_red(100);
    /// assert_eq!(c.channels(), [100, 0, 0, 255]);
    /// ```
    #[inline]
    pub fn set_red(&mut self, r: u8) {
        self.channels[0] = r;
    }

    /// Returns the green `Color` channel ranging from `0..=255`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(0, 100, 0);
    /// assert_eq!(c.green(), 100);
    /// ```
    #[inline]
    #[must_use]
    pub const fn green(&self) -> u8 {
        self.channels[1]
    }

    /// Set the green `Color` channel ranging from `0..=255`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::default();
    /// assert_eq!(c.channels(), [0, 0, 0, 255]);
    /// c.set_green(100);
    /// assert_eq!(c.channels(), [0, 100, 0, 255]);
    /// ```
    #[inline]
    pub fn set_green(&mut self, g: u8) {
        self.channels[1] = g;
    }

    /// Returns the blue `Color` channel ranging from `0..=255`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(0, 0, 100);
    /// assert_eq!(c.blue(), 100);
    /// ```
    #[inline]
    #[must_use]
    pub const fn blue(&self) -> u8 {
        self.channels[2]
    }

    /// Set the blue `Color` channel ranging from `0..=255`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::default();
    /// assert_eq!(c.channels(), [0, 0, 0, 255]);
    /// c.set_blue(100);
    /// assert_eq!(c.channels(), [0, 0, 100, 255]);
    /// ```
    #[inline]
    pub fn set_blue(&mut self, b: u8) {
        self.channels[2] = b;
    }

    /// Returns the alpha `Color` channel ranging from `0..=255`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgba(0, 0, 0, 100);
    /// assert_eq!(c.alpha(), 100);
    /// ```
    #[inline]
    #[must_use]
    pub const fn alpha(&self) -> u8 {
        self.channels[3]
    }

    /// Set the alpha `Color` channel ranging from `0..=255`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::default();
    /// assert_eq!(c.channels(), [0, 0, 0, 255]);
    /// c.set_alpha(100);
    /// assert_eq!(c.channels(), [0, 0, 0, 100]);
    /// ```
    #[inline]
    pub fn set_alpha(&mut self, a: u8) {
        self.channels[3] = a;
    }

    /// Returns the hue ranging from `0.0..=360.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(0, 100, 0);
    /// assert_eq!(c.hue(), 120.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn hue(&self) -> f64 {
        let maxes = maxes(Hsb);
        let levels = convert_levels(self.levels(), self.mode, Hsb);
        levels[0] * maxes[0]
    }

    /// Set the hue ranging from `0.0..=360.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::rgb(128, 0, 0);
    /// assert_eq!(c.channels(), [128, 0, 0, 255]);
    /// c.set_hue(100.0);
    /// assert_eq!(c.channels(), [43, 128, 0, 255]);
    /// ```
    #[inline]
    pub fn set_hue<H: Into<f64>>(&mut self, h: H) {
        let maxes = maxes(Hsb);
        let mut levels = convert_levels(self.levels(), self.mode, Hsb);
        levels[0] = h.into() / maxes[0];
        self.update_channels(levels, Hsb);
    }

    /// Returns the saturation ranging from `0.0..=100.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(0, 100, 0);
    /// assert_eq!(c.saturation(), 100.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn saturation(&self) -> f64 {
        let maxes = maxes(Hsb);
        let levels = convert_levels(self.levels(), self.mode, Hsb);
        levels[1] * maxes[1]
    }

    /// Set the saturation ranging from `0.0..=100.0`. Defaults to [Hsb] if the
    /// current mode is not [Hsb] or [Hsl] already.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::rgb(128, 0, 0);
    /// assert_eq!(c.channels(), [128, 0, 0, 255]);
    /// c.set_saturation(50.0);
    /// assert_eq!(c.channels(), [128, 64, 64, 255]);
    ///
    /// let mut c = Color::rgb(128, 0, 0);
    /// c.set_mode(ColorMode::Hsl);
    /// assert_eq!(c.channels(), [128, 0, 0, 255]);
    /// c.set_saturation(50.0);
    /// assert_eq!(c.channels(), [96, 32, 32, 255]);
    /// ```
    #[inline]
    pub fn set_saturation<S: Into<f64>>(&mut self, s: S) {
        let mode = match self.mode {
            Hsb | Hsl => self.mode,
            Rgb => Hsb,
        };
        let maxes = maxes(mode);
        let mut levels = convert_levels(self.levels(), self.mode, mode);
        levels[1] = s.into() / maxes[1];
        self.update_channels(levels, mode);
    }

    /// Returns the brightness ranging from `0.0..=100.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(0, 102, 0);
    /// assert_eq!(c.brightness(), 40.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn brightness(&self) -> f64 {
        let maxes = maxes(Hsb);
        let levels = convert_levels(self.levels(), self.mode, Hsb);
        levels[2] * maxes[2]
    }

    /// Set the brightness ranging from `0.0..=100.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::rgb(128, 0, 0);
    /// assert_eq!(c.channels(), [128, 0, 0, 255]);
    /// c.set_brightness(90.0);
    /// assert_eq!(c.channels(), [230, 0, 0, 255]);
    /// ```
    #[inline]
    pub fn set_brightness<B: Into<f64>>(&mut self, b: B) {
        let maxes = maxes(Hsb);
        let mut levels = convert_levels(self.levels(), self.mode, Hsb);
        levels[2] = b.into() / maxes[2];
        self.update_channels(levels, Hsb);
    }

    /// Returns the lightness ranging from `0.0..=100.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(0, 102, 0);
    /// assert_eq!(c.lightness(), 20.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn lightness(&self) -> f64 {
        let maxes = maxes(Hsl);
        let levels = convert_levels(self.levels(), self.mode, Hsl);
        levels[2] * maxes[2]
    }

    /// Set the lightness ranging from `0.0..=100.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::rgb(128, 0, 0);
    /// assert_eq!(c.channels(), [128, 0, 0, 255]);
    /// c.set_lightness(90.0);
    /// assert_eq!(c.channels(), [255, 204, 204, 255]);
    /// ```
    #[inline]
    pub fn set_lightness<L: Into<f64>>(&mut self, l: L) {
        let maxes = maxes(Hsl);
        let mut levels = convert_levels(self.levels(), self.mode, Hsl);
        levels[2] = l.into() / maxes[2];
        self.update_channels(levels, Hsl);
    }

    /// Returns `Color` as a [Vec] of `[red, green, blue, alpha]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = color!(100, 200, 50);
    /// assert_eq!(c.to_vec(), vec![100, 200, 50, 255]);
    /// ```
    #[inline]
    #[must_use]
    pub fn to_vec(self) -> Vec<u8> {
        self.channels.to_vec()
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::rgb(0, 0, 0)
    }
}

/// Display [Color] as "[r, g, b, a]".
impl fmt::Display for Color {
    #[allow(clippy::many_single_char_names)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [r, g, b, a] = self.channels();
        write!(f, "[{r}, {g}, {b}, {a}]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructors() {
        let expected = |mode| Color {
            mode,
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
