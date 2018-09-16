mod alpha;
mod rgb;
mod hsv;

pub use self::alpha::*;
pub use self::rgb::*;
pub use self::hsv::*;

use std::str;
use std::fmt;

use util::*;

/// Marker struct for the sRGB color space
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SRGBSpace;

/// Marker struct for the linear color space
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct LinearSpace;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
/// The basic colors of the rainbow
pub enum BaseColor {
    Black,
    Grey,
    White,
    Red,
    Yellow,
    Green,
    Cyan,
    Blue,
    Magenta,
}

impl BaseColor {
    #[inline]
    pub fn srgb24(&self) -> SRGB24Color {
        use self::BaseColor::*;

        let f = &SRGB24Color::new;
        match self {
            Black   => f(  0,   0,   0),
            Grey    => f(128, 128, 128),
            White   => f(255, 255, 255),
            Red     => f(255,   0,   0),
            Yellow  => f(255, 255,   0),
            Green   => f(  0, 255,   0),
            Cyan    => f(  0, 255, 255),
            Blue    => f(  0,   0, 255),
            Magenta => f(255,   0, 255),
        }
    }

    #[inline] pub fn srgb(&self) -> SRGBColor { self.srgb24().to_float() }
    #[inline] pub fn lin_rgb(&self) -> LinRGBColor { self.srgb().decode() }
    #[inline] pub fn lin_rgb24(&self) -> LinRGB48Color { self.lin_rgb().quantizate_u16() }

    #[inline]
    fn hsv(&self) -> HSVColor<SRGBSpace> {
        use self::BaseColor::*;

        let f = |h: f32, s: f32, v: f32| HSVColor::new(h.into(), s.into(), v.into());
        match self {
            Black   => f(  0.0, 0.0, 0.0),
            Grey    => f(  0.0, 0.0, 0.5),
            White   => f(  0.0, 0.0, 1.0),
            Red     => f(  0.0, 1.0, 1.0),
            Yellow  => f( 60.0, 1.0, 1.0),
            Green   => f(120.0, 1.0, 1.0),
            Cyan    => f(180.0, 1.0, 1.0),
            Blue    => f(240.0, 1.0, 1.0),
            Magenta => f(300.0, 1.0, 1.0),
        }
    }
}

impl fmt::Display for BaseColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::BaseColor::*;

        write!(f, "{}",
            match *self {
                Black   => "black",
                Grey    => "grey",
                White   => "white",
                Red     => "red",
                Yellow  => "yellow",
                Green   => "green",
                Cyan    => "cyan",
                Blue    => "blue",
                Magenta => "magenta",
            }
        )
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

    let (h, s, _) = color.hsv().to_tuple();

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

/// Returns the `text` with this color as it's background color using ANSI escapes.
pub fn ansi_bgcolor(color: SRGB24Color, text: &str) -> String {
    const CSI: &str = "\u{1B}[";
    let (r, g, b) = color.to_tuple();

    // color the text as black or white depending on the bg:s lightness
    let fg =
        if color.to_float().decode().relative_luminance() < std_gamma_decode(0.5).into() {
            format!("{}38;2;255;255;255m", CSI)
        } else {
            format!("{}38;2;;;m", CSI)
        };

    fg + &format!("{}48;2;{};{};{}m{}{0}0m", CSI, r, g, b, text)
}
