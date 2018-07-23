use std::str;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum BaseColor {
    Grey,
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Blue,
    Violet,
    Magenta,
}

impl BaseColor {

}

pub trait Color {
    fn rgb(&self) -> ColorRGB;
    fn hue(&self) -> f32;
    // sat(&self)
}

impl Color for BaseColor {
    fn rgb(&self) -> ColorRGB {
        use self::BaseColor::*;

        let f = &ColorRGB::new;
        match self {
            Grey    => f(128, 128, 128),
            Red     => f(255,   0,   0),
            Orange  => f(255, 128,   0),
            Yellow  => f(255, 255,   0),
            Green   => f(  0, 255,   0),
            Cyan    => f(  0, 255, 255),
            Blue    => f(  0,   0, 255),
            Violet  => f(128,   0, 255),
            Magenta => f(255,   0, 255),
        }
    }

    fn hue(&self) -> f32 {
        use self::BaseColor::*;

        match self {
            Grey    => 0.0,
            Red     => 0.0,
            Orange  => 30.0,
            Yellow  => 60.0,
            Green   => 120.0,
            Cyan    => 180.0,
            Blue    => 240.0,
            Violet  => 270.0,
            Magenta => 300.0,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ColorRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorRGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        ColorRGB {r, g, b}
    }

    /// Create `ColorRGB` from a hexcode.
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

        ColorRGB {
            r: f(h[0], h[1]),
            g: f(h[2], h[3]),
            b: f(h[4], h[5]),
        }
    }
}

pub struct ColorData {
    name: Box<str>,
    shades_of: BaseColor
}

impl ColorData {
    pub fn new(name: Box<str>, color: ColorRGB) -> Self {
        unimplemented!()
    }
}
