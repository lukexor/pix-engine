//! Color handling and manipulation used for draw operations within the `PixEngine`.
//!
//! Each `Color` instance stores the `ColorMode` and `ColorMaxes` it was created with (e.g. `Rgb`,
//! `Hsb`, or `Hsl`). Internally, color channels are stored as RGBA values normalized from 0.0 to 1.0.
//! Other color representations than the initial color mode are calculated and cached as needed.

use crate::{math::constrainf, StateData};
use conversion::ColorLevels;
use std::fmt;

pub mod prelude {
    pub use super::{constants::*, Color, ColorMode};
}
pub use constants::*;

mod constants;
mod conversion;

/// `ColoreMode` changes the way the `PixEngine` and `Color` instances interpret color data. The
/// default is `ColorMode::Rgb`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum ColorMode {
    Rgb,
    Hsb,
    Hsl,
}

impl Default for ColorMode {
    fn default() -> Self {
        Self::Rgb
    }
}

/// `ColorMaxes` limits the maximum levels the color can have per-channel based on the `ColorMode`.
///
/// - `ColorMode::Rgb` ranges from 0.0 to 255.0 for red, green, blue, and alpha.
/// - `ColorMode::Hsb` ranges from 0.0 to 360.0 for hue and 0.0 to 100.0 for saturation,
///   brightness, and 0.0 to 1.0 for alpha.
/// - `ColorMode::Hsl` mode ranges from 0.0 to 360.0 for hue and 0.0 to 100.0 for saturation,
///   lightness, and 0.0 to 1.0 for alpha.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct ColorMaxes {
    pub rgb: [f64; 4],
    pub hsb: [f64; 4],
    pub hsl: [f64; 4],
}

impl Default for ColorMaxes {
    fn default() -> Self {
        Self {
            rgb: [255.0, 255.0, 255.0, 255.0],
            hsb: [360.0, 100.0, 100.0, 1.0],
            hsl: [360.0, 100.0, 100.0, 1.0],
        }
    }
}

/// Represents a color. The default is "transparent".
#[derive(Default, Debug, Copy, Clone)]
#[repr(C)]
pub struct Color {
    values: [f64; 4],
    hsba: Option<[f64; 4]>,
    hsla: Option<[f64; 4]>,
    levels: [u8; 4],
    mode: ColorMode,
    maxes: ColorMaxes,
}

impl Color {
    /// Creates a new `Color` instance with `ColorMode::Rgb` given color levels with a max alpha
    /// and default `ColorMaxes` for `Rgb`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb([0, 126, 255]);
    /// assert_eq!(c.levels(), [0, 126, 255, 255]);
    /// ```
    #[inline(always)]
    pub fn rgb<L: Into<ColorLevels>>(levels: L) -> Self {
        Self::rgba(levels)
    }

    /// Creates a new `Color` instance with `ColorMode::Rgb` given color levels with default
    /// `ColorMaxes`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgba([0, 126, 255, 100]);
    /// assert_eq!(c.levels(), [0, 126, 255, 100]);
    /// ```
    #[inline(always)]
    pub fn rgba<L: Into<ColorLevels>>(levels: L) -> Self {
        let mut levels = levels.into();
        levels.mode = ColorMode::Rgb;
        levels.maxes = ColorMaxes::default();
        Color::from_levels(levels)
    }

    /// Creates a new `Color` instance with `ColorMode::Hsb` given color levels with a max alpha and
    /// default `ColorMaxes`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::hsb([300, 90, 60]);
    /// assert_eq!(c.levels(), [153, 15, 153, 255]);
    /// assert_eq!(c.hue(), 300.0);
    /// assert_eq!(c.saturation(), 90.0);
    /// assert_eq!(c.brightness(), 60.0);
    /// assert_eq!(c.alpha(), 1.0);
    /// ```
    #[inline(always)]
    pub fn hsb<L: Into<ColorLevels>>(levels: L) -> Self {
        Self::hsba(levels)
    }

    /// Creates a new `Color` instance with `ColorMode:Hsb` given color levels with default
    /// `ColorMaxes`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::hsb([300.0, 90.0, 60.0, 0.5]);
    /// assert_eq!(c.levels(), [153, 15, 153, 128]);
    /// assert_eq!(c.hue(), 300.0);
    /// assert_eq!(c.saturation(), 90.0);
    /// assert_eq!(c.brightness(), 60.0);
    /// assert_eq!(c.alpha(), 0.5);
    /// ```
    #[inline(always)]
    pub fn hsba<L: Into<ColorLevels>>(levels: L) -> Self {
        let mut levels = levels.into();
        levels.mode = ColorMode::Hsb;
        levels.maxes = ColorMaxes::default();
        Color::from_levels(levels)
    }

    /// Creates a new `Color` instance with `ColorMode::Hsl` given color levels with a max alpha and
    /// default `ColorMaxes`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::hsl([300, 90, 30]);
    /// assert_eq!(c.levels(), [145, 8, 145, 255]);
    /// assert_eq!(c.hue(), 300.0);
    /// assert_eq!(c.saturation(), 90.0);
    /// assert_eq!(c.lightness(), 30.0);
    /// assert_eq!(c.alpha(), 1.0);
    /// ```
    #[inline(always)]
    pub fn hsl<L: Into<ColorLevels>>(levels: L) -> Self {
        Self::hsla(levels)
    }

    /// Creates a new `Color` instance with `ColorMode::Hsl` given color levels with default
    /// `ColorMaxes`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::hsla([300.0, 90.0, 30.0, 0.5]);
    /// assert_eq!(c.levels(), [145, 8, 145, 128]);
    /// assert_eq!(c.hue(), 300.0);
    /// assert_eq!(c.saturation(), 90.0);
    /// assert_eq!(c.lightness(), 30.0);
    /// assert_eq!(c.alpha(), 0.5);
    /// ```
    #[inline(always)]
    pub fn hsla<L: Into<ColorLevels>>(levels: L) -> Self {
        let mut levels = levels.into();
        levels.mode = ColorMode::Hsl;
        levels.maxes = ColorMaxes::default();
        Color::from_levels(levels)
    }

    /// Creates a `Color` instance with given color levels, `ColorMode` and `ColorMaxes`. Used
    /// internally for From/Into traits and mode-specific constructors.
    fn from_levels<L: Into<ColorLevels>>(levels: L) -> Self {
        let levels = levels.into();
        let mode = levels.mode;
        let maxes = levels.maxes;
        let mut values = [0f64; 4];
        let mode_maxes = match mode {
            ColorMode::Hsb => maxes.hsb,
            ColorMode::Hsl => maxes.hsl,
            ColorMode::Rgb => maxes.rgb,
        };

        // Normalize from 0 to 1
        for i in 0..4 {
            values[i] = constrainf(levels[i] / mode_maxes[i], 0.0, 1.0);
        }

        let mut hsba = None;
        let mut hsla = None;
        let values = match mode {
            ColorMode::Hsb => {
                hsba = Some(values);
                Color::hsba_to_rgba(values)
            }
            ColorMode::Hsl => {
                hsla = Some(values);
                Color::hsla_to_rgba(values)
            }
            ColorMode::Rgb => values,
        };

        let mut color = Self {
            values,
            hsba,
            hsla,
            mode,
            maxes,
            ..Default::default()
        };
        color.calculate_levels();
        color
    }

    /// Get the red value of the color, ranging from 0.0 - 255.0. Level values are rounded to the
    /// nearest screen color.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb([127.5, 126.0, 255.0]);
    /// assert_eq!(c.levels()[0], 128);
    /// assert_eq!(c.red(), 127.5);
    /// ```
    #[inline(always)]
    pub fn red(self) -> f64 {
        self.values[0] * self.maxes.rgb[0]
    }

    /// Set the red value of the color, ranging from 0.0 to 255.0. Level values are rounded to the
    /// nearest screen color.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::rgb([0, 126, 255]);
    /// c.set_red(100.0);
    /// assert_eq!(c.red(), 100.0);
    /// ```
    #[inline(always)]
    pub fn set_red(&mut self, r: f64) {
        self.values[0] = r / self.maxes.rgb[0];
        self.calculate_levels();
    }

    /// Get the green value of the color, ranging from 0.0 to 255.0. Level values are rounded to
    /// the nearest screen color.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb([0, 126, 255]);
    /// assert_eq!(c.green(), 126.0);
    /// ```
    #[inline(always)]
    pub fn green(self) -> f64 {
        self.values[1] * self.maxes.rgb[1]
    }

    /// Set the green value of the color, ranging from 0.0 to 255.0. Level values are rounded to
    /// the nearest screen color.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::rgb([0, 126, 255]);
    /// c.set_green(100.0);
    /// assert_eq!(c.green(), 100.0);
    /// ```
    #[inline(always)]
    pub fn set_green(&mut self, g: f64) {
        self.values[1] = g / self.maxes.rgb[1];
        self.calculate_levels();
    }

    /// Get the blue value of the color, ranging from 0.0 to 255.0. Level values are rounded to the
    /// nearest screen color.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb([0, 126, 255]);
    /// assert_eq!(c.blue(), 255.0);
    /// ```
    #[inline(always)]
    pub fn blue(self) -> f64 {
        self.values[2] * self.maxes.rgb[2]
    }

    /// Set the blue value of the color, ranging from 0.0 to 255.0. Level values are rounded to the
    /// nearest screen color.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::rgb([0, 126, 255]);
    /// c.set_blue(100.0);
    /// assert_eq!(c.blue(), 100.0);
    /// ```
    #[inline(always)]
    pub fn set_blue(&mut self, b: f64) {
        self.values[2] = b / self.maxes.rgb[2];
        self.calculate_levels();
    }

    /// Get the alpha value of the color, ranging from 0.0 to 255.0 for RGB or 0.0 to 1.0 for
    /// HSB/HSL. Level values are rounded to the nearest screen color.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let c = Color::rgba([0, 126, 255, 102]);
    /// assert_eq!(c.alpha(), 102.0);
    ///
    /// let c = Color::hsba([300.0, 90.0, 60.0, 0.9]);
    /// assert_eq!(c.alpha(), 0.9);
    /// ```
    #[inline(always)]
    pub fn alpha(self) -> f64 {
        self.values[3] * self.maxes()[3]
    }

    /// Set the alpha value of the color, ranging from 0.0 to 255.0 for RGB or 0.0 to 1.0 for
    /// HSB/HSL. Level values are rounded to the nearest screen color.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let mut c = Color::rgba([0, 126, 255, 102]);
    /// c.set_alpha(100.0);
    /// assert_eq!(c.alpha(), 100.0);
    ///
    /// let mut c = Color::hsba([300.0, 90.0, 60.0, 1.0]);
    /// c.set_alpha(0.9);
    /// assert_eq!(c.alpha(), 0.9);
    /// ```
    #[inline(always)]
    pub fn set_alpha(&mut self, a: f64) {
        self.values[3] = a / self.maxes()[3];
        self.calculate_levels();
    }

    /// Get the hue value of the color.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::hsb([300, 90, 60]);
    /// assert_eq!(c.hue(), 300.0);
    /// ```
    #[inline(always)]
    pub fn hue(&mut self) -> f64 {
        if self.mode == ColorMode::Hsb {
            self.get_hsba()[0] * self.maxes.hsb[0]
        } else {
            self.get_hsla()[0] * self.maxes.hsl[0]
        }
    }

    /// Set the hue value of the color.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let mut c = Color::hsb([300, 90, 60]);
    /// c.set_hue(260.0);
    /// assert_eq!(c.levels(), [61, 15, 153, 255]);
    /// assert_eq!(c.hue(), 260.0);
    /// assert_eq!(c.saturation(), 90.0);
    /// assert_eq!(c.brightness(), 60.0);
    /// assert_eq!(c.alpha(), 1.0);
    ///
    /// let mut c = Color::hsl([300, 90, 30]);
    /// c.set_hue(260.0);
    /// assert_eq!(c.levels(), [54, 8, 145, 255]);
    /// assert_eq!(c.hue(), 260.0);
    /// assert_eq!(c.saturation(), 90.0);
    /// assert_eq!(c.lightness(), 30.0);
    /// assert_eq!(c.alpha(), 1.0);
    /// ```
    #[inline(always)]
    pub fn set_hue(&mut self, hue: f64) {
        if self.mode == ColorMode::Hsb {
            let hsba = self.get_hsba();
            self.hsba = Some([hue / self.maxes.hsb[0], hsba[1], hsba[2], hsba[3]]);
            self.values = Color::hsba_to_rgba(self.get_hsba());
        } else {
            let hsla = self.get_hsla();
            self.hsla = Some([hue / self.maxes.hsl[0], hsla[1], hsla[2], hsla[3]]);
            self.values = Color::hsla_to_rgba(self.get_hsla());
        }
        self.calculate_levels();
    }

    /// Get the saturation value of the color, ranging from 0.0 to 100.0.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::hsb([300, 90, 60]);
    /// assert_eq!(c.saturation(), 90.0);
    /// ```
    #[inline(always)]
    pub fn saturation(&mut self) -> f64 {
        if self.mode == ColorMode::Hsb {
            self.get_hsba()[1] * self.maxes.hsb[1]
        } else {
            self.get_hsla()[1] * self.maxes.hsl[1]
        }
    }

    /// Set the saturation value of the color.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let mut c = Color::hsb([300, 90, 60]);
    /// c.set_saturation(70.0);
    /// assert_eq!(c.levels(), [153, 46, 153, 255]);
    /// assert_eq!(c.hue(), 300.0);
    /// assert_eq!(c.saturation(), 70.0);
    /// assert_eq!(c.brightness(), 60.0);
    /// assert_eq!(c.alpha(), 1.0);
    ///
    /// let mut c = Color::hsl([300, 90, 30]);
    /// c.set_saturation(70.0);
    /// assert_eq!(c.levels(), [130, 23, 130, 255]);
    /// assert_eq!(c.hue(), 300.0);
    /// assert_eq!(c.saturation(), 70.0);
    /// assert_eq!(c.lightness(), 30.0);
    /// assert_eq!(c.alpha(), 1.0);
    /// ```
    #[inline(always)]
    pub fn set_saturation(&mut self, saturation: f64) {
        if self.mode == ColorMode::Hsb {
            let hsba = self.get_hsba();
            self.hsba = Some([hsba[0], saturation / self.maxes.hsb[1], hsba[2], hsba[3]]);
            self.values = Color::hsba_to_rgba(self.get_hsba());
        } else {
            let hsla = self.get_hsla();
            self.hsla = Some([hsla[0], saturation / self.maxes.hsb[1], hsla[2], hsla[3]]);
            self.values = Color::hsla_to_rgba(self.get_hsla());
        }
        self.calculate_levels();
    }

    /// Get the brightness value of the color, ranging from 0.0 to 100.0.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::hsb([300, 90, 60]);
    /// assert_eq!(c.brightness(), 60.0);
    /// ```
    #[inline(always)]
    pub fn brightness(&mut self) -> f64 {
        self.get_hsba()[2] * self.maxes.hsb[2]
    }

    /// Set the brightness value of the color.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let mut c = Color::hsb([300, 90, 60]);
    /// c.set_brightness(70.0);
    /// assert_eq!(c.levels(), [179, 18, 179, 255]);
    /// assert_eq!(c.hue(), 300.0);
    /// assert_eq!(c.saturation(), 90.0);
    /// assert_eq!(c.brightness(), 70.0);
    /// assert_eq!(c.alpha(), 1.0);
    /// ```
    #[inline(always)]
    pub fn set_brightness(&mut self, brightness: f64) {
        let hsba = self.get_hsba();
        self.hsba = Some([hsba[0], hsba[1], brightness / self.maxes.hsb[2], hsba[3]]);
        self.values = Color::hsba_to_rgba(self.get_hsba());
        self.calculate_levels();
    }

    /// Get the lightness value of the color, ranging from 0.0 to 100.0.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::hsl([300, 90, 30]);
    /// assert_eq!(c.lightness(), 30.0);
    /// ```
    #[inline(always)]
    pub fn lightness(&mut self) -> f64 {
        self.get_hsla()[2] * self.maxes.hsl[2]
    }

    /// Set the lightness value of the color.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let mut c = Color::hsl([300, 90, 60]);
    /// c.set_lightness(70.0);
    /// assert_eq!(c.levels(), [247, 110, 247, 255]);
    /// assert_eq!(c.hue(), 300.0);
    /// assert_eq!(c.saturation(), 90.0);
    /// assert_eq!(c.lightness(), 70.0);
    /// assert_eq!(c.alpha(), 1.0);
    /// ```
    #[inline(always)]
    pub fn set_lightness(&mut self, lightness: f64) {
        let hsla = self.get_hsla();
        self.hsla = Some([hsla[0], hsla[1], lightness / self.maxes.hsb[2], hsla[3]]);
        self.values = Color::hsla_to_rgba(self.get_hsla());
        self.calculate_levels();
    }

    /// Get the `ColorMode` of this color.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::rgb([128, 0, 255]);
    /// assert_eq!(c.mode(), ColorMode::Rgb);
    /// ```
    pub fn mode(self) -> ColorMode {
        self.mode
    }

    /// Set the `ColorMode` of this color.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let mut c = Color::rgb([128, 0, 255]);
    /// c.set_mode(ColorMode::Hsb);
    /// assert_eq!(c.mode(), ColorMode::Hsb);
    /// ```
    pub fn set_mode(&mut self, mode: ColorMode) {
        self.mode = mode;
    }

    /// Get the `ColorMaxes` for the colors current `ColorMode`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let c = Color::default();
    /// assert_eq!(c.maxes(), [255.0, 255.0, 255.0, 255.0]);
    ///
    /// let c = Color::hsb([300, 90, 100]);
    /// assert_eq!(c.maxes(), [360.0, 100.0, 100.0, 1.0]);
    ///
    /// let c = Color::hsl([300, 90, 100]);
    /// assert_eq!(c.maxes(), [360.0, 100.0, 100.0, 1.0]);
    /// ```
    #[inline(always)]
    pub fn maxes(self) -> [f64; 4] {
        match self.mode {
            ColorMode::Hsb => self.maxes.hsb,
            ColorMode::Hsl => self.maxes.hsl,
            ColorMode::Rgb => self.maxes.rgb,
        }
    }

    /// Set the `ColorMaxes` for the colors current `ColorMode`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    ///
    /// let mut c = Color::rgb([255.0, 255.0, 255.0, 255.0]);
    /// c.set_maxes([128.0, 128.0, 255.0, 100.0]);
    /// assert_eq!(c.maxes(), [128.0, 128.0, 255.0, 100.0]);
    /// assert_eq!(c.red(), 128.0);
    /// assert_eq!(c.green(), 128.0);
    /// assert_eq!(c.blue(), 255.0);
    /// assert_eq!(c.alpha(), 100.0);
    /// ```
    #[inline(always)]
    pub fn set_maxes(&mut self, maxes: [f64; 4]) {
        match self.mode {
            ColorMode::Hsb => self.maxes.hsb = maxes,
            ColorMode::Hsl => self.maxes.hsl = maxes,
            ColorMode::Rgb => self.maxes.rgb = maxes,
        }
        self.calculate_levels();
    }

    /// Gets an array containing the closest RGBA screen values, rounded.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # let mut state = StateData::default();
    ///
    /// let mut c1 = state.color([128, 0, 128]);
    /// assert_eq!(c1.levels(), [128, 0, 128, 255]);
    ///
    /// let mut c2 = state.color([128, 0, 128, 64]);
    /// assert_eq!(c2.levels(), [128, 0, 128, 64]);
    /// ```
    pub fn levels(self) -> [u8; 4] {
        self.levels
    }

    /// Gets an array containing the RGBA values normalized between 0.0 and 1.0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # let mut state = StateData::default();
    /// # use assert_approx_eq::assert_approx_eq;
    ///
    /// let mut c1 = state.color([128, 0, 128]);
    /// let levels = c1.norm_levels();
    /// assert_approx_eq!(levels[0], 0.501960);
    /// assert_approx_eq!(levels[1], 0.0);
    /// assert_approx_eq!(levels[2], 0.501960);
    /// assert_approx_eq!(levels[3], 1.0);
    /// ```
    pub fn norm_levels(self) -> [f64; 4] {
        [
            self.levels[0] as f64 / 255.0,
            self.levels[1] as f64 / 255.0,
            self.levels[2] as f64 / 255.0,
            self.levels[3] as f64 / 255.0,
        ]
    }

    /// Gets the cached hsba values, or calculates it and caches it.
    fn get_hsba(&mut self) -> [f64; 4] {
        self.hsba.unwrap_or_else(|| {
            let hsba = Color::rgba_to_hsba(self.values);
            self.hsba = Some(hsba);
            hsba
        })
    }

    /// Gets the cached hsla values, or calculates it and caches it.
    fn get_hsla(&mut self) -> [f64; 4] {
        self.hsla.unwrap_or_else(|| {
            let hsla = Color::rgba_to_hsla(self.values);
            self.hsla = Some(hsla);
            hsla
        })
    }

    /// Calculates and stores the closest RGBA screen levels. Used internally any time a channel
    /// value is changed.
    fn calculate_levels(&mut self) {
        for (i, v) in self.values.iter().enumerate() {
            self.levels[i] = (v * 255.0).round() as u8;
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.levels)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.levels() == other.levels()
    }
}
impl Eq for Color {}

impl StateData {
    /// Creates a new `Color` instance. The parameters are interpreted as either RGB, HSB, or HSL
    /// depending on the current `State::color_mode()`. The default is RGB.
    ///
    /// Values can be provided as either integer or floating point values. Floating point values
    /// will be rounded to the nearest RGBA screen color for drawing.
    ///
    /// The number of parameters provided alter how they are interpreted:
    ///
    /// # Syntax
    ///
    /// ```text
    /// state.color(gray);
    /// state.color([gray, [alpha]]);
    /// state.color([v1, v2, v3, [alpha]]);
    /// state.color(colorstring);
    /// state.color(slice);
    /// state.color(color);
    /// ```
    ///
    /// # Parameters
    ///
    /// - **gray**: 0 to 255 value ranging from black to white..
    /// - **alpha**: Transparency value ranging from 0 to 255 for RGB or 0.0 to 1.0 for HSB/HSL
    ///   (Optional).
    /// - **v1**: Red (0 to 255) or Hue (0 to 360).
    /// - **v2**: Green (0 to 255) or Saturation (0 to 100).
    /// - **v3**: Blue (0 to 255) or Brightness/Lightness (0 to 100).
    /// - **colorstring**: A string of from the `NAMED_COLORS` constant or a hexadecimal value (in 3, 4, 6, or 8 digit formats).
    /// - **slice**: A slice containing rgba or hsba/hsla values.
    /// - **color**: A `Color` instance.
    ///
    /// # Grayscale Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # let mut state = StateData::default();
    ///
    /// let c = state.color(128); // Gray
    /// assert_eq!(c.levels(), [128, 128, 128, 255]);
    ///
    /// let c = state.color([128]); // Gray
    /// assert_eq!(c.levels(), [128, 128, 128, 255]);
    ///
    /// let c = state.color([128, 64]); // Gray with Alpha
    /// assert_eq!(c.levels(), [128, 128, 128, 64]);
    /// ```
    ///
    /// # RGB Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # let mut state = StateData::default();
    ///
    /// let c = state.color([128, 64, 0]); // RGB
    /// assert_eq!(c.levels(), [128, 64, 0, 255]);
    ///
    /// let c = state.color([128, 64, 128, 128]); // RGBA
    /// assert_eq!(c.levels(), [128, 64, 128, 128]);
    /// ```
    ///
    /// # HSB/HSL Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # let mut state = StateData::default();
    ///
    /// state.color_mode(ColorMode::Hsb); // Change color mode to Hue/Saturation/Brightness
    /// let c = state.color([337, 100, 80]);
    /// assert_eq!(c.levels(), [204, 0, 78, 255]);
    ///
    /// state.color_mode(ColorMode::Hsl); // Change color mode to Hue/Saturation/Lightness
    /// let c = state.color([337, 100, 40]);
    /// assert_eq!(c.levels(), [204, 0, 78, 255]);
    /// ```
    ///
    /// # Colorstring Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # let mut state = StateData::default();
    ///
    /// let c = state.color("aliceblue"); // Named color string
    /// assert_eq!(c.levels(), [240, 248, 255, 255]);
    ///
    /// let c = state.color("#F0F"); // 3-digit Hex string
    /// assert_eq!(c.levels(), [255, 0, 255, 255]);
    ///
    /// let c = state.color("#F0F5"); // 4-digit Hex string
    /// assert_eq!(c.levels(), [255, 0, 255, 85]);
    ///
    /// let c = state.color("#F0F5BF"); // 6-digit Hex string
    /// assert_eq!(c.levels(), [240, 245, 191, 255]);
    ///
    /// let c = state.color("#F0F5BF5F"); // 8-digit Hex string
    /// assert_eq!(c.levels(), [240, 245, 191, 95]);
    /// ```
    ///
    /// # Other Examples
    /// ```
    /// # use pix_engine::prelude::*;
    /// # let mut state = StateData::default();
    ///
    /// let vals: Vec<u8> = vec![128, 64, 0];
    /// let c = state.color(&vals); // RGB Vec
    /// assert_eq!(c.levels(), [128, 64, 0, 255]);
    ///
    /// let vals: [u8; 4] = [128, 64, 0, 128];
    /// let c = state.color(&vals[..]); // RGBA slice
    /// assert_eq!(c.levels(), [128, 64, 0, 128]);
    ///
    /// let c = state.color(Color::rgb([128, 255, 0]));
    /// assert_eq!(c.mode(), ColorMode::Rgb);
    /// assert_eq!(c.levels(), [128, 255, 0, 255]);
    ///
    /// let mut c = state.color(Color::hsb([300, 90, 60]));
    /// assert_eq!(c.mode(), ColorMode::Hsb);
    /// assert_eq!(c.levels(), [153, 15, 153, 255]);
    /// assert_eq!(c.hue(), 300.0);
    /// assert_eq!(c.saturation(), 90.0);
    /// assert_eq!(c.brightness(), 60.0);
    /// assert_eq!(c.alpha(), 1.0);
    ///
    /// let mut c = state.color(Color::hsl([300.0, 90.0, 30.0, 0.9]));
    /// assert_eq!(c.mode(), ColorMode::Hsl);
    /// assert_eq!(c.levels(), [145, 8, 145, 230]);
    /// assert_eq!(c.hue(), 300.0);
    /// assert_eq!(c.saturation(), 90.0);
    /// assert_eq!(c.lightness(), 30.0);
    /// assert_eq!(c.alpha(), 0.9);
    /// ```
    pub fn color<L: Into<ColorLevels>>(&self, levels: L) -> Color {
        let mut levels = levels.into();
        if levels.mode == ColorMode::default() {
            levels.mode = self.get_color_mode();
            levels.maxes = self.get_color_maxes();
        }
        Color::from_levels(levels)
    }

    /// Creates a new `Color` instance by linear interpolating between two colors by a given
    /// amount.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::prelude::*;
    /// # let mut state = StateData::default();
    ///
    /// let from = Color::rgb([255, 0, 0]);
    /// let to = Color::rgb([0, 100, 255]);
    /// let lerped = state.lerp_color(from, to, 0.5);
    /// assert_eq!(lerped.levels(), [128, 50, 128, 255]);
    /// ```
    #[inline(always)]
    pub fn lerp_color<C1, C2>(&self, c1: C1, c2: C2, amt: f64) -> Color
    where
        C1: Into<Color>,
        C2: Into<Color>,
    {
        let mut c1 = c1.into();
        let mut c2 = c2.into();

        let (from, to) = match self.get_color_mode() {
            ColorMode::Rgb => (c1.norm_levels(), c2.norm_levels()),
            ColorMode::Hsb => {
                c1.hue(); // Ensure values exist in cache
                c2.hue();
                (c1.hsba.unwrap(), c2.hsba.unwrap())
            }
            ColorMode::Hsl => {
                c1.hue(); // Ensure values exist in cache
                c2.hue();
                (c1.hsla.unwrap(), c2.hsla.unwrap())
            }
        };

        let amt = constrainf(amt, 0.0, 1.0);

        let lerp = |start, stop, amt| amt * (stop - start) + start;

        let mode = self.get_color_mode();
        let maxes = self.get_color_maxes();
        let mode_maxes = match mode {
            ColorMode::Hsb => maxes.hsb,
            ColorMode::Hsl => maxes.hsl,
            ColorMode::Rgb => maxes.rgb,
        };

        let l0 = lerp(from[0], to[0], amt) * mode_maxes[0] as f64;
        let l1 = lerp(from[1], to[1], amt) * mode_maxes[1] as f64;
        let l2 = lerp(from[2], to[2], amt) * mode_maxes[2] as f64;
        let l3 = lerp(from[3], to[3], amt) * mode_maxes[3] as f64;

        let mut levels = ColorLevels::from([l0, l1, l2, l3]);
        levels.mode = mode;
        levels.maxes = maxes;
        Color::from_levels(levels)
    }
}
