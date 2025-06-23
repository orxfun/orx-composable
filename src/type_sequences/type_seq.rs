pub trait TypeSeq: Default {
    type Left;

    type RightSeq: TypeSeq;

    type AddToRight<X>: TypeSeq;
}
