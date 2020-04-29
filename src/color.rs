//! Color handling and manipulation used for draw operations.
//!
//! Each color stores the color mode it was created with (e.g. Rgb or Hsl). Internally, colors are
//! stored as RGBA values ranging from 0-255. Other color representations than the initial color
//! mode are calculated and cached as needed.

use crate::{math::constrainf, StateData};
use conversion::ColorLevels;
use std::fmt;

pub mod prelude {
    pub use super::{constants::*, Color, ColorMode};
}
pub use constants::*;

mod constants;
mod conversion;

/// ColoreMode changes the way PixEngine interprets color data. The default is Rgb.
///
/// RGB values range from 0-255 for red, green, blue, and alpha
/// HSB values range from 0-360 for hue, and 0-100 for saturation and brightness
/// HSL values range from 0-360 for hue, and 0-100 for saturation and lightness
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

/// ColorMaxes limits the maximum levels the color will have per-channel based on the ColorMode.
/// The default is 0-255 for red, green, blue, and alpha, 0-360 for hue, and 0-100 for saturation,
/// brightness, and lightness.
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
            hsb: [360.0, 100.0, 100.0, 100.0],
            hsl: [360.0, 100.0, 100.0, 100.0],
        }
    }
}

/// Represents a color (by default stored as RGBA values ranging from 0-255).  The default is
/// black.
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

impl<'a> Color {
    /// Creates a Color instance from an array of color levels with a given ColorMode and
    /// ColorMaxes. Primarily used internally for the From/Into traits.
    #[inline(always)]
    fn from_levels<L: Into<ColorLevels>>(levels: L, mode: ColorMode, maxes: ColorMaxes) -> Self {
        let levels = levels.into();
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

        let values = match mode {
            ColorMode::Hsb => Color::hsba_to_rgba(values),
            ColorMode::Hsl => Color::hsla_to_rgba(values),
            ColorMode::Rgb => values,
        };

        let mut color = Self {
            values,
            mode,
            maxes,
            ..Default::default()
        };
        color.calculate_levels();
        color
    }

    /// Get the red value of the color ranging from 0-255.
    #[inline(always)]
    pub fn red(self) -> f64 {
        self.values[0] * self.maxes.rgb[0] as f64
    }
    /// Set the red value of the color ranging from 0-255.
    #[inline(always)]
    pub fn set_red(&mut self, r: u8) {
        self.values[0] = r as f64 / self.maxes.rgb[0];
        self.calculate_levels();
    }

    /// Get the green value of the color ranging from 0-255.
    #[inline(always)]
    pub fn green(self) -> f64 {
        self.values[1] * self.maxes.rgb[1] as f64
    }
    /// Set the green value of the color ranging from 0-255.
    #[inline(always)]
    pub fn set_green(&mut self, g: u8) {
        self.values[1] = g as f64 / self.maxes.rgb[1];
        self.calculate_levels();
    }

    /// Get the blue value of the color ranging from 0-255.
    #[inline(always)]
    pub fn blue(self) -> f64 {
        self.values[2] * self.maxes.rgb[2] as f64
    }
    /// Set the blue value of the color ranging from 0-255.
    #[inline(always)]
    pub fn set_blue(&mut self, b: u8) {
        self.values[2] = b as f64 / self.maxes.rgb[2];
        self.calculate_levels();
    }

    /// Get the alpha value of the color ranging from 0-255.
    #[inline(always)]
    pub fn alpha(self) -> f64 {
        self.values[3] * self.maxes.rgb[3]
    }
    /// Set the alpha value of the color ranging from 0-255.
    #[inline(always)]
    pub fn set_alpha(&mut self, a: u8) {
        self.values[3] = a as f64 / self.maxes.rgb[3];
        self.calculate_levels();
    }

    /// Get the hue value of the color ranging from 0-360.
    #[inline(always)]
    pub fn hue(&mut self) -> f64 {
        if self.mode == ColorMode::Hsb {
            if self.hsba.is_none() {
                self.hsba = Some(Color::rgba_to_hsba(self.values));
            }
            (self.hsba.unwrap())[0] * self.maxes.hsb[0]
        } else {
            if self.hsla.is_none() {
                self.hsla = Some(Color::rgba_to_hsla(self.values));
            }
            (self.hsla.unwrap())[0] * self.maxes.hsl[0]
        }
    }

    /// Get the saturation value of the color ranging from 0-360.
    #[inline(always)]
    pub fn saturation(&mut self) -> f64 {
        if self.mode == ColorMode::Hsb {
            if self.hsba.is_none() {
                self.hsba = Some(Color::rgba_to_hsba(self.values));
            }
            (self.hsba.unwrap())[1] * self.maxes.hsb[1]
        } else {
            if self.hsla.is_none() {
                self.hsla = Some(Color::rgba_to_hsla(self.values));
            }
            (self.hsla.unwrap())[1] * self.maxes.hsl[1]
        }
    }

    /// Get the brightness value of the color ranging from 0-100.
    #[inline(always)]
    pub fn brightness(&mut self) -> f64 {
        if self.hsba.is_none() {
            self.hsba = Some(Color::rgba_to_hsba(self.values));
        }
        (self.hsba.unwrap())[2] * self.maxes.hsb[2]
    }

    /// Get the lightness value of the color ranging from 0-100.
    #[inline(always)]
    pub fn lightness(&mut self) -> f64 {
        if self.hsla.is_none() {
            self.hsla = Some(Color::rgba_to_hsla(self.values));
        }
        (self.hsla.unwrap())[2] * self.maxes.hsl[2]
    }

    /// Get the ColorMode of this color.
    pub fn get_mode(&self) -> ColorMode {
        self.mode
    }

    /// Get the ColorMaxes for this color.
    pub fn get_maxes(&self) -> ColorMaxes {
        self.maxes
    }

    /// Returns an array containing the RGBA values.
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::prelude::*;
    /// # let mut state = StateData::new("State", 100, 100).unwrap();
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
    pub fn norm_levels(self) -> [f64; 4] {
        [
            self.levels[0] as f64 / 255.0,
            self.levels[1] as f64 / 255.0,
            self.levels[2] as f64 / 255.0,
            self.levels[3] as f64 / 255.0,
        ]
    }

    /// Calculates and stores the closest RGBA screen levels.
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
    /// state.color(gray, [alpha]);
    ///
    /// state.color(v1, v2, v3, [alpha]);
    ///
    /// state.color(value);
    ///
    /// state.color(values);
    ///
    /// state.color(color);
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
    /// # let mut state = StateData::new("State", 100, 100).unwrap();
    ///
    /// let c = state.color([128, 64, 0]); // RGB
    /// assert_eq!(c.levels(), [128, 64, 0, 255]);
    ///
    /// let c = state.color([128, 64, 128, 128]); // RGBA
    /// assert_eq!(c.levels(), [128, 64, 128, 128]);
    ///
    /// let c = state.color([128]); // Gray
    /// assert_eq!(c.levels(), [128, 128, 128, 255]);
    ///
    /// let c = state.color([128, 64]); // Gray with Alpha
    /// assert_eq!(c.levels(), [128, 128, 128, 64]);
    ///
    /// let vals: Vec<u8> = vec![128, 64, 0];
    /// let c = state.color(&vals); // RGB from slice
    /// assert_eq!(c.levels(), [128, 64, 0, 255]);
    ///
    /// let vals: [u8; 4] = [128, 64, 0, 128];
    /// let c = state.color(&vals[..]); // RGBA from slice
    /// assert_eq!(c.levels(), [128, 64, 0, 128]);
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
    pub fn color<L: Into<ColorLevels>>(&self, levels: L) -> Color {
        let mode = self.get_color_mode();
        let maxes = self.get_color_maxes();
        Color::from_levels(levels, mode, maxes)
    }

    /// Creates a new Color by linear interpolating between two colors by a given amount.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::prelude::*;
    /// # let mut state = StateData::new("State", 100, 100).unwrap();
    ///
    /// let from = Color::from([255, 0, 0]);
    /// let to = Color::from([0, 100, 255]);
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

        let levels = ColorLevels::from([l0, l1, l2, l3]);
        Color::from_levels(levels, mode, maxes)
    }
}
