use super::{constants::NAMED_COLORS, Color, ColorMaxes, ColorMode};
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::{Deref, DerefMut};

impl Color {
    /// Returns a representation of this color as a Vec of u16 values based on the current
    /// `State::color_mode`.
    ///
    /// - RGB: (red, green, blue, alpha)
    /// - HSB/HSL: (hue, saturation, brightness/lightness, alpha)
    ///
    /// # Examples
    ///
    /// ```
    /// use pix_engine::prelude::*;
    /// # let mut state = StateData::new("State", 100, 100).unwrap();
    ///
    /// let mut c1 = state.color([128, 0, 128]);
    /// assert_eq!(c1.into_vec(), vec![128, 0, 128, 255]);
    ///
    /// let mut c2 = state.color([128, 0, 128, 64]);
    /// assert_eq!(c2.into_vec(), vec![128, 0, 128, 64]);
    /// ```
    pub fn into_vec(self) -> Vec<u16> {
        vec![
            self.red() as u16,
            self.green() as u16,
            self.blue() as u16,
            self.alpha() as u16,
        ]
    }

    #[allow(dead_code)]
    pub(super) fn hsba_to_hsla(values: [f64; 4]) -> [f64; 4] {
        let hue = values[0];
        let mut sat = values[1];
        let br = values[2];
        let alpha = values[3];

        // Calc lightness
        let li = (2.0 - sat) * br / 2.0;

        // Convert saturation
        if li != 0.0 {
            if (li - 1.0).abs() < f64::EPSILON {
                sat = 0.0;
            } else if li < 0.5 {
                sat = sat / (2.0 - sat);
            } else {
                sat = sat * br / (2.0 - li * 2.0);
            }
        }

        // Hue/alpha remain the same
        [hue, sat, li, alpha]
    }

    pub(super) fn hsba_to_rgba(values: [f64; 4]) -> [f64; 4] {
        let hue = values[0] * 6.0; // Split hue into 6 sectors
        let sat = values[1];
        let br = values[2];
        let alpha = values[3];

        if sat == 0.0 {
            // Grayscale
            [br, br, br, alpha]
        } else {
            let sector = hue.floor();
            let tint1 = br * (1.0 - sat);
            let tint2 = br * (1.0 - sat * (hue - sector));
            let tint3 = br * (1.0 - sat * (1.0 + sector - hue));
            let (r, g, b) = match sector as u8 {
                1 => (tint2, br, tint1), // Yellow to green
                2 => (tint1, br, tint3), // Green to cyan
                3 => (tint1, tint2, br), // Cyan to blue
                4 => (tint3, tint1, br), // Blue to magenta
                5 => (br, tint1, tint2), // Magenta to red
                _ => (br, tint3, tint1), // Red to yellow
            };
            [r, g, b, alpha]
        }
    }

    #[allow(dead_code)]
    pub(super) fn hsla_to_hsba(values: [f64; 4]) -> [f64; 4] {
        let hue = values[0];
        let mut sat = values[1];
        let li = values[2];
        let alpha = values[3];

        // Calc brightness
        let br = if li < 0.5 {
            (1.0 + sat) * li
        } else {
            li + sat - li * sat
        };

        // Convert saturation
        sat = 2.0 * (br - li) / br;

        // Hue/alpha remain the same
        [hue, sat, br, alpha]
    }

    pub(super) fn hsla_to_rgba(values: [f64; 4]) -> [f64; 4] {
        let hue = values[0] * 6.0; // Split hue into 6 sectors
        let sat = values[1];
        let li = values[2];
        let alpha = values[3];

        if sat == 0.0 {
            // Grayscale
            [li, li, li, alpha]
        } else {
            // Calc brightness
            let br = if li < 0.5 {
                (1.0 + sat) * li
            } else {
                li + sat - li * sat
            };

            // Define zest
            let zest = 2.0 * li - br;

            // Projection (onto green by default)
            let hzb_to_rgb = |mut hue, zest, br| {
                // Hue must wrap to allow projection onto red/blue
                if hue < 0.0 {
                    hue += 6.0;
                } else if hue >= 6.0 {
                    hue -= 6.0;
                }
                if hue < 1.0 {
                    // Red to yellow (increasing green)
                    zest + (br - zest) * hue
                } else if hue < 3.0 {
                    // Yellow to cyan (greatest green)
                    br
                } else if hue < 4.0 {
                    // Cyan to blue (decreasing green)
                    zest + (br - zest) * (4.0 - hue)
                } else {
                    // Blue to red (least green)
                    zest
                }
            };

            [
                hzb_to_rgb(hue + 2.0, zest, br),
                hzb_to_rgb(hue, zest, br),
                hzb_to_rgb(hue - 2.0, zest, br),
                alpha,
            ]
        }
    }

    pub(super) fn rgba_to_hsba(values: [f64; 4]) -> [f64; 4] {
        let red = values[0];
        let green = values[1];
        let blue = values[2];
        let alpha = values[3];

        let br = red.max(green).max(blue);
        let chroma = br - red.min(green).min(blue);

        let (hue, sat) = if chroma == 0.0 {
            // Grayscale
            (0.0, 0.0)
        } else {
            let sat = chroma / br;
            let mut hue = if (red - br).abs() < f64::EPSILON {
                // Magenta to yellow
                (green - blue) / chroma
            } else if (green - br).abs() < f64::EPSILON {
                // Yellow to cyan
                2.0 + (blue - red) / chroma
            } else if (blue - br).abs() < f64::EPSILON {
                // Cyan to magenta
                4.0 + (red - green) / chroma
            } else {
                0.0
            };
            // Constraint to [0, 1)
            if hue < 0.0 {
                hue += 6.0;
            } else if hue >= 6.0 {
                hue -= 6.0;
            }
            (hue, sat)
        };

        [hue / 6.0, sat, br, alpha]
    }

    pub(super) fn rgba_to_hsla(values: [f64; 4]) -> [f64; 4] {
        let red = values[0];
        let green = values[1];
        let blue = values[2];
        let alpha = values[3];

        let val = red.max(green).max(blue);
        let min = red.min(green).min(blue);
        let li = val + min; // Gets halved later
        let chroma = val - min;

        let (hue, sat) = if chroma == 0.0 {
            // Grayscale
            (0.0, 0.0)
        } else {
            let sat = if li < 1.0 {
                chroma / li
            } else {
                chroma / (2.0 - li)
            };
            let mut hue = if (red - val).abs() < f64::EPSILON {
                // Magenta to yellow
                (green - blue) / chroma
            } else if (green - val).abs() < f64::EPSILON {
                // Yellow to cyan
                2.0 + (blue - red) / chroma
            } else if (blue - val).abs() < f64::EPSILON {
                // Cyan to magenta
                4.0 + (red - green) / chroma
            } else {
                0.0
            };
            // Constraint to [0, 1)
            if hue < 0.0 {
                hue += 6.0;
            } else if hue >= 6.0 {
                hue -= 6.0;
            }
            (hue, sat)
        };

        [hue / 6.0, sat, li / 2.0, alpha]
    }
}

impl Deref for Color {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { ::std::slice::from_raw_parts(self as *const Self as *const u8, 4) }
    }
}
impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { ::std::slice::from_raw_parts_mut(self as *mut Self as *mut u8, 4) }
    }
}

// Represents 3 color channels and an optional alpha channel.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ColorLevels([f64; 4]);

impl Deref for ColorLevels {
    type Target = [f64];
    fn deref(&self) -> &[f64] {
        &self.0
    }
}
impl DerefMut for ColorLevels {
    fn deref_mut(&mut self) -> &mut [f64] {
        &mut self.0
    }
}

macro_rules! impl_color_level_from {
    ($T:ty) => {
        impl From<$T> for ColorLevels {
            #[inline]
            fn from(level: $T) -> Self {
                let level = level as f64;
                ColorLevels([level, level, level, 255.0])
            }
        }
        impl From<[$T; 1]> for ColorLevels {
            #[inline]
            fn from(arr: [$T; 1]) -> Self {
                let level = arr[0] as f64;
                ColorLevels([level, level, level, 255.0])
            }
        }
        impl From<[$T; 2]> for ColorLevels {
            #[inline]
            fn from(arr: [$T; 2]) -> Self {
                let level = arr[0] as f64;
                let alpha = arr[1] as f64;
                ColorLevels([level, level, level, alpha])
            }
        }
        impl From<[$T; 3]> for ColorLevels {
            #[inline]
            fn from(arr: [$T; 3]) -> Self {
                let l0 = arr[0] as f64;
                let l1 = arr[1] as f64;
                let l2 = arr[2] as f64;
                ColorLevels([l0, l1, l2, 255.0])
            }
        }
        impl From<[$T; 4]> for ColorLevels {
            #[inline]
            fn from(arr: [$T; 4]) -> Self {
                let l0 = arr[0] as f64;
                let l1 = arr[1] as f64;
                let l2 = arr[2] as f64;
                let alpha = arr[3] as f64;
                ColorLevels([l0, l1, l2, alpha])
            }
        }
        impl From<&[$T]> for ColorLevels {
            #[inline]
            fn from(slice: &[$T]) -> Self {
                match *slice {
                    [gray, a] => [gray, a].into(),
                    [l0, l1, l2] => [l0, l1, l2].into(),
                    [l0, l1, l2, a] => [l0, l1, l2, a].into(),
                    _ => panic!("invalid color slice"),
                }
            }
        }
        impl From<&Vec<$T>> for ColorLevels {
            #[inline]
            fn from(vector: &Vec<$T>) -> Self {
                vector.as_slice().into()
            }
        }
    };
}

impl_color_level_from!(isize);
impl_color_level_from!(i8);
impl_color_level_from!(i16);
impl_color_level_from!(i32);
impl_color_level_from!(i64);
impl_color_level_from!(usize);
impl_color_level_from!(u8);
impl_color_level_from!(u16);
impl_color_level_from!(u32);
impl_color_level_from!(u64);
impl_color_level_from!(f32);
impl_color_level_from!(f64);

lazy_static! {
    /// Match 3 digit hexadecimal format (rgb). e.g. #416.
    static ref HEX3: Regex = Regex::new(r"^#([a-f0-9])([a-f0-9])([a-f0-9])$").unwrap();

    /// Match 4 digit hexadecimal format (rgba). e.g. #4163.
    static ref HEX4: Regex = Regex::new(r"^#([a-f0-9])([a-f0-9])([a-f0-9])([a-f0-9])$").unwrap();

    /// Match 6 digit hexadecimal format (rrggbb). e.g. #b4d455.
    static ref HEX6: Regex = Regex::new(r"#([a-f0-9]{2})([a-f0-9]{2})([a-f0-9]{2})$").unwrap();

    /// Match 8 digit hexadecimal format (rrggbbaa). e.g. #b4d45535.
    static ref HEX8: Regex = Regex::new(r"^#([a-f0-9]{2})([a-f0-9]{2})([a-f0-9]{2})([a-f0-9]{2})$").unwrap();
}

impl From<&str> for ColorLevels {
    #[allow(clippy::needless_range_loop)]
    fn from(string: &str) -> Self {
        let mut s = string.trim().to_lowercase();
        if let Some(color) = NAMED_COLORS.get(&s.as_str()) {
            s = (*color).to_string();
        }

        let parse_hex = |hex: &str| u8::from_str_radix(hex, 16).unwrap();

        let mut levels = [0f64; 4];
        if HEX3.is_match(&s) {
            for (i, cap) in HEX3.captures_iter(&s).enumerate() {
                eprintln!("{:?}", cap);
                for i in 0..3 {
                    let c = cap.get(i + 1).unwrap();
                    let color = format!("{}{}", c.as_str(), c.as_str());
                    levels[i] = parse_hex(&color) as f64;
                }
            }
            levels[3] = 255.0;
        } else if HEX6.is_match(&s) {
            for (i, cap) in HEX6.captures_iter(&s).enumerate() {
                for i in 0..3 {
                    let c = cap.get(i + 1).unwrap();
                    levels[i] = parse_hex(c.as_str()) as f64;
                }
            }
            levels[3] = 255.0;
        } else if HEX4.is_match(&s) {
            for (i, cap) in HEX4.captures_iter(&s).enumerate() {
                for i in 0..4 {
                    let c = cap.get(i + 1).unwrap();
                    let color = parse_hex(c.as_str());
                    levels[i] = (color + color) as f64;
                    let color = format!("{}{}", c.as_str(), c.as_str());
                    levels[i] = parse_hex(&color) as f64;
                }
            }
        } else if HEX8.is_match(&s) {
            for (i, cap) in HEX8.captures_iter(&s).enumerate() {
                for i in 0..4 {
                    let c = cap.get(i + 1).unwrap();
                    levels[i] = parse_hex(c.as_str()) as f64;
                }
            }
        }
        Self(levels)
    }
}

macro_rules! impl_color_from {
    ($T:ty) => {
        impl From<$T> for Color {
            #[inline]
            fn from(level: $T) -> Self {
                Color::from_levels(level, ColorMode::default(), ColorMaxes::default())
            }
        }
        impl From<[$T; 1]> for Color {
            #[inline]
            fn from(levels: [$T; 1]) -> Self {
                Color::from_levels(levels, ColorMode::default(), ColorMaxes::default())
            }
        }
        impl From<[$T; 2]> for Color {
            #[inline]
            fn from(levels: [$T; 2]) -> Self {
                Color::from_levels(levels, ColorMode::default(), ColorMaxes::default())
            }
        }
        impl From<[$T; 3]> for Color {
            #[inline]
            fn from(levels: [$T; 3]) -> Self {
                Color::from_levels(levels, ColorMode::default(), ColorMaxes::default())
            }
        }
        impl From<[$T; 4]> for Color {
            #[inline]
            fn from(levels: [$T; 4]) -> Self {
                Color::from_levels(levels, ColorMode::default(), ColorMaxes::default())
            }
        }
        impl From<&[$T]> for Color {
            #[inline]
            fn from(levels: &[$T]) -> Self {
                Color::from_levels(levels, ColorMode::default(), ColorMaxes::default())
            }
        }
        impl From<&Vec<$T>> for Color {
            #[inline]
            fn from(vector: &Vec<$T>) -> Self {
                Color::from_levels(
                    vector.as_slice(),
                    ColorMode::default(),
                    ColorMaxes::default(),
                )
            }
        }
    };
}

impl_color_from!(isize);
impl_color_from!(i8);
impl_color_from!(i16);
impl_color_from!(i32);
impl_color_from!(i64);
impl_color_from!(usize);
impl_color_from!(u8);
impl_color_from!(u16);
impl_color_from!(u32);
impl_color_from!(u64);
impl_color_from!(f32);
impl_color_from!(f64);

impl From<&str> for Color {
    fn from(string: &str) -> Self {
        Color::from_levels(string, ColorMode::default(), ColorMaxes::default())
    }
}
