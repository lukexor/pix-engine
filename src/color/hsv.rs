//! `Hsv` [Color] format.

use super::rgb::Rgb;
use std::ops::*;

/// A `Hsv` value containing hue, saturation, value, and alpha channels.
#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Hsv {
    pub(crate) h: f32,
    pub(crate) s: f32,
    pub(crate) v: f32,
    pub(crate) a: f32,
}

impl Hsv {
    /// Create a new `Hsv` instance.
    pub fn hsv(h: f32, s: f32, v: f32) -> Self {
        Self::hsva(h, s, v, 1.0)
    }

    /// Create a new `Hsv` instance with alpha.
    pub fn hsva(h: f32, s: f32, v: f32, a: f32) -> Self {
        Self {
            h: h.clamp(0.0, 360.0),
            s: s.clamp(0.0, 1.0),
            v: v.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        }
    }

    /// Get the hue, saturation, value, and alpha channels as a tuple f32 values.
    pub fn channels(self) -> (f32, f32, f32, f32) {
        (self.h, self.s, self.v, self.a)
    }

    /// Convert to [Rgb] format.
    pub fn to_rgb(self) -> Rgb {
        if self.v == 0.0 {
            Rgb::rgb(0, 0, 0)
        } else if self.s == 0.0 {
            let gray = (self.v * 255.0).round() as u8;
            Rgb::rgb(gray, gray, gray)
        } else {
            let chroma = self.v * self.s;
            let hue_six = self.h / 60.0;
            let value = chroma * (1.0 - (hue_six % 2.0 - 1.0).abs());
            let (r1, g1, b1) = match hue_six.floor() as usize {
                0 | 6 => (chroma, value, 0.0),
                1 => (value, chroma, 0.0),
                2 => (0.0, chroma, value),
                3 => (0.0, value, chroma),
                4 => (value, 0.0, chroma),
                5 => (chroma, 0.0, value),
                _ => unreachable!(),
            };
            let add = self.v - chroma;
            let r = ((r1 + add) * 255.0).round() as u8;
            let g = ((g1 + add) * 255.0).round() as u8;
            let b = ((b1 + add) * 255.0).round() as u8;
            let a = (self.a * 255.0).round() as u8;
            Rgb::rgba(r, g, b, a)
        }
    }
}

impl Index<usize> for Hsv {
    type Output = f32;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.h,
            1 => &self.s,
            2 => &self.v,
            3 => &self.a,
            _ => panic!("index out of bounds: the len is 4 but the index is {}", idx),
        }
    }
}

impl IndexMut<usize> for Hsv {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.h,
            1 => &mut self.s,
            2 => &mut self.v,
            3 => &mut self.a,
            _ => panic!("index out of bounds: the len is 4 but the index is {}", idx),
        }
    }
}

impl From<f32> for Hsv {
    fn from(gray: f32) -> Self {
        Self::hsv(0.0, 0.0, gray)
    }
}

impl From<(f32, f32)> for Hsv {
    fn from((gray, alpha): (f32, f32)) -> Self {
        Self::hsva(0.0, 0.0, gray, alpha)
    }
}

impl From<(f32, f32, f32)> for Hsv {
    fn from((h, s, v): (f32, f32, f32)) -> Self {
        Self::hsv(h, s, v)
    }
}

impl From<(f32, f32, f32, f32)> for Hsv {
    fn from((h, s, v, a): (f32, f32, f32, f32)) -> Self {
        Self::hsva(h, s, v, a)
    }
}

impl From<Hsv> for (f32, f32, f32, f32) {
    fn from(hsv: Hsv) -> Self {
        hsv.channels()
    }
}
