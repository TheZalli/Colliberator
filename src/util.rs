use std::ops::{Add, Sub, Mul, Div, Rem};
use std::fmt;

/// sRGB gamma value, used for sRGB decoding and encoding.
pub const GAMMA: f32 = 2.4;

macro_rules! wrapper_struct_conv_impls {
    ($outer:ident, $inner:ty) => {
        impl From<$inner> for $outer {
            fn from(arg: $inner) -> Self {
                $outer :: new(arg)
            }
        }

        impl From<$outer> for $inner {
            fn from(arg: $outer) -> Self {
                arg.0
            }
        }

        impl AsRef<$inner> for $outer {
            fn as_ref(&self) -> &$inner {
                &self.0
            }
        }

        impl AsMut<$inner> for $outer {
            fn as_mut(&mut self) -> &mut $inner {
                &mut self.0
            }
        }
    };
}

macro_rules! wrapper_struct_impl_ops {
    ($outer:ty, $inner:ty ; $( $op_trait:ident),+ ; $( $op_fun:ident),+ ) => { $(
        impl $op_trait for $outer {
            type Output = $outer;
            fn $op_fun(self, rhs: Self) -> Self::Output {
                $op_trait :: $op_fun(self.0, rhs.0).into()
            }
        }

        impl $op_trait<$inner> for $outer {
            type Output = $outer;
            fn $op_fun(self, rhs: $inner) -> Self::Output {
                $op_trait :: $op_fun(self.0, rhs).into()
            }
        }

        impl $op_trait<$outer> for $inner {
            type Output = $outer;
            fn $op_fun(self, rhs: $outer) -> Self::Output {
                $op_trait :: $op_fun(self, rhs.0).into()
            }
        }
    )+ };
}

/// Clamps the given value into the inclusive range between the given minimum and maximum.
///
/// If no comparison can be made and the function `PartialOrd::partial_cmp` returns `None`, then
/// this function returns the minimum value.
///
/// If the original value is equal to minimum or maximum, the minimum or maximum value is returned
/// respectively.
#[inline]
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    use std::cmp::Ordering::*;
    match value.partial_cmp(&max) {
        Some(Less) =>
            match value.partial_cmp(&min) {
                Some(Greater) => value,
                _ => min,
            },
        _ => max
    }
}

/// Gamma encodes a linear value into the sRGB space
pub fn std_gamma_encode(linear: f32) -> f32 {
    const SRGB_CUTOFF: f32 = 0.0031308;
    if linear <= SRGB_CUTOFF {
        linear * 12.92
    } else {
        linear.powf(1.0/GAMMA) * 1.055 - 0.055
    }
}

/// Gamma decodes an sRGB value into the linear space
pub fn std_gamma_decode(encoded: f32) -> f32 {
    const SRGB_INV_CUTOFF: f32 = 0.04045;
    if encoded <= SRGB_INV_CUTOFF {
        encoded / 12.92
    } else {
        ((encoded + 0.055)/1.055).powf(GAMMA)
    }
}
