//! [Color] functions for drawing.
//!
//! Each [Color] can be constructed with a [ColorMode]. The default mode and internal
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
//! [random_alpha](Color::random_alpha) methods.
//!
//! [Color] also implements [FromStr](std::str::FromStr) allowing conversion from a 3, 4, 6, or
//! 8-digit [hexadecimal](https://en.wikipedia.org/wiki/Web_colors) string.
//!
//! The [Color] instance stores which [ColorMode] it was created with, modifying how manipulation
//! methods are interprted such as [set_alpha](Color::set_alpha) taking a range of `0.0..=255.0` or
//! `0.0..=1.0`. The [ColorMode] can be changed any time to alter this behavior using
//! [set_mode](Color::set_mode).
//!
//! There are also several named color [constants] available in the
//! [prelude](crate::prelude) matching the [SVG 1.0 Color
//! Keywords](https://www.w3.org/TR/SVG11/types.html#ColorKeywords).
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
//! let c = ALICE_BLUE;
//! assert_eq!(c.channels(), [240, 248, 255, 255]);
//!
//! let c = DARK_ORCHID;
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
//! # Ok::<(), ColorError<Scalar>>(())
//! ```

use crate::{prelude::Scalar, random};
use conversion::{calculate_channels, clamp_levels, convert_levels, maxes};
use ops::Iter;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt};

pub use conversion::ColorError;

pub mod constants;
pub mod conversion;
pub mod ops;

/// [Color] mode indicating level interpretation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ColorMode {
    /// Red, Green, Blue, and Alpha
    Rgb,
    /// Hue, Saturation, Brightness, and Alpha
    Hsb,
    /// Hue, Saturation, Lightness, and Alpha
    Hsl,
}

use ColorMode::*;

/// A color represented with a [ColorMode].
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Color {
    /// `Color` mode.
    mode: ColorMode,
    /// RGB values ranging `0.0..=1.0`.
    levels: [Scalar; 4],
    /// RGB values ranging from `0..=255`.
    channels: [u8; 4],
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
    pub fn new<T: Into<Scalar>>(r: T, g: T, b: T) -> Self {
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
    pub fn new_alpha<T: Into<Scalar>>(r: T, g: T, b: T, a: T) -> Self {
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
    /// assert_eq!(c.channels(), [128, 255, 140, 255]);
    /// ```
    pub fn with_mode<T: Into<Scalar>>(mode: ColorMode, v1: T, v2: T, v3: T) -> Self {
        // Normalize channels
        let [v1_max, v2_max, v3_max, _] = maxes(mode);
        let levels = clamp_levels([
            v1.into() / v1_max,
            v2.into() / v2_max,
            v3.into() / v3_max,
            1.0,
        ]);

        // Convert to Rgb
        let levels = convert_levels(levels, mode, Rgb);
        let channels = calculate_channels(levels);

        Self {
            mode,
            levels,
            channels,
        }
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
    /// assert_eq!(c.channels(), [128, 255, 140, 204]);
    /// ```
    pub fn with_mode_alpha<T: Into<Scalar>>(
        mode: ColorMode,
        v1: T,
        v2: T,
        v3: T,
        alpha: T,
    ) -> Self {
        // Normalize channels
        let [v1_max, v2_max, v3_max, alpha_max] = maxes(mode);
        let levels = clamp_levels([
            v1.into() / v1_max,
            v2.into() / v2_max,
            v3.into() / v3_max,
            alpha.into() / alpha_max,
        ]);

        // Convert to Rgb
        let levels = convert_levels(levels, mode, Rgb);
        let channels = calculate_channels(levels);

        Self {
            mode,
            levels,
            channels,
        }
    }

    /// Constructs a `Color` with `red`, `green`, `blue` and max `alpha`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb(128, 64, 0);
    /// assert_eq!(c.channels(), [128, 64, 0, 255]);
    /// ```
    pub fn rgb<T: Into<Scalar>>(r: T, g: T, b: T) -> Self {
        Self::with_mode(Rgb, r, g, b)
    }

    /// Constructs a `Color` with `red`, `green`, `blue` and `alpha`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgba(128, 64, 128, 128);
    /// assert_eq!(c.channels(), [128, 64, 128, 128]);
    /// ```
    pub fn rgba<T: Into<Scalar>>(r: T, g: T, b: T, a: T) -> Self {
        Self::with_mode_alpha(Rgb, r, g, b, a)
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
    pub fn hsb<T: Into<Scalar>>(h: T, s: T, b: T) -> Self {
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
    pub fn hsba<T: Into<Scalar>>(h: T, s: T, b: T, a: T) -> Self {
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
    pub fn hsl<T: Into<Scalar>>(h: T, s: T, l: T) -> Self {
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
    pub fn hsla<T: Into<Scalar>>(h: T, s: T, l: T, a: T) -> Self {
        Self::with_mode_alpha(Hsl, h, s, l, a)
    }

    /// Constructs a `Color` with the given [ColorMode] and alpha using the raw levels passed in
    /// as-is without normalizing them.
    ///
    /// # Safety
    ///
    /// This may result in unexpected behavior if values are outside the range `0.0..=1.0`. It is
    /// the responsibility of the caller to hold this invariant.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = unsafe { Color::from_raw(ColorMode::Rgb, 0.4, 0.5, 1.0, 0.8) };
    /// assert_eq!(c.channels(), [102, 128, 255, 204]);
    /// ```
    pub unsafe fn from_raw<T: Into<Scalar>>(
        mode: ColorMode,
        v1: T,
        v2: T,
        v3: T,
        alpha: T,
    ) -> Self {
        let levels = [v1.into(), v2.into(), v3.into(), alpha.into()];
        Self {
            mode,
            levels,
            channels: calculate_channels(levels),
        }
    }

    /// Constructs a `Color` from a [slice] of 1-4 values. The number of values
    /// provided alter how they are interpreted similar to the [color!], [rgb!], [hsb!], and
    /// [hsl!] macros.
    ///
    /// # Errors
    ///
    /// If the [slice] is empty or has more than 4 values, an error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let vals: Vec<Scalar> = vec![128.0, 64.0, 0.0];
    /// let c = Color::from_slice(ColorMode::Rgb, &vals)?; // RGB Vec
    /// assert_eq!(c.channels(), [128, 64, 0, 255]);
    ///
    /// let vals: [Scalar; 4] = [128.0, 64.0, 0.0, 128.0];
    /// let c = Color::from_slice(ColorMode::Rgb, &vals[..])?; // RGBA slice
    /// assert_eq!(c.channels(), [128, 64, 0, 128]);
    /// # Ok::<(), ColorError<Scalar>>(())
    /// ```
    pub fn from_slice<T: Into<Scalar>>(mode: ColorMode, slice: &[T]) -> Result<Self, ColorError<T>>
    where
        T: fmt::Debug + Copy + Clone,
    {
        let result = match *slice {
            [gray] => Self::with_mode(mode, gray, gray, gray),
            [gray, a] => Self::with_mode_alpha(mode, gray, gray, gray, a),
            [v1, v2, v3] => Self::with_mode(mode, v1, v2, v3),
            [v1, v2, v3, a] => Self::with_mode_alpha(mode, v1, v2, v3, a),
            _ => return Err(ColorError::InvalidSlice(Cow::from(slice.to_owned()))),
        };
        Ok(result)
    }
    /// Constructs a random `Color` with `red`, `green`, `blue` and max alpha.
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

    /// Constructs a random `Color` with `red`, `green`, `blue` and alpha.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # #[allow(unused_variables)]
    /// let c = Color::random_alpha();
    /// // `c.channels()` will return something like:
    /// // [132, 159, 233, 76]
    /// ```
    pub fn random_alpha() -> Self {
        Self::new_alpha(random!(255), random!(255), random!(255), random!(255))
    }

    /// Constructs a `Color` from a [u32] RGBA hexadecimal value.
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

    /// Returns a list of max values for each color channel based on [ColorMode].
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
    pub const fn maxes(&self) -> [Scalar; 4] {
        maxes(self.mode)
    }

    /// Returns the `Color` levels which range from `0.0..=1.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgba(128, 64, 128, 128);
    /// assert_eq!(c.channels(), [128, 64, 128, 128]);
    /// ```
    pub const fn levels(&self) -> [Scalar; 4] {
        self.levels
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
    pub const fn channels(&self) -> [u8; 4] {
        self.channels
    }

    /// Returns the current [ColorMode].
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
    pub const fn mode(&self) -> ColorMode {
        self.mode
    }

    /// Set the [ColorMode].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::rgb(100, 0, 0);
    /// c.set_mode(ColorMode::Hsb);
    /// assert_eq!(c.mode(), ColorMode::Hsb);
    /// ```
    pub fn set_mode(&mut self, mode: ColorMode) {
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
    pub fn set_red<R: Into<Scalar>>(&mut self, r: R) {
        let maxes = maxes(Rgb);
        self.levels[0] = r.into() / maxes[0];
        self.calculate_channels();
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
    pub fn set_green<G: Into<Scalar>>(&mut self, g: G) {
        let maxes = maxes(Rgb);
        self.levels[1] = g.into() / maxes[1];
        self.calculate_channels();
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
    pub fn set_blue<B: Into<Scalar>>(&mut self, b: B) {
        let maxes = maxes(Rgb);
        self.levels[2] = b.into() / maxes[2];
        self.calculate_channels();
    }

    /// Returns the alpha `Color` channel ranging from `0.0..=255.0` or `0.0..=1.0` depending on
    /// current [ColorMode].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgba(0, 0, 0, 100);
    /// assert_eq!(c.alpha(), 100.0);
    ///
    /// let c = Color::hsba(0.0, 0.0, 0.0, 0.8);
    /// assert_eq!(c.alpha(), 0.8);
    /// ```
    pub fn alpha(&self) -> Scalar {
        let maxes = self.maxes();
        self.levels[3] * maxes[3]
    }

    /// Set the alpha `Color` channel ranging from `0..=255` or `0.0..=1.0` depending on current
    /// [ColorMode].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::default();
    /// assert_eq!(c.channels(), [0, 0, 0, 255]);
    /// c.set_alpha(100);
    /// assert_eq!(c.channels(), [0, 0, 0, 100]);
    ///
    /// let mut c = Color::hsb(0.0, 0.0, 0.0);
    /// assert_eq!(c.channels(), [0, 0, 0, 255]);
    /// c.set_alpha(0.8);
    /// assert_eq!(c.channels(), [0, 0, 0, 204]);
    /// ```
    pub fn set_alpha<A: Into<Scalar>>(&mut self, a: A) {
        let maxes = self.maxes();
        self.levels[3] = a.into() / maxes[3];
        self.calculate_channels();
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
    pub fn hue(&self) -> Scalar {
        let maxes = maxes(Hsb);
        let levels = convert_levels(self.levels, Rgb, Hsb);
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
    pub fn set_hue<H: Into<Scalar>>(&mut self, h: H) {
        let maxes = maxes(Hsb);
        let mut levels = convert_levels(self.levels, Rgb, Hsb);
        levels[0] = h.into() / maxes[0];
        self.levels = convert_levels(levels, Hsb, Rgb);
        self.calculate_channels();
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
    pub fn saturation(&self) -> Scalar {
        let maxes = maxes(Hsb);
        let levels = convert_levels(self.levels, Rgb, Hsb);
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
    pub fn set_saturation<S: Into<Scalar>>(&mut self, s: S) {
        let mode = match self.mode {
            Hsb | Hsl => self.mode,
            _ => Hsb,
        };
        let maxes = maxes(mode);
        let mut levels = convert_levels(self.levels, Rgb, mode);
        levels[1] = s.into() / maxes[1];
        self.levels = convert_levels(levels, mode, Rgb);
        self.calculate_channels();
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
    pub fn brightness(&self) -> Scalar {
        let maxes = maxes(Hsb);
        let levels = convert_levels(self.levels, Rgb, Hsb);
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
    pub fn set_brightness<B: Into<Scalar>>(&mut self, b: B) {
        let maxes = maxes(Hsb);
        let mut levels = convert_levels(self.levels, Rgb, Hsb);
        levels[2] = b.into() / maxes[2];
        self.levels = convert_levels(levels, Hsb, Rgb);
        self.calculate_channels();
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
    pub fn lightness(&self) -> Scalar {
        let maxes = maxes(Hsl);
        let levels = convert_levels(self.levels, Rgb, Hsl);
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
    pub fn set_lightness<L: Into<Scalar>>(&mut self, l: L) {
        let maxes = maxes(Hsl);
        let mut levels = convert_levels(self.levels, Rgb, Hsl);
        levels[2] = l.into() / maxes[2];
        self.levels = convert_levels(levels, Hsl, Rgb);
        self.calculate_channels();
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
    pub fn to_vec(self) -> Vec<u8> {
        Vec::from(self.channels)
    }

    /// Returns an iterator over the `Color` RGBA channels `[red, green, blue, alpha]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = color!(100, 200, 50);
    /// let mut iterator = c.iter();
    ///
    /// assert_eq!(iterator.next(), Some(100));
    /// assert_eq!(iterator.next(), Some(200));
    /// assert_eq!(iterator.next(), Some(50));
    /// assert_eq!(iterator.next(), Some(255));
    /// assert_eq!(iterator.next(), None);
    /// ```
    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }
}

/// Constructs a `Color` with `red`, `green`, `blue` and optional `alpha`.
///
/// Alias for [rgb!].
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

/// Constructs a `Color` with `red`, `green`, `blue` and optional `alpha`.
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

/// Constructs a `Color` with `hue`, `saturation`, `brightness` and optional `alpha`.
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
        $crate::color::Color::hsba($h, $s, $b, $a)
    };
}

/// Constructs a `Color` with `hue`, `saturation`, `lightness` and optional `alpha`.
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
        $crate::color::Color::hsla($h, $s, $l, $a)
    };
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
        write!(f, "[{}, {}, {}, {}]", r, g, b, a)
    }
}

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
