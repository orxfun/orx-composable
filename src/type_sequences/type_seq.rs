pub trait TypeSeq: Default {
    type Left;

    type RightSeq;

    type AddToLeft<T>: TypeSeq;

    type AddToRight<T>: TypeSeq;
}
