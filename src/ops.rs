//! Common [`Engine`] trait implementations for types.

use crate::prelude::*;
use num_traits::AsPrimitive;
use std::{
    array::IntoIter,
    iter::{FromIterator, Product, Sum},
    ops::{
        Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub,
        SubAssign,
    },
};

#[inline]
pub(crate) fn clamp_size(val: u32) -> i32 {
    val.clamp(0, i32::MAX as u32 / 2) as i32
}

#[inline]
pub(crate) fn clamp_dimensions(width: u32, height: u32) -> (i32, i32) {
    (clamp_size(width), clamp_size(height))
}

/// Helper macro to create From conversion traits for arrays into generic shape types.
macro_rules! impl_from_array {
    ($($Type:ident<T$(, $N:ident)?>),* => [$T:ty; $M:expr]) => {$(
        impl<T$(, const $N: usize)?> From<[$T; $M]> for $Type<T$(, $N)?> {
            #[doc = concat!("Converts `[T; M]` to ", stringify!($Type<T$(, $N)?>), ".")]
            #[inline]
            fn from(arr: [$T; $M]) -> Self {
                Self(arr)
            }
        }
        impl<T: Copy$(, const $N: usize)?> From<&[$T; $M]> for $Type<T$(, $N)?> {
            #[doc = concat!("Converts `&[T; M]` to ", stringify!($Type<T$(, $N)?>), ".")]
            #[inline]
            fn from(&arr: &[$T; $M]) -> Self {
                Self(arr)
            }
        }
        impl<T$(, const $N: usize)?> From<$Type<T$(, $N)?>> for [$T; $M] {
            #[doc = concat!("Converts ", stringify!($Type<T$(, $N)?>), " to `[T; M]`.")]
            #[inline]
            fn from(t: $Type<T$(, $N)?>) -> Self {
                t.0
            }
        }
        impl<T: Copy$(, const $N: usize)?> From<&$Type<T$(, $N)?>> for [$T; $M] {
            #[doc = concat!("Converts ", stringify!($Type<T$(, $N)?>), " to `&[T; M]`.")]
            #[inline]
            fn from(t: &$Type<T$(, $N)?>) -> Self {
                t.0
            }
        }
    )*};
}

impl_from_array!(Ellipse<T>, Rect<T>, Sphere<T> => [T; 4]);
impl_from_array!(Point<T, N>, Vector<T, N> => [T; N]);
impl_from_array!(Line<T, N> => [Point<T, N>; 2]);
impl_from_array!(Tri<T, N> => [Point<T, N>; 3]);
impl_from_array!(Quad<T, N> => [Point<T, N>; 4]);

/// Implement an `as_` methods for types for lossy conversions using the `as` operator.
macro_rules! impl_as {
    ($($Type:ident<T$(, $N:ident)?>),*) => {$(
        impl<T$(, const $N: usize)?> $Type<T$(, $N)?> {
            #[doc = concat!("Converts ", stringify!($Type<T$(, $N)?>),
                " to ", stringify!($Type<U$(, $N)?>), ".")]
            #[inline]
            pub fn as_<U>(&self) -> $Type<U$(, $N)?>
            where
                U: 'static + Copy,
                T: AsPrimitive<U>
            {
                $Type(self.map(AsPrimitive::as_))
            }
        }
    )*};
    ($($Type:ident<T$(, $N:ident)?>),* from $U:ident) => {$(
        impl<T$(, const $N: usize)?> $Type<T$(, $N)?> {
            /// Returns `Self` with the numbers cast using `as` operator.
            #[doc = concat!("Converts ", stringify!($Type<T$(, $N)?>),
                " to ", stringify!($Type<U$(, $N)?>), ".")]
            #[inline]
            pub fn as_<U>(&self) -> $Type<U$(, $N)?>
            where
                U: 'static + Copy,
                T: AsPrimitive<U>
            {
                $Type(self.map(|p| $U(p.map(AsPrimitive::as_))))
            }
        }
    )*};
}

impl_as!(Point<T, N>, Vector<T, N>, Ellipse<T>, Rect<T>, Sphere<T>);
impl_as!(Line<T, N>, Tri<T, N>, Quad<T, N> from Point);

/// Helper functions for types that contain floats giving similar element-size methods as [f32] and
/// [f64].
macro_rules! impl_float_conversion {
    ($($Type:ident<T$(, $N:ident)?>),*) => {$(
        impl<T: Float$(, const $N: usize)?> $Type<T$(, $N)?> {
            #[doc = concat!("Returns ", stringify!($Type<T$(, $N)?>),
                " with the nearest integers to the numbers. Round half-way cases away from 0.0.")]
            #[inline]
            pub fn round(&self) -> Self {
                Self(self.map(num_traits::Float::round))
            }
            #[doc = concat!("Returns ", stringify!($Type<T$(, $N)?>),
                " with the largest integers less than or equal to the numbers.")]
            #[inline]
            pub fn floor(&self) -> Self {
                Self(self.map(num_traits::Float::floor))
            }
            #[doc = concat!("Returns ", stringify!($Type<T$(, $N)?>),
                " with the smallest integers greater than or equal to the numbers.")]
            #[inline]
            pub fn ceil(&self) -> Self {
                Self(self.map(num_traits::Float::ceil))
            }
        }
    )*};
    ($($Type:ident<T$(, $N:ident)?>),* from $U:ident) => {$(
        impl<T: Float$(, const $N: usize)?> $Type<T$(, $N)?> {
            #[doc = concat!("Returns ", stringify!($Type<T$(, $N)?>),
                " with the nearest integers to the numbers. Round half-way cases away from 0.0.")]
            #[inline]
            pub fn round(&self) -> Self {
                Self(self.map(|p| $U(p.map(num_traits::Float::round))))
            }
            #[doc = concat!("Returns ", stringify!($Type<T$(, $N)?>),
                "with the largest integers less than or equal to the numbers.")]
            #[inline]
            pub fn floor(&self) -> Self {
                Self(self.map(|p| $U(p.map(num_traits::Float::floor))))
            }
            #[doc = concat!("Returns ", stringify!($Type<T$(, $N)?>),
                " with the smallest integers greater than or equal to the numbers.")]
            #[inline]
            pub fn ceil(&self) -> Self {
                Self(self.map(|p| $U(p.map(num_traits::Float::ceil))))
            }
        }
    )*};
}

impl_float_conversion!(Point<T, N>, Vector<T, N>, Ellipse<T>, Rect<T>, Sphere<T>);
impl_float_conversion!(Line<T, N>, Tri<T, N>, Quad<T, N> from Point);

/// Helper macro to generate standard ops for generic shape types.
macro_rules! impl_wrapper_traits {
    ($($Type:ident<T$(, $N:ident)?>),* => [$T:ty; $M:expr]) => {
        $(
            impl<T$(, const $N: usize)?> Deref for $Type<T$(, $N)?> {
                type Target = [$T; $M];
                #[inline]
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            impl<T$(, const $N: usize)?> DerefMut for $Type<T$(, $N)?> {
                #[inline]
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl<T$(, const $N: usize)?> AsRef<[$T; $M]> for $Type<T$(, $N)?> {
                #[inline]
                fn as_ref(&self) -> &[$T; $M] {
                    &self.0
                }
            }
            impl<T$(, const $N: usize)?> AsMut<[$T; $M]> for $Type<T$(, $N)?> {
                #[inline]
                fn as_mut(&mut self) -> &mut [$T; $M] {
                    &mut self.0
                }
            }

            impl<T$(, const $N: usize)?> Index<usize> for $Type<T$(, $N)?> {
                type Output = $T;
                #[inline]
                fn index(&self, idx: usize) -> &Self::Output {
                    &self.0[idx]
                }
            }
            impl<T$(, const $N: usize)?> IndexMut<usize> for $Type<T$(, $N)?> {
                #[inline]
                fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
                    &mut self.0[idx]
                }
            }

            impl<T$(, const $N: usize)?> IntoIterator for $Type<T$(, $N)?> {
                type Item = $T;
                type IntoIter = IntoIter<Self::Item, $M>;
                #[inline]
                fn into_iter(self) -> Self::IntoIter {
                    self.0.into_iter()
                }
            }
            impl<'a, T$(, const $N: usize)?> IntoIterator for &'a $Type<T$(, $N)?> {
                type Item = &'a $T;
                type IntoIter = std::slice::Iter<'a, $T>;
                #[inline]
                fn into_iter(self) -> Self::IntoIter {
                    self.0.iter()
                }
            }
            impl<'a, T$(, const $N: usize)?> IntoIterator for &'a mut $Type<T$(, $N)?> {
                type Item = &'a mut $T;
                type IntoIter = std::slice::IterMut<'a, $T>;
                #[inline]
                fn into_iter(self) -> Self::IntoIter {
                    self.0.iter_mut()
                }
            }
            impl <T: Default$(, const $N: usize)?> FromIterator<$T> for $Type<T$(, $N)?> {
                #[inline]
                fn from_iter<I>(iter: I) -> Self
                where
                    I: IntoIterator<Item = $T>
                {
                    let mut iter = iter.into_iter();
                    let arr = [(); $M].map(|_| iter.next().unwrap_or_else(<$T>::default));
                    Self(arr)
                }
            }
        )*
    };
}

impl_wrapper_traits!(Ellipse<T>, Rect<T>, Sphere<T> => [T; 4]);
impl_wrapper_traits!(Point<T, N>, Vector<T, N> => [T; N]);
impl_wrapper_traits!(Line<T, N> => [Point<T, N>; 2]);
impl_wrapper_traits!(Tri<T, N> => [Point<T, N>; 3]);
impl_wrapper_traits!(Quad<T, N> => [Point<T, N>; 4]);

/// Multiply `T` * `Type<T, N>` = `Type<T, N>`. Required because of orphan rule: Cannot implement
/// foreign traits on foreign generic types, thus we use concrete primitive types.
macro_rules! impl_primitive_mul {
    ($Type:ident => $($target:ty),*) => {
        $(
            impl<const N: usize> Mul<$Type<$target, N>> for $target {
                type Output = $Type<$target, N>;
                /// T * [Point].
                fn mul(self, t: $Type<$target, N>) -> Self::Output {
                    $Type(t.map(|v| self * v))
                }
            }
        )*
    };
}

impl_primitive_mul!(Point => i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);
impl_primitive_mul!(Vector => i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);

macro_rules! impl_num_assign_op {
    (@ $OpTrait:ident, $func:ident, $op:tt, $Type:ty) => {
        impl<T, U, const N: usize> $OpTrait<U> for $Type
        where
            T: Num + $OpTrait<U>,
            U: Num,
        {
            fn $func(&mut self, val: U) {
                for v in self.iter_mut() {
                    *v $op val;
                }
            }
        }
    };
    ($OpTrait:ident, $func:ident, $Lhs:ty, $op:tt, $Rhs:ty = $Output:ty) => {
        impl<T: Num, const N: usize> $OpTrait<$Rhs> for $Lhs {
            fn $func(&mut self, other: $Rhs) {
                for (v, o) in self.iter_mut().zip(other) {
                    *v $op o;
                }
            }
        }
    };
}

impl_num_assign_op!(AddAssign, add_assign, Point<T, N>, +=, Vector<T, N> = Point<T, N>);
impl_num_assign_op!(AddAssign, add_assign, Point<T, N>, +=, Point<T, N> = Point<T, N>);
impl_num_assign_op!(AddAssign, add_assign, Vector<T, N>, +=, Vector<T, N> = Vector<T, N>);
impl_num_assign_op!(SubAssign, sub_assign, Point<T, N>, -=, Vector<T, N> = Point<T, N>);
impl_num_assign_op!(SubAssign, sub_assign, Point<T, N>, -=, Point<T, N> = Point<T, N>);
impl_num_assign_op!(SubAssign, sub_assign, Vector<T, N>, -=, Vector<T, N> = Vector<T, N>);

macro_rules! impl_num_op {
    (@ $IterTrait:ident, $func:ident, $Bound:ident, $op:tt, $Type:ty) => {
        impl<T, const N: usize> $IterTrait for $Type
        where
            Self: Default + $Bound<Output = Self>,
            T: Num,
        {
            fn $func<I>(iter: I) -> Self
            where
                I: Iterator<Item = Self>,
            {
                let t = <$Type>::default();
                iter.fold(t, |a, b| a $op b)
            }
        }
        impl<'a, T, const N: usize> $IterTrait<&'a $Type> for $Type
        where
            Self: Default + $Bound<Output = Self>,
            T: Num,
        {
            fn $func<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                let t = <$Type>::default();
                iter.fold(t, |a, b| a $op *b)
            }
        }
    };
    (@ $OpTrait:ident, $func:ident, $op:tt, $Type:ty) => {
        impl<T, U, const N: usize> $OpTrait<U> for $Type
        where
            T: Num + $OpTrait<U, Output = T>,
            U: Num,
        {
            type Output = Self;
            fn $func(self, val: U) -> Self::Output {
                let mut t = <$Type>::default();
                for (v, s) in t.iter_mut().zip(self) {
                    *v = s $op val;
                }
                t
            }
        }
    };
    ($($Type:ty),*) => {
        $(
            impl_num_op!(@ Sum, sum, Add, +, $Type);
            impl_num_op!(@ Product, product, Mul, *, $Type);
            impl_num_op!(@ Add, add, +, $Type);
            impl_num_op!(@ Sub, sub, -, $Type);
            impl_num_op!(@ Mul, mul, *, $Type);
            impl_num_op!(@ Div, div, /, $Type);
            impl_num_assign_op!(@ AddAssign, add_assign, +=, $Type);
            impl_num_assign_op!(@ SubAssign, sub_assign, -=, $Type);
            impl_num_assign_op!(@ MulAssign, mul_assign, *=, $Type);
            impl_num_assign_op!(@ DivAssign, div_assign, /=, $Type);
            impl<T, const N: usize> Neg for $Type
            where
                T: Num + Neg<Output = T>,
            {
                type Output = Self;
                fn neg(self) -> Self::Output {
                    let mut t = <$Type>::default();
                    for (v, s) in t.iter_mut().zip(self) {
                        *v = s.neg();
                    }
                    t
                }
            }

        )*
    };
    ($OpTrait:ident, $func:ident, $Lhs:ty, $op:tt, $Rhs:ty = $Output:ty) => {
        impl<T, const N: usize> $OpTrait<$Rhs> for $Lhs
        where
            T: Num + $OpTrait,
        {
            type Output = $Output;
            fn $func(self, other: $Rhs) -> Self::Output {
                let mut t = <$Output>::default();
                for ((v, s), o) in t.iter_mut().zip(self).zip(other) {
                    *v = s $op o;
                }
                t
            }
        }
    };
}

impl_num_op!(Point<T, N>, Vector<T, N>);
impl_num_op!(Add, add, Point<T, N>, +, Point<T, N> = Vector<T, N>);
impl_num_op!(Add, add, Point<T, N>, +, Vector<T, N> = Point<T, N>);
impl_num_op!(Add, add, Vector<T, N>, +, Point<T, N> = Point<T, N>);
impl_num_op!(Add, add, Vector<T, N>, +, Vector<T, N> = Vector<T, N>);
impl_num_op!(Sub, sub, Point<T, N>, -, Point<T, N> = Vector<T, N>);
impl_num_op!(Sub, sub, Point<T, N>, -, Vector<T, N> = Point<T, N>);
impl_num_op!(Sub, sub, Vector<T, N>, -, Point<T, N> = Point<T, N>);
impl_num_op!(Sub, sub, Vector<T, N>, -, Vector<T, N> = Vector<T, N>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_ops() {
        let p = point!(1, 2, 3);
        assert_eq!(p.deref(), &[1, 2, 3]);
        let mut p = point!(1, 2, 3);
        assert_eq!(p.deref_mut(), &mut [1, 2, 3]);

        let p = point!(1, 2, 3);
        assert_eq!(p[0], 1);
        assert_eq!(p[1], 2);
        assert_eq!(p[2], 3);
        assert_eq!(p.get(3), None);

        let p = point!(1, 2, 3);
        for (i, v) in p.into_iter().enumerate() {
            assert_eq!(v, p[i]);
        }
        let i = IntoIterator::into_iter([1, 2, 3]);
        let p = Point::from_iter(i);
        assert_eq!(p, point!(1, 2, 3));
    }

    #[test]
    fn from_array() {
        macro_rules! test {
            ($Type:ident, $N:expr; $($T:ty),* => $arr:expr) => {$(
                let t: $Type<$T, $N> = $arr.into();
                assert_eq!(t.deref(), &$arr);
                let v: [$T; $N] = <$Type<$T, $N>>::new($arr).into();
                assert_eq!(v, $arr);
            )*};
        }
        test!(Point, 1; i8, u8, i16, u16, i32, u32, i64, u64, i128, u128 => [1]);
        test!(Point, 2; i8, u8, i16, u16, i32, u32, i64, u64, i128, u128 => [1, 2]);
        test!(Point, 3; i8, u8, i16, u16, i32, u32, i64, u64, i128, u128 => [1, 2, 3]);
        test!(Vector, 1; i8, u8, i16, u16, i32, u32, i64, u64, i128, u128 => [1]);
        test!(Vector, 2; i8, u8, i16, u16, i32, u32, i64, u64, i128, u128 => [1, 2]);
        test!(Vector, 3; i8, u8, i16, u16, i32, u32, i64, u64, i128, u128 => [1, 2, 3]);
    }
}
