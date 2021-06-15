//! `Rgb` [Color] format.

use super::{hsv::Hsv, ColorError};
use crate::random;
use std::{
    borrow::Cow,
    convert::TryFrom,
    fmt::{self, LowerHex, UpperHex},
    ops::*,
    str::FromStr,
};

/// An `Rgb` value containing red, green, blue, and alpha channels.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rgb {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) a: u8,
}

impl Rgb {
    /// Create a new `Rgb` instance, defaulting to black.
    pub const fn new() -> Self {
        Self::rgb(0, 0, 0)
    }

    /// Create a new `Rgb` instance.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    /// Create a new `Rgb` instance with alpha.
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create a new `Rgb` instance with random red, green, and blue with alpha of 255.
    pub fn random() -> Self {
        Self::rgb(random!(255), random!(255), random!(255))
    }

    /// Create a new `Rgb` instance with random red, green, blue and alpha.
    pub fn random_alpha() -> Self {
        Self::rgba(random!(255), random!(255), random!(255), random!(255))
    }

    /// Get the red, green, blue, and alpha channels as a tuple u8 values.
    pub fn channels(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    /// Convert to [Hsv] format.
    pub fn to_hsv(self) -> Hsv {
        let r1 = f32::from(self.r) / 255.0;
        let g1 = f32::from(self.g) / 255.0;
        let b1 = f32::from(self.b) / 255.0;
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
            Hsv::hsv(h, s, c_max)
        } else {
            Hsv::hsv(0.0, 0.0, c_max)
        }
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
            return Err(ColorError::InvalidString(Cow::from(s.to_owned())));
        }

        let mut channels: [u8; 4] = [0, 0, 0, 255];
        let parse_hex = |hex: &str| {
            if let Ok(value) = u8::from_str_radix(hex, 16) {
                Ok(value)
            } else {
                Err(ColorError::InvalidString(Cow::from(hex.to_owned())))
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
            _ => return Err(ColorError::InvalidString(Cow::from(s))),
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

impl From<u8> for Rgb {
    fn from(gray: u8) -> Self {
        Self::rgb(gray, gray, gray)
    }
}

impl From<(u8, u8)> for Rgb {
    fn from((gray, alpha): (u8, u8)) -> Self {
        Self::rgba(gray, gray, gray, alpha)
    }
}

impl From<(u8, u8, u8)> for Rgb {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::rgb(r, g, b)
    }
}

impl From<(u8, u8, u8, u8)> for Rgb {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self::rgba(r, g, b, a)
    }
}

impl From<Rgb> for (u8, u8, u8, u8) {
    fn from(rgb: Rgb) -> Self {
        rgb.channels()
    }
}
