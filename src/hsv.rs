use std::fmt;
use std::marker::PhantomData;

use crate::*;

/// A HSV color
///
/// ## Type arguments
/// `H` is the type of hue channel, `T` is the type of the saturation and value channels.
///
/// `S` is this color's colorspace.
#[derive(Debug, PartialOrd, PartialEq)]
pub struct HSVColor<H, T, S> {
    pub h: H,
    pub s: T,
    pub v: T,
    _space: PhantomData<S>,
}

impl<H, T, S> HSVColor<H, T, S> {
    /// Deconstructs this color into a tuple of it's channels
    #[inline]
    pub fn tuple(self) -> (H, T, T) {
        (self.h, self.s, self.v)
    }
    /// Deconstructs this color into an array of it's channels
    #[inline]
    pub fn array<U: From<H> + From<T>>(self) -> [U; 3] {
        [self.h.into(), self.s.into(), self.v.into()]
    }
}

impl<H, T, S> HSVColor<H, T, S>
where
    Self: Color,
{
    /// Create a new HSV value.
    ///
    /// The value is normalized on creation.
    pub fn new<H2: Into<H>>(h: H2, s: T, v: T) -> Self {
        HSVColor {
            h: h.into(),
            s,
            v,
            _space: PhantomData,
        }
        .normalize()
    }
}

impl<H: Channel, T: Channel, S> HSVColor<H, T, S> {
    /// Transform this color into RGB form
    ///
    /// This should be done to a normalized HSV color.
    pub fn rgb(self) -> RGBColor<T, S> {
        let h = cuwtf(self.h.conv::<Deg<f32>>()) / 60.0;
        let (s, v) = (cuwtf(self.s), cuwtf(self.v));

        // largest, second largest and the smallest component
        let mc = s * v;
        let xc = mc * (1.0 - (h % 2.0 - 1.0).abs());
        let min = v - mc;

        let (r, g, b) = match h as u8 {
            0 => (mc, xc, 0.),
            1 => (xc, mc, 0.),
            2 => (0., mc, xc),
            3 => (0., xc, mc),
            4 => (xc, 0., mc),
            5 | 6 => (mc, 0., xc),
            _ => panic!("Invalid hue value: {:?}", h),
        };

        (cuwf::<T>(r + min), cuwf::<T>(g + min), cuwf::<T>(b + min)).into()
    }

    /// Converts the channels of this color into another type
    #[inline]
    pub fn conv<H2: Channel, T2: Channel>(self) -> HSVColor<H2, T2, S> {
        HSVColor {
            h: self.h.conv(),
            s: self.s.conv(),
            v: self.v.conv(),
            _space: PhantomData,
        }
    }
}

impl<H: Channel, T: Channel, S> Color for HSVColor<H, T, S> {
    /// Normalize the color's values by normalizing the hue and zeroing the unnecessary channels
    ///
    /// If value channel is zero, black is returned.
    /// If saturation channel is zero, hue is set to zero.
    ///
    /// Otherwise the color itself is returned, with it's channels put to their proper ranges
    fn normalize(self) -> Self {
        let (h, s, v) = self.tuple();
        if v == T::ch_zero() {
            Self::default()
        } else if s == T::ch_zero() {
            HSVColor {
                h: H::ch_zero(),
                s: T::ch_zero(),
                v: v.clamp(),
                _space: PhantomData,
            }
        } else {
            HSVColor {
                h: h.clamp(),
                s: s.clamp(),
                v: v.clamp(),
                _space: PhantomData,
            }
        }
    }

    fn is_normal(&self) -> bool {
        let (h, s, v) = (&self.h, &self.s, &self.v);
        let (h0, t0) = (H::ch_zero(), T::ch_zero());

        if !h.in_range() || !s.in_range() || !v.in_range() {
            false
        } else if *v == t0 {
            // color black
            if *h == h0 && *s == t0 {
                true
            } else {
                false
            }
        } else if *s == t0 {
            // a grey color
            if *h == h0 {
                true
            } else {
                false
            }
        } else {
            true
        }
    }
}

impl<H: Channel, T: Channel> From<BaseColor> for HSVColor<H, T, SRGBSpace>
where
    Self: Color,
{
    #[inline]
    fn from(base_color: BaseColor) -> Self {
        use self::BaseColor::*;

        let f = |h: f32, s: f32, v: f32| Self::new(Deg(h).conv::<H>(), s.conv(), v.conv());

        match base_color {
            Black => f(0.0, 0.0, 0.0),
            Grey => f(0.0, 0.0, 0.5),
            White => f(0.0, 0.0, 1.0),
            Red => f(0.0, 1.0, 1.0),
            Yellow => f(60.0, 1.0, 1.0),
            Green => f(120.0, 1.0, 1.0),
            Cyan => f(180.0, 1.0, 1.0),
            Blue => f(240.0, 1.0, 1.0),
            Magenta => f(300.0, 1.0, 1.0),
        }
    }
}

impl<H: Channel, T: Channel> From<BaseColor> for HSVColor<H, T, LinearSpace> {
    #[inline]
    fn from(base_color: BaseColor) -> Self {
        RGBColor::<f32, LinearSpace>::from(base_color)
            .hsv::<H>()
            .conv()
    }
}

impl<H2, H, T, S> From<(H2, T, T)> for HSVColor<H, T, S>
where
    Self: Color,
    H2: Into<H>,
{
    fn from(tuple: (H2, T, T)) -> Self {
        let (h, s, v) = tuple;
        HSVColor::new(h, s, v)
    }
}

impl<H2, H, T, S> From<&(H2, T, T)> for HSVColor<H, T, S>
where
    Self: Color,
    H2: Into<H> + Clone,
    T: Clone,
{
    fn from(tuple: &(H2, T, T)) -> Self {
        let (h, s, v) = tuple.clone();
        HSVColor::new(h, s, v)
    }
}

impl<U, H, T, S> From<[U; 3]> for HSVColor<H, T, S>
where
    Self: Color,
    U: Clone + Into<H> + Into<T>,
{
    fn from(array: [U; 3]) -> Self {
        Self::new(
            array[0].clone(),
            array[1].clone().into(),
            array[2].clone().into(),
        )
    }
}

impl<U, H, T, S> From<&[U; 3]> for HSVColor<H, T, S>
where
    Self: Color,
    U: Clone + Into<H> + Into<T>,
{
    fn from(array: &[U; 3]) -> Self {
        Self::new(
            array[0].clone(),
            array[1].clone().into(),
            array[2].clone().into(),
        )
    }
}

impl<H: Channel, T: Channel, S> Default for HSVColor<H, T, S> {
    fn default() -> Self {
        HSVColor {
            h: H::ch_zero(),
            s: T::ch_zero(),
            v: T::ch_zero(),
            _space: PhantomData,
        }
    }
}

impl<H: Clone, T: Clone, S> Clone for HSVColor<H, T, S> {
    fn clone(&self) -> Self {
        HSVColor {
            h: self.h.clone(),
            s: self.s.clone(),
            v: self.v.clone(),
            _space: PhantomData,
        }
    }
}

impl<H: Copy, T: Copy, S> Copy for HSVColor<H, T, S> {}

// TODO make more generic
impl<S> fmt::Display for HSVColor<f32, f32, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:>5.1}°,{:>5.1}%,{:>5.1}%",
            self.h,
            self.s * 100.0,
            self.v * 100.0
        )
    }
}
