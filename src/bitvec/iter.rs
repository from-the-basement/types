use super::{BitSlice, BitVec};

#[derive(Debug, Clone)]
pub struct BitVecIter {
    vec: BitVec,
    id: usize,
}

impl Iterator for BitVecIter {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let b = self.vec.get(self.id);
        self.id += 1;
        b
    }
}

impl IntoIterator for BitVec {
    type Item = bool;

    type IntoIter = BitVecIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BitVecIter { vec: self, id: 0 }
    }
}

pub struct BitSliceIter<'iter> {
    slice: BitSlice<'iter>,
    id: usize,
}

impl<'iter> BitSliceIter<'iter> {
    #[inline]
    pub(crate) fn new(slice: BitSlice<'iter>) -> Self {
        BitSliceIter { slice, id: 0 }
    }
}

impl<'iter> Iterator for BitSliceIter<'iter> {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let b: Option<bool> = self.slice.get(self.id);
        self.id += 1;
        b
    }
}

impl<'slice> IntoIterator for BitSlice<'slice> {
    type Item = bool;

    type IntoIter = BitSliceIter<'slice>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BitSliceIter::new(self)
    }
}
