use std::fmt;

use color::*;

#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
/// An sRGB color with channels normalized between 0 and 1.
pub struct SRGBColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    _priv: ()
}

impl SRGBColor {
    /// Creates a new `LinRGBColor` using the given rgb-values.
    ///
    /// The values are clamped into 0.0 - 1.0 range.
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        let f = |x| clamp(x, 0.0, 1.0);
        SRGBColor { r: f(r), g: f(g), b: f(b), _priv: () }
    }

    pub fn to_tuple(&self) -> (f32, f32, f32) { (self.r, self.g, self.b) }
}

impl Color for SRGBColor {
    fn srgb(&self) -> SRGBColor { *self }

    fn srgb24(&self) -> SRGB24Color {
        let (r, g, b) = self.to_tuple();
        SRGB24Color::new((255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8)
    }

    fn lin_rgb(&self) -> LinRGBColor {
        let (r, g, b) = self.to_tuple();
        LinRGBColor::new(gamma_decode(r), gamma_decode(g), gamma_decode(b))
    }

    fn hsv(&self) -> HSVColor {
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
            } else { // max == b
                (r - g) / delta + 4.0
            };

        HSVColor::new(hue, saturation, value)
    }
}

impl fmt::Display for SRGBColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>5.1}%,{:>5.1}%,{:>5.1}%", self.r * 100.0, self.g * 100.0, self.b * 100.0)
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SRGB24Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl SRGB24Color {
    /// Create a new RGB color.
    pub fn new(r: u8, g: u8, b: u8) -> Self { SRGB24Color { r, g, b } }

    /// Destructure self into a tuple
    pub fn to_tuple(&self) -> (u8, u8, u8) { (self.r, self.g, self.b) }

    /// Create `SRGB24Color` from a hexcode.
    ///
    /// # Safety
    /// If `hex_str` is not a valid utf-8 string then this function will result in undefined
    /// behaviour.
    ///
    /// If `hex_str` doesn't consist only of the characters `[0-9a-fA-F]` then this function will
    /// result in a panic.
    pub unsafe fn from_hex_unchecked(hex_str: Box<str>) -> Self {
        let f = |h1: u8, h2: u8|
            u8::from_str_radix(str::from_utf8_unchecked(&[h1, h2]), 16).unwrap();

        let mut hex_str = hex_str;
        let h = hex_str.as_bytes_mut();
        h.make_ascii_lowercase();

        SRGB24Color::new(f(h[0], h[1]), f(h[2], h[3]), f(h[4], h[5]))
    }
}

impl Color for SRGB24Color {
    fn srgb(&self) -> SRGBColor {
        let (r, g, b) = self.to_tuple();
        SRGBColor::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }

    fn srgb24(&self) -> SRGB24Color { *self }
}

impl fmt::Display for SRGB24Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>3}, {:>3}, {:>3}", self.r, self.g, self.b)
    }
}
