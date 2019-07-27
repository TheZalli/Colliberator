use std::iter::{IntoIterator, ExactSizeIterator, FusedIterator};
use std::mem;

use crate::{Alpha, RGBColor, HSVColor};

/// An iterator to a color's channels
pub struct IntoIter<T> {
    array: [T; 4],
    idx: u8,
}

impl<T> IntoIter<T> {
    fn from4(x: T, y: T, z: T, w: T) -> Self {
        IntoIter {
            array: [x, y, z, w],
            idx: 0
        }
    }

    fn from3(x: T, y: T, z: T) -> Self {
        IntoIter {
            array: [ unsafe { mem::zeroed() }, x, y, z],
            idx: 1
        }
    }
}

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == 4 { return None; }
        let output = self.array[self.idx as usize].clone();
        self.idx += 1;
        Some(output)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let x = (4 - self.idx) as usize;
        (x, Some(x))
    }
}

impl<T: Clone> ExactSizeIterator for IntoIter<T> {}
impl<T: Clone> FusedIterator for IntoIter<T> {}

impl<T: Clone, S> IntoIterator for RGBColor<T, S> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::from3(self.r, self.g, self.b)
    }
}

impl<S> IntoIterator for HSVColor<S> {
    type Item = f32;
    type IntoIter = IntoIter<f32>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::from3(self.h, self.s, self.v)
    }
}

impl<T: Clone, S> IntoIterator for Alpha<RGBColor<T, S>, T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::from4(self.color.r, self.color.g, self.color.b, self.alpha)
    }
}

impl<S> IntoIterator for Alpha<HSVColor<S>, f32> {
    type Item = f32;
    type IntoIter = IntoIter<f32>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::from4(self.color.h, self.color.s, self.color.v, self.alpha)
    }
}
