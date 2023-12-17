use std::ops::{Range, Sub};

use super::{Scalar, ScalarMut, ScalarRef};
use crate::{
    bitvec::{BitSlice, BitSliceMut, BitVec},
    primitive::Primitive,
};

#[derive(Debug, Clone)]
pub struct OptionList<P> {
    pub(crate) validity: BitVec,
    pub(crate) data: Vec<P>,
}

impl<P: Primitive> OptionList<P> {
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            validity: BitVec::with_capacity(capacity),
            data: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn push(&mut self, item: Option<P>) {
        match item {
            Some(value) => {
                self.validity.push(true);
                self.data.push(value);
            }
            None => {
                self.validity.push(false);
                self.data.push(Default::default());
            }
        }
    }

    #[inline]
    pub fn get(&self, offset: usize) -> Option<&P> {
        match self.validity.get(offset) {
            Some(v) => match v {
                true => Some(&self.data[offset]),
                false => None,
            },
            None => None,
        }
    }
}

impl<P: Primitive> Default for OptionList<P> {
    #[inline]
    fn default() -> Self {
        Self {
            validity: Default::default(),
            data: Default::default(),
        }
    }
}

impl<T: AsRef<[Option<P>]>, P: Primitive> From<T> for OptionList<P> {
    #[inline]
    fn from(value: T) -> Self {
        let mut this = Self::with_capacity(value.as_ref().len());
        for item in value.as_ref().iter().cloned() {
            this.push(item);
        }
        this
    }
}

impl<P: Primitive> Scalar for OptionList<P> {
    type Ref<'r> = OptionSlice<'r, P>
    where
        Self: 'r;

    type Mut<'r> = OptionSliceMut<'r, P>
    where
        Self: 'r;

    #[inline]
    fn as_ref(&self) -> Self::Ref<'_> {
        OptionSlice {
            validity: self.validity.as_slice(),
            data: &self.data,
        }
    }

    #[inline]
    fn as_mut(&mut self) -> Self::Mut<'_> {
        OptionSliceMut {
            validity: self.validity.as_slice_mut(),
            data: &mut self.data,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OptionSlice<'slice, P> {
    pub(crate) validity: BitSlice<'slice>,
    pub(crate) data: &'slice [P],
}

impl<'slice, P: Primitive> OptionSlice<'slice, P> {
    #[inline]
    pub fn slice(&self, range: Range<usize>) -> OptionSlice<'_, P> {
        OptionSlice {
            validity: self.validity.slice(range.clone()),
            data: &self.data[range],
        }
    }

    #[inline]
    pub fn get(&self, n: usize) -> Option<Option<&P>> {
        if let Some(bit) = self.validity.get(n) {
            if bit {
                Some(Some(&self.data[n]))
            } else {
                Some(None)
            }
        } else {
            None
        }
    }
}

impl<'slice, P: Primitive> ScalarRef<'slice> for OptionSlice<'slice, P> {
    type Owned = OptionList<P>;
}

impl<'slice, P: Primitive + Sub<Output = P>> Sub for OptionSlice<'slice, P> {
    type Output = OptionList<P>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        #[allow(clippy::suspicious_arithmetic_impl)]
        OptionList {
            validity: self.validity & rhs.validity,
            data: self
                .data
                .iter()
                .zip(rhs.data)
                .map(|(l, r)| *l - *r)
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct OptionSliceMut<'slice, P> {
    pub(crate) validity: BitSliceMut<'slice>,
    pub(crate) data: &'slice mut [P],
}

impl<'slice, P> OptionSliceMut<'slice, P> {
    #[inline]
    pub fn set(&mut self, n: usize, value: Option<P>) {
        match value {
            Some(v) => {
                self.validity.set(n, true);
                self.data[n] = v;
            }
            None => {
                self.validity.set(n, false);
            }
        }
    }
}

impl<'slice, P: Primitive> ScalarMut<'slice> for OptionSliceMut<'slice, P> {
    type Owned = OptionList<P>;

    #[inline]
    fn as_ref<'r>(self) -> <Self::Owned as Scalar>::Ref<'r>
    where
        'slice: 'r,
    {
        OptionSlice {
            validity: self.validity.as_ref(),
            data: &*self.data,
        }
    }
}

impl<S: Scalar, const SIZE: usize> Scalar for [S; SIZE] {
    type Ref<'r> = &'r [S; SIZE]
    where
        Self: 'r;

    type Mut<'r> = &'r mut [S; SIZE]
    where
        Self: 'r;

    #[inline]
    fn as_ref(&self) -> Self::Ref<'_> {
        self
    }

    #[inline]
    fn as_mut(&mut self) -> Self::Mut<'_> {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::OptionList;
    use crate::scalar::Scalar;

    #[test]
    fn sub_option_slice() {
        let lhs = OptionList::from(vec![Some(2), None, Some(8)]);
        let rhs = OptionList::from(vec![Some(1), Some(2), Some(3)]);
        let result = lhs.as_ref() - rhs.as_ref();
        assert_eq!(result.get(0), Some(&1));
        assert_eq!(result.get(1), None);
        assert_eq!(result.get(2), Some(&5));
        assert_eq!(result.get(3), None);
    }
}
