use std::fmt;

use super::*;

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

impl BaseColor {
    #[inline]
    pub fn srgb24(&self) -> SRGB24Color {
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

    #[inline] pub fn srgb(&self) -> SRGBColor { self.srgb24().into_float() }
    #[inline] pub fn lin_rgb(&self) -> LinRGBColor { self.srgb().decode() }
    #[inline] pub fn lin_rgb24(&self) -> LinRGB48Color { self.lin_rgb().quantizate_u16() }

    #[inline]
    pub fn hsv(&self) -> HSVColor<SRGBSpace> {
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

impl Default for BaseColor {
    fn default() -> Self {
        BaseColor::Black
    }
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
