use std::hash::Hash;

use super::{slotmap::SlotMap, Array};

#[derive(Debug, Clone)]
pub struct IdArray<A: Array> {
    values: SlotMap<A>,
    data: Vec<usize>,
}

impl<A: Array> IdArray<A> {
    #[inline]
    pub fn new(array: A) -> Self {
        Self {
            values: SlotMap::new(array),
            data: Vec::new(),
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize, array: A) -> Self {
        Self {
            values: SlotMap::new(array),
            data: Vec::<usize>::with_capacity(capacity),
        }
    }
}

impl<A: Array> IdArray<A>
where
    for<'a, 'b> A::ItemRef<'a>: PartialEq<A::ItemRef<'b>>,
    for<'a> A::ItemRef<'a>: Hash,
{
    #[inline]
    pub fn lookup_id(&self, value: A::ItemRef<'_>) -> Option<usize> {
        self.values.lookup(value)
    }

    #[inline]
    pub fn push_and_get_id(&mut self, value: <Self as Array>::Item) -> usize {
        match value {
            Some(value) => {
                let valud_id = self.values.lookup_or_insert(value);
                self.data.push(valud_id);
                valud_id
            }
            None => {
                self.push_zero();
                0
            }
        }
    }
}

impl<A: Array> Array for IdArray<A>
where
    for<'a, 'b> A::ItemRef<'a>: PartialEq<A::ItemRef<'b>>,
    for<'a> A::ItemRef<'a>: Hash,
{
    type Item = Option<A::Item>;

    type ItemRef<'s> = Option<A::ItemRef<'s>>
    where
        Self: 's;

    type ItemMut<'s> = Option<A::ItemMut<'s>>
    where
        Self: 's;

    #[inline]
    fn push(&mut self, item: Self::Item) {
        match item {
            Some(item) => {
                self.data.push(self.values.lookup_or_insert(item));
            }
            None => {
                self.push_zero();
            }
        }
    }

    #[inline]
    fn push_zero(&mut self) {
        self.data.push(0);
    }

    #[inline]
    fn get(&self, offset: usize) -> Option<Self::ItemRef<'_>> {
        let id = self.data.get(offset)?;
        self.values.get(*id)
    }

    #[inline]
    unsafe fn get_unchecked(&self, offset: usize) -> Self::ItemRef<'_> {
        self.values.get_unchecked(self.data[offset])
    }

    #[inline]
    fn get_mut(&mut self, offset: usize) -> Option<Self::ItemMut<'_>> {
        let id = self.data.get(offset)?;
        self.values.get_mut(*id)
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, offset: usize) -> Self::ItemMut<'_> {
        self.values.get_unchecked_mut(self.data[offset])
    }

    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }
}
