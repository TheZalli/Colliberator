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

impl<C, A: Channel> Alpha<C, A> {
    /// Creates a new alpha channel.
    ///
    /// This makes sure that the alpha-channel is in the proper range
    /// by calling `Channel::to_range`
    pub fn new(color: C, alpha: A) -> Self {
        Alpha { color, alpha: alpha.to_range() }
    }
}

impl<C: Color, A: Channel> Color for Alpha<C, A> {
    fn normalize(self) -> Self {
        let color = self.color.normalize();
        let alpha = self.alpha.to_range();
        Alpha { color, alpha }
    }

    fn is_normal(&self) -> bool {
        self.color.is_normal() && self.alpha.in_range()
    }
}

impl<T, A, S> Alpha<RGBColor<T, S>, A> {
    /// Deconstructs this color into a tuple of it's channels
    pub fn tuple(self) -> (T, T, T, A) {
        (self.color.r, self.color.g, self.color.b, self.alpha)
    }
}

impl<T, S> Alpha<RGBColor<T, S>, T> {
    /// Deconstructs this color into an array of it's channels
    pub fn array(self) -> [T; 4] {
        [self.color.r, self.color.g, self.color.b, self.alpha]
    }
}

impl<A> Alpha<SRGBColor, A> {
    pub fn std_decode(self) -> Alpha<LinRGBColor, A> {
        Alpha { color: self.color.std_decode(), alpha: self.alpha }
    }
}

impl<A> Alpha<LinRGBColor, A> {
    pub fn std_encode(self) -> Alpha<SRGBColor, A> {
        Alpha { color: self.color.std_encode(), alpha: self.alpha }
    }
}

impl<C: Default, A: Channel> Default for Alpha<C, A> {
    fn default() -> Self {
        Alpha::new(C::default(), A::ch_max())
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

impl<T, C: From<(T, T, T)>, A: Channel> From<(T, T, T, A)> for Alpha<C, A> {
    fn from(tuple: (T, T, T, A)) -> Self {
        Alpha::new((tuple.0, tuple.1, tuple.2).into(), tuple.3)
    }
}

impl<T: Clone + Channel, C: From<[T; 3]>> From<[T; 4]> for Alpha<C, T> {
    fn from(array: [T; 4]) -> Self {
        Alpha::new(
            [array[0].clone(), array[1].clone(), array[2].clone()].into(),
            array[3].clone()
        )
    }
}

impl<T: Clone + Channel, C: From<[T; 3]>> From<&[T; 4]> for Alpha<C, T> {
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
