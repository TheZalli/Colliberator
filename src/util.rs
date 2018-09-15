/// sRGB gamma value, used for sRGB decoding and encoding.
pub const GAMMA: f32 = 2.4;

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
