use std::str;
use std::fmt;

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
    /// Return the RGB representation
    fn rgb(&self) -> ColorRGB;
    fn r(&self) -> u8 { self.rgb().r }
    fn g(&self) -> u8 { self.rgb().g }
    fn b(&self) -> u8 { self.rgb().b }

    /// Return the HSV representation
    fn hsv(&self) -> ColorHSV;
    fn h(&self) -> f32 { self.hsv().h }
    fn s(&self) -> f32 { self.hsv().s }
    fn v(&self) -> f32 { self.hsv().v }

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

        // HSV value under this value is considered to be just black
        const BLACK_CUTOFF_VALUE: f32 = 0.07;

        // saturation under this value is considered to be just greyscale without any color
        const GREYSCALE_SATURATION: f32 = 0.05;

        // saturation and value borders for the greyscale shades
        const WHITE_SATURATION: f32 = 0.35;
        const WHITE_VALUE: f32 = 0.60;

        const GREY_SATURATION: f32 = 0.45;
        const GREY_VALUE_MAX: f32 = 0.80;
        const GREY_VALUE_MIN: f32 = 0.15;

        const BLACK_VALUE: f32 = 0.25;

        assert!(GREYSCALE_SATURATION <= WHITE_SATURATION);
        assert!(GREYSCALE_SATURATION <= GREY_SATURATION);

        let mut shades = Vec::with_capacity(3);
        let (h, s, v) = self.hsv().to_tuple();

        if v < BLACK_CUTOFF_VALUE {
            return vec![(Black, 1.0)];
        }

        if s > GREYSCALE_SATURATION {
            // red is a special case
            if h >= 360.0 - HUE_MARGIN || h <= 0.0 + HUE_MARGIN {
                let amount = 1.0 -
                    if h <= 0.0 + HUE_MARGIN {
                        h
                    } else {
                        h - 360.0
                    } / HUE_MARGIN;

                shades.push((Red, amount));
            }
            for (hue, color) in COLOR_HUES.iter() {
                let dist = (h - hue).abs();
                if dist <= HUE_MARGIN {
                    shades.push((*color, 1.0 - dist / HUE_MARGIN));
                }
            }
        }

        if v <= BLACK_VALUE {
            shades.push((Black, 1.0));
        } else if v >= WHITE_VALUE && s <= WHITE_SATURATION {
            //let amount = 1.0 - (WHITE_SATURATION - s) / WHITE_SATURATION;
            shades.push((White, 1.0));
        }

        if s <= GREY_SATURATION && v <= GREY_VALUE_MAX && v >= GREY_VALUE_MIN {
            //let amount = 1.0 - (GREY_SATURATION - s) / GREY_SATURATION;
            shades.push((Grey, 1.0));
        }

        return shades;
    }

    /// Returns the `text` with this color as it's background color.
    fn ansi_escape_bgcolor(&self, text: &str) -> String {
        const CSI: &str = "\u{1B}[";
        let (r, g, b) = self.rgb().to_tuple();

        // color the text as black or white depending on the bg:s lightness
        let fg =
            if self.v() < 0.5 {
                format!("{}38;2;255;255;255m", CSI)
            } else {
                format!("{}38;2;;;m", CSI)
            };

        fg + &format!("{}48;2;{};{};{}m{}{0}0m", CSI, r, g, b, text)
    }
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
/// A 24-bit color with red, green and blue channels.
pub struct ColorRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorRGB {
    /// Create a new RGB color.
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

    pub fn to_tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
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
        let delta = max - min;

        let value = max;

        let saturation =
            if max == 0.0 {
                0.0
            } else {
                delta / max
            };

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

        ColorHSV::new(hue, saturation, value)
    }
}

impl fmt::Display for ColorRGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>3}, {:>3}, {:>3}", self.r, self.g, self.b)
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct ColorHSV {
    pub h: f32,
    pub s: f32,
    pub v: f32,
    // this is so we have to use the constructor `new`
    _void: (),
}

impl ColorHSV {
    /// Create a new HSV value.
    ///
    /// Hue is given in degrees and it is wrapped between [0, 360).
    /// Saturation and value are given as a percentage between \[0, 1\].
    ///
    /// # Panic
    /// If saturation and value are not between 0.0 and 1.0, this function will panic.
    pub fn new(h: f32, s: f32, v: f32) -> Self {
        if s < 0.0 || s > 1.0 {
            panic!("Invalid HSV saturation: {}", s);
        }
        if v < 0.0 || v > 1.0 {
            panic!("Invalid HSV value: {}", v);
        }

        let mut h = h % 360.0;
        if h < 0.0 {
            h = h + 360.0;
        }
        ColorHSV {h, s, v, _void: ()}
    }

    pub fn to_tuple(&self) -> (f32, f32, f32) {
        (self.h, self.s, self.v)
    }
}

impl Color for ColorHSV {
    fn rgb(&self) -> ColorRGB {
        let (h, s, v) = self.to_tuple();
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
impl fmt::Display for ColorHSV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>width$.precision$}°, {:>width$.precision$}%, {:>width$.precision$}%",
               self.h, self.s * 100.0, self.v * 100.0, width=5, precision=1)
    }
}
