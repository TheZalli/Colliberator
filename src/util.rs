use std::ops::{Add, Sub, Mul, Div, Rem};

/// sRGB gamma value, used for sRGB decoding and encoding.
pub const GAMMA: f32 = 2.4;

macro_rules! wrapper_struct_conv_impls {
    ($outer:ty, $inner:ty) => {
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
pub fn gamma_encode(linear: f32) -> f32 {
    const SRGB_CUTOFF: f32 = 0.0031308;
    if linear <= SRGB_CUTOFF {
        linear * 12.92
    } else {
        linear.powf(1.0/GAMMA) * 1.055 - 0.055
    }
}

/// Gamma decodes an sRGB value into the linear space
pub fn gamma_decode(encoded: f32) -> f32 {
    const SRGB_INV_CUTOFF: f32 = 0.04045;
    if encoded <= SRGB_INV_CUTOFF {
        encoded / 12.92
    } else {
        ((encoded + 0.055)/1.055).powf(GAMMA)
    }
}

/// Percentage value that will always be in the range \[0.0, 1.0\].
///
/// Any value outside that will be saturated to fit within the range.
///
/// Trying to convert NaN or infinity floating point into `Portion` will cause a panic.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Portion(f32);

impl Portion {
    /// Inverts this value.
    pub fn inv(self) -> Self {
        Portion(1.0 - self.0)
    }

    /// Quantizates this value from the range 0.0 - 1.0 into range 0 - 255.
    pub fn quantizate_u8(self) -> u8 {
        (self.0 * 255.0) as u8
    }

    /// Quantizates this value from the range 0.0 - 1.0 into range 0 - 65535.
    pub fn quantizate_u16(self) -> u16 {
        (self.0 * u16::max_value() as f32) as u16
    }
}

/// Degrees that will always be in the range [0, 360).
///
/// Any value outside that is made to fit the range using modulo.
///
/// Trying to convert NaN and infinity floating point into `Deg` will cause a panic.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Deg(f32);

impl Deg {
    pub fn inv(self) -> Self {
        Deg(360.0 - self.0)
    }
}

impl From<f32> for Portion {
    fn from(p: f32) -> Self {
        if !p.is_finite() {
            panic!("`Portion`: Tried to convert NaN or infinite value into a percentage!")
        }

        Portion(clamp(p, 0.0, 1.0))
    }
}

impl Ord for Portion {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Portion {}

impl From<f32> for Deg {
    fn from(h: f32) -> Self {
        if !h.is_finite() { panic!("`Deg`: Tried to convert NaN or infinite value into degrees!") }

        let mut h = h % 360.0;
        if h < 0.0 {
            h = h + 360.0;
        }
        Deg(h)
    }
}

impl Ord for Deg {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Deg {}

wrapper_struct_conv_impls!(Portion, f32);
wrapper_struct_conv_impls!(Deg, f32);

wrapper_struct_impl_ops!(
    Portion, f32;
    Add, Sub, Mul, Div, Rem;
    add, sub, mul, div, rem
);

wrapper_struct_impl_ops!(
    Deg, f32;
    Add, Sub, Mul, Div, Rem;
    add, sub, mul, div, rem
);
