mod hsv;

pub use self::hsv::*;

use std::str;
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div};

use util::*;

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

impl BaseColor {
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
    let (h, s): (f32, f32) = (h.into(), s.into());

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
        if color.to_float().decode().relative_luminance() < gamma_decode(0.5).into() {
            format!("{}38;2;255;255;255m", CSI)
        } else {
            format!("{}38;2;;;m", CSI)
        };

    fg + &format!("{}48;2;{};{};{}m{}{0}0m", CSI, r, g, b, text)
}


/// Marker struct for the sRGB color space
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SRGBSpace;

/// Marker struct for the linear color space
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct LinearSpace;

/// An RGB color in the `S` color space.
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct RGBColor<T, S> {
    pub r: T,
    pub g: T,
    pub b: T,
    _space: PhantomData<S>
}

pub type SRGBColor = RGBColor<f32, SRGBSpace>;
pub type SRGB24Color = RGBColor<u8, SRGBSpace>;

pub type LinRGBColor = RGBColor<f32, LinearSpace>;
pub type LinRGB48Color = RGBColor<u16, LinearSpace>;

impl<T, S> RGBColor<T, S> {
    pub fn new(r: T, g: T, b: T) -> Self {
        RGBColor { r, g, b, _space: PhantomData }
    }

    /// Applies the given function to all color channels.
    pub fn map<U, F: Fn(T) -> U>(self, fun: F) -> RGBColor<U, S> {
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
    /// Converts this channel into a floating point channel with range 0.0 - 1.0 .
    pub fn to_float(self) -> RGBColor<f32, S> {
        self.map(|x| (x as f32 / 255.0).into())
    }

    /// Create 24-bit RGB color from a 6-character hexcode.
    ///
    /// # Safety
    /// If `hex_str` is not a valid utf-8 string then this function will result in undefined
    /// behaviour.
    ///
    /// If `hex_str` doesn't consist only from 6 characters from range `[0-9a-fA-F]` then this
    /// function will result in a panic.
    pub unsafe fn from_hex_unchecked(hex_str: Box<str>) -> Self {
        let f = |h1: u8, h2: u8|
            u8::from_str_radix(str::from_utf8_unchecked(&[h1, h2]), 16).unwrap();

        let mut hex_str = hex_str;
        let h = hex_str.as_bytes_mut();
        h.make_ascii_lowercase();

        (f(h[0], h[1]), f(h[2], h[3]), f(h[4], h[5])).into()
    }
}

impl<S> RGBColor<u16, S> {
    /// Converts this channel into a floating point channel from range 0.0 - 1.0 .
    pub fn to_float(self) -> RGBColor<f32, S> {
        self.map(|x| (x as f32 / u16::max_value() as f32).into())
    }
}

impl<S> RGBColor<f32, S> {
    /// Quantizates this value from the range 0.0 - 1.0 into range 0 - 255.
    pub fn quantizate_u8(self) -> RGBColor<u8, S> {
        self.map(|x| (x * 255.0) as u8)
    }

    /// Quantizates this value from the range 0.0 - 1.0 into range 0 - 65535.
    pub fn quantizate_u16(self) -> RGBColor<u16, S> {
        self.map(|x| (x * u16::max_value() as f32) as u16)
    }

    pub fn hsv(self) -> HSVColor<S> {
        let (r, g, b) = self.to_tuple();

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let value = max;
        let saturation = if max == 0.0 { 0.0 } else { delta / max };
        let hue = 60.0 *
            if delta == 0.0 {
                0.0
            } else if max == r {
                ((g - b) / delta) % 6.0
            } else if max == g {
                (b - r) / delta + 2.0
            } else {
                (r - g) / delta + 4.0
            };

        HSVColor::new(hue.into(), saturation, value)
    }
}

impl RGBColor<f32, SRGBSpace> {
    /// Gamma decodes this color channel value into the linear color space
    pub fn decode(self) -> RGBColor<f32, LinearSpace> {
        self.map(&gamma_decode).to_tuple().into()
    }
}

impl RGBColor<f32, LinearSpace> {
    /// Gamma encodes this color channel value into the sRGB color space
    pub fn encode(self) -> RGBColor<f32, SRGBSpace> {
        self.map(&gamma_encode).to_tuple().into()
    }

    /// Blends this color with another using the given ratio.
    ///
    /// Blends in the linear RGB space.
    ///
    /// Ratio of 0.5 means both colors are used equally.
    /// Ratio of 1.0 means only `self` is used, while ratio of 0.0 means only `other` is used.
    /// If ratio is outside 0.0 - 1.0, this function is undefined behaviour.
    fn blend(self, other: Self, ratio: f32) -> Self {
        self * ratio + other * (1.0-ratio)
    }

    /// Returns the relative luminance of this color between 0 and 1.
    ///
    /// Tells the whiteness of the color as perceived by humans.
    /// Values nearer 0 are darker, and values nearer 1 are lighter.
    ///
    /// The returned values are linear, so to get perceptually uniform luminance, use
    /// `gamma_encode`.
    pub fn relative_luminance(&self) -> f32 {
        let (r, g, b) = self.to_tuple();
        0.2126*r + 0.7152*g + 0.0722*b
    }
}

impl<T, U, S> From<(T, T, T)> for RGBColor<U, S>
    where U: From<T>
{
    fn from(tuple: (T, T, T)) -> Self {
        let (r, g, b) = tuple;
        RGBColor { r: r.into(), g: g.into(), b: b.into(), _space: PhantomData}
    }
}

impl<T: Clone, S> From<[T; 3]> for RGBColor<T, S>
{
    fn from(array: [T; 3]) -> Self {
        let array = array;
        RGBColor::new(array[0].clone(), array[1].clone(), array[2].clone())
    }
}

impl<T> Add for RGBColor<T, LinearSpace>
    where T: Add<Output=T>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        (self.r + rhs.r, self.g + rhs.g, self.b + rhs.b).into()
    }
}

impl<T> Sub for RGBColor<T, LinearSpace>
    where T: Sub<Output=T>
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        (self.r - rhs.r, self.g - rhs.g, self.b - rhs.b).into()
    }
}

impl<T> Mul for RGBColor<T, LinearSpace>
    where T: Mul<Output=T>
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        (self.r * rhs.r, self.g * rhs.g, self.b * rhs.b).into()
    }
}

impl<T> Div for RGBColor<T, LinearSpace>
    where T: Div<Output=T>
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        (self.r / rhs.r, self.g / rhs.g, self.b / rhs.b).into()
    }
}

impl<T> Mul<T> for RGBColor<T, LinearSpace>
    where T: Mul<Output=T> + Clone
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        (self.r * rhs.clone(), self.g * rhs.clone(), self.b * rhs).into()
    }
}

impl<T> Div<T> for RGBColor<T, LinearSpace>
    where T: Div<Output=T> + Clone
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        (self.r / rhs.clone(), self.g / rhs.clone(), self.b / rhs).into()
    }
}

impl<S> fmt::Display for RGBColor<u8, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:3},{:3},{:3}", self.r, self.g, self.b)
    }
}

impl<S> fmt::Display for RGBColor<u16, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:5},{:5},{:5}", self.r, self.g, self.b)
    }
}

impl<S> fmt::Display for RGBColor<f32, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:5.1},{:5.1},{:5.1}", self.r, self.g, self.b)
    }
}
