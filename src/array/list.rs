use super::Array;
use crate::{
    bitvec::BitVec,
    primitive::Primitive,
    scalar::list::{OptionList, OptionSlice, OptionSliceMut},
};

#[derive(Debug, Clone)]
pub struct OptionListArray<P> {
    validity: BitVec,
    data: Vec<P>,
    list_size: usize,
}

impl<P> OptionListArray<P> {
    #[inline]
    pub fn new(list_size: usize) -> Self {
        Self {
            validity: Default::default(),
            data: Default::default(),
            list_size,
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize, list_size: usize) -> Self {
        Self {
            validity: BitVec::with_capacity(capacity * list_size),
            data: Vec::with_capacity(capacity * list_size),
            list_size,
        }
    }
}

impl<P: Primitive> Array for OptionListArray<P> {
    type Item = OptionList<P>;

    type ItemRef<'s> = OptionSlice<'s, P>
    where
        Self: 's;

    type ItemMut<'s> = OptionSliceMut<'s, P>
    where
        Self: 's;

    #[inline]
    fn push(&mut self, item: Self::Item) {
        self.validity.extend(item.validity);
        self.data.extend(item.data);
    }

    #[inline]
    fn push_zero(&mut self) {
        for _ in 0..self.list_size {
            self.validity.push(false);
        }
        self.data.resize_with(self.list_size, Default::default);
    }

    #[inline]
    fn get(&self, id: usize) -> Option<Self::ItemRef<'_>> {
        if id * self.list_size > self.data.len() {
            None
        } else {
            Some(unsafe { self.get_unchecked(id) })
        }
    }

    #[inline]
    unsafe fn get_unchecked(&self, id: usize) -> Self::ItemRef<'_> {
        OptionSlice {
            validity: self
                .validity
                .slice(id * self.list_size..(id + 1) * self.list_size),
            data: self
                .data
                .get_unchecked(id * self.list_size..(id + 1) * self.list_size),
        }
    }

    #[inline]
    fn get_mut(&mut self, offset: usize) -> Option<Self::ItemMut<'_>> {
        if offset * self.list_size > self.data.len() {
            None
        } else {
            Some(unsafe { self.get_unchecked_mut(offset) })
        }
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, offset: usize) -> Self::ItemMut<'_> {
        OptionSliceMut {
            validity: self
                .validity
                .slice_mut(offset * self.list_size..(offset + 1) * self.list_size),
            data: self
                .data
                .get_unchecked_mut(offset * self.list_size..(offset + 1) * self.list_size),
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.data.len() / self.list_size
    }
}

#[derive(Debug, Clone)]
pub struct ListArray<P> {
    data: Vec<P>,
    offsets: Vec<usize>,
}

impl<P> Default for ListArray<P> {
    #[inline]
    fn default() -> Self {
        Self {
            data: Default::default(),
            offsets: Default::default(),
        }
    }
}

impl<P> ListArray<P> {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

impl<P: Primitive> Array for ListArray<P> {
    type Item = Vec<P>;

    type ItemRef<'s> = &'s [P]
    where
        Self: 's;

    type ItemMut<'s> = &'s mut [P]
    where
        Self: 's;

    #[inline]
    fn push(&mut self, item: Self::Item) {
        self.data.extend(item);
        self.offsets.push(self.data.len());
    }

    #[inline]
    fn push_zero(&mut self) {
        self.offsets.push(self.data.len());
    }

    #[inline]
    fn get(&self, offset: usize) -> Option<Self::ItemRef<'_>> {
        if offset < self.offsets.len() {
            Some(unsafe { self.get_unchecked(offset) })
        } else {
            None
        }
    }

    #[inline]
    unsafe fn get_unchecked(&self, offset: usize) -> Self::ItemRef<'_> {
        let end = *self.offsets.get_unchecked(offset);
        let start = if offset != 0 {
            *self.offsets.get_unchecked(offset - 1)
        } else {
            0
        };
        &self.data[start..end]
    }

    #[inline]
    fn get_mut(&mut self, offset: usize) -> Option<Self::ItemMut<'_>> {
        if offset < self.offsets.len() {
            Some(unsafe { self.get_unchecked_mut(offset) })
        } else {
            None
        }
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, offset: usize) -> Self::ItemMut<'_> {
        let end = *self.offsets.get_unchecked(offset);
        let start = if offset != 0 {
            *self.offsets.get_unchecked(offset - 1)
        } else {
            0
        };
        &mut self.data[start..end]
    }

    #[inline]
    fn len(&self) -> usize {
        self.offsets.len()
    }
}

#[derive(Default, Debug, Clone)]
pub struct ConstSizeListArray<P, const SIZE: usize> {
    data: Vec<P>,
}

impl<P: Primitive, const SIZE: usize> Array for ConstSizeListArray<P, SIZE> {
    type Item = [P; SIZE];
    type ItemRef<'a> = &'a [P; SIZE];
    type ItemMut<'a> = &'a mut [P; SIZE];

    #[inline]
    fn push(&mut self, value: Self::Item) {
        self.data.extend_from_slice(&value[..])
    }

    #[inline]
    fn push_zero(&mut self) {
        self.data.extend_from_slice(&[P::default(); SIZE][..])
    }

    #[inline]
    fn get(&self, id: usize) -> Option<Self::ItemRef<'_>> {
        if (id + 1) * SIZE > self.data.len() {
            return None;
        }
        Some(unsafe { self.get_unchecked(id) })
    }

    #[inline]
    unsafe fn get_unchecked(&self, id: usize) -> Self::ItemRef<'_> {
        self.data
            .get_unchecked(id * SIZE..(id + 1) * SIZE)
            .split_array_ref::<SIZE>()
            .0
    }

    #[inline]
    fn get_mut(&mut self, id: usize) -> Option<Self::ItemMut<'_>> {
        if (id + 1) * SIZE > self.data.len() {
            return None;
        }

        Some(
            self.data[id * SIZE..(id + 1) * SIZE]
                .split_array_mut::<SIZE>()
                .0,
        )
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, id: usize) -> Self::ItemMut<'_> {
        self.data
            .get_unchecked_mut(id * SIZE..(id + 1) * SIZE)
            .split_array_mut::<SIZE>()
            .0
    }

    #[inline]
    fn len(&self) -> usize {
        self.data.len() / SIZE
    }
}
