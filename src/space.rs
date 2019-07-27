//! Colorspaces and conversions between them

use num_traits::Float;
use crate::cuw;

/// The sRGB gamma value, used for sRGB decoding and encoding
pub const STD_GAMMA: f32 = 2.4;

/// Marker struct for the sRGB color space
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SRGBSpace;

/// Marker struct for the linear color space
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct LinearSpace;

/// Gamma encode a linear color channel into the sRGB space
pub fn std_gamma_encode<T: Float>(linear: T) -> T {
    const SRGB_CUTOFF: f32 = 0.0031308;
    if linear <= cuw(SRGB_CUTOFF) {
        linear * cuw(12.92)
    } else {
        linear.powf(cuw(1.0 / STD_GAMMA)) * cuw(1.055) - cuw(0.055)
    }
}

/// Gamma decode an sRGB color channel into the linear space
pub fn std_gamma_decode<T: Float>(encoded: T) -> T {
    const SRGB_INV_CUTOFF: f32 = 0.04045;
    if encoded <= cuw(SRGB_INV_CUTOFF) {
        encoded / cuw(12.92)
    } else {
        ((encoded + cuw(0.055)) / cuw(1.055)).powf(cuw(STD_GAMMA))
    }
}
