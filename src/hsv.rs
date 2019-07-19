use std::fmt;
use std::marker::PhantomData;

use super::*;

/// An HSV color
///
/// `S` is this color's colorspace
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct HSVColor<S> {
    pub h: f32,
    pub s: f32,
    pub v: f32,
    _space: PhantomData<S>
}

impl<S> HSVColor<S> {
    /// Create a new HSV value.
    ///
    /// The value is unnormalized, to normalize it, call `HSVColor::normalize`.
    pub fn new(h: f32, s: f32, v: f32) -> Self {
        HSVColor { h, s, v, _space: PhantomData }
    }

    /// Normalize the color's values by normalizing the hue and zeroing the unnecessary channels
    ///
    /// If value channel is zero, black is returned.
    /// If saturation channel is zero, hue is set to zero.
    ///
    /// Otherwise the color itself is returned, with it's hue normalized to fit in the [0, 360)
    /// range.
    pub fn normalize(self) -> Self {
        let (h, s, v) = self.tuple();
        if v == 0.0 { Self::default() }
        else if s == 0.0 {
            HSVColor::new(0.0, 0.0, v)
        }
        else {
            let mut h = h % 360.0;
            if h < 0.0 {
                h += 360.0;
            }
            HSVColor { h, s, v, _space: PhantomData }
        }
    }

    #[inline]
    pub fn tuple(self) -> (f32, f32, f32) {
        (self.h, self.s, self.v)
    }

    #[inline]
    pub fn array(self) -> [f32; 3] {
        [self.h, self.s, self.v]
    }

    #[inline]
    pub fn as_tuple(&self) -> (f32, f32, f32) {
        (self.h, self.s, self.v)
    }

    #[inline]
    pub fn as_array(&self) -> [f32; 3] {
        [self.h, self.s, self.v]
    }

    /// Transform this color into RGB form
    ///
    /// This should be done to a normalized HSV color.
    pub fn rgb(&self) -> RGBColor<f32, S> {
        let h = self.h / 60.0;
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
                _   => panic!("Invalid hue value: {}", self.h)
            };

        (r+min, g+min, b+min).into()
    }
}

impl<S> From<(f32, f32, f32)> for HSVColor<S> {
    fn from(tuple: (f32, f32, f32)) -> Self {
        let (h, s, v) = tuple;
        HSVColor::new(h, s, v)
    }
}

impl<S> From<[f32; 3]> for HSVColor<S> {
    fn from(array: [f32; 3]) -> Self {
        HSVColor::new(array[0], array[1], array[2])
    }
}

impl<S> From<&[f32; 3]> for HSVColor<S> {
    fn from(array: &[f32; 3]) -> Self {
        HSVColor::new(array[0], array[1], array[2])
    }
}

impl<S> Default for HSVColor<S> {
    fn default() -> Self {
        HSVColor::new(0.0, 0.0, 0.0)
    }
}

impl<S> fmt::Display for HSVColor<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>5.1}°,{:>5.1}%,{:>5.1}%", self.h, self.s * 100.0, self.v * 100.0)
    }
}
