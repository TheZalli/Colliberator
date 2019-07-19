use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div};

use super::*;

/// An RGB color
///
/// `S` is this color's colorspace.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct RGBColor<T, S> {
    pub r: T,
    pub g: T,
    pub b: T,
    _space: PhantomData<S>
}

impl<T: Channel, S> RGBColor<T, S>  {
    /// Creates a new RGB-color with the given values
    ///
    /// They are clamped to the allowed color channel range
    pub fn new(r: T, g: T, b: T) -> Self {
        RGBColor { r, g, b, _space: PhantomData }.normalize()
    }
}

impl<T: Channel, S> Color for RGBColor<T, S> {
     #[inline]
    fn normalize(self) -> Self {
        self.map(Channel::to_range)
    }

    #[inline]
    fn is_normal(&self) -> bool {
        self.r.in_range() && self.g.in_range() && self.b.in_range()
    }
}

impl<T, S> RGBColor<T, S> {
    /// Applies the given function to all color channels.
    #[inline]
    pub fn map<U, F: Fn(T) -> U>(self, fun: F) -> RGBColor<U, S> {
        RGBColor { r: fun(self.r), g: fun(self.g), b: fun(self.b), _space: PhantomData }
    }

    #[inline]
    pub fn tuple(self) -> (T, T, T) {
        (self.r, self.g, self.b)
    }

    #[inline]
    pub fn array(self) -> [T; 3] {
        [self.r, self.g, self.b]
    }
}

impl<S> RGBColor<u8, S> {
    /// Converts this channel into a floating point channel with range 0.0 - 1.0 .
    #[inline]
    pub fn float(self) -> RGBColor<f32, S> {
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
    #[inline]
    pub fn float(self) -> RGBColor<f32, S> {
        self.map(|x| (x as f32 / u16::max_value() as f32).into())
    }
}

impl<S> RGBColor<f32, S> {
    /// Quantizates this value from the range 0.0 - 1.0 into range 0 - 255.
    #[inline]
    pub fn uint8(self) -> RGBColor<u8, S> {
        self.map(|x| (x * 255.0).round() as u8)
    }

    /// Quantizates this value from the range 0.0 - 1.0 into range 0 - 65535.
    #[inline]
    pub fn uint16(self) -> RGBColor<u16, S> {
        self.map(|x| (x * u16::max_value() as f32).round() as u16)
    }

    pub fn hsv(self) -> HSVColor<S> {
        let (r, g, b) = self.tuple();

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

        HSVColor::new(hue, saturation, value)
    }
}

impl RGBColor<f32, SRGBSpace> {
    /// Gamma decodes this color channel value into the linear color space
    #[inline]
    pub fn std_decode(self) -> RGBColor<f32, LinearSpace> {
        self.map(&std_gamma_decode).tuple().into()
    }
}

impl RGBColor<f32, LinearSpace> {
    /// Gamma encodes this color channel value into the sRGB color space
    #[inline]
    pub fn std_encode(self) -> RGBColor<f32, SRGBSpace> {
        self.map(&std_gamma_encode).tuple().into()
    }

    /// Returns the relative luminance of this color between 0 and 1.
    ///
    /// Tells the whiteness of the color as perceived by humans.
    /// Values nearer 0 are darker, and values nearer 1 are lighter.
    ///
    /// The returned values are linear, so to get perceptually uniform luminance, use
    /// `gamma_encode`.
    #[inline]
    pub fn relative_luminance(&self) -> f32 {
        let (r, g, b) = self.tuple();
        0.2126*r + 0.7152*g + 0.0722*b
    }
}

impl<T: Channel, S> Default for RGBColor<T, S> {
    fn default() -> Self {
        RGBColor::new(T::ch_min(), T::ch_min(), T::ch_min())
    }
}

impl<T: Channel, S> From<(T, T, T)> for RGBColor<T, S> {
    fn from(tuple: (T, T, T)) -> Self {
        let (r, g, b) = tuple;
        RGBColor::new(r, g, b)
    }
}

impl<T: Clone + Channel, S> From<[T; 3]> for RGBColor<T, S> {
    fn from(array: [T; 3]) -> Self {
        RGBColor::new(array[0].clone(), array[1].clone(), array[2].clone())
    }
}

impl<T: Clone + Channel, S> From<&[T; 3]> for RGBColor<T, S> {
    fn from(array: &[T; 3]) -> Self {
        RGBColor::new(array[0].clone(), array[1].clone(), array[2].clone())
    }
}

impl<T> Add for RGBColor<T, LinearSpace>
    where T: Channel + Add<Output=T>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        (self.r + rhs.r, self.g + rhs.g, self.b + rhs.b).into()
    }
}

impl<T> Sub for RGBColor<T, LinearSpace>
    where T: Channel + Sub<Output=T>
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        (self.r - rhs.r, self.g - rhs.g, self.b - rhs.b).into()
    }
}

impl<T> Mul for RGBColor<T, LinearSpace>
    where T: Channel + Mul<Output=T>
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        (self.r * rhs.r, self.g * rhs.g, self.b * rhs.b).into()
    }
}

impl<T> Div for RGBColor<T, LinearSpace>
    where T: Channel + Div<Output=T>
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        (self.r / rhs.r, self.g / rhs.g, self.b / rhs.b).into()
    }
}

impl<T> Mul<T> for RGBColor<T, LinearSpace>
    where T: Channel + Mul<Output=T> + Clone
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        (self.r * rhs.clone(), self.g * rhs.clone(), self.b * rhs).into()
    }
}

impl<T> Div<T> for RGBColor<T, LinearSpace>
    where T: Channel + Div<Output=T> + Clone
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        (self.r / rhs.clone(), self.g / rhs.clone(), self.b / rhs).into()
    }
}

impl<S> fmt::Display for RGBColor<u8, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:3}, {:3}, {:3}", self.r, self.g, self.b)
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

impl<S> fmt::UpperHex for RGBColor<u8, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl<S> fmt::LowerHex for RGBColor<u8, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}
