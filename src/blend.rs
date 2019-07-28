use crate::*;

/// Trait for colors that can be blended
pub trait Blend<FG> {
    /// Type of the value that determines the ratio of the colors being blended
    type Ratio;

    /// Blend this color with another using the given ratio
    fn blend(&self, foreground: &FG, ratio: Self::Ratio) -> Self;
}

/// Trait for colors that can be alpha blended
pub trait AlphaBlend<FG> {
    /// Blend this color with another using the alpha channel of the foreground
    fn alpha_blend(&self, foreground: &FG) -> Self;
}

impl Blend<LinRGBColor> for LinRGBColor {
    type Ratio = f32;

    /// Blend this color with another using the given ratio
    ///
    /// If ratio is outside 0.0 - 1.0, this function is undefined behaviour.
    #[inline]
    fn blend(&self, foreground: &Self, ratio: f32) -> Self {
        *self * ratio + *foreground * (1.0-ratio)
    }
}

impl Blend<LinRGB48Color> for LinRGB48Color {
    type Ratio = u16;

    #[inline]
    fn blend(&self, foreground: &Self, ratio: u16) -> Self {
        self.float().blend(
            &foreground.float(), ratio as f32 / u16::max_value() as f32
        ).conv()
    }
}
