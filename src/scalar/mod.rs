pub mod list;
pub mod primitive;

pub trait Scalar: 'static + Sized {
    type Ref<'r>: ScalarRef<'r, Owned = Self>
    where
        Self: 'r;

    type Mut<'r>: ScalarMut<'r, Owned = Self>
    where
        Self: 'r;

    fn as_ref(&self) -> Self::Ref<'_>;

    fn as_mut(&mut self) -> Self::Mut<'_>;
}

pub trait ScalarRef<'r> {
    type Owned: Scalar;
}

pub trait ScalarMut<'r> {
    type Owned: Scalar;

    #[allow(clippy::wrong_self_convention)]
    fn as_ref<'s>(self) -> <Self::Owned as Scalar>::Ref<'s>
    where
        'r: 's;
}

impl<'r, T> ScalarRef<'r> for &'r T
where
    T: Scalar,
{
    type Owned = T;
}

impl<'r, T> ScalarMut<'r> for &'r mut T
where
    T: for<'a> Scalar<Ref<'a> = &'a T>,
{
    type Owned = T;

    #[inline]
    fn as_ref<'s>(self) -> <Self::Owned as Scalar>::Ref<'s>
    where
        'r: 's,
    {
        &*self
    }
}

impl<T: 'static> Scalar for Vec<T> {
    type Ref<'r> = &'r [T]
    where
        Self: 'r;

    type Mut<'r> = &'r mut [T]
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

impl<'r, T: 'static> ScalarRef<'r> for &'r [T] {
    type Owned = Vec<T>;
}

impl<'r, T: 'static> ScalarMut<'r> for &'r mut [T] {
    type Owned = Vec<T>;

    #[inline]
    fn as_ref<'s>(self) -> <Self::Owned as Scalar>::Ref<'s>
    where
        'r: 's,
    {
        self
    }
}

impl<S: Scalar> Scalar for Option<S> {
    type Ref<'r> = Option<S::Ref<'r>>
    where
        Self: 'r;

    type Mut<'r> = Option<S::Mut<'r>>
    where
        Self: 'r;

    #[inline]
    fn as_ref(&self) -> Self::Ref<'_> {
        self.as_ref().map(<S as Scalar>::as_ref)
    }

    #[inline]
    fn as_mut(&mut self) -> Self::Mut<'_> {
        self.as_mut().map(<S as Scalar>::as_mut)
    }
}

impl<'r, S: ScalarRef<'r>> ScalarRef<'r> for Option<S> {
    type Owned = Option<S::Owned>;
}

impl<'r, S: ScalarMut<'r>> ScalarMut<'r> for Option<S> {
    type Owned = Option<S::Owned>;

    #[inline]
    fn as_ref<'s>(self) -> <Self::Owned as Scalar>::Ref<'s>
    where
        'r: 's,
    {
        self.map(S::as_ref)
    }
}

#[cfg(test)]
mod tests {
    use crate::scalar::{list::OptionList, Scalar};

    #[test]
    fn test_list() {
        let list = OptionList::from(vec![None, Some(1)]);
        assert_eq!(list.as_ref().get(0), Some(None));
        assert_eq!(list.as_ref().get(1).map(|s| s.cloned()), Some(Some(1)));
        assert_eq!(list.as_ref().get(2), None);
    }
}
