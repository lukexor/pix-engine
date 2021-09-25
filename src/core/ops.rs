//! Common [PixEngine] trait implementations for types.
//!
//! [PixEngine]: crate::prelude::PixEngine

use crate::prelude_3d::*;
use num_traits::AsPrimitive;
use std::{array::IntoIter, iter::FromIterator, ops::*};

// TODO: These macros all work - but could be condensed and cleaned up

/// Helper macro to create From and TryFrom conversion traits for arrays into generic shape types.
macro_rules! impl_from_array {
    // Convert [U; M] to Type<T, N> and vice versa using U: Into<T> and T: Into<U>.
    ($($Type:ident<T$(, $N:ident)?>),* => [$T:ty; $M:expr]) => {
        $(
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
        )*
    };
}

macro_rules! impl_from_num_array {
    // Convert [U; M] to Type<T, N> and vice versa using U: Into<T> and T: Into<U>.
    ($($Type:ident<T$(, $N:ident)?>),* => [$T:ty; $M:expr]) => {
        $(
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

            // Tuples?
        )*
    };
}

/// Helper macro to create From and TryFrom conversion traits between generic shape types.
macro_rules! impl_convert {
    // Convert Type<T, N> to Type<U, N> using T: AsPrimitive<U>.
    (@ const as $Type:ident: $T:ty => { $($U:ty),* } ) => {
        $(
            impl<const N: usize> From<$Type<$T, N>> for $Type<$U, N>
            where
                $T: AsPrimitive<$U>
            {
                #[inline]
                fn from(t: $Type<$T, N>) -> Self {
                    Self(t.0.map(|v| v.as_()))
                }
            }
        )*
    };
    // Convert Type<T> to Type<U> using T: AsPrimitive<U>.
    (@ as $Type:ident: $T:ty => { $($U:ty),* } ) => {
        $(
            impl From<$Type<$T>> for $Type<$U>
            where
                $T: AsPrimitive<$U>
            {
                #[inline]
                fn from(t: $Type<$T>) -> Self {
                    Self(t.0.map(|v| v.as_()))
                }
            }
        )*
    };
    // Convert Type<T, N> to Type<U, N> using OtherType<T>: Into<OtherType<<U>>.
    (@ into $Type:ident: $T:ident<$V:ty> => { $($U:ty),* } ) => {
        $(
            impl<const N: usize> From<$Type<$V, N>> for $Type<$U, N>
            where
                $T<$V, N>: Into<$T<$U, N>>
            {
                #[inline]
                fn from(t: $Type<$V, N>) -> Self {
                    Self(t.0.map(|v| v.into()))
                }
            }
        )*
    };
    // Convert Type<T, N> to Other and all Type<T, N> to Type<U, N> for all numeric T.
    ($Type:ident<$T:ident$(, $N:ident)?> as $Other:ty) => {
        impl<T, U$(, const $N: usize)?> From<$Other> for $Type<$T$(, $N)?>
        where
            T: 'static + Copy,
            U: AsPrimitive<$T>,
        {
            #[inline]
            fn from(t: $Other) -> Self {
                Self(t.0.map(|v| v.as_()))
            }
        }
        impl_convert!(@ const as $Type: u8 => { i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64 });
        impl_convert!(@ const as $Type: i8 => { u8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64 });
        impl_convert!(@ const as $Type: u16 => { i8, u8, i16, u32, i32, u64, i64, usize, isize, f32, f64 });
        impl_convert!(@ const as $Type: i16 => { i8, u8, u16, u32, i32, u64, i64, usize, isize, f32, f64 });
        impl_convert!(@ const as $Type: u32 => { i8, u8, i16, u16, i32, u64, i64, usize, isize, f32, f64 });
        impl_convert!(@ const as $Type: i32 => { i8, u8, i16, u16, u32, u64, i64, usize, isize, f32, f64 });
        impl_convert!(@ const as $Type: u64 => { i8, u8, i16, u16, i32, u32, i64, usize, isize, f32, f64 });
        impl_convert!(@ const as $Type: i64 => { i8, u8, i16, u16, i32, u32, u64, usize, isize, f32, f64 });
        impl_convert!(@ const as $Type: usize => { i8, u8, i16, u16, i32, u32, u64, isize, f32, f64 });
        impl_convert!(@ const as $Type: isize => { i8, u8, i16, u16, i32, u32, u64, usize, f32, f64 });
        impl_convert!(@ const as $Type: f32 => { i8, u8, i16, u16, i32, u32, u64, isize, usize, f64 });
        impl_convert!(@ const as $Type: f64 => { i8, u8, i16, u16, i32, u32, u64, isize, usize, f32 });
    };
    // Convert Type<T> to Type<U> for all numeric T.
    ($($Type:ident<T>),*) => {
        $(
            impl_convert!(@ as $Type: u8 => { i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64 });
            impl_convert!(@ as $Type: i8 => { u8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64 });
            impl_convert!(@ as $Type: u16 => { i8, u8, i16, u32, i32, u64, i64, usize, isize, f32, f64 });
            impl_convert!(@ as $Type: i16 => { i8, u8, u16, u32, i32, u64, i64, usize, isize, f32, f64 });
            impl_convert!(@ as $Type: u32 => { i8, u8, i16, u16, i32, u64, i64, usize, isize, f32, f64 });
            impl_convert!(@ as $Type: i32 => { i8, u8, i16, u16, u32, u64, i64, usize, isize, f32, f64 });
            impl_convert!(@ as $Type: u64 => { i8, u8, i16, u16, i32, u32, i64, usize, isize, f32, f64 });
            impl_convert!(@ as $Type: i64 => { i8, u8, i16, u16, i32, u32, u64, usize, isize, f32, f64 });
            impl_convert!(@ as $Type: usize => { i8, u8, i16, u16, i32, u32, u64, isize, f32, f64 });
            impl_convert!(@ as $Type: isize => { i8, u8, i16, u16, i32, u32, u64, usize, f32, f64 });
            impl_convert!(@ as $Type: f32 => { i8, u8, i16, u16, i32, u32, u64, isize, usize, f64 });
            impl_convert!(@ as $Type: f64 => { i8, u8, i16, u16, i32, u32, u64, isize, usize, f32 });
        )*
    };
    // Convert Type<T, N> to Type<U, N> for all numeric OtherType<T>.
    ($($Type:ident<T, N>: $T:ident),*) => {
        $(
            impl_convert!(@ into $Type: $T<u8> => { i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64 });
            impl_convert!(@ into $Type: $T<i8> => { u8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64 });
            impl_convert!(@ into $Type: $T<u16> => { i8, u8, i16, u32, i32, u64, i64, usize, isize, f32, f64 });
            impl_convert!(@ into $Type: $T<i16> => { i8, u8, u16, u32, i32, u64, i64, usize, isize, f32, f64 });
            impl_convert!(@ into $Type: $T<u32> => { i8, u8, i16, u16, i32, u64, i64, usize, isize, f32, f64 });
            impl_convert!(@ into $Type: $T<i32> => { i8, u8, i16, u16, u32, u64, i64, usize, isize, f32, f64 });
            impl_convert!(@ into $Type: $T<u64> => { i8, u8, i16, u16, i32, u32, i64, usize, isize, f32, f64 });
            impl_convert!(@ into $Type: $T<i64> => { i8, u8, i16, u16, i32, u32, u64, usize, isize, f32, f64 });
            impl_convert!(@ into $Type: $T<usize> => { i8, u8, i16, u16, i32, u32, u64, isize, f32, f64 });
            impl_convert!(@ into $Type: $T<isize> => { i8, u8, i16, u16, i32, u32, u64, usize, f32, f64 });
            impl_convert!(@ into $Type: $T<f32> => { i8, u8, i16, u16, i32, u32, u64, isize, usize, f64 });
            impl_convert!(@ into $Type: $T<f64> => { i8, u8, i16, u16, i32, u32, u64, isize, usize, f32 });
        )*
    }
}

/// Helper macro to generate standard ops for generic shape types.
macro_rules! impl_ops {
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

impl_ops!(Ellipse<T>, Rect<T>, Sphere<T> => [T; 4]);
impl_ops!(Point<T, N>, Vector<T, N> => [T; N]);
impl_ops!(Line<T, N> => [Point<T, N>; 2]);
impl_ops!(Tri<T, N> => [Point<T, N>; 3]);
impl_ops!(Quad<T, N> => [Point<T, N>; 4]);

impl_from_num_array!(Ellipse<T>, Rect<T>, Sphere<T> => [T; 4]);
impl_from_num_array!(Point<T, N>, Vector<T, N> => [T; N]);
impl_from_array!(Line<T, N> => [Point<T, N>; 2]);
impl_from_array!(Tri<T, N> => [Point<T, N>; 3]);
impl_from_array!(Quad<T, N> => [Point<T, N>; 4]);

impl_convert!(Point<T, N> as Vector<U, N>);
impl_convert!(Vector<T, N> as Point<U, N>);

impl_convert!(Ellipse<T>, Rect<T>, Sphere<T>);
impl_convert!(Line<T, N>: Point);
impl_convert!(Tri<T, N>: Point);
impl_convert!(Quad<T, N>: Point);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_ops() {
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
    fn point_from_array() {
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
    fn point_to_array() {
        let v: [i8; 3] = point!(1i8, 2, 3).into();
        assert_eq!(v, [1i8, 2, 3]);
        todo!("Test all types");
    }

    #[test]
    fn point_convert_array() {
        // Smaller -> Larger
        let v: [i16; 3] = point!(1i8, 2, 3).into();
        assert_eq!(v, [1i16, 2, 3]);

        // Larger -> Smaller
        let v: [i8; 3] = point!(1i16, 2, 3).into();
        assert_eq!(v, [1i8, 2, 3]);

        todo!("Test all types");
    }

    #[test]
    fn point_convert_self() {
        // Smaller -> Larger
        let p: Point<i16, 3> = point!(1i8, 2, 3).into();
        assert_eq!(p, Point::new([1i16, 2, 3]));

        // Larger -> Smaller
        let p: Point<i8, 3> = point!(1i16, 2, 3).into();
        assert_eq!(p, Point::new([1i8, 2, 3]));

        todo!("Test all types");
    }

    #[test]
    fn point_convert_other() {
        // Smaller -> Larger
        let v: Vector<i16, 3> = point!(1i8, 2, 3).into();
        assert_eq!(v, Vector::new([1i16, 2, 3]));

        // Larger -> Smaller
        let v: Vector<i8, 3> = point!(1i16, 2, 3).into();
        assert_eq!(v, Vector::new([1i8, 2, 3]));

        todo!("Test all types");
    }
}
