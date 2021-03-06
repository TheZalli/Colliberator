use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

use num_traits::Float;

use crate::*;

/// An RGB color
///
/// `T` is the type of this color's channels, and `S` is this color's colorspace.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct RGBColor<T, S> {
    pub r: T,
    pub g: T,
    pub b: T,
    _space: PhantomData<S>,
}

impl<T, S> RGBColor<T, S> {
    /// Applies the given function to all color channels.
    #[inline]
    pub fn map<U, F: Fn(T) -> U>(self, fun: F) -> RGBColor<U, S> {
        RGBColor {
            r: fun(self.r),
            g: fun(self.g),
            b: fun(self.b),
            _space: PhantomData,
        }
    }

    /// Deconstructs this color into a tuple of it's channels
    #[inline]
    pub fn tuple(self) -> (T, T, T) {
        (self.r, self.g, self.b)
    }

    /// Deconstructs this color into an array of it's channels
    #[inline]
    pub fn array(self) -> [T; 3] {
        [self.r, self.g, self.b]
    }
}

impl<T: Channel, S> RGBColor<T, S> {
    /// Creates a new RGB-color with the given values
    ///
    /// They are clamped to the allowed color channel range.
    pub fn new(r: T, g: T, b: T) -> Self {
        RGBColor {
            r,
            g,
            b,
            _space: PhantomData,
        }
        .map(Channel::clamp)
    }

    /// Converts the channels of this color into another type
    #[inline]
    pub fn conv<U: Channel>(self) -> RGBColor<U, S> {
        self.map(Channel::conv)
    }
}

impl<T: Channel, S> Color for RGBColor<T, S> {
    #[inline]
    fn normalize(self) -> Self {
        self.map(Channel::clamp)
    }

    #[inline]
    fn is_normal(&self) -> bool {
        self.r.in_range() && self.g.in_range() && self.b.in_range()
    }
}

impl<S> RGBColor<u8, S> {
    /// Create 24-bit RGB color from a 6 or 3 character hexcode, panicking if unsuccesful.
    ///
    /// Any characters outside of the used range are ignored.
    ///
    /// If the string is less than 6 characters, only 3 first characters are used.
    /// In three character version every character is repeated to get the full channel value,
    /// eg. `F5A` is equivalent to `FF55AA`.
    ///
    /// # Safety
    /// If `hex_str` is not a valid utf-8 string then this function will result in undefined
    /// behaviour.
    ///
    /// If `hex_str` doesn't consist only from characters from range `[0-9a-fA-F]` then this
    /// function will result in a panic.
    pub unsafe fn from_hex_unchecked<T: Into<Box<str>>>(hex_str: T) -> Self {
        let f =
            |h1: u8, h2: u8| u8::from_str_radix(str::from_utf8_unchecked(&[h1, h2]), 16).unwrap();

        let mut hex_str = hex_str.into();
        let len = hex_str.len();
        let h = hex_str.as_bytes_mut();
        h.make_ascii_lowercase();

        if len >= 6 {
            (f(h[0], h[1]), f(h[2], h[3]), f(h[4], h[5])).into()
        } else {
            (f(h[0], h[0]), f(h[1], h[1]), f(h[2], h[2])).into()
        }
    }

    /// Create 24-bit RGB color from a 6 or 3 character hexcode, returning `None` if unsuccesful.
    ///
    /// Same as `from_hex_unchecked` except returns `None` if the input is not valid or too short.
    pub fn from_hex<T: AsRef<str>>(hex_str: T) -> Option<Self> {
        let len = hex_str.as_ref().len();
        let mut h = hex_str.as_ref().bytes().map(|b| {
            let mut b = b;
            b.make_ascii_lowercase();
            b
        });

        let mut f = || -> Option<u8> {
            u8::from_str_radix(
                str::from_utf8(&if len >= 6 {
                    [h.next()?, h.next()?]
                } else {
                    let x = h.next()?;
                    [x, x]
                })
                .ok()?,
                16,
            )
            .ok()
        };

        Some((f()?, f()?, f()?).into())
    }
}

impl<T: Channel, S> RGBColor<T, S> {
    pub fn hsv<H: Channel>(self) -> HSVColor<H, T, S> {
        let (r, g, b) = self.map(Channel::conv::<f32>).tuple();

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let value = max;
        let saturation = if max == 0.0 { 0.0 } else { delta / max };
        let hue = Deg(60.0
            * if delta == 0.0 {
                0.0
            } else if max == r {
                ((g - b) / delta) % 6.0
            } else if max == g {
                (b - r) / delta + 2.0
            } else {
                (r - g) / delta + 4.0
            });

        HSVColor::new(hue.conv::<H>(), saturation.conv(), value.conv())
    }
}

impl<T: Float + Channel> RGBColor<T, SRGBSpace> {
    /// Gamma decodes this color channel value into the linear color space
    #[inline]
    pub fn std_decode(self) -> RGBColor<T, LinearSpace> {
        self.map(std_gamma_decode).tuple().into()
    }
}

impl<T: Float + Channel> RGBColor<T, LinearSpace> {
    /// Gamma encodes this color channel value into the sRGB color space
    #[inline]
    pub fn std_encode(self) -> RGBColor<T, SRGBSpace> {
        self.map(std_gamma_encode).tuple().into()
    }

    /// Returns the relative luminance of this color between 0 and 1.
    ///
    /// Tells the whiteness of the color as perceived by humans.
    /// Values nearer 0 are darker, and values nearer 1 are lighter.
    ///
    /// The returned values are linear, so to get perceptually uniform luminance, use
    /// `gamma_encode`.
    #[inline]
    pub fn relative_luminance(&self) -> T {
        let (r, g, b) = self.tuple();
        cuwf::<T>(0.2126) * r + cuwf::<T>(0.7152) * g + cuwf::<T>(0.0722) * b
    }
}

impl<T: Channel, S> Default for RGBColor<T, S> {
    fn default() -> Self {
        RGBColor::new(T::ch_zero(), T::ch_zero(), T::ch_zero())
    }
}

impl<T: Clone, S> Clone for RGBColor<T, S> {
    fn clone(&self) -> Self {
        RGBColor {
            r: self.r.clone(),
            g: self.g.clone(),
            b: self.b.clone(),
            _space: PhantomData,
        }
    }
}

impl<T: Copy, S> Copy for RGBColor<T, S> {}

impl<T: Channel> From<BaseColor> for RGBColor<T, SRGBSpace> {
    #[inline]
    fn from(base_color: BaseColor) -> Self {
        use crate::BaseColor::*;

        let c0 = || T::ch_zero();
        let cm = || T::ch_mid();
        let c1 = || T::ch_max();

        let f = &RGBColor::new;
        match base_color {
            Black => f(c0(), c0(), c0()),
            Grey => f(cm(), cm(), cm()),
            White => f(c1(), c1(), c1()),
            Red => f(c1(), c0(), c0()),
            Yellow => f(c1(), c1(), c0()),
            Green => f(c0(), c1(), c0()),
            Cyan => f(c0(), c1(), c1()),
            Blue => f(c0(), c0(), c1()),
            Magenta => f(c1(), c0(), c1()),
        }
    }
}

impl<T: Channel> From<BaseColor> for RGBColor<T, LinearSpace> {
    #[inline]
    fn from(base_color: BaseColor) -> Self {
        RGBColor::<f32, SRGBSpace>::from(base_color)
            .std_decode()
            .conv()
    }
}

impl<T: Channel, S> From<(T, T, T)> for RGBColor<T, S> {
    fn from(tuple: (T, T, T)) -> Self {
        let (r, g, b) = tuple;
        RGBColor::new(r, g, b)
    }
}

impl<T: Clone + Channel, S> From<&(T, T, T)> for RGBColor<T, S> {
    fn from(tuple: &(T, T, T)) -> Self {
        let (r, g, b) = tuple.clone();
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
where
    T: Channel + Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        (self.r + rhs.r, self.g + rhs.g, self.b + rhs.b).into()
    }
}

impl<T> Sub for RGBColor<T, LinearSpace>
where
    T: Channel + Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        (self.r - rhs.r, self.g - rhs.g, self.b - rhs.b).into()
    }
}

impl<T> Mul for RGBColor<T, LinearSpace>
where
    T: Channel + Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        (self.r * rhs.r, self.g * rhs.g, self.b * rhs.b).into()
    }
}

impl<T> Div for RGBColor<T, LinearSpace>
where
    T: Channel + Div<Output = T>,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        (self.r / rhs.r, self.g / rhs.g, self.b / rhs.b).into()
    }
}

impl<T> Mul<T> for RGBColor<T, LinearSpace>
where
    T: Channel + Mul<Output = T> + Clone,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        (self.r * rhs.clone(), self.g * rhs.clone(), self.b * rhs).into()
    }
}

impl<T> Div<T> for RGBColor<T, LinearSpace>
where
    T: Channel + Div<Output = T> + Clone,
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
