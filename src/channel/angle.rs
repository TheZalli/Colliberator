//! A module for angle types
//!
//! All operations done for these types immediately wrap around, so it is impossible to
//! create a value out of bounds with them

use std::f32::consts::PI as PI32;
use std::ops::*;

use num_traits::{NumCast, NumOps, ToPrimitive};

use crate::{cuwf, cuwtf, Channel};

/// A trait for all angle types
///
/// Operations done for these types wrap into it's normal range, starting from 0 and ending in
/// the value of full revolution (360° in degrees, 2π in radians).
pub trait Angle: Sized {
    /// The inner type of this angle
    type Inner: PartialOrd + From<Self> + Into<Self> + NumCast + NumOps;

    /// Tells whether this angle's inner value is an integer type
    ///
    /// If false the angle has a floating point value.
    const INTEGER: bool;

    /// Value of a full angle (360° or 2π rad)
    ///
    /// Any angle with this value should be wrapped to 0.
    fn full_angle() -> Self;

    /// Value of a zero angle (0° or 0 rad)
    fn zero_angle() -> Self;

    /// Wraps the angle to the value between zero and full angle
    fn wrap(self) -> Self {
        let full = || Self::full_angle().into();
        let zero = || Self::zero_angle().into();
        let a = Into::<Self::Inner>::into(self) % full();
        if a < zero() {
            (a + full()).into()
        } else {
            a.into()
        }
    }
}

/// A wrapper type for angles in degrees
#[derive(Debug, Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Deg<T>(pub T);

/// A wrapper type for angles in radians
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct Rad(pub f32);

/// A wrapper type for angles in revolutions
pub struct Rev<T>(pub T);

macro_rules! impl_deg_angles {
    ( $struct_name:ident; $( $type:ty, $isint:expr );* ) => { $(
        impl Angle for $struct_name<$type> {
            type Inner = $type;
            const INTEGER: bool = $isint;
            fn full_angle() -> Self { Self(360 as $type) }
            fn zero_angle() -> Self { Self(0 as $type) }
        }
    )* };
}

impl_deg_angles!(Deg; i16, true; i32, true; f32, false);

impl Angle for Rad {
    type Inner = f32;
    const INTEGER: bool = false;
    fn full_angle() -> Self {
        Self(2.0 * PI32)
    }
    fn zero_angle() -> Self {
        Self(0.0)
    }
}

macro_rules! impl_int_rev_angles {
    ( $struct_name:ident, $( $type:ty ),* ) => { $(
        impl Angle for $struct_name<$type> {
            type Inner = $type;
            const INTEGER: bool = true;
            fn full_angle() -> Self { Self(<$type>::ch_max()) }
            fn zero_angle() -> Self { Self(<$type>::ch_zero()) }
            fn wrap(self) -> Self { self }
        }
    )* };
}

impl_int_rev_angles!(Rev, u8, u16, u32);

impl Angle for Rev<f32> {
    type Inner = f32;
    const INTEGER: bool = false;
    fn full_angle() -> Self {
        Self(1.0)
    }
    fn zero_angle() -> Self {
        Self(0.0)
    }
}

impl<T: Angle + NumCast + PartialOrd> Channel for T {
    const INTEGER: bool = T::INTEGER;

    fn ch_max() -> Self {
        T::full_angle()
    }
    fn ch_mid() -> Self {
        cuwf(cuwtf(Self::ch_max()) / cuwtf(2))
    }
    fn ch_zero() -> Self {
        T::zero_angle()
    }
    fn clamp(self) -> Self {
        self.wrap()
    }
}

impl<T: NumCast> NumCast for Deg<T> {
    fn from<U: ToPrimitive>(n: U) -> Option<Self> {
        T::from(n).map(Self)
    }
}

impl NumCast for Rad {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        n.to_f32().map(Self)
    }
}

impl<T: NumCast> NumCast for Rev<T> {
    fn from<U: ToPrimitive>(n: U) -> Option<Self> {
        T::from(n).map(Self)
    }
}

macro_rules! impl_to_prim_fns {
    () => (
        fn to_i64(&self) -> Option<i64>     { self.0.to_i64() }
        fn to_u64(&self) -> Option<u64>     { self.0.to_u64() }
        fn to_isize(&self) -> Option<isize> { self.0.to_isize() }
        fn to_i8(&self) -> Option<i8>       { self.0.to_i8() }
        fn to_i16(&self) -> Option<i16>     { self.0.to_i16() }
        fn to_i32(&self) -> Option<i32>     { self.0.to_i32() }
        #[cfg(has_i128)]
        fn to_i128(&self) -> Option<i128>   { self.0.to_i128() }
        fn to_usize(&self) -> Option<usize> { self.0.to_usize() }
        fn to_u8(&self) -> Option<u8>       { self.0.to_u8() }
        fn to_u16(&self) -> Option<u16>     { self.0.to_u16() }
        fn to_u32(&self) -> Option<u32>     { self.0.to_u32() }
        #[cfg(has_i128)]
        fn to_u128(&self) -> Option<u128>   { self.0.to_u128() }
        fn to_f32(&self) -> Option<f32>     { self.0.to_f32() }
        fn to_f64(&self) -> Option<f64>     { self.0.to_f64() }
    );
}

impl<T: ToPrimitive> ToPrimitive for Deg<T> {
    impl_to_prim_fns!();
}

impl ToPrimitive for Rad {
    impl_to_prim_fns!();
}

impl<T: ToPrimitive> ToPrimitive for Rev<T> {
    impl_to_prim_fns!();
}

macro_rules! generic_newtype_from_impls {
    ( $struct_name:ident, $( $num_t:ty ),* ) => { $(
        impl From<$num_t> for $struct_name<$num_t> {
            fn from(n: $num_t) -> Self { Self(n) }
        }

        impl From<$struct_name<$num_t>> for $num_t {
            fn from(angle: $struct_name<$num_t>) -> Self { angle.0 }
        }
    )* };
}

generic_newtype_from_impls!(Deg, i16, i32, f32);

impl From<f32> for Rad {
    fn from(n: f32) -> Self {
        Self(n)
    }
}

impl From<Rad> for f32 {
    fn from(angle: Rad) -> Self {
        angle.0
    }
}

generic_newtype_from_impls!(Rev, u8, u16, u32, f32);

macro_rules! impl_newtype_ops {
    ( $struct_name:ident;
      $( $trait:ident, $fun:ident, $as_trait:ident, $as_fun:ident );*
    ) => { $(
        impl<T> $trait for $struct_name<T>
            where T: $trait<Output=T>, Self: Angle
        {
            type Output = Self;
            fn $fun(self, rhs: Self) -> Self {
                (Self((self.0).$fun(rhs.0))).wrap()
            }
        }

        impl<T> $as_trait for $struct_name<T>
            where T: $as_trait, Self: Angle
        {
            fn $as_fun(&mut self, rhs: Self) {
                (self.0).$as_fun(rhs.0);
            }
        }
    )* };
}

macro_rules! impl_rad_ops {
    ( $struct_name:ident;
      $( $trait:ident, $fun:ident, $as_trait:ident, $as_fun:ident );*
    ) => { $(
        impl $trait for $struct_name
            where Self: Angle
        {
            type Output = Self;
            fn $fun(self, rhs: Self) -> Self {
                (Self((self.0).$fun(rhs.0))).wrap()
            }
        }

        impl $as_trait for $struct_name
            where Self: Angle
        {
            fn $as_fun(&mut self, rhs: Self) {
                (self.0).$as_fun(rhs.0);
            }
        }
    )* };
}

impl_newtype_ops!(Deg;
    Add, add, AddAssign, add_assign;
    Sub, sub, SubAssign, sub_assign;
    Mul, mul, MulAssign, mul_assign;
    Div, div, DivAssign, div_assign;
    Rem, rem, RemAssign, rem_assign
);

impl_rad_ops!(Rad;
    Add, add, AddAssign, add_assign;
    Sub, sub, SubAssign, sub_assign;
    Mul, mul, MulAssign, mul_assign;
    Div, div, DivAssign, div_assign;
    Rem, rem, RemAssign, rem_assign
);

impl_newtype_ops!(Rev;
    Add, add, AddAssign, add_assign;
    Sub, sub, SubAssign, sub_assign;
    Mul, mul, MulAssign, mul_assign;
    Div, div, DivAssign, div_assign;
    Rem, rem, RemAssign, rem_assign
);
