pub trait TypeSeq: Default {
    type Left;

    type RightSeq: TypeSeq;

    type AddToLeft<T>: TypeSeq;
}
