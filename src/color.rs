use std::str;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum BaseColor {
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

}

pub trait Color {
    fn rgb(&self) -> ColorRGB;
    fn r(&self) -> u8 { self.rgb().r }
    fn g(&self) -> u8 { self.rgb().g }
    fn b(&self) -> u8 { self.rgb().b }

    fn hsv(&self) -> ColorHSV;
    fn h(&self) -> f32 { self.hsv().h }
    fn s(&self) -> f32 { self.hsv().s }
    fn v(&self) -> f32 { self.hsv().v }
}

impl Color for BaseColor {
    fn rgb(&self) -> ColorRGB {
        use self::BaseColor::*;

        let f = &ColorRGB::new;
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

    fn hsv(&self) -> ColorHSV {
        use self::BaseColor::*;

        let f = &ColorHSV::new;
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

impl Color for ColorRGB {
    fn rgb(&self) -> ColorRGB { *self }

    fn hsv(&self) -> ColorHSV {
        let (r, g, b) =
            (self.r as f32 / 255.0,
             self.g as f32 / 255.0,
             self.b as f32 / 255.0);

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let chroma = max - min;

        let value = (r + g + b) / 3.0;

        let saturation =
            if value == 0.0 {
                0.0
            } else {
                chroma / value
            };

        let hue = 60.0 *
            if chroma == 0.0 {
                0.0
            } else if max == r {
                ((g - b) / chroma) % 6.0
            } else if max == g {
                (b - r) / chroma + 2.0
            } else { // max == b
                (r - g) / chroma + 4.0
            };

        ColorHSV::new(hue, saturation, value)
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct ColorHSV {
    h: f32,
    s: f32,
    v: f32,
}

impl ColorHSV {
    pub fn new(h: f32, s: f32, v: f32) -> Self {
        ColorHSV {h, s, v}
    }
}

impl Color for ColorHSV {
    fn rgb(&self) -> ColorRGB {
        let (h, s, v) = (self.h, self.s, self.v);
        let h = h / 60.0;

        // chroma, largest component
        let c = s * v;

        // second largest component
        let x = c * (1.0 - (h % 2.0 - 1.0).abs());

        // smallest component
        let min = v - c;

        let (r, g, b) =
            match h as u8 {
                0   => (  c,   x, 0.0),
                1   => (  x,   c, 0.0),
                2   => (0.0,   c,   x),
                3   => (0.0,   x,   c),
                4   => (  x, 0.0,   c),
                5|6 => (  c, 0.0,   x),
                _   => panic!("Invalid hue value: {}", self.h)
            };

        let (r, g, b) =
            ((r+min) as u8,
             (g+min) as u8,
             (b+min) as u8);

        ColorRGB{ r, g, b }
    }

    fn hsv(&self) -> ColorHSV { *self }
}

pub struct ColorData {
    name: Box<str>,
    shade1: BaseColor,
    shade2: BaseColor,
}

impl ColorData {
    pub fn new(name: Box<str>, color: ColorRGB) -> Self {
        unimplemented!()
    }
}
