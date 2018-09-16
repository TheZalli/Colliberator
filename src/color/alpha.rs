use cgmath::Deg;

use super::*;

/// A transparent color `C` with an alpha channel of type `A`.
///
/// Alpha of 1 means the color is fully opaque, and alpha of 0 means it's fully transparent.
///
/// This uses a straight alpha, not a premultiplied alpha.
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Alpha<C, A> {
    pub color: C,
    pub alpha: A
}

impl<C, A> Alpha<C, A> {
    pub fn new(color: C, alpha: A) -> Self {
        Alpha { color, alpha }
    }

    pub fn to_tuple2(self) -> (C, A) {
        (self.color, self.alpha)
    }
}

impl<T, S, A> Alpha<RGBColor<T, S>, A> {
    pub fn to_tuple4(self) -> (T, T, T, A) {
        (self.color.r, self.color.g, self.color.b, self.alpha)
    }
}

impl<T, S> Alpha<RGBColor<T, S>, T> {
    pub fn to_array4(self) -> [T; 4] {
        [self.color.r, self.color.g, self.color.b, self.alpha]
    }
}

impl<S, A> Alpha<HSVColor<S>, A> {
    pub fn to_tuple4(self) -> (Deg<f32>, f32, f32, A) {
        (self.color.h, self.color.s, self.color.v, self.alpha)
    }
}

impl<S> Alpha<HSVColor<S>, f32> {
    pub fn to_array4(self) -> [f32; 4] {
        [self.color.h.0, self.color.s, self.color.v, self.alpha]
    }
}

impl Alpha<LinRGBColor, f32> {
    /// Alpha blend this color with the given opaque background color.
    pub fn blend(self, bg: LinRGBColor) -> LinRGBColor {
        self.color.blend(bg, self.alpha)
    }

    /// Alpha blend this color with the given background color.
    pub fn alpha_blend<T: Into<Self>>(self, bg: T) -> Self {
        let bg = bg.into();

        let alpha = self.alpha + bg.alpha * (1.0 - self.alpha);
        if alpha == 0.0 {
            return Alpha::default();
        }

        let color = (
            self.color * self.alpha +
            bg.color * bg.alpha * (1.0 - self.alpha)
        ) / alpha;

        Alpha { color, alpha }
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

/*macro_rules! alpha_impl_ops {
    ( $( $op_trait: ident ),+ ; $( $op_fun:ident ),+ ) => { $(
        impl<C, A> $op_trait for Alpha<C, A>
            where C: $op_trait
    ),+ };
}*/