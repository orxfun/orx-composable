pub trait TypeSeq: Default {
    type Left;

    type Right;

    type AddToLeft<T>: TypeSeq;

    type AddToRight<T>: TypeSeq;
}
