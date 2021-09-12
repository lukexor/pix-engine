//! [Color] conversion functions.

use super::{
    Color,
    ColorMode::{self, *},
};
use crate::prelude::Scalar;
use std::{borrow::Cow, convert::TryFrom, error, fmt, result, str::FromStr};

/// The result type for [Color] conversions.
pub type Result<'a, T, U> = result::Result<T, Error<'a, U>>;

/// The error type for [Color] operations.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Error<'a, T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    /// Error when creating a [Color] from an invalid [slice].
    InvalidSlice(Cow<'a, [T]>),
    /// Error when creating a [Color] from an invalid string using [FromStr](std::str::FromStr).
    InvalidString(Cow<'a, str>),
}

impl<'a, T> fmt::Display for Error<'a, T>
where
    T: fmt::Debug,
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            InvalidSlice(slice) => write!(f, "invalid color slice: {:?}", slice),
            InvalidString(s) => write!(f, "invalid color string format: {}", s),
        }
    }
}

impl<'a, T> error::Error for Error<'a, T> where T: fmt::Debug + Clone {}

/// Return the max value for each [ColorMode].
pub(crate) const fn maxes(mode: ColorMode) -> [Scalar; 4] {
    match mode {
        Rgb => [255.0; 4],
        Hsb => [360.0, 100.0, 100.0, 1.0],
        Hsl => [360.0, 100.0, 100.0, 1.0],
    }
}

/// Clamp levels to `0.0..=1.0`.
pub(crate) fn clamp_levels(levels: [Scalar; 4]) -> [Scalar; 4] {
    [
        levels[0].clamp(0.0, 1.0),
        levels[1].clamp(0.0, 1.0),
        levels[2].clamp(0.0, 1.0),
        levels[3].clamp(0.0, 1.0),
    ]
}

/// Converts levels from one [ColorMode] to another.
pub(crate) fn convert_levels(levels: [Scalar; 4], from: ColorMode, to: ColorMode) -> [Scalar; 4] {
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
pub(crate) fn rgb_to_hsb([r, g, b, a]: [Scalar; 4]) -> [Scalar; 4] {
    let c_max = r.max(g).max(b);
    let c_min = r.min(g).min(b);
    let chr = c_max - c_min;
    if chr.abs() < Scalar::EPSILON {
        [0.0, 0.0, c_max, a]
    } else {
        let mut h = if (r - c_max).abs() < Scalar::EPSILON {
            // Magenta to yellow
            (g - b) / chr
        } else if (g - c_max).abs() < Scalar::EPSILON {
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
pub(crate) fn rgb_to_hsl([r, g, b, a]: [Scalar; 4]) -> [Scalar; 4] {
    let c_max = r.max(g).max(b);
    let c_min = r.min(g).min(b);
    let l = c_max + c_min;
    let chr = c_max - c_min;
    if chr.abs() < Scalar::EPSILON {
        [0.0, 0.0, l / 2.0, a]
    } else {
        let mut h = if (r - c_max).abs() < Scalar::EPSILON {
            // Magenta to yellow
            (g - b) / chr
        } else if (g - c_max).abs() < Scalar::EPSILON {
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
pub(crate) fn hsb_to_rgb([h, s, b, a]: [Scalar; 4]) -> [Scalar; 4] {
    if b.abs() < Scalar::EPSILON {
        [0.0, 0.0, 0.0, a]
    } else if s.abs() < Scalar::EPSILON {
        [b, b, b, a]
    } else {
        let h = h * 6.0;
        let sector = h.floor() as usize;
        let tint1 = b * (1.0 - s);
        let tint2 = b * (1.0 - s * (h - sector as Scalar));
        let tint3 = b * (1.0 - s * (1.0 + sector as Scalar - h));
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
pub(crate) fn hsl_to_rgb([h, s, l, a]: [Scalar; 4]) -> [Scalar; 4] {
    if s.abs() < Scalar::EPSILON {
        [l, l, l, a]
    } else {
        let h = h * 6.0;
        let b = if l < 0.5 {
            (1.0 + s) * l
        } else {
            l + s - l * s
        };
        let zest = 2.0 * l - b;
        let hzb_to_rgb = |mut h, z, b| -> Scalar {
            if h < 0.0 {
                h += 6.0;
            } else if h >= 6.0 {
                h -= 6.0;
            }
            match h {
                // Red to yellow (increasing green)
                _ if h < 1.0 => z + (b - z) * h,
                // Yellow to cyan (greatest green)
                _ if h < 3.0 => b,
                // Cyan to blue (decreasing green)
                _ if h < 4.0 => z + (b - z) * (4.0 - h),
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
pub(crate) fn hsl_to_hsb([h, s, l, a]: [Scalar; 4]) -> [Scalar; 4] {
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
pub(crate) fn hsb_to_hsl([h, s, b, a]: [Scalar; 4]) -> [Scalar; 4] {
    let l = (2.0 - s) * b / 2.0;
    let s = match l {
        _ if (l - 1.0).abs() < Scalar::EPSILON => 0.0,
        _ if l < 0.5 => s / 2.0 - s,
        _ => s * b / (2.0 - l * 2.0),
    };
    [h, s, l, a]
}

/// Converts levels to [u8] RGBA channels.
pub(crate) fn calculate_channels(levels: [Scalar; 4]) -> [u8; 4] {
    let [r, g, b, a] = levels;
    let [r_max, g_max, b_max, a_max] = maxes(Rgb);
    [
        (r * r_max).round() as u8,
        (g * g_max).round() as u8,
        (b * b_max).round() as u8,
        (a * a_max).round() as u8,
    ]
}

impl Color {
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
        A: Into<Scalar>,
    {
        let lerp = |start, stop, amt| amt * (stop - start) + start;

        let amt = amt.into().clamp(0.0, 1.0);
        let [v1, v2, v3, a] = self.levels();
        let [ov1, ov2, ov3, oa] = other.levels();
        let levels = clamp_levels([
            lerp(v1, ov1, amt),
            lerp(v2, ov2, amt),
            lerp(v3, ov3, amt),
            lerp(a, oa, amt),
        ]);
        let channels = calculate_channels(levels);
        Self {
            mode: self.mode,
            levels,
            channels,
        }
    }

    /// Update RGB channels by calculating them from the current levels.
    pub(crate) fn calculate_channels(&mut self) {
        self.channels = calculate_channels(self.levels);
    }
}

impl FromStr for Color {
    type Err = Error<'static, Scalar>;

    /// Converts to [Color] from a hexadecimal string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// use std::str::FromStr;
    ///
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
    /// # Ok::<(), ColorError<Scalar>>(())
    /// ```
    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        if !s.starts_with('#') {
            return Err(Error::InvalidString(Cow::from(s.to_owned())));
        }

        let mut channels: [u8; 4] = [0, 0, 0, 255];
        let parse_hex = |hex: &str| {
            if let Ok(value) = u8::from_str_radix(hex, 16) {
                Ok(value)
            } else {
                Err(Error::InvalidString(Cow::from(hex.to_owned())))
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
            _ => return Err(Error::InvalidString(Cow::from(s))),
        }

        Ok(Self::rgba(
            channels[0],
            channels[1],
            channels[2],
            channels[3],
        ))
    }
}

impl TryFrom<&str> for Color {
    type Error = Error<'static, Scalar>;
    /// Try to create a `Color` from a hexadecimal string.
    fn try_from(s: &str) -> result::Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

macro_rules! impl_from {
    ($($source: ty),*) => {
        $(
            impl From<$source> for Color {
                /// Convert from `value` to grayscale `Color`.
                fn from(gray: $source) -> Self {
                    let gray = Scalar::from(gray);
                    Self::with_mode(Rgb, gray, gray, gray)
                }
            }

            impl From<[$source; 1]> for Color {
                /// Convert from `[value]` to grayscale `Color`.
                fn from([gray]: [$source; 1]) -> Self {
                    let gray = Scalar::from(gray);
                    Self::with_mode(Rgb, gray, gray, gray)
                }
            }

            impl From<[$source; 2]> for Color {
                /// Convert from `[value, alpha]` to grayscale `Color` with alpha.
                fn from([gray, alpha]: [$source; 2]) -> Self {
                    let gray = Scalar::from(gray);
                    let alpha = Scalar::from(alpha);
                    Self::with_mode_alpha(Rgb, gray, gray, gray, alpha)
                }
            }

            impl From<[$source; 3]> for Color {
                /// Convert from `[r, g, b]` to `Color` with max alpha.
                fn from([r, g, b]: [$source; 3]) -> Self {
                    Self::with_mode(Rgb, Scalar::from(r), Scalar::from(g), Scalar::from(b))
                }
            }

            impl From<[$source; 4]> for Color {
                /// Convert from `[r, g, b, a]` to `Color`.
                fn from([r, g, b, a]: [$source; 4]) -> Self {
                    Self::with_mode_alpha(Rgb, Scalar::from(r), Scalar::from(g), Scalar::from(b), Scalar::from(a))
                }
            }
        )*
    };
}

impl_from!(u8, i8, u16, i16, u32, i32, f32, f64);

#[cfg(test)]
mod tests {
    use crate::prelude::{hsb, hsl, rgb, Color};

    macro_rules! assert_approx_eq {
        ($c1:expr, $c2:expr) => {
            let [v1, v2, v3, a] = $c1.levels();
            let [ov1, ov2, ov3, oa] = $c2.levels();
            let v1d = v1 - ov1;
            let v2d = v2 - ov2;
            let v3d = v3 - ov3;
            let ad = a - oa;
            let e = 0.002;
            assert!(v1d < e, "v1: ({} - {}) < {}", v1, ov1, e);
            assert!(v2d < e, "v2: ({} - {}) < {}", v2, ov2, e);
            assert!(v3d < e, "v3: ({} - {}) < {}", v3, ov3, e);
            assert!(ad < e, "a: ({} - {}) < {}", a, oa, e);
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
        assert_approx_eq!(hsb!(0.0, 0.0, 0.0), rgb!(0, 0, 0));
        assert_approx_eq!(hsb!(0.0, 0.0, 100.0), rgb!(255, 255, 255));
        assert_approx_eq!(hsb!(0.0, 100.0, 100.0), rgb!(255, 0, 0));
        assert_approx_eq!(hsb!(120.0, 100.0, 100.0), rgb!(0, 255, 0));
        assert_approx_eq!(hsb!(240.0, 100.0, 100.0), rgb!(0, 0, 255));
        assert_approx_eq!(hsb!(60.0, 100.0, 100.0), rgb!(255, 255, 0));
        assert_approx_eq!(hsb!(180.0, 100.0, 100.0), rgb!(0, 255, 255));
        assert_approx_eq!(hsb!(300.0, 100.0, 100.0), rgb!(255, 0, 255));
        assert_approx_eq!(hsb!(0.0, 0.0, 75.0), rgb!(191, 191, 191));
        assert_approx_eq!(hsb!(0.0, 0.0, 50.0), rgb!(128, 128, 128));
        assert_approx_eq!(hsb!(0.0, 100.0, 50.0), rgb!(128, 0, 0));
        assert_approx_eq!(hsb!(60.0, 100.0, 50.0), rgb!(128, 128, 0));
        assert_approx_eq!(hsb!(120.0, 100.0, 50.0), rgb!(0, 128, 0));
        assert_approx_eq!(hsb!(300.0, 100.0, 50.0), rgb!(128, 0, 128));
        assert_approx_eq!(hsb!(180.0, 100.0, 50.0), rgb!(0, 128, 128));
        assert_approx_eq!(hsb!(240.0, 100.0, 50.0), rgb!(0, 0, 128));
    }

    #[test]
    fn test_hsb_to_hsl() {
        assert_approx_eq!(hsb!(0.0, 0.0, 0.0), hsl!(0.0, 0.0, 0.0));
        assert_approx_eq!(hsb!(0.0, 0.0, 100.0), hsl!(0.0, 0.0, 100.0));
        assert_approx_eq!(hsb!(0.0, 100.0, 100.0), hsl!(0.0, 100.0, 50.0));
        assert_approx_eq!(hsb!(120.0, 100.0, 100.0), hsl!(120.0, 100.0, 50.0));
        assert_approx_eq!(hsb!(240.0, 100.0, 100.0), hsl!(240.0, 100.0, 50.0));
        assert_approx_eq!(hsb!(60.0, 100.0, 100.0), hsl!(60.0, 100.0, 50.0));
        assert_approx_eq!(hsb!(180.0, 100.0, 100.0), hsl!(180.0, 100.0, 50.0));
        assert_approx_eq!(hsb!(300.0, 100.0, 100.0), hsl!(300.0, 100.0, 50.0));
        assert_approx_eq!(hsb!(0.0, 0.0, 75.0), hsl!(0.0, 0.0, 75.0));
        assert_approx_eq!(hsb!(0.0, 0.0, 50.0), hsl!(0.0, 0.0, 50.0));
        assert_approx_eq!(hsb!(0.0, 100.0, 50.0), hsl!(0.0, 100.0, 25.0));
        assert_approx_eq!(hsb!(60.0, 100.0, 50.0), hsl!(60.0, 100.0, 25.0));
        assert_approx_eq!(hsb!(120.0, 100.0, 50.0), hsl!(120.0, 100.0, 25.0));
        assert_approx_eq!(hsb!(300.0, 100.0, 50.0), hsl!(300.0, 100.0, 25.0));
        assert_approx_eq!(hsb!(180.0, 100.0, 50.0), hsl!(180.0, 100.0, 25.0));
        assert_approx_eq!(hsb!(240.0, 100.0, 50.0), hsl!(240.0, 100.0, 25.0));
    }

    #[test]
    fn test_hsl_to_rgb() {
        assert_approx_eq!(hsl!(0.0, 0.0, 0.0), rgb!(0, 0, 0));
        assert_approx_eq!(hsl!(0.0, 0.0, 100.0), rgb!(255, 255, 255));
        assert_approx_eq!(hsl!(0.0, 100.0, 100.0), rgb!(255, 255, 255));
        assert_approx_eq!(hsl!(120.0, 100.0, 100.0), rgb!(255, 255, 255));
        assert_approx_eq!(hsl!(240.0, 100.0, 100.0), rgb!(255, 255, 255));
        assert_approx_eq!(hsl!(60.0, 100.0, 100.0), rgb!(255, 255, 255));
        assert_approx_eq!(hsl!(180.0, 100.0, 100.0), rgb!(255, 255, 255));
        assert_approx_eq!(hsl!(300.0, 100.0, 100.0), rgb!(255, 255, 255));
        assert_approx_eq!(hsl!(0.0, 0.0, 75.0), rgb!(191, 191, 191));
        assert_approx_eq!(hsl!(0.0, 0.0, 50.0), rgb!(128, 128, 128));
        assert_approx_eq!(hsl!(0.0, 100.0, 50.0), rgb!(255, 0, 0));
        assert_approx_eq!(hsl!(60.0, 100.0, 50.0), rgb!(255, 255, 0));
        assert_approx_eq!(hsl!(120.0, 100.0, 50.0), rgb!(0, 255, 0));
        assert_approx_eq!(hsl!(300.0, 100.0, 50.0), rgb!(255, 0, 255));
        assert_approx_eq!(hsl!(180.0, 100.0, 50.0), rgb!(0, 255, 255));
        assert_approx_eq!(hsl!(240.0, 100.0, 50.0), rgb!(0, 0, 255));
    }

    #[test]
    fn test_hsl_to_hsb() {
        assert_approx_eq!(hsl!(0.0, 0.0, 0.0), hsb!(0.0, 0.0, 0.0));
        assert_approx_eq!(hsl!(0.0, 0.0, 100.0), hsb!(0.0, 0.0, 100.0));
        assert_approx_eq!(hsl!(0.0, 100.0, 100.0), hsb!(0.0, 0.0, 100.0));
        assert_approx_eq!(hsl!(120.0, 100.0, 100.0), hsb!(120.0, 0.0, 100.0));
        assert_approx_eq!(hsl!(240.0, 100.0, 100.0), hsb!(240.0, 0.0, 100.0));
        assert_approx_eq!(hsl!(60.0, 100.0, 100.0), hsb!(60.0, 0.0, 100.0));
        assert_approx_eq!(hsl!(180.0, 100.0, 100.0), hsb!(180.0, 0.0, 100.0));
        assert_approx_eq!(hsl!(300.0, 100.0, 100.0), hsb!(300.0, 0.0, 100.0));
        assert_approx_eq!(hsl!(0.0, 0.0, 75.0), hsb!(0.0, 0.0, 75.0));
        assert_approx_eq!(hsl!(0.0, 0.0, 50.0), hsb!(0.0, 0.0, 50.0));
        assert_approx_eq!(hsl!(0.0, 100.0, 50.0), hsb!(0.0, 100.0, 100.0));
        assert_approx_eq!(hsl!(60.0, 100.0, 50.0), hsb!(60.0, 100.0, 100.0));
        assert_approx_eq!(hsl!(120.0, 100.0, 50.0), hsb!(120.0, 100.0, 100.0));
        assert_approx_eq!(hsl!(300.0, 100.0, 50.0), hsb!(300.0, 100.0, 100.0));
        assert_approx_eq!(hsl!(180.0, 100.0, 50.0), hsb!(180.0, 100.0, 100.0));
        assert_approx_eq!(hsl!(240.0, 100.0, 50.0), hsb!(240.0, 100.0, 100.0));
    }

    #[test]
    fn test_rgb_to_hsb() {
        assert_approx_eq!(rgb!(0, 0, 0), hsb!(0.0, 0.0, 0.0));
        assert_approx_eq!(rgb!(255, 255, 255), hsb!(0.0, 0.0, 100.0));
        assert_approx_eq!(rgb!(255, 0, 0), hsb!(0.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(0, 255, 0), hsb!(120.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(0, 0, 255), hsb!(240.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(255, 255, 0), hsb!(60.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(0, 255, 255), hsb!(180.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(255, 0, 255), hsb!(300.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(191, 191, 191), hsb!(0.0, 0.0, 74.9));
        assert_approx_eq!(rgb!(128, 128, 128), hsb!(0.0, 0.0, 50.0));
        assert_approx_eq!(rgb!(128, 0, 0), hsb!(0.0, 100.0, 50.2));
        assert_approx_eq!(rgb!(128, 128, 0), hsb!(60.0, 100.0, 50.0));
        assert_approx_eq!(rgb!(0, 128, 0), hsb!(120.0, 100.0, 50.0));
        assert_approx_eq!(rgb!(128, 0, 128), hsb!(300.0, 100.0, 50.0));
        assert_approx_eq!(rgb!(0, 128, 128), hsb!(180.0, 100.0, 50.0));
        assert_approx_eq!(rgb!(0, 0, 128), hsb!(240.0, 100.0, 50.0));
    }

    #[test]
    fn test_rgb_to_hsl() {
        assert_approx_eq!(rgb!(0, 0, 0), hsl!(0.0, 0.0, 0.0));
        assert_approx_eq!(rgb!(255, 255, 255), hsl!(0.0, 0.0, 100.0));
        assert_approx_eq!(rgb!(255, 0, 0), hsl!(0.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(0, 255, 0), hsl!(120.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(0, 0, 255), hsl!(240.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(255, 255, 0), hsl!(60.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(0, 255, 255), hsl!(180.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(255, 0, 255), hsl!(300.0, 100.0, 100.0));
        assert_approx_eq!(rgb!(191, 191, 191), hsl!(0.0, 0.0, 74.9));
        assert_approx_eq!(rgb!(128, 128, 128), hsl!(0.0, 0.0, 50.0));
        assert_approx_eq!(rgb!(128, 0, 0), hsl!(0.0, 100.0, 50.2));
        assert_approx_eq!(rgb!(128, 128, 0), hsl!(60.0, 100.0, 50.0));
        assert_approx_eq!(rgb!(0, 128, 0), hsl!(120.0, 100.0, 50.0));
        assert_approx_eq!(rgb!(128, 0, 128), hsl!(300.0, 100.0, 50.0));
        assert_approx_eq!(rgb!(0, 128, 128), hsl!(180.0, 100.0, 50.0));
        assert_approx_eq!(rgb!(0, 0, 128), hsl!(240.0, 100.0, 50.0));
    }
}
