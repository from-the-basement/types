use super::Scalar;
use crate::primitive::Primitive;

impl<P: Primitive> Scalar for P {
    type Ref<'r> = &'r P
    where
        Self: 'r;

    type Mut<'r> = &'r mut P
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
