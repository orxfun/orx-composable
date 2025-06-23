use crate::type_sequences::{never::Never, non_empty::NonEmptyTypeSeq, type_seq::TypeSeq};

#[derive(Default)]
pub struct EmptyTypeSeq;

impl TypeSeq for EmptyTypeSeq {
    type Left = Never;

    type RightSeq = Self;

    type AddToRight<X> = NonEmptyTypeSeq<X, Self>;
}
