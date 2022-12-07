//! [Color] conversion functions.
//!
//! Provides methods for converting [Color]s:
//!
//! - [`Color::from_str`]: Parse from a hexadecimal string (e.g. "#FF0", or "#00FF00").
//! - [`Color::from_slice`]: Parse from an RGB/A slice (e.g. [0, 125, 0] or [255, 255, 0, 125]).
//! - [`Color::from_hex`]: Parse from a `u32` RGBA hexadecimal value (e.g. 0x00FF00FF).
//! - [`Color::as_hex`]: Convert back to a `u32` RGBA hexadecimal value.
//! - [`Color::inverted`]: Invert the RGB colors channel-wise, ignoring the alpha channel.
//! - [`Color::lerp`]: Linear interpolate between two colors, channel-wise, including the alpha
//!   channel.
//!
//! Examples
//!
//! ```no_run
//! # // no_run because these tests are run on their respective functions
//! # use pix_engine::prelude::*;
//! use std::str::FromStr;
//!
//! # fn main() -> PixResult<()> {
//! let c = Color::from_str("#F0F5BF")?;
//! assert_eq!(c.channels(), [240, 245, 191, 255]);
//!
//! let vals = [128.0, 64.0, 0.0];
//! let c = Color::from_slice(ColorMode::Rgb, &vals)?;
//! assert_eq!(c.channels(), [128, 64, 0, 255]);
//!
//! let c = Color::from_hex(0xF0FF_00FF);
//! assert_eq!(c.channels(), [240, 255, 0, 255]);
//!
//! let c = Color::rgba(255, 0, 255, 125);
//! assert_eq!(c.inverted().as_hex(), 0xFF00_FF7D);
//!
//! let from = rgb!(255, 0, 0);
//! let to = rgb!(0, 100, 255);
//! let lerped = from.lerp(to, 0.5);
//! assert_eq!(lerped.channels(), [128, 50, 128, 255]);
//! # Ok(())
//! # }
//! ```

use super::Mode::{Hsb, Hsl, Rgb};
use crate::prelude::*;
use std::{convert::TryFrom, result, str::FromStr};

impl Color {
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
    /// # fn main() -> PixResult<()> {
    /// let vals: Vec<f64> = vec![128.0, 64.0, 0.0];
    /// let c = Color::from_slice(ColorMode::Rgb, &vals)?; // RGB Vec
    /// assert_eq!(c.channels(), [128, 64, 0, 255]);
    ///
    /// let vals: [f64; 4] = [128.0, 64.0, 0.0, 128.0];
    /// let c = Color::from_slice(ColorMode::Rgb, &vals[..])?; // RGBA slice
    /// assert_eq!(c.channels(), [128, 64, 0, 128]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_slice<T, S>(mode: ColorMode, slice: S) -> PixResult<Self>
    where
        T: Copy + Into<f64>,
        S: AsRef<[T]>,
    {
        let slice = slice.as_ref();
        let result = match *slice {
            [gray] => Self::with_mode(mode, gray, gray, gray),
            [gray, a] => Self::with_mode_alpha(mode, gray, gray, gray, a),
            [v1, v2, v3] => Self::with_mode(mode, v1, v2, v3),
            [v1, v2, v3, a] => Self::with_mode_alpha(mode, v1, v2, v3, a),
            _ => return Err(PixError::InvalidColorSlice.into()),
        };
        Ok(result)
    }

    /// Constructs a `Color` from a [u32] RGB hexadecimal value with max alpha.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::from_hex(0xF0FF00);
    /// assert_eq!(c.channels(), [240, 255, 0, 255]);
    /// ```
    #[inline]
    pub const fn from_hex(hex: u32) -> Self {
        let [_, r, g, b] = hex.to_be_bytes();
        Self::rgba(r, g, b, 255)
    }

    /// Constructs a `Color` from a [u32] RGBA hexadecimal value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::from_hex_alpha(0xF0FF_00FF);
    /// assert_eq!(c.channels(), [240, 255, 0, 255]);
    ///
    /// let c = Color::from_hex_alpha(0xF0FF_0080);
    /// assert_eq!(c.channels(), [240, 255, 0, 128]);
    /// ```
    #[inline]
    pub const fn from_hex_alpha(hex: u32) -> Self {
        let [r, g, b, a] = hex.to_be_bytes();
        Self::rgba(r, g, b, a)
    }

    /// Constructs a `Color` by inverting the RGBA values.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = Color::from_hex(0xF0FF00);
    /// assert_eq!(c.inverted().as_hex(), 0x0F00FF);
    /// ```
    #[inline]
    pub const fn inverted(&self) -> Self {
        let hex = self.as_hex();
        Self::from_hex(0x00FF_FFFF ^ hex)
    }

    /// Constructs an opaque `Color` blended over a given background, using an alpha value.
    pub fn blended<A>(&self, bg: Color, alpha: A) -> Self
    where
        A: Into<f64>,
    {
        let a = alpha.into().clamp(0.0, 1.0);
        let [v1, v2, v3, _] = convert_levels(bg.levels(), bg.mode(), self.mode());
        let [bv1, bv2, bv3, _] = self.levels();

        let blended = |bg: f64, fg: f64, alpha: f64| bg.mul_add(1.0 - alpha, fg * alpha);
        let levels = clamp_levels([
            blended(v1, bv1, a),
            blended(v2, bv2, a),
            blended(v3, bv3, a),
            1.0,
        ]);
        Self {
            mode: self.mode,
            channels: calculate_channels(levels),
        }
    }

    /// Constructs a `Color` by linear interpolating between two `Color`s by a given amount between
    /// `0.0` and `1.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let from = rgb!(255, 0, 0);
    /// let to = rgb!(0, 100, 255);
    /// let lerped = from.lerp(to, 0.5);
    /// assert_eq!(lerped.channels(), [128, 50, 128, 255]);
    ///
    /// let from = rgb!(255, 0, 0);
    /// let to = hsb!(120.0, 80.0, 100.0, 0.5);
    /// let lerped = from.lerp(to, 0.25); // `to` is implicity converted to RGB
    /// assert_eq!(lerped.channels(), [204, 64, 13, 223]);
    /// ```
    pub fn lerp<A>(&self, other: Color, amt: A) -> Self
    where
        A: Into<f64>,
    {
        let lerp = |start: f64, stop: f64, amt: f64| amt.mul_add(stop - start, start);

        let amt = amt.into().clamp(0.0, 1.0);
        let [v1, v2, v3, a] = self.levels();
        let [ov1, ov2, ov3, oa] = convert_levels(other.levels(), other.mode(), self.mode());
        let levels = clamp_levels([
            lerp(v1, ov1, amt),
            lerp(v2, ov2, amt),
            lerp(v3, ov3, amt),
            lerp(a, oa, amt),
        ]);
        Self {
            mode: self.mode,
            channels: calculate_channels(levels),
        }
    }
}

impl FromStr for Color {
    type Err = PixError;

    /// Converts to [Color] from a hexadecimal string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// use std::str::FromStr;
    ///
    /// # fn main() -> PixResult<()> {
    /// let c = Color::from_str("#F0F")?; // 3-digit Hex string
    /// assert_eq!(c.channels(), [255, 0, 255, 255]);
    ///
    /// let c = Color::from_str("#F0F5")?; // 4-digit Hex string
    /// assert_eq![c.channels(), [255, 0, 255, 85]];
    ///
    /// let c = Color::from_str("#F0F5BF")?; // 6-digit Hex string
    /// assert_eq!(c.channels(), [240, 245, 191, 255]);
    ///
    /// let c = Color::from_str("#F0F5BF5F")?; // 8-digit Hex string
    /// assert_eq!(c.channels(), [240, 245, 191, 95]);
    /// # Ok(())
    /// # }
    /// ```
    fn from_str(string: &str) -> result::Result<Self, Self::Err> {
        if !string.starts_with('#') {
            return Err(PixError::ParseColorError);
        }

        let mut channels: [u8; 4] = [0, 0, 0, 255];
        let parse_hex =
            |hex: &str| u8::from_str_radix(hex, 16).map_err(|_| PixError::ParseColorError);

        let string = string.trim().to_lowercase();
        match string.len() - 1 {
            3 | 4 => {
                for (i, _) in string[1..].char_indices() {
                    let hex = parse_hex(&string[i + 1..i + 2])?;
                    channels[i] = (hex << 4) | hex;
                }
            }
            6 | 8 => {
                for (i, _) in string[1..].char_indices().step_by(2) {
                    channels[i / 2] = parse_hex(&string[i + 1..i + 3])?;
                }
            }
            _ => return Err(PixError::ParseColorError),
        }

        let [r, g, b, a] = channels;
        Ok(Self::rgba(r, g, b, a))
    }
}

impl TryFrom<&str> for Color {
    type Error = PixError;
    /// Try to create a `Color` from a hexadecimal string.
    fn try_from(s: &str) -> result::Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

/// Return the max value for each [`ColorMode`].
pub(crate) const fn maxes(mode: ColorMode) -> [f64; 4] {
    match mode {
        Rgb => [255.0; 4],
        Hsb | Hsl => [360.0, 100.0, 100.0, 1.0],
    }
}

/// Clamp levels to `0.0..=1.0`.
pub(crate) fn clamp_levels(levels: [f64; 4]) -> [f64; 4] {
    [
        levels[0].clamp(0.0, 1.0),
        levels[1].clamp(0.0, 1.0),
        levels[2].clamp(0.0, 1.0),
        levels[3].clamp(0.0, 1.0),
    ]
}

/// Converts levels from one [`ColorMode`] to another.
pub(crate) fn convert_levels(levels: [f64; 4], from: ColorMode, to: ColorMode) -> [f64; 4] {
    match (from, to) {
        (Hsb, Rgb) => hsb_to_rgb(levels),
        (Hsl, Rgb) => hsl_to_rgb(levels),
        (Rgb, Hsb) => rgb_to_hsb(levels),
        (Rgb, Hsl) => rgb_to_hsl(levels),
        (Hsb, Hsl) => hsb_to_hsl(levels),
        (Hsl, Hsb) => hsl_to_hsb(levels),
        (_, _) => levels,
    }
}

/// Converts to [Rgb] to [Hsb] format.
#[allow(clippy::many_single_char_names)]
pub(crate) fn rgb_to_hsb([r, g, b, a]: [f64; 4]) -> [f64; 4] {
    let c_max = r.max(g).max(b);
    let c_min = r.min(g).min(b);
    let chr = c_max - c_min;
    if chr.abs() < f64::EPSILON {
        [0.0, 0.0, c_max, a]
    } else {
        let mut h = if (r - c_max).abs() < f64::EPSILON {
            // Magenta to yellow
            (g - b) / chr
        } else if (g - c_max).abs() < f64::EPSILON {
            // Yellow to cyan
            2.0 + (b - r) / chr
        } else {
            // Cyan to magenta
            4.0 + (r - g) / chr
        };
        if h < 0.0 {
            h += 6.0;
        } else if h >= 6.0 {
            h -= 6.0;
        }
        let s = chr / c_max;
        [h / 6.0, s, c_max, a]
    }
}

/// Converts to [Rgb] to [Hsl] format.
#[allow(clippy::many_single_char_names)]
pub(crate) fn rgb_to_hsl([r, g, b, a]: [f64; 4]) -> [f64; 4] {
    let c_max = r.max(g).max(b);
    let c_min = r.min(g).min(b);
    let l = c_max + c_min;
    let chr = c_max - c_min;
    if chr.abs() < f64::EPSILON {
        [0.0, 0.0, l / 2.0, a]
    } else {
        let mut h = if (r - c_max).abs() < f64::EPSILON {
            // Magenta to yellow
            (g - b) / chr
        } else if (g - c_max).abs() < f64::EPSILON {
            // Yellow to cyan
            2.0 + (b - r) / chr
        } else {
            // Cyan to magenta
            4.0 + (r - g) / chr
        };
        if h < 0.0 {
            h += 6.0;
        } else if h >= 6.0 {
            h -= 6.0;
        }
        let s = if l < 1.0 { chr / l } else { chr / (2.0 - l) };
        [h / 6.0, s, l / 2.0, a]
    }
}

/// Converts to [Hsb] to [Rgb] format.
#[allow(clippy::many_single_char_names)]
pub(crate) fn hsb_to_rgb([h, s, b, a]: [f64; 4]) -> [f64; 4] {
    if b.abs() < f64::EPSILON {
        [0.0, 0.0, 0.0, a]
    } else if s.abs() < f64::EPSILON {
        [b, b, b, a]
    } else {
        let h = h * 6.0;
        let sector = h.floor().clamp(0.0, 2160.0) as usize;
        let tint1 = b * (1.0 - s);
        let tint2 = b * (1.0 - s * (h - sector as f64));
        let tint3 = b * (1.0 - s * (1.0 + sector as f64 - h));
        let (r, g, b) = match sector {
            // Yellow to green
            1 => (tint2, b, tint1),
            // Green to cyan
            2 => (tint1, b, tint3),
            // Cyan to blue
            3 => (tint1, tint2, b),
            // Blue to magenta
            4 => (tint3, tint1, b),
            // Magenta to red
            5 => (b, tint1, tint2),
            // Red to yellow (sector is 0 or 6)
            _ => (b, tint3, tint1),
        };
        [r, g, b, a]
    }
}

/// Converts to [Hsl] to [Rgb] format.
#[allow(clippy::many_single_char_names)]
pub(crate) fn hsl_to_rgb([h, s, l, a]: [f64; 4]) -> [f64; 4] {
    if s.abs() < f64::EPSILON {
        [l, l, l, a]
    } else {
        let h = h * 6.0;
        let b = if l < 0.5 {
            (1.0 + s) * l
        } else {
            l + s - l * s
        };
        let zest = 2.0 * l - b;
        let hzb_to_rgb = |h: f64, z: f64, b: f64| -> f64 {
            let h = if h < 0.0 {
                h + 6.0
            } else if h >= 6.0 {
                h - 6.0
            } else {
                h
            };
            match h {
                // Red to yellow (increasing green)
                _ if h < 1.0 => (b - z).mul_add(h, z),
                // Yellow to cyan (greatest green)
                _ if h < 3.0 => b,
                // Cyan to blue (decreasing green)
                _ if h < 4.0 => (b - z).mul_add(4.0 - h, z),
                // Blue to red (least green)
                _ => z,
            }
        };
        [
            hzb_to_rgb(h + 2.0, zest, b),
            hzb_to_rgb(h, zest, b),
            hzb_to_rgb(h - 2.0, zest, b),
            a,
        ]
    }
}

/// Converts to [Hsl] to [Hsb] format.
#[allow(clippy::many_single_char_names)]
pub(crate) fn hsl_to_hsb([h, s, l, a]: [f64; 4]) -> [f64; 4] {
    let b = if l < 0.5 {
        (1.0 + s) * l
    } else {
        l + s - l * s
    };
    let s = 2.0 * (b - l) / b;
    [h, s, b, a]
}

/// Converts to [Hsb] to [Hsl] format.
#[allow(clippy::many_single_char_names)]
pub(crate) fn hsb_to_hsl([h, s, b, a]: [f64; 4]) -> [f64; 4] {
    let l = (2.0 - s) * b / 2.0;
    let s = match l {
        _ if (l - 1.0).abs() < f64::EPSILON => 0.0,
        _ if l < 0.5 => s / 2.0 - s,
        _ => s * b / (2.0 - l * 2.0),
    };
    [h, s, l, a]
}

/// Converts levels to [u8] RGBA channels.
pub(crate) fn calculate_channels(levels: [f64; 4]) -> [u8; 4] {
    let [r, g, b, a] = levels;
    let [r_max, g_max, b_max, a_max] = maxes(Rgb);
    [
        (r * r_max).round().clamp(0.0, 255.0) as u8,
        (g * g_max).round().clamp(0.0, 255.0) as u8,
        (b * b_max).round().clamp(0.0, 255.0) as u8,
        (a * a_max).round().clamp(0.0, 255.0) as u8,
    ]
}

impl Color {
    /// Update RGB channels by calculating them from the current levels.
    pub(crate) fn update_channels(&mut self, levels: [f64; 4], mode: ColorMode) {
        let levels = convert_levels(levels, mode, Rgb);
        self.channels = calculate_channels(levels);
    }
}

macro_rules! impl_from {
    ($($source: ty),*) => {
        $(
            impl From<$source> for Color {
                #[doc = concat!("Convert [", stringify!($source), "] to grayscale `Color`")]
                fn from(gray: $source) -> Self {
                    let gray = f64::from(gray);
                    Self::with_mode(Rgb, gray, gray, gray)
                }
            }

            impl From<[$source; 1]> for Color {
                #[doc = concat!("Convert [", stringify!($source), "] to grayscale `Color`")]
                fn from([gray]: [$source; 1]) -> Self {
                    let gray = f64::from(gray);
                    Self::with_mode(Rgb, gray, gray, gray)
                }
            }

            impl From<[$source; 2]> for Color {
                #[doc = concat!("Convert `[", stringify!($source), "; 2]` to grayscale `Color` with alpha")]
                fn from([gray, alpha]: [$source; 2]) -> Self {
                    let gray = f64::from(gray);
                    let alpha = f64::from(alpha);
                    Self::with_mode_alpha(Rgb, gray, gray, gray, alpha)
                }
            }

            impl From<[$source; 3]> for Color {
                #[doc = concat!("Convert `[", stringify!($source), "; 3]` to `Color` with max alpha")]
                fn from([r, g, b]: [$source; 3]) -> Self {
                    Self::with_mode(Rgb, f64::from(r), f64::from(g), f64::from(b))
                }
            }

            impl From<[$source; 4]> for Color {
                #[doc = concat!("Convert `[", stringify!($source), "; 4]` to `Color`")]
                fn from([r, g, b, a]: [$source; 4]) -> Self {
                    Self::with_mode_alpha(Rgb, f64::from(r), f64::from(g), f64::from(b), f64::from(a))
                }
            }
        )*
    };
}

impl_from!(i8, u8, i16, u16, f32);
impl_from!(i32, u32, f64);

#[cfg(test)]
mod tests {
    use crate::prelude::{hsb, hsl, rgb, Color};

    macro_rules! assert_color_eq {
        ($c1:expr, $c2:expr) => {
            assert_eq!($c1.channels(), $c2.channels());
        };
    }

    #[test]
    fn test_slice_conversions() {
        let _: Color = 50u8.into();
        let _: Color = 50i8.into();
        let _: Color = 50u16.into();
        let _: Color = 50i16.into();
        let _: Color = 50u32.into();
        let _: Color = 50i32.into();
        let _: Color = 50.0f32.into();
        let _: Color = 50.0f64.into();

        let _: Color = [50u8].into();
        let _: Color = [50i8].into();
        let _: Color = [50u16].into();
        let _: Color = [50i16].into();
        let _: Color = [50u32].into();
        let _: Color = [50i32].into();
        let _: Color = [50.0f32].into();
        let _: Color = [50.0f64].into();

        let _: Color = [50u8, 100].into();
        let _: Color = [50i8, 100].into();
        let _: Color = [50u16, 100].into();
        let _: Color = [50i16, 100].into();
        let _: Color = [50u32, 100].into();
        let _: Color = [50i32, 100].into();
        let _: Color = [50.0f32, 100.0].into();
        let _: Color = [50.0f64, 100.0].into();

        let _: Color = [50u8, 100, 55].into();
        let _: Color = [50i8, 100, 55].into();
        let _: Color = [50u16, 100, 55].into();
        let _: Color = [50i16, 100, 55].into();
        let _: Color = [50u32, 100, 55].into();
        let _: Color = [50i32, 100, 55].into();
        let _: Color = [50.0f32, 100.0, 55.0].into();
        let _: Color = [50.0f64, 100.0, 55.0].into();

        let _: Color = [50u8, 100, 55, 100].into();
        let _: Color = [50i8, 100, 55, 100].into();
        let _: Color = [50u16, 100, 55, 100].into();
        let _: Color = [50i16, 100, 55, 100].into();
        let _: Color = [50u32, 100, 55, 100].into();
        let _: Color = [50i32, 100, 55, 100].into();
        let _: Color = [50.0f32, 100.0, 55.0, 100.0].into();
        let _: Color = [50.0f64, 100.0, 55.0, 100.0].into();
    }

    #[test]
    fn test_hsb_to_rgb() {
        assert_color_eq!(hsb!(0.0, 0.0, 0.0), rgb!(0, 0, 0));
        assert_color_eq!(hsb!(0.0, 0.0, 100.0), rgb!(255, 255, 255));
        assert_color_eq!(hsb!(0.0, 100.0, 100.0), rgb!(255, 0, 0));
        assert_color_eq!(hsb!(120.0, 100.0, 100.0), rgb!(0, 255, 0));
        assert_color_eq!(hsb!(240.0, 100.0, 100.0), rgb!(0, 0, 255));
        assert_color_eq!(hsb!(60.0, 100.0, 100.0), rgb!(255, 255, 0));
        assert_color_eq!(hsb!(180.0, 100.0, 100.0), rgb!(0, 255, 255));
        assert_color_eq!(hsb!(300.0, 100.0, 100.0), rgb!(255, 0, 255));
        assert_color_eq!(hsb!(0.0, 0.0, 75.0), rgb!(191, 191, 191));
        assert_color_eq!(hsb!(0.0, 0.0, 50.0), rgb!(128, 128, 128));
        assert_color_eq!(hsb!(0.0, 100.0, 50.0), rgb!(128, 0, 0));
        assert_color_eq!(hsb!(60.0, 100.0, 50.0), rgb!(128, 128, 0));
        assert_color_eq!(hsb!(120.0, 100.0, 50.0), rgb!(0, 128, 0));
        assert_color_eq!(hsb!(300.0, 100.0, 50.0), rgb!(128, 0, 128));
        assert_color_eq!(hsb!(180.0, 100.0, 50.0), rgb!(0, 128, 128));
        assert_color_eq!(hsb!(240.0, 100.0, 50.0), rgb!(0, 0, 128));
    }

    #[test]
    fn test_hsb_to_hsl() {
        assert_color_eq!(hsb!(0.0, 0.0, 0.0), hsl!(0.0, 0.0, 0.0));
        assert_color_eq!(hsb!(0.0, 0.0, 100.0), hsl!(0.0, 0.0, 100.0));
        assert_color_eq!(hsb!(0.0, 100.0, 100.0), hsl!(0.0, 100.0, 50.0));
        assert_color_eq!(hsb!(120.0, 100.0, 100.0), hsl!(120.0, 100.0, 50.0));
        assert_color_eq!(hsb!(240.0, 100.0, 100.0), hsl!(240.0, 100.0, 50.0));
        assert_color_eq!(hsb!(60.0, 100.0, 100.0), hsl!(60.0, 100.0, 50.0));
        assert_color_eq!(hsb!(180.0, 100.0, 100.0), hsl!(180.0, 100.0, 50.0));
        assert_color_eq!(hsb!(300.0, 100.0, 100.0), hsl!(300.0, 100.0, 50.0));
        assert_color_eq!(hsb!(0.0, 0.0, 75.0), hsl!(0.0, 0.0, 75.0));
        assert_color_eq!(hsb!(0.0, 0.0, 50.0), hsl!(0.0, 0.0, 50.0));
        assert_color_eq!(hsb!(0.0, 100.0, 50.0), hsl!(0.0, 100.0, 25.0));
        assert_color_eq!(hsb!(60.0, 100.0, 50.0), hsl!(60.0, 100.0, 25.0));
        assert_color_eq!(hsb!(120.0, 100.0, 50.0), hsl!(120.0, 100.0, 25.0));
        assert_color_eq!(hsb!(300.0, 100.0, 50.0), hsl!(300.0, 100.0, 25.0));
        assert_color_eq!(hsb!(180.0, 100.0, 50.0), hsl!(180.0, 100.0, 25.0));
        assert_color_eq!(hsb!(240.0, 100.0, 50.0), hsl!(240.0, 100.0, 25.0));
    }

    #[test]
    fn test_hsl_to_rgb() {
        assert_color_eq!(hsl!(0.0, 0.0, 0.0), rgb!(0, 0, 0));
        assert_color_eq!(hsl!(0.0, 0.0, 100.0), rgb!(255, 255, 255));
        assert_color_eq!(hsl!(0.0, 100.0, 100.0), rgb!(255, 255, 255));
        assert_color_eq!(hsl!(120.0, 100.0, 100.0), rgb!(255, 255, 255));
        assert_color_eq!(hsl!(240.0, 100.0, 100.0), rgb!(255, 255, 255));
        assert_color_eq!(hsl!(60.0, 100.0, 100.0), rgb!(255, 255, 255));
        assert_color_eq!(hsl!(180.0, 100.0, 100.0), rgb!(255, 255, 255));
        assert_color_eq!(hsl!(300.0, 100.0, 100.0), rgb!(255, 255, 255));
        assert_color_eq!(hsl!(0.0, 0.0, 75.0), rgb!(191, 191, 191));
        assert_color_eq!(hsl!(0.0, 0.0, 50.0), rgb!(128, 128, 128));
        assert_color_eq!(hsl!(0.0, 100.0, 50.0), rgb!(255, 0, 0));
        assert_color_eq!(hsl!(60.0, 100.0, 50.0), rgb!(255, 255, 0));
        assert_color_eq!(hsl!(120.0, 100.0, 50.0), rgb!(0, 255, 0));
        assert_color_eq!(hsl!(300.0, 100.0, 50.0), rgb!(255, 0, 255));
        assert_color_eq!(hsl!(180.0, 100.0, 50.0), rgb!(0, 255, 255));
        assert_color_eq!(hsl!(240.0, 100.0, 50.0), rgb!(0, 0, 255));
    }

    #[test]
    fn test_hsl_to_hsb() {
        assert_color_eq!(hsl!(0.0, 0.0, 0.0), hsb!(0.0, 0.0, 0.0));
        assert_color_eq!(hsl!(0.0, 0.0, 100.0), hsb!(0.0, 0.0, 100.0));
        assert_color_eq!(hsl!(0.0, 100.0, 100.0), hsb!(0.0, 0.0, 100.0));
        assert_color_eq!(hsl!(120.0, 100.0, 100.0), hsb!(120.0, 0.0, 100.0));
        assert_color_eq!(hsl!(240.0, 100.0, 100.0), hsb!(240.0, 0.0, 100.0));
        assert_color_eq!(hsl!(60.0, 100.0, 100.0), hsb!(60.0, 0.0, 100.0));
        assert_color_eq!(hsl!(180.0, 100.0, 100.0), hsb!(180.0, 0.0, 100.0));
        assert_color_eq!(hsl!(300.0, 100.0, 100.0), hsb!(300.0, 0.0, 100.0));
        assert_color_eq!(hsl!(0.0, 0.0, 75.0), hsb!(0.0, 0.0, 75.0));
        assert_color_eq!(hsl!(0.0, 0.0, 50.0), hsb!(0.0, 0.0, 50.0));
        assert_color_eq!(hsl!(0.0, 100.0, 50.0), hsb!(0.0, 100.0, 100.0));
        assert_color_eq!(hsl!(60.0, 100.0, 50.0), hsb!(60.0, 100.0, 100.0));
        assert_color_eq!(hsl!(120.0, 100.0, 50.0), hsb!(120.0, 100.0, 100.0));
        assert_color_eq!(hsl!(300.0, 100.0, 50.0), hsb!(300.0, 100.0, 100.0));
        assert_color_eq!(hsl!(180.0, 100.0, 50.0), hsb!(180.0, 100.0, 100.0));
        assert_color_eq!(hsl!(240.0, 100.0, 50.0), hsb!(240.0, 100.0, 100.0));
    }

    #[test]
    fn test_rgb_to_hsb() {
        assert_color_eq!(rgb!(0, 0, 0), hsb!(0.0, 0.0, 0.0));
        assert_color_eq!(rgb!(255, 255, 255), hsb!(0.0, 0.0, 100.0));
        assert_color_eq!(rgb!(255, 0, 0), hsb!(0.0, 100.0, 100.0));
        assert_color_eq!(rgb!(0, 255, 0), hsb!(120.0, 100.0, 100.0));
        assert_color_eq!(rgb!(0, 0, 255), hsb!(240.0, 100.0, 100.0));
        assert_color_eq!(rgb!(255, 255, 0), hsb!(60.0, 100.0, 100.0));
        assert_color_eq!(rgb!(0, 255, 255), hsb!(180.0, 100.0, 100.0));
        assert_color_eq!(rgb!(255, 0, 255), hsb!(300.0, 100.0, 100.0));
        assert_color_eq!(rgb!(191, 191, 191), hsb!(0.0, 0.0, 75.0));
        assert_color_eq!(rgb!(128, 128, 128), hsb!(0.0, 0.0, 50.0));
        assert_color_eq!(rgb!(128, 0, 0), hsb!(0.0, 100.0, 50.0));
        assert_color_eq!(rgb!(128, 128, 0), hsb!(60.0, 100.0, 50.0));
        assert_color_eq!(rgb!(0, 128, 0), hsb!(120.0, 100.0, 50.0));
        assert_color_eq!(rgb!(128, 0, 128), hsb!(300.0, 100.0, 50.0));
        assert_color_eq!(rgb!(0, 128, 128), hsb!(180.0, 100.0, 50.0));
        assert_color_eq!(rgb!(0, 0, 128), hsb!(240.0, 100.0, 50.0));
    }

    #[test]
    fn test_rgb_to_hsl() {
        assert_color_eq!(rgb!(0, 0, 0), hsl!(0.0, 0.0, 0.0));
        assert_color_eq!(rgb!(255, 255, 255), hsl!(0.0, 0.0, 100.0));
        assert_color_eq!(rgb!(255, 0, 0), hsl!(0.0, 100.0, 50.0));
        assert_color_eq!(rgb!(0, 255, 0), hsl!(120.0, 100.0, 50.0));
        assert_color_eq!(rgb!(0, 0, 255), hsl!(240.0, 100.0, 50.0));
        assert_color_eq!(rgb!(255, 255, 0), hsl!(60.0, 100.0, 50.0));
        assert_color_eq!(rgb!(0, 255, 255), hsl!(180.0, 100.0, 50.0));
        assert_color_eq!(rgb!(255, 0, 255), hsl!(300.0, 100.0, 50.0));
        assert_color_eq!(rgb!(191, 191, 191), hsl!(0.0, 0.0, 75.0));
        assert_color_eq!(rgb!(128, 128, 128), hsl!(0.0, 0.0, 50.0));
        assert_color_eq!(rgb!(128, 0, 0), hsl!(0.0, 100.0, 25.0));
        assert_color_eq!(rgb!(128, 128, 0), hsl!(60.0, 100.0, 25.0));
        assert_color_eq!(rgb!(0, 128, 0), hsl!(120.0, 100.0, 25.0));
        assert_color_eq!(rgb!(128, 0, 128), hsl!(300.0, 100.0, 25.0));
        assert_color_eq!(rgb!(0, 128, 128), hsl!(180.0, 100.0, 25.0));
        assert_color_eq!(rgb!(0, 0, 128), hsl!(240.0, 100.0, 25.0));
    }
}
