use std::marker::PhantomData;

pub trait TypeSequence: Default {
    type Next: TypeSequence;

    type ComposeWith<X>: TypeSequence;
}

#[derive(Default)]
pub struct End;

impl TypeSequence for End {
    type Next = Self;

    type ComposeWith<X> = One<X>;
}

pub struct One<T>(PhantomData<T>);

impl<T> TypeSequence for One<T> {
    type Next = End;

    type ComposeWith<X> = Many<Self, One<X>>;
}

impl<T> Default for One<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub struct Many<T, U>(PhantomData<(T, U)>)
where
    T: TypeSequence,
    U: TypeSequence;

impl<T, U> TypeSequence for Many<T, U>
where
    T: TypeSequence,
    U: TypeSequence,
{
    type Next = End;

    type ComposeWith<X> = Many<T, U::ComposeWith<X>>;
}

impl<T, U> Default for Many<T, U>
where
    T: TypeSequence,
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
