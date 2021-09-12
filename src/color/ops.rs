//! [Color] operation functions.

use super::{
    conversion::{calculate_channels, clamp_levels, convert_levels},
    Color,
};
use crate::prelude::Scalar;
use std::{
    fmt::{self, LowerHex, UpperHex},
    iter::FromIterator,
    ops::*,
};

impl LowerHex for Color {
    #[allow(clippy::many_single_char_names)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [r, g, b, a] = self.channels();
        write!(f, "#{:x}{:x}{:x}{:x}", r, g, b, a)
    }
}

impl UpperHex for Color {
    #[allow(clippy::many_single_char_names)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [r, g, b, a] = self.channels();
        write!(f, "#{:X}{:X}{:X}{:X}", r, g, b, a)
    }
}

impl Index<usize> for Color {
    type Output = Scalar;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            i if i < 4 => &self.levels[i],
            _ => panic!("index out of bounds: the len is 4 but the index is {}", idx),
        }
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

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        let [v1, v2, v3, a] = self.levels;
        let [ov1, ov2, ov3, _] = convert_levels(other.levels, other.mode, self.mode);
        self.levels = clamp_levels([v1 + ov1, v2 + ov2, v3 + ov3, a]);
        self.calculate_channels();
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

impl SubAssign for Color {
    fn sub_assign(&mut self, other: Color) {
        let [v1, v2, v3, a] = self.levels;
        let [ov1, ov2, ov3, _] = convert_levels(other.levels, other.mode, self.mode);
        self.levels = clamp_levels([v1 - ov1, v2 - ov2, v3 - ov3, a]);
        self.calculate_channels();
    }
}

impl FromIterator<u8> for Color {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = u8>,
    {
        let mut iter = iter.into_iter();
        Self::rgba(
            iter.next().unwrap_or(0),
            iter.next().unwrap_or(0),
            iter.next().unwrap_or(0),
            iter.next().unwrap_or(0),
        )
    }
}

impl Deref for Color {
    type Target = [u8; 4];
    fn deref(&self) -> &Self::Target {
        &self.channels
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.channels
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

impl_ops!(i8, u8, i16, u16, i32, u32, f32, f64);
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
