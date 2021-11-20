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
    conversion::{calculate_channels, clamp_levels, convert_levels},
    Color,
};
use crate::prelude::Scalar;
use std::{
    fmt::{self, LowerHex, UpperHex},
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Deref, Div, DivAssign, Index, Mul, MulAssign, Sub, SubAssign},
};

impl LowerHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [red, green, blue, alpha] = self.channels();
        write!(f, "#{:x}{:x}{:x}{:x}", red, green, blue, alpha)
    }
}

impl UpperHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [red, green, blue, alpha] = self.channels();
        write!(f, "#{:X}{:X}{:X}{:X}", red, green, blue, alpha)
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
        let [v1, v2, v3, a] = self.levels;
        let [ov1, ov2, ov3, _] = convert_levels(other.levels, other.mode, self.mode);
        let levels = clamp_levels([v1 + ov1, v2 + ov2, v3 + ov3, a]);
        let channels = calculate_channels(levels);
        Self {
            mode: self.mode,
            levels,
            channels,
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
        let [v1, v2, v3, a] = self.levels;
        let [ov1, ov2, ov3, _] = convert_levels(other.levels, other.mode, self.mode);
        self.levels = clamp_levels([v1 + ov1, v2 + ov2, v3 + ov3, a]);
        self.calculate_channels();
    }
}

impl AddAssign<u8> for Color {
    fn add_assign(&mut self, val: u8) {
        let [r, g, b, a] = self.channels;
        *self = Self::rgba(
            r.saturating_add(val),
            g.saturating_add(val),
            b.saturating_add(val),
            a,
        );
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, other: Color) -> Self::Output {
        let [v1, v2, v3, a] = self.levels;
        let [ov1, ov2, ov3, _] = convert_levels(other.levels, other.mode, self.mode);
        let levels = clamp_levels([v1 - ov1, v2 - ov2, v3 - ov3, a]);
        let channels = calculate_channels(levels);
        Self {
            mode: self.mode,
            levels,
            channels,
        }
    }
}

impl Sub<u8> for Color {
    type Output = Self;
    fn sub(self, val: u8) -> Self::Output {
        let [r, g, b, _] = self.channels;
        Self::rgb(
            r.saturating_sub(val),
            g.saturating_sub(val),
            b.saturating_sub(val),
        )
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, other: Color) {
        let [v1, v2, v3, a] = self.levels;
        let [ov1, ov2, ov3, _] = convert_levels(other.levels, other.mode, self.mode);
        self.levels = clamp_levels([v1 - ov1, v2 - ov2, v3 - ov3, a]);
        self.calculate_channels();
    }
}

impl SubAssign<u8> for Color {
    fn sub_assign(&mut self, val: u8) {
        let [r, g, b, a] = self.channels;
        *self = Self::rgba(
            r.saturating_sub(val),
            g.saturating_sub(val),
            b.saturating_sub(val),
            a,
        );
    }
}

impl Deref for Color {
    type Target = [u8; 4];
    /// Deref `Color` to `&[u8; 4]`.
    fn deref(&self) -> &Self::Target {
        &self.channels
    }
}

macro_rules! impl_ops {
    ($($target:ty),*) => {
        $(
            impl Mul<$target> for Color where $target: Into<Scalar> {
                type Output = Self;
                fn mul(self, s: $target) -> Self::Output {
                    let [v1, v2, v3, a] = self.levels;
                    let s = Scalar::from(s);
                    let levels = clamp_levels([v1 * s, v2 * s, v3 * s, a]);
                    let channels = calculate_channels(levels);
                    Self {
                        mode: self.mode,
                        levels,
                        channels,
                    }
                }
            }

            impl Mul<Color> for $target where $target: Into<Scalar> {
                type Output = Color;
                fn mul(self, c: Color) -> Self::Output {
                    let [v1, v2, v3, a] = c.levels();
                    let s = Scalar::from(self);
                    let levels = clamp_levels([v1 * s, v2 * s, v3 * s, a]);
                    let channels = calculate_channels(levels);
                    Color {
                        mode: c.mode,
                        levels,
                        channels,
                    }
                }
            }

            impl MulAssign<$target> for Color where $target: Into<Scalar> {
                fn mul_assign(&mut self, s: $target) {
                    let [v1, v2, v3, a] = self.levels;
                    let s = Scalar::from(s);
                    self.levels = clamp_levels([v1 * s, v2 * s, v3 * s, a]);
                    self.calculate_channels();
                }
            }

            impl Div<$target> for Color where $target: Into<Scalar> {
                type Output = Self;
                fn div(self, s: $target) -> Self::Output {
                    let [v1, v2, v3, a] = self.levels;
                    let s = Scalar::from(s);
                    let levels = clamp_levels([v1 / s, v2 / s, v3 / s, a]);
                    let channels = calculate_channels(levels);
                    Self {
                        mode: self.mode,
                        levels,
                        channels,
                    }
                }
            }

            impl DivAssign<$target> for Color where $target: Into<Scalar> {
                fn div_assign(&mut self, s: $target) {
                    let [v1, v2, v3, a] = self.levels;
                    let s = Scalar::from(s);
                    self.levels = clamp_levels([v1 / s, v2 / s, v3 / s, a]);
                    self.calculate_channels();
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
                    let [v1, v2, v3, a] = self.levels;
                    let s = s as Scalar;
                    let levels = clamp_levels([v1 * s, v2 * s, v3 * s, a]);
                    let channels = calculate_channels(levels);
                    Self {
                        mode: self.mode,
                        levels,
                        channels,
                    }
                }
            }

            impl Mul<Color> for $target {
                type Output = Color;
                fn mul(self, c: Color) -> Self::Output {
                    let [v1, v2, v3, a] = c.levels();
                    let s = self as Scalar;
                    let levels = clamp_levels([v1 * s, v2 * s, v3 * s, a]);
                    let channels = calculate_channels(levels);
                    Color {
                        mode: c.mode,
                        levels,
                        channels,
                    }
                }
            }

            impl MulAssign<$target> for Color {
                fn mul_assign(&mut self, s: $target) {
                    let [v1, v2, v3, a] = self.levels;
                    let s = s as Scalar;
                    self.levels = clamp_levels([v1 * s, v2 * s, v3 * s, a]);
                    self.calculate_channels();
                }
            }

            impl Div<$target> for Color {
                type Output = Self;
                fn div(self, s: $target) -> Self::Output {
                    let [v1, v2, v3, a] = self.levels;
                    let s = s as Scalar;
                    let levels = clamp_levels([v1 / s, v2 / s, v3 / s, a]);
                    let channels = calculate_channels(levels);
                    Self {
                        mode: self.mode,
                        levels,
                        channels,
                    }
                }
            }

            impl DivAssign<$target> for Color {
                fn div_assign(&mut self, s: $target) {
                    let [v1, v2, v3, a] = self.levels;
                    let s = s as Scalar;
                    self.levels = clamp_levels([v1 / s, v2 / s, v3 / s, a]);
                    self.calculate_channels();
                }
            }
        )*
    }
}

impl_ops!(i8, u8, i16, u16, f32);
#[cfg(target_pointer_width = "64")]
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
}
