use std::fmt;
use std::marker::PhantomData;

use util::*;
use color::*;

#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct HSVColor<S> {
    pub h: Deg,
    pub s: f32,
    pub v: f32,
    _space: PhantomData<S>
}

impl<S> HSVColor<S> {
    /// Create a new HSV value.
    pub fn new(h: Deg, s: f32, v: f32) -> Self {
        HSVColor { h, s, v, _space: PhantomData }
    }

    #[inline]
    pub fn to_tuple(&self) -> (Deg, f32, f32) {
        (self.h, self.s, self.v)
    }

    #[inline]
    pub fn to_array(self) -> [f32; 3] {
        [self.h.into(), self.s, self.v]
    }

    pub fn rgb(&self) -> RGBColor<f32, S> {
        let (h, s, v) = self.to_tuple();
        let (h, s, v): (f32, f32, f32) = ((h / 60.0).into(), s.into(), v.into());

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

        (r+min, g+min, b+min).into()
    }
}

impl<S> fmt::Display for HSVColor<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>5.1}°,{:>5.1}%,{:>5.1}%", self.h, self.s * 100.0, self.v * 100.0)
    }
}
