use std::fmt;
use std::ops::{Add, Sub, Mul, Div};

use color::*;

#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
/// An RGB color with channels normalized between 0 and 1 in the linear space.
pub struct LinRGBColor {
    r: f32,
    g: f32,
    b: f32
}

impl LinRGBColor {
    /// Creates a new `LinRGBColor` using the given rgb-values.
    ///
    /// The values are clamped into 0.0 - 1.0 range.
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        let f = |x| clamp(x, 0.0, 1.0);
        LinRGBColor { r: f(r), g: f(g), b: f(b) }
    }

    pub fn to_tuple(&self) -> (f32, f32, f32) { (self.r, self.g, self.b) }
}

impl Color for LinRGBColor {
    fn srgb(&self) -> SRGBColor {
        let (r, g, b) = self.to_tuple();
        SRGBColor::new(gamma_encode(r), gamma_encode(g), gamma_encode(b))
    }

    fn lin_rgb(&self) -> LinRGBColor { *self }

    fn lin_rgb48(&self) -> LinRGB48Color {
        let (r, g, b) = self.to_tuple();
        LinRGB48Color::new((r * 255.0) as u16, (g * 255.0) as u16, (b * 255.0) as u16)
    }
}

impl Add for LinRGBColor {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (r1, g1, b1) = self.to_tuple();
        let (r2, g2, b2) = rhs.to_tuple();
        (r1 + r2, g1 + g2, b1 + b2).into()
    }
}

impl Sub for LinRGBColor {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (r1, g1, b1) = self.to_tuple();
        let (r2, g2, b2) = rhs.to_tuple();
        (r1 - r2, g1 - g2, b1 - b2).into()
    }
}

impl Mul<f32> for LinRGBColor {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let (r, g, b) = self.to_tuple();
        LinRGBColor::new(r * rhs, g * rhs, b * rhs)
    }
}

impl Mul<LinRGBColor> for f32 {
    type Output = LinRGBColor;

    fn mul(self, rhs: LinRGBColor) -> Self::Output {
        let (r, g, b) = rhs.to_tuple();
        LinRGBColor::new(self * r, self * g, self * b)
    }
}

impl Div<f32> for LinRGBColor {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let (r, g, b) = self.to_tuple();
        LinRGBColor::new(r / rhs, g / rhs, b / rhs)
    }
}

impl Div<LinRGBColor> for f32 {
    type Output = LinRGBColor;

    fn div(self, rhs: LinRGBColor) -> Self::Output {
        let (r, g, b) = rhs.to_tuple();
        LinRGBColor::new(self / r, self / g, self / b)
    }
}

impl From<(f32, f32, f32)> for LinRGBColor {
    fn from(arg: (f32, f32, f32)) -> Self {
        let (r, g, b) = arg;
        LinRGBColor::new(r, g, b)
    }
}

impl From<[f32; 3]> for LinRGBColor {
    fn from(arg: [f32; 3]) -> Self {
        (arg[0], arg[1], arg[2]).into()
    }
}

impl fmt::Display for LinRGBColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>5.1}%,{:>5.1}%,{:>5.1}%", self.r * 100.0, self.g * 100.0, self.b * 100.0)
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
/// A 48-bit color with red, green and blue channels in the linear color space.
pub struct LinRGB48Color {
    pub r: u16,
    pub g: u16,
    pub b: u16
}

impl LinRGB48Color {
    /// Create a new sRGB color.
    pub fn new(r: u16, g: u16, b: u16) -> Self { LinRGB48Color { r, g, b } }

    /// Destructure self into a tuple
    pub fn to_tuple(&self) -> (u16, u16, u16) { (self.r, self.g, self.b) }
}

impl Color for LinRGB48Color {
    fn srgb(&self) -> SRGBColor { self.lin_rgb().srgb() }

    fn lin_rgb(&self) -> LinRGBColor {
        let (r, g, b) = self.to_tuple();
        LinRGBColor::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }

    fn lin_rgb48(&self) -> LinRGB48Color { *self }
}

impl fmt::Display for LinRGB48Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>3}, {:>3}, {:>3}", self.r, self.g, self.b)
    }
}
