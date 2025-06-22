use std::marker::PhantomData;

pub trait TypeSequence: Default {
    type ComposeWith<X>: TypeSequence;

    type SplitLeft;

    type SplitRight: TypeSequence;
}

pub trait TypeSequenceEnd {}

#[derive(Default)]
pub struct End;

impl TypeSequence for End {
    type ComposeWith<X> = One<X>;

    type SplitLeft = Self;

    type SplitRight = Self;
}

impl TypeSequenceEnd for End {}

pub struct One<T>(PhantomData<T>);

impl<T> TypeSequence for One<T> {
    type ComposeWith<X> = Many<Self, One<X>>;

    type SplitLeft = T;

    type SplitRight = End;
}

impl<T> Default for One<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub struct Many<T, U>(PhantomData<(T, U)>)
where
    U: TypeSequence;

impl<T, U> TypeSequence for Many<T, U>
where
    U: TypeSequence,
{
    type ComposeWith<X> = Many<T, U::ComposeWith<X>>;

    type SplitLeft = T;

    type SplitRight = U;
}

impl<T, U> Default for Many<T, U>
where
    U: TypeSequence,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abc() {
        let x = End;
        let y = compose_with::<_, u32>(x);
        let z = compose_with::<_, char>(y);
        let w = compose_with::<_, bool>(z);
    }

    fn compose_with<S, Y>(_: S) -> <S as TypeSequence>::ComposeWith<Y>
    where
        S: TypeSequence,
    {
        Default::default()
    }
}

pub trait InputBuilder: Sized {
    type Left;

    type Right: TypeSequence;

    type X;

    type ComposedWith<I>: InputBuilder;

    fn add(self, value: Self::Left) -> Self::ComposedWith<Self::Left> {
        todo!()
    }

    fn value(self) -> Self::X;
}

pub struct InputBuilder0<Left, Right>(PhantomData<(Left, Right)>)
where
    Right: TypeSequence;

impl<Left, Right> InputBuilder for InputBuilder0<Left, Right>
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

    fn add(self, value: Self::Left) -> Self::ComposedWith<Self::Left> {
        InputBuilder1(PhantomData, value)
    }

    fn value(self) -> Self::X {
        ()
    }
}

pub struct InputBuilder1<Left, Right, X>(PhantomData<(Left, Right)>, X)
where
    Right: TypeSequence;

impl<Left, Right, X> InputBuilder for InputBuilder1<Left, Right, X>
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

    fn value(self) -> Self::X {
        self.1
    }
}

pub struct InputBuilder2<Left, Right, X1, X2>(PhantomData<(Left, Right)>, X1, X2)
where
    Right: TypeSequence;

impl<Left, Right, X1, X2> InputBuilder for InputBuilder2<Left, Right, X1, X2>
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

    fn value(self) -> Self::X {
        (self.1, self.2)
    }
}
