use std::iter::{IntoIterator, ExactSizeIterator, FusedIterator};
use std::mem;

use crate::{Alpha, RGBColor};

const ARRAY_MAX_LEN: u8 = 4;

/// An iterator to a color's channels
pub struct IntoIter<T> {
    array: [T; ARRAY_MAX_LEN as usize],
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
        if self.idx == ARRAY_MAX_LEN { return None; }
        let output = self.array[self.idx as usize].clone();
        self.idx += 1;
        Some(output)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let x = (ARRAY_MAX_LEN - self.idx) as usize;
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

impl<T: Clone, S> IntoIterator for Alpha<RGBColor<T, S>, T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::from4(self.color.r, self.color.g, self.color.b, self.alpha)
    }
}
