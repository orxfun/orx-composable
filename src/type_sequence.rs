use std::marker::PhantomData;

pub trait TypeSequence: Default {
    type ComposeWith<X>: TypeSequence;

    type SplitLeft;

    type SplitRight: TypeSequence;
}

#[derive(Default)]
pub struct End;

impl TypeSequence for End {
    type ComposeWith<X> = One<X>;

    type SplitLeft = Self;

    type SplitRight = Self;
}

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

    type In;

    fn add(
        self,
        value: Self::Left,
    ) -> impl InputBuilder<
        Left = <Self::Right as TypeSequence>::SplitLeft,
        Right = <Self::Right as TypeSequence>::SplitRight,
        In = (Self::In, Self::Left),
    >;

    fn value(self) -> Self::In
    where
        Self: InputBuilder<Left = End>;
}
