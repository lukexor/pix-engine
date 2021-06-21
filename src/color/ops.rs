//! [`Color`] operation functions.

use super::{
    conversion::{calculate_channels, convert_levels},
    Color,
};
use std::{
    array::IntoIter,
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
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            i if i < 4 => self.levels.get(i).unwrap(),
            _ => panic!("index out of bounds: the len is 4 but the index is {}", idx),
        }
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Color) -> Self::Output {
        let [v1, v2, v3, a] = self.levels;
        let [ov1, ov2, ov3, ova] = convert_levels(other.levels, other.mode, self.mode);
        let levels = [v1 + ov1, v2 + ov2, v3 + ov3, a + ova];
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
        let [ov1, ov2, ov3, ova] = convert_levels(other.levels, other.mode, self.mode);
        self.levels = [v1 + ov1, v2 + ov2, v3 + ov3, a + ova];
        for level in &mut self.levels {
            *level = level.clamp(0.0, 1.0);
        }
        self.calculate_channels();
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, other: Color) -> Self::Output {
        let [v1, v2, v3, a] = self.levels;
        let [ov1, ov2, ov3, ova] = convert_levels(other.levels, other.mode, self.mode);
        let levels = [v1 - ov1, v2 - ov2, v3 - ov3, a - ova];
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
        let [ov1, ov2, ov3, ova] = convert_levels(other.levels, other.mode, self.mode);
        self.levels = [v1 - ov1, v2 - ov2, v3 - ov3, a - ova];
        for level in &mut self.levels {
            *level = level.clamp(0.0, 1.0);
        }
        self.calculate_channels();
    }
}

impl ExactSizeIterator for Iter {}

impl<T: Into<f64>> FromIterator<T> for Color {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut rgba = [0.0, 0.0, 0.0, 255.0];
        for (i, v) in iter.into_iter().enumerate() {
            rgba[i] = v.into();
        }
        let [r, g, b, a] = rgba;
        Self::rgba(r, g, b, a)
    }
}

impl IntoIterator for Color {
    type Item = u8;
    type IntoIter = IntoIter<Self::Item, 4>;

    /// Owned `Color` iterator over `[r, g, b, a]`.
    ///
    /// This struct is created by the [`into_iter`](Color::into_iter) method on [`Color`]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let c = color!(100, 200, 50);
    /// let mut iterator = c.into_iter();
    ///
    /// assert_eq!(iterator.next(), Some(100));
    /// assert_eq!(iterator.next(), Some(200));
    /// assert_eq!(iterator.next(), Some(50));
    /// assert_eq!(iterator.next(), Some(255));
    /// assert_eq!(iterator.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.channels())
    }
}

/// Immutable `Color` iterator over `[r, g, b, a]`.
///
/// This struct is created by the [`iter`](Color::iter) method on [`Color`]s.
///
/// # Example
///
/// ```
/// # use pix_engine::prelude::*;
/// let c: Color = color!(100, 200, 50);
/// let mut iterator = c.iter();
///
/// assert_eq!(iterator.next(), Some(100));
/// assert_eq!(iterator.next(), Some(200));
/// assert_eq!(iterator.next(), Some(50));
/// assert_eq!(iterator.next(), Some(255));
/// assert_eq!(iterator.next(), None);
/// ```
#[derive(Debug, Clone)]
pub struct Iter {
    inner: [u8; 4],
    current: usize,
}

impl Iter {
    #[inline]
    pub(super) fn new(color: &Color) -> Self {
        Self {
            inner: color.channels(),
            current: 0,
        }
    }
}

impl Iterator for Iter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current > 3 {
            return None;
        }
        let next = self.inner[self.current];
        self.current += 1;
        Some(next)
    }
}

impl IntoIterator for &Color {
    type Item = u8;
    type IntoIter = Iter;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

macro_rules! impl_ops {
    ($($target:ty),*) => {
        $(
            impl Mul<$target> for Color where $target: Into<f64> {
                type Output = Self;
                fn mul(self, s: $target) -> Self::Output {
                    let [v1, v2, v3, a] = self.levels;
                    let s = f64::from(s);
                    let levels = [v1 * s, v2 * s, v3 * s, a * s];
                    let channels = calculate_channels(levels);
                    Self {
                        mode: self.mode,
                        levels,
                        channels,
                    }
                }
            }

            impl Mul<Color> for $target where $target: Into<f64> {
                type Output = Color;
                fn mul(self, c: Color) -> Self::Output {
                    let [v1, v2, v3, a] = c.levels();
                    let s = f64::from(self);
                    let levels = [v1 * s, v2 * s, v3 * s, a * s];
                    let channels = calculate_channels(levels);
                    Color {
                        mode: c.mode,
                        levels,
                        channels,
                    }
                }
            }

            impl MulAssign<$target> for Color where $target: Into<f64> {
                fn mul_assign(&mut self, s: $target) {
                    let [v1, v2, v3, a] = self.levels;
                    let s = f64::from(s);
                    self.levels = [v1 * s, v2 * s, v3 * s, a * s];
                    for level in &mut self.levels {
                        *level = level.clamp(0.0, 1.0);
                    }
                    self.calculate_channels();
                }
            }

            impl Div<$target> for Color where $target: Into<f64> {
                type Output = Self;
                fn div(self, s: $target) -> Self::Output {
                    let [v1, v2, v3, a] = self.levels;
                    let s = f64::from(s);
                    let levels = [v1 / s, v2 / s, v3 / s, a / s];
                    let channels = calculate_channels(levels);
                    Self {
                        mode: self.mode,
                        levels,
                        channels,
                    }
                }
            }

            impl DivAssign<$target> for Color where $target: Into<f64> {
                fn div_assign(&mut self, s: $target) {
                    let [v1, v2, v3, a] = self.levels;
                    let s = f64::from(s);
                    self.levels = [v1 / s, v2 / s, v3 / s, a / s];
                    for level in &mut self.levels {
                        *level = level.clamp(0.0, 1.0);
                    }
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
                    let s = s as f64;
                    let levels = [v1 * s, v2 * s, v3 * s, a * s];
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
                    let s = self as f64;
                    let levels = [v1 * s, v2 * s, v3 * s, a * s];
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
                    let s = s as f64;
                    self.levels = [v1 * s, v2 * s, v3 * s, a * s];
                    for level in &mut self.levels {
                        *level = level.clamp(0.0, 1.0);
                    }
                    self.calculate_channels();
                }
            }

            impl Div<$target> for Color {
                type Output = Self;
                fn div(self, s: $target) -> Self::Output {
                    let [v1, v2, v3, a] = self.levels;
                    let s = s as f64;
                    let levels = [v1 / s, v2 / s, v3 / s, a / s];
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
                    let s = s as f64;
                    self.levels = [v1 / s, v2 / s, v3 / s, a / s];
                    for level in &mut self.levels {
                        *level = level.clamp(0.0, 1.0);
                    }
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
                assert_eq!(c.channels(), [255, 100, 20, 200]);

                // Mul<Color> for T
                let c: Color = $val * color!(200, 50, 10, 100);
                assert_eq!(c.channels(), [255, 100, 20, 200]);

                // MulAssign<T> for Color
                let mut c = color!(200, 50, 10, 100);
                c *= $val;
                assert_eq!(c.channels(), [255, 100, 20, 200]);

                // Div<T> for Color
                let c: Color = color!(100, 255, 0, 100) / $val;
                assert_eq!(c.channels(), [50, 128, 0, 50]);

                // DivAssign<T> for Color
                let mut c = color!(200, 50, 10, 100);
                c /= $val;
                assert_eq!(c.channels(), [100, 25, 5, 50]);
            )*
        };
    }

    #[test]
    fn test_ops() {
        // Add
        let c1 = color!(200, 50, 10, 100);
        let c2 = color!(100, 50, 10, 100);
        let c3 = c1 + c2;
        assert_eq!(c3.channels(), [255, 100, 20, 200]);

        // AddAssign
        let mut c1 = color!(200, 50, 10, 100);
        let c2 = color!(100, 50, 10, 100);
        c1 += c2;
        assert_eq!(c1.channels(), [255, 100, 20, 200]);

        // Sub
        let c1 = color!(200, 100, 20, 200);
        let c2 = color!(100, 50, 30, 100);
        let c3 = c1 - c2;
        assert_eq!(c3.channels(), [100, 50, 0, 100]);

        // SubAssign
        let mut c1 = color!(200, 100, 20, 200);
        let c2 = color!(100, 50, 30, 100);
        c1 -= c2;
        assert_eq!(c1.channels(), [100, 50, 0, 100]);

        test_ops!(2i8, 2u8, 2i16, 2u16, 2i32, 2u32, 2f32, 2f64);
    }
}
