use crate::type_sequence::TypeSequence;
use std::marker::PhantomData;

pub trait InputBuilderGeneric: Sized {
    type Left;

    type Right: TypeSequence;

    type X;

    type ComposedWith<I>: InputBuilderGeneric;

    fn compose(self, value: Self::Left) -> Self::ComposedWith<Self::Left> {
        todo!()
    }

    fn value(self) -> Self::X;
}

pub struct InputBuilder0<Left, Right>(PhantomData<(Left, Right)>)
where
    Right: TypeSequence;

impl<Left, Right> Default for InputBuilder0<Left, Right>
where
    Right: TypeSequence,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<Left, Right> InputBuilderGeneric for InputBuilder0<Left, Right>
where
    Right: TypeSequence,
{
    type Left = Left;

    type Right = Right;

    type X = ();

    type ComposedWith<I> = InputBuilder1<
        <Self::Right as TypeSequence>::SplitLeft,
        <Self::Right as TypeSequence>::SplitRight,
        I,
    >;

    fn compose(self, value: Self::Left) -> Self::ComposedWith<Self::Left> {
        InputBuilder1(PhantomData, value)
    }

    fn value(self) -> Self::X {
        ()
    }
}

pub struct InputBuilder1<Left, Right, X>(PhantomData<(Left, Right)>, X)
where
    Right: TypeSequence;

impl<Left, Right, X> InputBuilderGeneric for InputBuilder1<Left, Right, X>
where
    Right: TypeSequence,
{
    type Left = Left;

    type Right = Right;

    type X = X;

    type ComposedWith<I> = InputBuilder2<
        <Self::Right as TypeSequence>::SplitLeft,
        <Self::Right as TypeSequence>::SplitRight,
        Self::X,
        I,
    >;

    fn compose(self, value: Self::Left) -> Self::ComposedWith<Self::Left> {
        InputBuilder2(PhantomData, self.1, value)
    }

    fn value(self) -> Self::X {
        self.1
    }
}

pub struct InputBuilder2<Left, Right, X1, X2>(PhantomData<(Left, Right)>, X1, X2)
where
    Right: TypeSequence;

impl<Left, Right, X1, X2> InputBuilderGeneric for InputBuilder2<Left, Right, X1, X2>
where
    Right: TypeSequence,
{
    type Left = Left;

    type Right = Right;

    type X = (X1, X2);

    type ComposedWith<I> = InputBuilder2<
        <Self::Right as TypeSequence>::SplitLeft,
        <Self::Right as TypeSequence>::SplitRight,
        Self::X,
        I,
    >;

    fn compose(self, value: Self::Left) -> Self::ComposedWith<Self::Left> {
        InputBuilder2(PhantomData, (self.1, self.2), value)
    }

    fn value(self) -> Self::X {
        (self.1, self.2)
    }
}
