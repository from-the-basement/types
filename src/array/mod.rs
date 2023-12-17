pub mod id;
pub mod list;
pub mod primitive;
pub mod slotmap;

pub use list::OptionListArray;

use crate::scalar::{Scalar, ScalarMut, ScalarRef};

pub trait Array: 'static + Sized {
    type Item: for<'s> Scalar<Ref<'s> = Self::ItemRef<'s>, Mut<'s> = Self::ItemMut<'s>>;

    type ItemRef<'s>: ScalarRef<'s, Owned = Self::Item>
    where
        Self: 's;

    type ItemMut<'s>: ScalarMut<'s, Owned = Self::Item>
    where
        Self: 's;

    fn push(&mut self, item: Self::Item);

    fn push_zero(&mut self);

    fn get(&self, offset: usize) -> Option<Self::ItemRef<'_>>;

    /// # Safety
    ///
    /// This function is unsafe because it does not perform bounds checking.
    unsafe fn get_unchecked(&self, offset: usize) -> Self::ItemRef<'_>;

    fn get_mut(&mut self, offset: usize) -> Option<Self::ItemMut<'_>>;

    /// # Safety
    ///
    /// This function is unsafe because it does not perform bounds checking.
    unsafe fn get_unchecked_mut(&mut self, offset: usize) -> Self::ItemMut<'_>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::{
        id::IdArray,
        list::{ListArray, OptionListArray},
        primitive::PrimitiveArray,
        Array,
    };
    use crate::scalar::list::OptionList;

    #[test]
    fn list_array() {
        let mut array = ListArray::new();
        array.push(vec![1, 2]);
        array.push(vec![2, 3, 4]);
        assert_eq!(array.get(0), Some(&[1, 2][..]));
        assert_eq!(array.get(1), Some(&[2, 3, 4][..]));
    }

    #[test]
    fn nullable_array() {
        let mut array = OptionListArray::new(2);
        array.push(OptionList::<_>::from(vec![None, Some(1)]));
        array.push(OptionList::<_>::from(vec![Some(2), Some(3)]));
        assert_eq!(array.get(0).unwrap().get(0), Some(None));
        assert_eq!(array.get(0).unwrap().get(1), Some(Some(&1)));
        assert_eq!(array.get(1).unwrap().get(0), Some(Some(&2)));
        assert_eq!(array.get(1).unwrap().get(1), Some(Some(&3)));
        let mut ref_mut = array.get_mut(0).unwrap();
        ref_mut.set(0, Some(1));
        assert_eq!(array.get(0).unwrap().get(0), Some(Some(&1)));
    }

    #[test]
    fn id_array() {
        let mut array = IdArray::<ListArray<u8>>::new(ListArray::<u8>::new());
        array.push(Some(Vec::from("foo")));
        array.push(Some(Vec::from("bar")));
        array.push(Some(Vec::from("quaz")));
        array.push(Some(Vec::from("bar")));
        assert_eq!(array.get(0), Some(Some("foo".as_ref())));
        assert_eq!(array.get(1), Some(Some("bar".as_ref())));
        assert_eq!(array.get(2), Some(Some("quaz".as_ref())));
        assert_eq!(
            array.get(3).unwrap().unwrap().as_ptr(),
            array.get(1).unwrap().unwrap().as_ptr()
        );
    }

    #[test]
    fn primitive_array() {
        let mut array = PrimitiveArray::new();
        array.push(1);
        array.push(2);
        array.push(3);
        assert_eq!(array.get(0), Some(&1));
        assert_eq!(array.get(1), Some(&2));
        assert_eq!(array.get(2), Some(&3));
    }
}
