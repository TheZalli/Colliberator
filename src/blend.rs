use num_traits::Float;

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

impl<T: Channel + Float> Blend<RGBColor<T, LinearSpace>> for RGBColor<T, LinearSpace> {
    type Ratio = T;

    /// Blend this color with another using the given ratio
    ///
    /// If ratio is outside allowed channel range,
    /// this function is undefined behaviour.
    #[inline]
    fn blend(&self, foreground: &Self, ratio: T) -> Self {
        *self * ratio + *foreground * (T::max_value() - ratio)
    }
}
