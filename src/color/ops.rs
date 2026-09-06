//! [Color] operation functions.
//!
//! Provides numeric operations and trait implementations:
//!
//! - [`LowerHex`]: Allows displaying as lowercase hexadecimal value.
//! - [`UpperHex`]: Allows displaying as uppercase hexadecimal value.
//! - [`Index`]: Allows indexing to retrieve RGBA values. (e.g. `color[0]` for the red
//!   channel).
//! - [`PartialEq`] and [Eq]: Allows comparison.
//! - [`Hash`]: Allows hashing.
//!
//! Also implemented are [`Add`], [`Sub`], [`AddAssign`], and [`SubAssign`] with other `Color`s and u8
//! values channel-wise. [`Deref`] is also implemented which returns `[u8; 4]`.

use super::{
    conversion::{calculate_channels, clamp_levels, convert_levels, to_channel},
    Color,
    Mode::Rgb,
};
use std::{
    fmt::{self, LowerHex, UpperHex},
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Deref, Div, DivAssign, Index, Mul, MulAssign, Sub, SubAssign},
};

impl LowerHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [red, green, blue, alpha] = self.channels();
        write!(f, "#{red:x}{green:x}{blue:x}{alpha:x}")
    }
}

impl UpperHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [red, green, blue, alpha] = self.channels();
        write!(f, "#{red:X}{green:X}{blue:X}{alpha:X}")
    }
}

impl Index<usize> for Color {
    type Output = u8;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.channels[idx]
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.channels.eq(&other.channels)
    }
}

impl Eq for Color {}

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.channels.hash(state);
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Color) -> Self::Output {
        if self.mode == other.mode {
            let [r, g, b, a] = self.channels();
            let [or, og, ob, _] = other.channels();
            Self {
                mode: self.mode,
                channels: [
                    r.saturating_add(or),
                    g.saturating_add(og),
                    b.saturating_add(ob),
                    a,
                ],
            }
        } else {
            let [v1, v2, v3, a] = self.levels();
            let [ov1, ov2, ov3, _] = convert_levels(other.levels(), other.mode, self.mode);
            let levels = clamp_levels([v1 + ov1, v2 + ov2, v3 + ov3, a]);
            Self {
                mode: self.mode,
                channels: calculate_channels(levels),
            }
        }
    }
}

impl Add<u8> for Color {
    type Output = Self;
    fn add(self, val: u8) -> Self::Output {
        let [r, g, b, _] = self.channels;
        Self::rgb(
            r.saturating_add(val),
            g.saturating_add(val),
            b.saturating_add(val),
        )
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        if self.mode == other.mode {
            for (i, v) in self.channels.iter_mut().enumerate().take(3) {
                *v = v.saturating_add(other[i]);
            }
        } else {
            let [v1, v2, v3, a] = self.levels();
            let [ov1, ov2, ov3, _] = convert_levels(other.levels(), other.mode, self.mode);
            let levels = clamp_levels([v1 + ov1, v2 + ov2, v3 + ov3, a]);
            self.update_channels(levels, self.mode);
        }
    }
}

impl AddAssign<u8> for Color {
    fn add_assign(&mut self, val: u8) {
        for v in &mut self.channels {
            *v = v.saturating_add(val);
        }
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, other: Color) -> Self::Output {
        if self.mode == other.mode {
            let [r, g, b, a] = self.channels();
            let [or, og, ob, _] = other.channels();
            Self {
                mode: self.mode,
                channels: [
                    r.saturating_sub(or),
                    g.saturating_sub(og),
                    b.saturating_sub(ob),
                    a,
                ],
            }
        } else {
            let [v1, v2, v3, a] = self.levels();
            let [ov1, ov2, ov3, _] = convert_levels(other.levels(), other.mode, self.mode);
            let levels = clamp_levels([v1 - ov1, v2 - ov2, v3 - ov3, a]);
            Self {
                mode: self.mode,
                channels: calculate_channels(levels),
            }
        }
    }
}

impl Sub<u8> for Color {
    type Output = Self;
    fn sub(self, val: u8) -> Self::Output {
        let [r, g, b, a] = self.channels;
        Self::rgba(
            r.saturating_sub(val),
            g.saturating_sub(val),
            b.saturating_sub(val),
            a,
        )
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, other: Color) {
        if self.mode == other.mode {
            for (i, v) in self.channels.iter_mut().enumerate().take(3) {
                *v = v.saturating_sub(other[i]);
            }
        } else {
            let [v1, v2, v3, a] = self.levels();
            let [ov1, ov2, ov3, _] = convert_levels(other.levels(), other.mode, self.mode);
            let levels = clamp_levels([v1 - ov1, v2 - ov2, v3 - ov3, a]);
            self.update_channels(levels, self.mode);
        }
    }
}

impl SubAssign<u8> for Color {
    fn sub_assign(&mut self, val: u8) {
        for v in self.channels.iter_mut().take(3) {
            *v = v.saturating_sub(val);
        }
    }
}

impl Deref for Color {
    type Target = [u8; 4];
    /// Deref `Color` to `&[u8; 4]`.
    fn deref(&self) -> &Self::Target {
        &self.channels
    }
}

/// Applies `f` to each color level, leaving alpha alone, and returns the resulting channels.
#[inline]
fn scaled(color: &Color, f: impl Fn(f64) -> f64) -> [u8; 4] {
    // `Color::levels` divides every channel by its maximum and converts the result into the
    // current mode, and `calculate_channels` reverses both. Under `Rgb` the conversion is the
    // identity and the normalizing clamp cannot fire, so the channels are scaled where they are
    // and alpha is copied. This runs once per channel per pixel in `3d_raytracing`, where the
    // round trip measured a tenth of a frame.
    if color.mode == Rgb {
        let [r, g, b, a] = color.channels;
        let [r, g, b] = scaled_channels([r, g, b], f);
        [r, g, b, a]
    } else {
        let [v1, v2, v3, a] = color.levels();
        calculate_channels(clamp_levels([f(v1), f(v2), f(v3), a]))
    }
}

/// Applies `f` to each color level in place, leaving alpha alone.
#[inline]
fn scale_in_place(color: &mut Color, f: impl Fn(f64) -> f64) {
    // The assigning operators convert the scaled levels back out of the current mode where
    // `scaled` does not, so the two agree only under `Rgb`.
    if color.mode == Rgb {
        let [r, g, b, _] = color.channels;
        let [r, g, b] = scaled_channels([r, g, b], f);
        color.channels[0] = r;
        color.channels[1] = g;
        color.channels[2] = b;
    } else {
        let [v1, v2, v3, a] = color.levels();
        let levels = clamp_levels([f(v1), f(v2), f(v3), a]);
        color.update_channels(levels, color.mode);
    }
}

/// Normalizes three [`Rgb`] bytes, applies `f`, and converts them back.
#[inline]
fn scaled_channels(channels: [u8; 3], f: impl Fn(f64) -> f64) -> [u8; 3] {
    // The operation order matches `clamp_levels` followed by `calculate_channels`, so these are
    // the bytes the level round trip produces. The three stay in one array expression so the
    // compiler vectorizes the arithmetic.
    let [r, g, b] = channels;
    let level = |c: u8| f(f64::from(c) / 255.0).clamp(0.0, 1.0);
    let levels = [level(r), level(g), level(b)];
    [
        to_channel(levels[0], 255.0),
        to_channel(levels[1], 255.0),
        to_channel(levels[2], 255.0),
    ]
}

macro_rules! impl_ops {
    ($($target:ty),*) => {
        $(
            impl Mul<$target> for Color where $target: Into<f64> {
                type Output = Self;
                fn mul(self, s: $target) -> Self::Output {
                    let s = f64::from(s);
                    Self {
                        mode: self.mode,
                        channels: scaled(&self, |v| v * s),
                    }
                }
            }

            impl Mul<Color> for $target where $target: Into<f64> {
                type Output = Color;
                fn mul(self, c: Color) -> Self::Output {
                    let s = f64::from(self);
                    Color {
                        mode: c.mode,
                        channels: scaled(&c, |v| v * s),
                    }
                }
            }

            impl MulAssign<$target> for Color where $target: Into<f64> {
                fn mul_assign(&mut self, s: $target) {
                    let s = f64::from(s);
                    scale_in_place(self, |v| v * s);
                }
            }

            impl Div<$target> for Color where $target: Into<f64> {
                type Output = Self;
                fn div(self, s: $target) -> Self::Output {
                    let s = f64::from(s);
                    Self {
                        mode: self.mode,
                        channels: scaled(&self, |v| v / s),
                    }
                }
            }

            impl DivAssign<$target> for Color where $target: Into<f64> {
                fn div_assign(&mut self, s: $target) {
                    let s = f64::from(s);
                    scale_in_place(self, |v| v / s);
                }
            }
        )*
    };
}

macro_rules! impl_as_ops {
    ($($target:ty),*) => {
        $(
            impl Mul<$target> for Color {
                type Output = Self;
                fn mul(self, s: $target) -> Self::Output {
                    let s = s as f64;
                    Self {
                        mode: self.mode,
                        channels: scaled(&self, |v| v * s),
                    }
                }
            }

            impl Mul<Color> for $target {
                type Output = Color;
                fn mul(self, c: Color) -> Self::Output {
                    let s = self as f64;
                    Color {
                        mode: c.mode,
                        channels: scaled(&c, |v| v * s),
                    }
                }
            }

            impl MulAssign<$target> for Color {
                fn mul_assign(&mut self, s: $target) {
                    let s = s as f64;
                    scale_in_place(self, |v| v * s);
                }
            }

            impl Div<$target> for Color {
                type Output = Self;
                fn div(self, s: $target) -> Self::Output {
                    let s = s as f64;
                    Self {
                        mode: self.mode,
                        channels: scaled(&self, |v| v / s),
                    }
                }
            }

            impl DivAssign<$target> for Color {
                fn div_assign(&mut self, s: $target) {
                    let s = s as f64;
                    scale_in_place(self, |v| v / s);
                }
            }
        )*
    }
}

impl_ops!(i8, u8, i16, u16, f32);
impl_ops!(i32, u32, f64);
impl_as_ops!(isize, usize, i64, u64, i128, u128);

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    macro_rules! test_ops {
        ($($val: expr),*) => {
            $(
                // Mul<T> for Color
                let c = color!(200, 50, 10, 100) * $val;
                assert_eq!(c.channels(), [255, 100, 20, 100]);

                // Mul<Color> for T
                let c: Color = $val * color!(200, 50, 10, 100);
                assert_eq!(c.channels(), [255, 100, 20, 100]);

                // MulAssign<T> for Color
                let mut c = color!(200, 50, 10, 100);
                c *= $val;
                assert_eq!(c.channels(), [255, 100, 20, 100]);

                // Div<T> for Color
                let c: Color = color!(100, 255, 0, 100) / $val;
                assert_eq!(c.channels(), [50, 128, 0, 100]);

                // DivAssign<T> for Color
                let mut c = color!(200, 50, 10, 100);
                c /= $val;
                assert_eq!(c.channels(), [100, 25, 5, 100]);
            )*
        };
    }

    #[test]
    fn test_ops() {
        // Add
        let c1 = color!(200, 50, 10, 100);
        let c2 = color!(100, 50, 10, 100);
        let c3 = c1 + c2;
        assert_eq!(c3.channels(), [255, 100, 20, 100]);

        // AddAssign
        let mut c1 = color!(200, 50, 10, 100);
        let c2 = color!(100, 50, 10, 100);
        c1 += c2;
        assert_eq!(c1.channels(), [255, 100, 20, 100]);

        // Sub
        let c1 = color!(200, 100, 20, 200);
        let c2 = color!(100, 50, 30, 100);
        let c3 = c1 - c2;
        assert_eq!(c3.channels(), [100, 50, 0, 200]);

        // SubAssign
        let mut c1 = color!(200, 100, 20, 200);
        let c2 = color!(100, 50, 30, 100);
        c1 -= c2;
        assert_eq!(c1.channels(), [100, 50, 0, 200]);

        test_ops!(2i8, 2u8, 2i16, 2u16, 2i32, 2u32, 2f32, 2f64);
    }

    /// The reference arithmetic the scaling operators have to agree with.
    ///
    /// Spelled out rather than calling [`calculate_channels`], so it keeps the [`f64::round`]
    /// call and pins the behavior independently of the code under test.
    fn round_trip(c: Color, s: f64) -> [u8; 4] {
        use crate::color::conversion::clamp_levels;
        let [v1, v2, v3, a] = c.levels();
        let levels = clamp_levels([v1 * s, v2 * s, v3 * s, a]);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        [
            (levels[0] * 255.0).round().clamp(0.0, 255.0) as u8,
            (levels[1] * 255.0).round().clamp(0.0, 255.0) as u8,
            (levels[2] * 255.0).round().clamp(0.0, 255.0) as u8,
            (levels[3] * 255.0).round().clamp(0.0, 255.0) as u8,
        ]
    }

    /// Scalars either side of the interesting boundaries, including ones that land a channel on
    /// a rounding tie.
    const SCALARS: [f64; 12] = [
        0.0,
        0.1,
        0.25,
        1.0 / 3.0,
        0.5,
        0.5019,
        0.9,
        0.999,
        1.0,
        1.5,
        2.0,
        255.0,
    ];

    #[test]
    fn rgb_scaling_matches_the_level_round_trip() {
        for byte in 0..=u8::MAX {
            let c = color!(
                byte,
                byte.wrapping_add(83),
                byte.wrapping_mul(3),
                byte.wrapping_sub(41)
            );
            for s in SCALARS {
                assert_eq!(
                    (c * s).channels(),
                    round_trip(c, s),
                    "{:?} * {s}",
                    c.channels()
                );
            }
        }
    }

    #[test]
    fn scaling_in_place_matches_scaling_by_value() {
        for byte in 0..=u8::MAX {
            let c = color!(byte, 255 - byte, byte.wrapping_mul(7), byte);
            for s in SCALARS {
                let mut assigned = c;
                assigned *= s;
                assert_eq!(assigned.channels(), (c * s).channels(), "{byte} *= {s}");

                let mut assigned = c;
                assigned /= s;
                assert_eq!(assigned.channels(), (c / s).channels(), "{byte} /= {s}");
            }
        }
    }

    #[test]
    fn scaling_leaves_alpha_alone() {
        for alpha in 0..=u8::MAX {
            let c = color!(200, 50, 10, alpha);
            for s in SCALARS {
                assert_eq!((c * s).channels()[3], alpha, "alpha {alpha} * {s}");
                assert_eq!((c / s).channels()[3], alpha, "alpha {alpha} / {s}");
            }
        }
    }
}
