use num_traits::{NumCast, Num};

use crate::{cuw, cuwf};

/// A trait for color channels
pub trait Channel: Sized + PartialOrd + NumCast + Num {
    /// The maximum value for this channel, inclusive
    ///
    /// with integers it's usually the max value, with floats it's one.
    fn ch_max() -> Self;

    /// The zero value for this channel
    fn ch_zero() -> Self;

    /// The middle value for this channel
    ///
    /// Half of `ch_max`
    fn ch_mid() -> Self;

    /// Takes this channel value and converts it into any other channel type
    ///
    /// The channel's range is taken into account, eg. 1.0 in f32 is converted into 255 in u8.
    ///
    /// The values will be made to fit into their range.
    ///
    /// If you are implementing a custom type with a conversion that might fail, re-implement
    /// this method, because this assumes the conversion can't fail.
    fn conv<T: Channel>(self) -> T {
        cuwf((
            cuw::<_, f32>(self.to_range()) /
            cuw::<_, f32>(Self::ch_max()) *
            cuw::<_, f32>(T::ch_max())
        ).round())
    }

    /// Return whether this value is inside the channel's allowed range
    fn in_range(&self) -> bool {
        (self <= &Self::ch_max()) && (self >= &Self::ch_zero())
    }

    /// Returns this value clamped to channel's range
    fn to_range(self) -> Self {
        if self > Self::ch_max()        { Self::ch_max() }
        else if self < Self::ch_zero()  { Self::ch_zero() }
        else                            { self }
    }
}

macro_rules! impl_uint_channels {
    ( $( $type:ty ),* ) => { $(
        impl Channel for $type {
            fn ch_max() -> Self { <$type>::max_value() }
            fn ch_mid() -> Self { <$type>::max_value() / 2 }
            fn ch_zero() -> Self { 0 }
        }
    )* };
}

impl_uint_channels!(u8, u16, u32);

impl Channel for u64 {
    fn ch_max() -> Self { u64::max_value() }
    fn ch_mid() -> Self { u64::max_value() / 2 }
    fn ch_zero() -> Self { 0 }

    fn conv<T: Channel>(self) -> T {
        cuw::<_, T>((
            self.to_range() as f64 /
            Self::ch_max() as f64 *
            cuw::<_, f64>(T::ch_max())
        ).round())
    }
}

impl Channel for f32 {
    fn ch_max() -> Self { 1.0 }
    fn ch_mid() -> Self { 0.5 }
    fn ch_zero() -> Self { 0.0 }
}

impl Channel for f64 {
    fn ch_max() -> Self { 1.0 }
    fn ch_mid() -> Self { 0.5 }
    fn ch_zero() -> Self { 0.0 }

    fn conv<T: Channel>(self) -> T {
        cuw::<_, T>((
            self.to_range() / Self::ch_max() *
            cuw::<_, f64>(T::ch_max())
        ).round())
    }
}
