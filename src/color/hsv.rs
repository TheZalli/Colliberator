use std::fmt;
use std::marker::PhantomData;

use cgmath::prelude::*;
use cgmath::Deg;

//use util::*;
use super::*;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct HSVColor<S> {
    pub h: Deg<f32>,
    pub s: f32,
    pub v: f32,
    _space: PhantomData<S>
}

impl<S> HSVColor<S> {
    /// Create a new HSV value.
    ///
    /// The hue value is normalized to fit in the [0, 360) range.
    pub fn new(h: f32, s: f32, v: f32) -> Self {
        let h = Deg(h).normalize();
        HSVColor { h, s, v, _space: PhantomData }
    }

    /// Normalizes the color's values by zeroing the unnecessary channels.
    ///
    /// If value is zero, black is returned.
    /// If saturation is zero, hue is set to zero.
    ///
    /// Otherwise the color itself is returned, with it's hue normalized to fit in the [0, 360)
    /// range.
    pub fn normalize(self) -> Self {
        if self.v == 0.0 { Self::default() }
        else if self.s == 0.0 {
            HSVColor::new(0.0, 0.0, self.v)
        }
        else {
            HSVColor {
                h: self.h.normalize(),
                s: self.s,
                v: self.v,
                _space: PhantomData
            }
        }
    }

    #[inline]
    pub fn to_tuple(self) -> (Deg<f32>, f32, f32) {
        (self.h, self.s, self.v)
    }

    #[inline]
    pub fn to_array(self) -> [f32; 3] {
        [self.h.0, self.s, self.v]
    }

    pub fn rgb(&self) -> RGBColor<f32, S> {
        let h = self.h.0 / 60.0;
        let (s, v) = (self.s, self.v);

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
                _   => panic!("Invalid hue value: {}", self.h.0)
            };

        (r+min, g+min, b+min).into()
    }
}

impl<S> Default for HSVColor<S> {
    fn default() -> Self {
        HSVColor::new(0.0, 0.0, 0.0)
    }
}

impl<S> fmt::Display for HSVColor<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>5.1}°,{:>5.1}%,{:>5.1}%", self.h.0, self.s * 100.0, self.v * 100.0)
    }
}
