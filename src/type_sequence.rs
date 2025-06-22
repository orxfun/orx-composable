use std::marker::PhantomData;

pub trait TypeSequence {
    type Next: TypeSequence;

    type ComposeWith<X>: TypeSequence;
}

pub struct TypeSequenceEnd {}

impl TypeSequence for TypeSequenceEnd {
    type Next = Self;

    type ComposeWith<X> = TypeSequenceOne<X>;
}

pub struct TypeSequenceOne<T>(PhantomData<T>);

impl<T> TypeSequence for TypeSequenceOne<T> {
    type Next = TypeSequenceEnd;

    type ComposeWith<X> = TypeSequenceMany<Self, TypeSequenceOne<X>>;
}

pub struct TypeSequenceMany<T, U>(PhantomData<(T, U)>)
where
    T: TypeSequence,
    U: TypeSequence;

impl<T, U> TypeSequence for TypeSequenceMany<T, U>
where
    T: TypeSequence,
    U: TypeSequence,
{
    type Next = TypeSequenceEnd;

    type ComposeWith<X> = TypeSequenceMany<T, U::ComposeWith<X>>;
}
