mod srgb;
mod linrgb;
mod hsv;

pub use self::srgb::*;
pub use self::linrgb::*;
pub use self::hsv::*;

use std::str;
use std::fmt;
use std::marker::PhantomData;

use util::*;

macro_rules! impl_triple_froms {
    ($name:ident, $cha_ty:ty, $x:ident, $y:ident, $z:ident) => {
        impl From<($cha_ty, $cha_ty, $cha_ty)> for $name {
            fn from(arg: ($cha_ty, $cha_ty, $cha_ty)) -> Self {
                $name::new(arg.0, arg.1, arg.2)
            }
        }

        impl From<[$cha_ty; 3]> for $name {
            fn from(arg: [$cha_ty; 3]) -> Self {
                $name::new(arg[0], arg[1], arg[2])
            }
        }

        impl From<$name> for ($cha_ty, $cha_ty, $cha_ty) {
            fn from(color: $name) -> Self {
                (color.$x, color.$y, color.$z)
            }
        }

        impl From<$name> for [$cha_ty; 3] {
            fn from(color: $name) -> Self {
                [color.$x, color.$y, color.$z]
            }
        }

        impl<'a> From<&'a $name> for ($cha_ty, $cha_ty, $cha_ty) {
            fn from(color: & $name) -> Self { (*color).into() }
        }

        impl<'a> From<&'a $name> for [$cha_ty; 3] {
            fn from(color: & $name) -> Self { (*color).into() }
        }
    };
}

impl_triple_froms!(SRGBColor, f32, r, g, b);
impl_triple_froms!(SRGB24Color, u8, r, g, b);
impl_triple_froms!(LinRGBColor, f32, r, g, b);
impl_triple_froms!(LinRGB48Color, u16, r, g, b);
impl_triple_froms!(HSVColor, f32, h, s, v);

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

pub trait Color {
    /// Returns this color in the normalized sRGB color space
    fn srgb(&self) -> SRGBColor;

    /// Returns this color in the 24-bit sRGB color space
    fn srgb24(&self) -> SRGB24Color { self.srgb().srgb24() }

    /// Return the normalised RGB representation in the linear color space
    fn lin_rgb(&self) -> LinRGBColor { self.srgb().lin_rgb() }

    /// Return the 48-bit RGB representation in the linear color space
    fn lin_rgb48(&self) -> LinRGB48Color { self.lin_rgb().lin_rgb48() }

    /// Return the HSV representation
    fn hsv(&self) -> HSVColor { self.srgb().hsv() }

    /// Blends this color with another using the given ratio.
    ///
    /// Blends in the linear RGB space.
    ///
    /// Ratio of 0.5 means both colors are used equally.
    /// Ratio of 1.0 means only `self` is used, while ratio of 0.0 means only `other` is used.
    /// If ratio is outside 0.0 - 1.0, this function is undefined behaviour.
    fn blend<T: Color>(&self, other: &T, ratio: f32) -> LinRGBColor {
        self.lin_rgb() * ratio + other.lin_rgb() * (1.0 - ratio)
    }

    /// Returns the relative luminance of this color between 0 and 1.
    ///
    /// Tells the whiteness of the color as perceived by humans.
    /// Values nearer 0 are darker, and values nearer 1 are lighter.
    fn relative_luminance(&self) -> f32 {
        let (r, g, b) = self.lin_rgb().into();
        0.2126*r + 0.7152*g + 0.0722*b
    }

    /// Same as `relative_luminance`, but the values are gamma compressed.
    ///
    /// Uses the same gamma encoding function as linear RGB to sRGB conversion.
    fn gamma_relative_luminance(&self) -> f32 {
        gamma_encode(self.relative_luminance())
    }

    /// Categorize this color's most prominent shades
    fn shades(&self) ->  Vec<(BaseColor, f32)> {
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

        let (h, s, _v) = self.hsv().to_tuple();
        let lum = self.relative_luminance();

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
    fn ansi_bgcolor(&self, text: &str) -> String {
        const CSI: &str = "\u{1B}[";
        let (r, g, b) = self.srgb24().into();

        // color the text as black or white depending on the bg:s lightness
        let fg =
            if self.relative_luminance() < gamma_decode(0.5) {
                format!("{}38;2;255;255;255m", CSI)
            } else {
                format!("{}38;2;;;m", CSI)
            };

        fg + &format!("{}48;2;{};{};{}m{}{0}0m", CSI, r, g, b, text)
    }
}

impl Color for BaseColor {
    fn srgb(&self) -> SRGBColor { self.srgb24().srgb() }

    fn srgb24(&self) -> SRGB24Color {
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

    fn hsv(&self) -> HSVColor {
        use self::BaseColor::*;

        let f = &HSVColor::new;
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

/// Marker struct for the sRGB color space
pub struct SRGBSpace;

/// Marker struct for the linear color space
pub struct LinearSpace;

/// An RGB color in the `S` color space.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct RGBColor<T, S> {
    pub r: T,
    pub g: T,
    pub b: T,
    _space: PhantomData<S>
}

impl<T, S> RGBColor<T, S> {
    pub fn new(r: T, g: T, b: T) -> Self {
        RGBColor { r, g, b, _space: PhantomData }
    }

    /// Applies the given function to all color channels.
    pub fn map<U: Clone, F: Fn(T) -> U>(self, fun: F) -> RGBColor<U, S> {
        let (r, g, b) = self.to_tuple();
        (fun(r), fun(g), fun(b)).into()
    }

    pub fn to_tuple(self) -> (T, T, T) {
        (self.r, self.g, self.b)
    }

    pub fn to_array(self) -> [T; 3] {
        [self.r, self.g, self.b]
    }
}

impl<S> RGBColor<u8, S> {
    /// Converts this channel into a floating point channel from range 0.0 - 1.0 .
    pub fn portions(self) -> RGBColor<Portion, S> {
        self.map(|x| (x as f32 / 255.0).into())
    }
}

impl<S> RGBColor<u16, S> {
    /// Converts this channel into a floating point channel from range 0.0 - 1.0 .
    pub fn portions(self) -> RGBColor<Portion, S> {
        self.map(|x| (x as f32 / u16::max_value() as f32).into())
    }
}

impl<S> RGBColor<Portion, S> {
    /// Quantizates this value from the range 0.0 - 1.0 into range 0 - 255.
    pub fn quantizate_u8(self) -> RGBColor<u8, S> {
        self.map(&Portion::quantizate_u8)
    }

    /// Quantizates this value from the range 0.0 - 1.0 into range 0 - 65535.
    pub fn quantizate_u16(self) -> RGBColor<u16, S> {
        self.map(&Portion::quantizate_u16)
    }
}

impl RGBColor<Portion, SRGBSpace> {
    /// Gamma decodes this color channel value into the linear color space
    pub fn decode(self) -> RGBColor<Portion, LinearSpace> {
        self.map(|x| gamma_decode(x.into()).into()).to_tuple().into()
    }
}

impl RGBColor<Portion, LinearSpace> {
    /// Gamma encodes this color channel value into the sRGB color space
    pub fn encode(self) -> RGBColor<Portion, SRGBSpace> {
        self.map(|x| gamma_encode(x.into()).into()).to_tuple().into()
    }
}

impl<T: Clone, S> From<(T, T, T)> for RGBColor<T, S> {
    fn from(tuple: (T, T, T)) -> Self {
        let tuple = tuple.clone();
        RGBColor::new(tuple.0, tuple.1, tuple.2)
    }
}

impl<T: Clone, S> From<[T; 3]> for RGBColor<T, S> {
    fn from(array: [T; 3]) -> Self {
        RGBColor::new(array[0].clone(), array[1].clone(), array[2].clone())
    }
}
