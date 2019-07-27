use num_traits::NumCast;

/// A trait for color channels
pub trait Channel: Sized + PartialOrd + NumCast {
    /// The maximum value for this channel, inclusive.
    ///
    /// with integers it's usually the max value, with floats it's one.
    fn ch_max() -> Self;

    /// The minimum value for this channel, inclusive.
    ///
    /// Usually 0.
    fn ch_min() -> Self;

    /// The middle value for this channel
    ///
    /// Half of the max value
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
        T::from(self.to_range()).unwrap()
    }

    /// Return whether this value is inside it's max and min values.
    fn in_range(&self) -> bool {
        (self <= &Self::ch_max()) && (self >= &Self::ch_min())
    }

    /// Returns this value clamped to the range between max and min values.
    fn to_range(self) -> Self {
        if self > Self::ch_max()        { Self::ch_max() }
        else if self < Self::ch_min()   { Self::ch_min() }
        else                            { self }
    }
}

macro_rules! impl_uint_channels {
    ( $( $type:ty ),* ) => { $(
        impl Channel for $type {
            fn ch_max() -> Self { <$type>::max_value() }
            fn ch_mid() -> Self { <$type>::max_value() / 2 }
            fn ch_min() -> Self { 0 }
        }
    )* };
}

macro_rules! impl_float_channels {
    ( $( $type:ty ),* ) => { $(
        impl Channel for $type {
            fn ch_max() -> Self { 1.0 }
            fn ch_mid() -> Self { 0.5 }
            fn ch_min() -> Self { 0.0 }
        }
    )* };
}

impl_uint_channels!(u8, u16, u32, u64);
impl_float_channels!(f32, f64);

