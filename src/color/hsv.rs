use std::fmt;

use color::*;

#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct HSVColor {
    pub h: f32,
    pub s: f32,
    pub v: f32,
    _priv: ()
}

impl HSVColor {
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
        HSVColor { h, s, v, _priv: () }
    }

    pub fn to_tuple(&self) -> (f32, f32, f32) {
        (self.h, self.s, self.v)
    }
}

impl Color for HSVColor {
    fn srgb(&self) -> SRGBColor {
        let (h, s, v) = self.to_tuple();
        let h = h / 60.0;

        // largest, second largest and the smallest component
        let c = s * v;
        let x = c * (1.0 - (h % 2.0 - 1.0).abs());
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

        SRGBColor::new(r+min, g+min, b+min)
    }

    fn hsv(&self) -> HSVColor { *self }
}

impl fmt::Display for HSVColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>5.1}°,{:>5.1}%,{:>5.1}%", self.h, self.s * 100.0, self.v * 100.0)
    }
}
