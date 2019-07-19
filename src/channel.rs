/// A trait for color channels
pub trait Channel: Sized + PartialOrd {
    /// The maximum value for this channel, inclusive.
    ///
    /// with integers it's usually the max value, with floats it's one.
    fn ch_max() -> Self;

    /// The minimum value for this channel, inclusive.
    ///
    /// Usually 0.
    fn ch_min() -> Self;

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

macro_rules! impl_int_channels {
    ( $( $type:ty ),* ) => { $(
        impl Channel for $type {
            fn ch_max() -> Self { <$type>::max_value() }
            fn ch_min() -> Self { <$type>::min_value() }
        }
    )* };
}

macro_rules! impl_float_channels {
    ( $( $type:ty ),* ) => { $(
        impl Channel for $type {
            fn ch_max() -> Self { 1.0 }
            fn ch_min() -> Self { 0.0 }
        }
    )* };
}

impl_int_channels!(u8, u16, u32, u64);
impl_float_channels!(f32, f64);

