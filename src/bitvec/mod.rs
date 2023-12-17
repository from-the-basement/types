pub mod iter;

use std::{
    cmp::min,
    ops::{BitAnd, Range},
};

const BIT_MASK: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
const UNSET_BIT_MASK: [u8; 8] = [
    255 - 1,
    255 - 2,
    255 - 4,
    255 - 8,
    255 - 16,
    255 - 32,
    255 - 64,
    255 - 128,
];

#[inline]
fn set(byte: u8, i: usize, value: bool) -> u8 {
    if value {
        byte | BIT_MASK[i]
    } else {
        byte & UNSET_BIT_MASK[i]
    }
}

#[inline]
fn is_set(byte: u8, i: usize) -> bool {
    (byte & BIT_MASK[i]) != 0
}

#[inline]
fn get_bit(bytes: &[u8], i: usize) -> bool {
    is_set(bytes[i / 8], i % 8)
}

#[inline]
pub fn set_bit(data: &mut [u8], i: usize, value: bool) {
    data[i / 8] = set(data[i / 8], i % 8, value);
}

#[derive(Default, Debug, Clone)]
pub struct BitVec {
    vec: Vec<u8>,
    len: usize,
}

impl BitAnd for BitVec {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        self.as_slice() & rhs.as_slice()
    }
}

impl BitVec {
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
            len: 0,
        }
    }

    #[inline]
    pub fn push(&mut self, value: bool) {
        if self.len % 8 == 0 {
            self.vec.push(0);
        }
        let byte = self.vec.as_mut_slice().last_mut().unwrap();
        *byte = set(*byte, self.len % 8, value);
        self.len += 1;
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.len {
            return None;
        }
        Some(get_bit(&self.vec, index))
    }

    #[inline]
    pub fn slice(&self, range: Range<usize>) -> BitSlice<'_> {
        BitSlice {
            offset: (range.start % 8) as _,
            len: range.end - range.start,
            slice: &self.vec[range.start / 8..(range.end + 7) / 8],
        }
    }

    #[inline]
    pub fn slice_mut(&mut self, range: Range<usize>) -> BitSliceMut<'_> {
        BitSliceMut {
            offset: (range.start % 8) as _,
            len: range.end - range.start,
            slice: &mut self.vec[range.start / 8..(range.end + 7) / 8],
        }
    }

    #[inline]
    pub fn as_slice(&self) -> BitSlice<'_> {
        BitSlice {
            offset: 0,
            len: self.len,
            slice: &self.vec,
        }
    }

    #[inline]
    pub fn as_slice_mut(&mut self) -> BitSliceMut<'_> {
        BitSliceMut {
            offset: 0,
            len: self.len,
            slice: &mut self.vec,
        }
    }
}

impl Extend<bool> for BitVec {
    #[inline]
    fn extend<I: IntoIterator<Item = bool>>(&mut self, iter: I) {
        for b in iter {
            self.push(b);
        }
    }
}

impl PartialEq for BitVec {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.vec == other.vec
    }
}

impl<A: AsRef<[bool]>> From<A> for BitVec {
    #[inline]
    fn from(value: A) -> Self {
        let slice = value.as_ref();
        let mut vec = Self::with_capacity(slice.len());
        for v in slice {
            vec.push(*v)
        }
        vec
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BitSlice<'slice> {
    offset: u8,
    len: usize,
    slice: &'slice [u8],
}

impl<'slice> BitAnd for BitSlice<'slice> {
    type Output = BitVec;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        BitVec {
            vec: self
                .slice
                .iter()
                .zip(rhs.slice)
                .map(|(l, r)| l & r)
                .collect(),
            len: min(self.len, rhs.len),
        }
    }
}

impl<'slice> BitSlice<'slice> {
    #[inline]
    pub fn get(&self, n: usize) -> Option<bool> {
        if n >= self.len {
            return None;
        }
        Some(get_bit(self.slice, n + self.offset as usize))
    }

    #[inline]
    pub fn to_vec(self) -> BitVec {
        let mut vec = BitVec::with_capacity(self.len);
        for b in self {
            vec.push(b);
        }
        vec
    }

    #[inline]
    pub fn slice(&self, range: Range<usize>) -> BitSlice<'_> {
        let start = range.start + self.offset as usize;
        let end = range.end + self.offset as usize;
        BitSlice {
            offset: (start % 8) as _,
            len: end - start,
            slice: &self.slice[start / 8..(end + 7) / 8],
        }
    }
}

#[derive(Debug)]
pub struct BitSliceMut<'slice> {
    offset: u8,
    len: usize,
    slice: &'slice mut [u8],
}

impl<'slice> BitSliceMut<'slice> {
    #[inline]
    pub fn as_ref<'r>(self) -> BitSlice<'r>
    where
        'slice: 'r,
    {
        BitSlice {
            offset: self.offset,
            len: self.len,
            slice: &*self.slice,
        }
    }

    #[inline]
    pub fn set(&mut self, index: usize, value: bool) {
        set_bit(self.slice, index + self.offset as usize, value)
    }
}

#[cfg(test)]
mod tests {
    use super::BitVec;

    #[test]
    fn get_set() {
        let origin = [
            true, false, false, true, false, true, false, false, true, true, false, true, false,
            true, false, false,
        ];

        let v = BitVec::from(&origin[..]);

        for (id, o) in origin.iter().enumerate() {
            assert_eq!(v.get(id).unwrap(), *o);
        }
    }

    #[test]
    fn bit_and() {
        let left = [
            true, false, false, true, false, true, false, false, true, true, false, true, false,
            true, false, false,
        ];
        let right = [
            true, false, true, false, false, false, false, true, true, false, true, true, true,
            false, true, false,
        ];

        let lhs = BitVec::from(left);
        let rhs = BitVec::from(right);

        let mut expect = [false; 16];
        for (id, (l, r)) in left.iter().zip(right).enumerate() {
            expect[id] = l & r;
        }

        let result = lhs & rhs;

        for (id, o) in expect.iter().enumerate() {
            assert_eq!(result.get(id).unwrap(), *o);
        }
    }

    #[test]
    fn slice() {
        let vec = BitVec::from(vec![true, false, true, false, true, false]);
        let slice = vec.slice(1..5);
        assert_eq!(slice.get(0), Some(false));
        assert_eq!(slice.get(3), Some(true));
        assert_eq!(slice.get(4), None);
        let slice = slice.slice(1..3);
        assert_eq!(slice.get(0), Some(true));
        assert_eq!(slice.get(1), Some(false));
        assert_eq!(slice.get(2), None);
    }
}
