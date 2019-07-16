use std::fmt;

use super::*;

/// A transparent color with an alpha channel
///
/// Alpha of 1 means the color is fully opaque, and alpha of 0 means it's fully transparent.
///
/// This uses a straight alpha, not a premultiplied alpha.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Alpha<C, A> {
    pub color: C,
    pub alpha: A
}

impl<C, A> Alpha<C, A> {
    pub fn new(color: C, alpha: A) -> Self {
        Alpha { color, alpha }
    }

    pub fn into_tuple2(self) -> (C, A) {
        (self.color, self.alpha)
    }
}

impl<T, S, A> Alpha<RGBColor<T, S>, A> {
    pub fn into_tuple4(self) -> (T, T, T, A) {
        (self.color.r, self.color.g, self.color.b, self.alpha)
    }
}

impl<T, S> Alpha<RGBColor<T, S>, T> {
    pub fn into_array4(self) -> [T; 4] {
        [self.color.r, self.color.g, self.color.b, self.alpha]
    }
}

impl<S, A> Alpha<HSVColor<S>, A> {
    pub fn into_tuple4(self) -> (f32, f32, f32, A) {
        (self.color.h, self.color.s, self.color.v, self.alpha)
    }
}

impl<S> Alpha<HSVColor<S>, f32> {
    pub fn into_array4(self) -> [f32; 4] {
        [self.color.h, self.color.s, self.color.v, self.alpha]
    }
}

impl<C: Default> Default for Alpha<C, f32> {
    fn default() -> Self {
        Alpha::new(C::default(), 1.0)
    }
}

impl<C: Default> Default for Alpha<C, u8> {
    fn default() -> Self {
        Alpha::new(C::default(), 255u8)
    }
}

impl<C: Default> Default for Alpha<C, u16> {
    fn default() -> Self {
        Alpha::new(C::default(), u16::max_value())
    }
}

impl<C, A> AsRef<C> for Alpha<C, A> {
    fn as_ref(&self) -> &C {
        &self.color
    }
}

impl<C, A> AsMut<C> for Alpha<C, A> {
    fn as_mut(&mut self) -> &mut C {
        &mut self.color
    }
}

impl<S> From<RGBColor<u8, S>> for Alpha<RGBColor<u8, S>, u8> {
    fn from(color: RGBColor<u8, S>) -> Self {
        Alpha::new(color, 255)
    }
}

impl<S> From<RGBColor<u16, S>> for Alpha<RGBColor<u16, S>, u16> {
    fn from(color: RGBColor<u16, S>) -> Self {
        Alpha::new(color, u16::max_value())
    }
}

impl<S> From<RGBColor<f32, S>> for Alpha<RGBColor<f32, S>, f32> {
    fn from(color: RGBColor<f32, S>) -> Self {
        Alpha::new(color, 1.0)
    }
}

impl<T, C: From<(T, T, T)>, A> From<(T, T, T, A)> for Alpha<C, A> {
    fn from(tuple: (T, T, T, A)) -> Self {
        Alpha::new((tuple.0, tuple.1, tuple.2).into(), tuple.3)
    }
}

impl<T: Clone, C: From<[T; 3]>> From<[T; 4]> for Alpha<C, T> {
    fn from(array: [T; 4]) -> Self {
        Alpha::new(
            [array[0].clone(), array[1].clone(), array[2].clone()].into(),
            array[3].clone()
        )
    }
}

impl<T: Clone, C: From<[T; 3]>> From<&[T; 4]> for Alpha<C, T> {
    fn from(array: &[T; 4]) -> Self {
        Alpha::new(
            [array[0].clone(), array[1].clone(), array[2].clone()].into(),
            array[3].clone()
        )
    }
}
impl<C: fmt::UpperHex> fmt::UpperHex for Alpha<C, u8> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}{:02X}", self.color, self.alpha)
    }
}

impl<C: fmt::LowerHex> fmt::LowerHex for Alpha<C, u8> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}{:02x}", self.color, self.alpha)
    }
}
