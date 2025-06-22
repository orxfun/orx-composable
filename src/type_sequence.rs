use std::marker::PhantomData;

pub enum Never {}

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

    type SplitLeft = Never;

    type SplitRight = Self;
}

impl TypeSequenceEnd for End {}

#[derive(Debug)]
pub struct One<T>(PhantomData<T>);

impl<T> TypeSequence for One<T> {
    type ComposeWith<X> = Many<T, One<X>>;

    type SplitLeft = T;

    type SplitRight = End;
}

impl<T> Default for One<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub struct Many<L, R>(PhantomData<(L, R)>)
where
    R: TypeSequence;

impl<L, R> TypeSequence for Many<L, R>
where
    R: TypeSequence,
{
    type ComposeWith<X> = Many<L, R::ComposeWith<X>>;

    type SplitLeft = L;

    type SplitRight = R;
}

impl<L, R> Default for Many<L, R>
where
    R: TypeSequence,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::input_builder::{InputBuilder, InputBuilder0};

    use super::*;

    #[test]
    fn abc() {
        let x = End;
        let y = compose_with::<_, u32>(x);
        let z = compose_with::<_, char>(y);
        let w = compose_with::<_, bool>(z);
        let t = compose_with::<_, String>(w);

        type X = Many<One<u32>, Many<One<char>, Many<One<bool>, One<String>>>>;

        type TypeSeq = Many<u32, Many<char, Many<bool, One<String>>>>;
        type Left = <TypeSeq as TypeSequence>::SplitLeft;
        type Right = <TypeSeq as TypeSequence>::SplitRight;

        let builder_x = InputBuilder0::<Left, Right>::default();
        let builder_y = builder_x.compose(42);
        let builder_z = builder_y.compose('x');
        let builder_w = builder_z.compose(true);
        let builder_t = builder_w.compose(String::from("abc"));
        let result = builder_t.value();
        dbg!(result);

        // assert_eq!(12, 33);
    }

    fn compose_with<S, Y>(_: S) -> <S as TypeSequence>::ComposeWith<Y>
    where
        S: TypeSequence,
    {
        Default::default()
    }
}
