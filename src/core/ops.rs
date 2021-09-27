//! Common [PixEngine] trait implementations for types.
//!
//! [PixEngine]: crate::prelude::PixEngine

use crate::prelude_3d::*;
use num_traits::AsPrimitive;
use std::{
    array::IntoIter,
    iter::{FromIterator, Product, Sum},
    ops::*,
};

// TODO: AsPrimitive is convenient but not semantically accurate

/// Helper macro to create From conversion traits for arrays into generic shape types.
macro_rules! impl_from_array {
    // Convert array [U; M] to Type<T, N> and vice versa using U: Into<T> and T: Into<U>.
    ($($Type:ident<T$(, $N:ident)?>),* => [$T:ty; $M:expr]) => {$(
        impl<T, U$(, const $N: usize)?> From<[U; $M]> for $Type<T$(, $N)?>
        where
            $T: 'static + Copy,
            U: AsPrimitive<$T>,
        {
            #[inline]
            fn from(arr: [U; $M]) -> Self {
                Self(arr.map(|v| v.as_()))
            }
        }
        impl<T, U$(, const $N: usize)?> From<$Type<T$(, $N)?>> for [U; $M]
        where
            U: 'static + Copy,
            $T: AsPrimitive<U>,
        {
            #[inline]
            fn from(t: $Type<T$(, $N)?>) -> Self {
                t.0.map(|v| v.as_())
            }
        }
    )*};
}

/// Helper macro to create From conversion traits for arrays into generic shape types.
macro_rules! impl_from_generic_array {
    // Convert array [U; M] to Type<T, N> and vice versa using U: Into<T> and T: Into<U>.
    ($($Type:ident<T$(, $N:ident)?>),* => [$T:ty; $M:expr]) => {$(
        impl<T, U$(, const $N: usize)?> From<[U; $M]> for $Type<T$(, $N)?>
        where
            U: Into<$T>,
        {
            #[inline]
            fn from(arr: [U; $M]) -> Self {
                Self(arr.map(|v| v.into()))
            }
        }
        impl<T, U$(, const $N: usize)?> From<$Type<T$(, $N)?>> for [U; $M]
        where
            $T: Into<U>,
        {
            #[inline]
            fn from(t: $Type<T$(, $N)?>) -> Self {
                t.0.map(|v| v.into())
            }
        }
    )*};
}

/// Helper macro to create From conversion traits between generic shape types.
macro_rules! impl_from {
    // Convert Type<U> to Type<T> using U: Into<T>.
    // e.g. Ellipse<U> into Ellipse<T> where U: Into<T>.
    (@ into $Type:ident: $U:ty => { $($T:ty),* } ) => {$(
        impl From<$Type<$U>> for $Type<$T>
        where
            $T: 'static + Copy,
            $U: AsPrimitive<$T>,
        {
            #[inline]
            fn from(t: $Type<$U>) -> Self {
                Self(t.0.map(|v| v.as_()))
            }
        }
    )*};
    // Convert Type<U, N> to Type<T, N> using U: Into<T>.
    // e.g. Point<U, N> to Vector<T, N> where U: Into<T>.
    (@ $N:ident into $Type:ident: $U:ty => { $($T:ty),* } ) => {$(
        impl<const N: usize> From<$Type<$U, N>> for $Type<$T, N>
        where
            $T: 'static + Copy,
            $U: AsPrimitive<$T>,
        {
            #[inline]
            fn from(t: $Type<$U, N>) -> Self {
                Self(t.0.map(|v| v.as_()))
            }
        }
    )*};
    // Convert Type<U, N> to Type<V, N> using T<U, N>: Into<T<V, N>>.
    // Required to get around the existing blanket impl From<U> where U: Into<T> for complex types
    // like Point<T, N>.
    // e.g. Line<U, N> to Line<V, N> where Point<U, N>: Into<Point<V, N>>.
    (@ $N:ident into $Type:ident: $T:ident, $U:ty => { $($V:ty),* } ) => {$(
        impl<const N: usize> From<$Type<$U, N>> for $Type<$V, N>
        where
            $T<$U, N>: Into<$T<$V, N>>,
        {
            #[inline]
            fn from(t: $Type<$U, N>) -> Self {
                Self(t.0.map(|v| v.into()))
            }
        }
    )*};
    (@ $Type:ident<T$(, $N:ident)?> into $($T:ident)?: primitive) => {
        impl_from!(@ $($N)? into $Type: $($T,)? u8 => { i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64 });
        impl_from!(@ $($N)? into $Type: $($T,)? i8 => { u8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64 });
        impl_from!(@ $($N)? into $Type: $($T,)? u16 => { i8, u8, i16, u32, i32, u64, i64, usize, isize, f32, f64 });
        impl_from!(@ $($N)? into $Type: $($T,)? i16 => { i8, u8, u16, u32, i32, u64, i64, usize, isize, f32, f64 });
        impl_from!(@ $($N)? into $Type: $($T,)? u32 => { i8, u8, i16, u16, i32, u64, i64, usize, isize, f32, f64 });
        impl_from!(@ $($N)? into $Type: $($T,)? i32 => { i8, u8, i16, u16, u32, u64, i64, usize, isize, f32, f64 });
        impl_from!(@ $($N)? into $Type: $($T,)? u64 => { i8, u8, i16, u16, i32, u32, i64, usize, isize, f32, f64 });
        impl_from!(@ $($N)? into $Type: $($T,)? i64 => { i8, u8, i16, u16, i32, u32, u64, usize, isize, f32, f64 });
        impl_from!(@ $($N)? into $Type: $($T,)? usize => { i8, u8, i16, u16, i32, u32, u64, isize, f32, f64 });
        impl_from!(@ $($N)? into $Type: $($T,)? isize => { i8, u8, i16, u16, i32, u32, u64, usize, f32, f64 });
        impl_from!(@ $($N)? into $Type: $($T,)? f32 => { i8, u8, i16, u16, i32, u32, u64, isize, usize, f64 });
        impl_from!(@ $($N)? into $Type: $($T,)? f64 => { i8, u8, i16, u16, i32, u32, u64, isize, usize, f32 });
    };
    // Convert Type<T, N> to Other and all Type<T, N> to Type<U, N> for all numeric T.
    ($Type:ident<T$(, $N:ident)?> into $Other:ty) => {
        impl<T, U$(, const $N: usize)?> From<$Other> for $Type<T$(, $N)?>
        where
            T: 'static + Copy,
            U: AsPrimitive<T>,
        {
            #[inline]
            fn from(t: $Other) -> Self {
                Self(t.0.map(|v| v.as_()))
            }
        }
    };
    ($($Type:ident<T$(, $N:ident)?>),*) => {$(
        impl_from!(@ $Type<T$(, $N)?> into : primitive);
    )*};
    ($($Type:ident<T$(, $N:ident)?>),* from $T:ident) => {$(
        impl_from!(@ $Type<T$(, $N)?> into $T: primitive);
    )*};
}

/// Helper macro to generate standard ops for generic shape types.
macro_rules! impl_core_traits {
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

            impl<T: Copy$(, const $N: usize)?> Index<usize> for $Type<T$(, $N)?> {
                type Output = $T;
                #[inline]
                fn index(&self, idx: usize) -> &Self::Output {
                    &self.0[idx]
                }
            }
            impl<T: Copy$(, const $N: usize)?> IndexMut<usize> for $Type<T$(, $N)?> {
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
                    Self::IntoIter::new(self.0)
                }
            }
            impl <T: Copy + Default$(, const $N: usize)?> FromIterator<$T> for $Type<T$(, $N)?> {
                #[inline]
                fn from_iter<I>(iter: I) -> Self
                where
                    I: IntoIterator<Item = $T>
                {
                    let mut arr = [<$T>::default(); $M];
                    for (i, v) in iter.into_iter().enumerate() {
                        arr[i] = v;
                    }
                    Self(arr)
                }
            }
        )*
    };
}

impl_core_traits!(Ellipse<T>, Rect<T>, Sphere<T> => [T; 4]);
impl_core_traits!(Point<T, N>, Vector<T, N> => [T; N]);
impl_core_traits!(Line<T, N> => [Point<T, N>; 2]);
impl_core_traits!(Tri<T, N> => [Point<T, N>; 3]);
impl_core_traits!(Quad<T, N> => [Point<T, N>; 4]);

impl_from_array!(Ellipse<T>, Rect<T>, Sphere<T> => [T; 4]);
impl_from_array!(Point<T, N>, Vector<T, N> => [T; N]);
impl_from_generic_array!(Line<T, N> => [Point<T, N>; 2]);
impl_from_generic_array!(Tri<T, N> => [Point<T, N>; 3]);
impl_from_generic_array!(Quad<T, N> => [Point<T, N>; 4]);

impl_from!(Point<T, N> into Vector<U, N>);
impl_from!(Vector<T, N> into Point<U, N>);

impl_from!(Point<T, N>, Vector<T, N>, Ellipse<T>, Rect<T>, Sphere<T>);
impl_from!(Line<T, N>, Tri<T, N>, Quad<T, N> from Point);

// Required because of orphan rule: Cannot implement foreign traits on foreign generic types, thus
// we use concrete primitive types.
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

macro_rules! impl_num_op {
    (@ $IterTrait:ident, $func:ident, $Bound:ident, $op:tt, $Type:ty) => {
        impl<T, const N: usize> $IterTrait for $Type
        where
            Self: Default + $Bound<Output = Self>
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
            T: Copy,
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
                for i in 0..N {
                    t[i] = self[i] $op val;
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
                    for i in 0..N {
                        t[i] = self[i].neg();
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
                for i in 0..N {
                    t[i] = self[i] $op other[i];
                }
                t
            }
        }
    };
}

macro_rules! impl_num_assign_op {
    (@ $OpTrait:ident, $func:ident, $op:tt, $Type:ty) => {
        impl<T, U, const N: usize> $OpTrait<U> for $Type
        where
            T: Num + $OpTrait<U>,
            U: Num,
        {
            fn $func(&mut self, val: U) {
                for i in 0..N {
                    self[i] $op val;
                }
            }
        }
    };
    ($OpTrait:ident, $func:ident, $Lhs:ty, $op:tt, $Rhs:ty = $Output:ty) => {
        impl<T, const N: usize> $OpTrait<$Rhs> for $Lhs
        where
            T: Num,
        {
            fn $func(&mut self, other: $Rhs) {
                for i in 0..N {
                    self[i] $op other[i];
                }
            }
        }
    };
}

impl_num_op!(Point<T, N>, Vector<T, N>);

impl_num_op!(Add, add, Point<T, N>, +, Point<T, N> = Vector<T, N>);
impl_num_op!(Sub, sub, Point<T, N>, -, Point<T, N> = Vector<T, N>);

impl_num_op!(Add, add, Vector<T, N>, +, Point<T, N> = Point<T, N>);
impl_num_op!(Sub, sub, Vector<T, N>, -, Point<T, N> = Point<T, N>);

impl_num_op!(Add, add, Vector<T, N>, +, Vector<T, N> = Vector<T, N>);
impl_num_op!(Sub, sub, Vector<T, N>, -, Vector<T, N> = Vector<T, N>);
impl_num_assign_op!(AddAssign, add_assign, Vector<T, N>, +=, Vector<T, N> = Vector<T, N>);
impl_num_assign_op!(SubAssign, sub_assign, Vector<T, N>, -=, Vector<T, N> = Vector<T, N>);

impl_num_op!(Add, add, Point<T, N>, +, Vector<T, N> = Point<T, N>);
impl_num_op!(Sub, sub, Point<T, N>, -, Vector<T, N> = Point<T, N>);
impl_num_assign_op!(AddAssign, add_assign, Point<T, N>, +=, Vector<T, N> = Point<T, N>);
impl_num_assign_op!(SubAssign, sub_assign, Point<T, N>, -=, Vector<T, N> = Point<T, N>);

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
        let p: Point<u8, 1> = [1u8].into();
        assert_eq!(p, Point::new([1u8]));
        let p: Point<i8, 1> = [1i8].into();
        assert_eq!(p, Point::new([1i8]));
        let p: Point<u16, 1> = [1u16].into();
        assert_eq!(p, Point::new([1u16]));
        let p: Point<i16, 1> = [1i16].into();
        assert_eq!(p, Point::new([1i16]));
        let p: Point<u32, 1> = [1u32].into();
        assert_eq!(p, Point::new([1u32]));
        let p: Point<i32, 1> = [1i32].into();
        assert_eq!(p, Point::new([1i32]));
        let p: Point<u64, 1> = [1u64].into();
        assert_eq!(p, Point::new([1u64]));
        let p: Point<i64, 1> = [1i64].into();
        assert_eq!(p, Point::new([1i64]));
        let p: Point<f32, 1> = [1.0f32].into();
        assert_eq!(p, Point::new([1f32]));
        let p: Point<f64, 1> = [1.0f64].into();
        assert_eq!(p, Point::new([1f64]));

        let p: Point<u8, 2> = [1u8, 2].into();
        assert_eq!(p, Point::new([1u8, 2]));
        let p: Point<i8, 2> = [1i8, 2].into();
        assert_eq!(p, Point::new([1i8, 2]));
        let p: Point<u16, 2> = [1u16, 2].into();
        assert_eq!(p, Point::new([1u16, 2]));
        let p: Point<i16, 2> = [1i16, 2].into();
        assert_eq!(p, Point::new([1i16, 2]));
        let p: Point<i32, 2> = [1i32, 2].into();
        assert_eq!(p, Point::new([1i32, 2]));
        let p: Point<u32, 2> = [1u32, 2].into();
        assert_eq!(p, Point::new([1u32, 2]));
        let p: Point<u64, 2> = [1u64, 2].into();
        assert_eq!(p, Point::new([1u64, 2]));
        let p: Point<i64, 2> = [1i64, 2].into();
        assert_eq!(p, Point::new([1i64, 2]));
        let p: Point<f32, 2> = [1.0f32, 2.0].into();
        assert_eq!(p, Point::new([1f32, 2.0]));
        let p: Point<f64, 2> = [1.0f64, 2.0].into();
        assert_eq!(p, Point::new([1f64, 2.0]));

        let p: Point<u8, 3> = [1u8, 2, 3].into();
        assert_eq!(p, Point::new([1u8, 2, 3]));
        let p: Point<i8, 3> = [1i8, 2, 3].into();
        assert_eq!(p, Point::new([1i8, 2, 3]));
        let p: Point<u16, 3> = [1u16, 2, 3].into();
        assert_eq!(p, Point::new([1u16, 2, 3]));
        let p: Point<i16, 3> = [1i16, 2, 3].into();
        assert_eq!(p, Point::new([1i16, 2, 3]));
        let p: Point<u32, 3> = [1u32, 2, 3].into();
        assert_eq!(p, Point::new([1u32, 2, 3]));
        let p: Point<i32, 3> = [1i32, 2, 3].into();
        assert_eq!(p, Point::new([1i32, 2, 3]));
        let p: Point<u64, 3> = [1u64, 2, 3].into();
        assert_eq!(p, Point::new([1u64, 2, 3]));
        let p: Point<i64, 3> = [1i64, 2, 3].into();
        assert_eq!(p, Point::new([1i64, 2, 3]));
        let p: Point<f32, 3> = [1.0f32, 2.0, 3.0].into();
        assert_eq!(p, Point::new([1f32, 2.0, 3.0]));
        let p: Point<f64, 3> = [1.0f64, 2.0, 3.0].into();
        assert_eq!(p, Point::new([1f64, 2.0, 3.0]));
    }

    #[test]
    fn to_array() {
        let v: [i8; 3] = point!(1i8, 2, 3).into();
        assert_eq!(v, [1i8, 2, 3]);
        todo!("Test all types");
    }

    #[test]
    fn convert_array() {
        // Smaller -> Larger
        let v: [i16; 3] = point!(1i8, 2, 3).into();
        assert_eq!(v, [1i16, 2, 3]);

        // Larger -> Smaller
        let v: [i8; 3] = point!(1i16, 2, 3).into();
        assert_eq!(v, [1i8, 2, 3]);

        todo!("Test all types");
    }

    #[test]
    fn convert_self() {
        // Smaller -> Larger
        let p: Point<i16, 3> = point!(1i8, 2, 3).into();
        assert_eq!(p, Point::new([1i16, 2, 3]));

        // Larger -> Smaller
        let p: Point<i8, 3> = point!(1i16, 2, 3).into();
        assert_eq!(p, Point::new([1i8, 2, 3]));

        todo!("Test all types");
    }

    #[test]
    fn convert_other() {
        // Smaller -> Larger
        let v: Vector<i16, 3> = point!(1i8, 2, 3).into();
        assert_eq!(v, Vector::new([1i16, 2, 3]));

        // Larger -> Smaller
        let v: Vector<i8, 3> = point!(1i16, 2, 3).into();
        assert_eq!(v, Vector::new([1i8, 2, 3]));

        todo!("Test all types");
    }
}
