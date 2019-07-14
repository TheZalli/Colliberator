mod base_color;
mod alpha;
mod rgb;
mod hsv;

#[cfg(test)] mod test;

pub use self::alpha::*;
pub use base_color::*;
pub use rgb::*;
pub use hsv::*;

use std::str;

/// The sRGB gamma value, used for sRGB decoding and encoding
pub const STD_GAMMA: f32 = 2.4;

/// Marker struct for the sRGB color space
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SRGBSpace;

/// Marker struct for the linear color space
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct LinearSpace;

/// Trait for colors
pub trait Color: Sized {
    /// Normalizes this color
    ///
    /// All the channels should be made to fit their regular ranges and special cases (like HSV
    /// value channel being zero) should be unified.
    fn normalize(self) -> Self;

    /// Return true if this color is in it's colorspaces gamut
    ///
    /// If it's not, calling `normalize` should put it back.
    fn in_gamut(&self) -> bool;
}

/// Trait for colors that can be blended
pub trait Blend<FG> {
    type Output;

    /// Blend this color with another
    fn blend(&self, other: &FG) -> Self::Output;
}

pub type SRGBColor = RGBColor<f32, SRGBSpace>;
pub type SRGB24Color = RGBColor<u8, SRGBSpace>;

pub type LinRGBColor = RGBColor<f32, LinearSpace>;
pub type LinRGB48Color = RGBColor<u16, LinearSpace>;

pub type SRGBAColor = Alpha<RGBColor<f32, SRGBSpace>, f32>;
pub type SRGBA24Color = Alpha<RGBColor<u8, SRGBSpace>, u8>;

pub type LinRGBAColor = Alpha<RGBColor<f32, LinearSpace>, f32>;
pub type LinRGBA48Color = Alpha<RGBColor<u16, LinearSpace>, u16>;

/// Gamma encode a linear value into the sRGB space
pub fn std_gamma_encode(linear: f32) -> f32 {
    const SRGB_CUTOFF: f32 = 0.0031308;
    if linear <= SRGB_CUTOFF {
        linear * 12.92
    } else {
        linear.powf(1.0/ STD_GAMMA) * 1.055 - 0.055
    }
}

/// Gamma decode an sRGB value into the linear space
pub fn std_gamma_decode(encoded: f32) -> f32 {
    const SRGB_INV_CUTOFF: f32 = 0.04045;
    if encoded <= SRGB_INV_CUTOFF {
        encoded / 12.92
    } else {
        ((encoded + 0.055)/1.055).powf(STD_GAMMA)
    }
}

/// Categorize this color's most prominent shades
pub fn shades(color: SRGBColor) -> Vec<(BaseColor, f32)> {
    use self::BaseColor::*;

    const COLOR_HUES: [(f32, BaseColor); 5] =
        [(60.0, Yellow),
         (120.0, Green),
         (180.0, Cyan),
         (240.0, Blue),
         (300.0, Magenta)];

    // all of these borders have been picked by what looks nice
    // they could be improved

    // how many degrees from the main hue can a shade be
    const HUE_MARGIN: f32 = 60.0 * 0.75;

    // relative luminance under this value is considered to be just black
    const BLACK_CUTOFF_LUMINANCE: f32 = 0.005;

    // saturation under this value is considered to be just greyscale without any color
    const GREYSCALE_SATURATION: f32 = 0.05;

    // borders for the greyscale shades
    const WHITE_SATURATION: f32 = 0.35;
    const WHITE_LUMINANCE: f32 = 0.40;

    const GREY_SATURATION: f32 = 0.45;
    const GREY_LUMINANCE_MAX: f32 = 0.80;
    const GREY_LUMINANCE_MIN: f32 = 0.03;

    const BLACK_LUMINANCE: f32 = 0.045;

    let mut shades = Vec::with_capacity(3);

    let (h, s, _) = color.hsv().into_tuple();

    let lum: f32 = color.decode().relative_luminance().into();

    if lum < BLACK_CUTOFF_LUMINANCE {
        return vec![(Black, 1.0)];
    }

    let mut sum = 0.0;

    if s > GREYSCALE_SATURATION {
        // red is a special case
        if h >= 360.0 - HUE_MARGIN || h <= 0.0 + HUE_MARGIN {
            let amount = 1.0 -
                if h <= 0.0 + HUE_MARGIN {
                    h
                } else {
                    h - 360.0
                } / HUE_MARGIN;

            sum += amount;
            shades.push((Red, amount));
        }
        for (hue, color) in COLOR_HUES.iter() {
            let dist = (h - hue).abs();
            if dist <= HUE_MARGIN {
                let amount = 1.0 - dist / HUE_MARGIN;
                sum += amount;
                shades.push((*color, amount));
            }
        }
    }

    if lum <= BLACK_LUMINANCE {
        sum += 1.0;
        shades.push((Black, 1.0));
    } else if lum >= WHITE_LUMINANCE && s <= WHITE_SATURATION {
        //let amount = 1.0 - (WHITE_SATURATION - s) / WHITE_SATURATION;
        sum += 1.0;
        shades.push((White, 1.0));
    }

    if s <= GREY_SATURATION && lum <= GREY_LUMINANCE_MAX && lum >= GREY_LUMINANCE_MIN {
        //let amount = 1.0 - (GREY_SATURATION - s) / GREY_SATURATION;
        sum += 1.0;
        shades.push((Grey, 1.0));
    }
    // sort and normalize
    shades.sort_unstable_by(
        |(_, amount), (_, amount2)| amount2.partial_cmp(amount).unwrap()
    );

    return shades.iter_mut().map(|(color, amount)| (*color, *amount/sum)).collect();
}

/// Return the `text` with this color as it's background color using ANSI escapes
pub fn ansi_bgcolor(color: SRGB24Color, text: &str) -> String {
    const CSI: &str = "\u{1B}[";
    let (r, g, b) = color.into_tuple();

    // color the text as black or white depending on the bg:s lightness
    let fg =
        if color.into_float().decode().relative_luminance() < std_gamma_decode(0.5).into() {
            format!("{}38;2;255;255;255m", CSI)
        } else {
            format!("{}38;2;;;m", CSI)
        };

    fg + &format!("{}48;2;{};{};{}m{}{0}0m", CSI, r, g, b, text)
}
