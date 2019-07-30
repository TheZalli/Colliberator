mod channel;
mod base;
mod alpha;
mod rgb;
mod hsv;
mod blend;
mod iter;

pub mod space;

#[cfg(test)] mod test;

use std::str;

use num_traits::NumCast;

pub use channel::*;
pub use base::*;
pub use self::alpha::*;
pub use rgb::*;
pub use hsv::*;
pub use blend::*;
pub use iter::*;

use angle::*;
use space::{LinearSpace, SRGBSpace, std_gamma_decode, std_gamma_encode};

/// A trait for colors
pub trait Color: Sized {
    /// Normalizes this color
    ///
    /// All the channels should be made to fit their regular ranges and special cases (like HSV
    /// value channel being zero) should be unified.
    fn normalize(self) -> Self;

    /// Return true if this color is normalized
    ///
    /// If it's not, calling `normalize` should put it back.
    fn is_normal(&self) -> bool;
}

/// A 96-bit sRGB color with 32-bit floating point channels
pub type SRGBColor = RGBColor<f32, SRGBSpace>;
/// A 24-bit sRGB color with 8-bit integer channels
pub type SRGB24Color = RGBColor<u8, SRGBSpace>;

/// A 96-bit linear RGB color with 32-bit floating point channels
pub type LinRGBColor = RGBColor<f32, LinearSpace>;
/// A 48-bit linear RGB color with 16-bit integer channels
pub type LinRGB48Color = RGBColor<u16, LinearSpace>;

/// A 128-bit sRGBA color with 32-bit floating point channels
pub type SRGBAColor = Alpha<RGBColor<f32, SRGBSpace>, f32>;
/// A 32-bit sRGBA color with 8-bit integer channels
pub type SRGBA32Color = Alpha<RGBColor<u8, SRGBSpace>, u8>;

/// A 128-bit linear RGBA color with 32-bit floating point channels
pub type LinRGBAColor = Alpha<RGBColor<f32, LinearSpace>, f32>;
/// A 64-bit linear RGBA color with 16-bit integer channels
pub type LinRGBA64Color = Alpha<RGBColor<u16, LinearSpace>, u16>;

/// A 128-bit HSV color in sRGB colorspace with 32-bit floating point channels
///
/// The hue channel is in degrees in the range [0, 360).
pub type StdHSVColor = HSVColor<Deg<f32>, f32, SRGBSpace>;
/// A 128-bit HSV color in linear colorspace with 32-bit floating point channels
///
/// The hue channel is in degrees in the range [0, 360).
pub type LinHSVColor = HSVColor<Deg<f32>, f32, LinearSpace>;

/// Classify this color's most prominent shades
pub fn shades(color: SRGBColor) -> Vec<(BaseColor, f32)> {
    use self::BaseColor::*;

    const COLOR_HUES: [(f32, BaseColor); 5] =
        [(60.0, Yellow),
         (120.0, Green),
         (180.0, Cyan),
         (240.0, Blue),
         (300.0, Magenta)];

    // these values below have been picked by what gives nice results
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

    let (h, s, _) = color.hsv::<Deg<f32>>().tuple();
    let h = h.0;

    let lum: f32 = color.std_decode().relative_luminance().into();

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
///
/// The text itself will be colored white or black, depending on the relative
/// luminance (or "whiteness") of the color.
pub fn ansi_bgcolor(color: SRGB24Color, text: &str) -> String {
    const CSI: &str = "\u{1B}[";
    let (r, g, b) = color.tuple();

    // color the text as black or white depending on the bg:s lightness
    let fg =
        if color.conv::<f32>().std_decode().relative_luminance() < std_gamma_decode(0.5) {
            format!("{}38;2;255;255;255m", CSI)
        } else {
            format!("{}38;2;;;m", CSI)
        };

    fg + &format!("{}48;2;{};{};{}m{}{0}0m", CSI, r, g, b, text)
}

#[inline]
fn cuw<T: NumCast, U: NumCast>(n: T) -> U { U::from(n).unwrap() }

#[inline]
fn cuwf<T: NumCast>(float: f32) -> T { T::from(float).unwrap() }

#[inline]
fn cuwtf<T: NumCast>(n: T) -> f32 { n.to_f32().unwrap() }
