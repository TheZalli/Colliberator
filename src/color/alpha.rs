use super::*;

/// A transparent color `C` with an alpha channel of type `A`.
///
/// Alpha of 1 means the color is fully opaque, and alpha of 0 means it's fully transparent.
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Alpha<C, A> {
    pub color: C,
    pub alpha: A
}

impl<C, A> Alpha<C, A> {
    pub fn new(color: C, alpha: A) -> Self {
        Alpha { color, alpha }
    }
}

impl Alpha<LinRGBColor, f32> {
    /// Blends this color with the background color using this color's alpha.
    pub fn blend(self, other: LinRGBColor) -> LinRGBColor {
        self.color.blend(other, self.alpha)
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
