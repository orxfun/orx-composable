use super::type_seq::TypeSeq;
use std::marker::PhantomData;

pub struct NonEmptyTypeSeq<L, R> {
    p: PhantomData<(L, R)>,
}

impl<L, R> Default for NonEmptyTypeSeq<L, R> {
    fn default() -> Self {
        Self {
            p: Default::default(),
        }
    }
}

impl<L, R> TypeSeq for NonEmptyTypeSeq<L, R> {
    type Left = L;

    type Right = R;

    type AddToLeft<T> = NonEmptyTypeSeq<T, Self>;

    type AddToRight<T> = NonEmptyTypeSeq<Self, T>;
}
