use super::Array;
use crate::primitive::Primitive;

#[derive(Default, Debug, Clone)]
pub struct PrimitiveArray<P> {
    data: Vec<P>,
}

impl<P: Primitive> PrimitiveArray<P> {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

impl<P: Primitive> Array for PrimitiveArray<P> {
    type Item = P;
    type ItemRef<'a> = &'a P;
    type ItemMut<'a> = &'a mut P;

    #[inline]
    fn push(&mut self, item: Self::Item) {
        self.data.push(item);
    }

    #[inline]
    fn push_zero(&mut self) {
        self.push(Default::default())
    }

    #[inline]
    fn get(&self, offset: usize) -> Option<Self::ItemRef<'_>> {
        if self.len() <= offset {
            None
        } else {
            Some(unsafe { self.get_unchecked(offset) })
        }
    }

    #[inline]
    unsafe fn get_unchecked(&self, offset: usize) -> Self::ItemRef<'_> {
        self.data.get_unchecked(offset)
    }

    #[inline]
    fn get_mut(&mut self, offset: usize) -> Option<Self::ItemMut<'_>> {
        if self.len() <= offset {
            None
        } else {
            Some(unsafe { self.get_unchecked_mut(offset) })
        }
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, offset: usize) -> Self::ItemMut<'_> {
        self.data.get_unchecked_mut(offset)
    }

    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }
}
