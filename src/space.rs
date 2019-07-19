//! Colorspaces and conversions between them

/// The sRGB gamma value, used for sRGB decoding and encoding
pub const STD_GAMMA: f32 = 2.4;

/// Marker struct for the sRGB color space
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SRGBSpace;

/// Marker struct for the linear color space
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct LinearSpace;

/// Gamma encode a linear color channel into the sRGB space
pub fn std_gamma_encode(linear: f32) -> f32 {
    const SRGB_CUTOFF: f32 = 0.0031308;
    if linear <= SRGB_CUTOFF {
        linear * 12.92
    } else {
        linear.powf(1.0/ STD_GAMMA) * 1.055 - 0.055
    }
}

/// Gamma decode an sRGB color channel into the linear space
pub fn std_gamma_decode(encoded: f32) -> f32 {
    const SRGB_INV_CUTOFF: f32 = 0.04045;
    if encoded <= SRGB_INV_CUTOFF {
        encoded / 12.92
    } else {
        ((encoded + 0.055)/1.055).powf(STD_GAMMA)
    }
}
