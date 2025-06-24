use crate::type_sequences::{never::Never, non_empty::NonEmptyTypeSeq, type_seq::TypeSeq};

#[derive(Default)]
pub struct EmptyTypeSeq;

impl TypeSeq for EmptyTypeSeq {
    type Left = Never;

    type RightSeq = Self;

    type AddToLeft<T> = NonEmptyTypeSeq<T, Self>;

    type AddToRight<T> = NonEmptyTypeSeq<Self, T>;
}
