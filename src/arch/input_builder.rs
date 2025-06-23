use crate::{
    Compute,
    type_sequence::{TypeSequence, TypeSequenceEnd},
};
use std::marker::PhantomData;

pub trait InputBuilder: Sized {
    type Left: Compute;

    type Right: TypeSequence;

    type X;

    type ComposedWith<I>: InputBuilder;

    fn compose(self, value: Self::Left) -> Self::ComposedWith<Self::Left>;

    fn value(self) -> Self::X
    where
        Self::Left: TypeSequenceEnd;
}

pub struct InputBuilder0<Left, Right>(PhantomData<(Left, Right)>)
where
    Left: Compute,
    Right: TypeSequence;

impl<Left, Right> Default for InputBuilder0<Left, Right>
where
    Left: Compute,
    Right: TypeSequence,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<Left, Right> InputBuilder for InputBuilder0<Left, Right>
where
    Left: Compute,
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
    Left: Compute,
    Right: TypeSequence;

impl<Left, Right, X> InputBuilder for InputBuilder1<Left, Right, X>
where
    Left: Compute,
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
    Left: Compute,
    Right: TypeSequence;

impl<Left, Right, X1, X2> InputBuilder for InputBuilder2<Left, Right, X1, X2>
where
    Left: Compute,
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

#[cfg(test)]
mod tests {
    use std::any::type_name;

    use super::*;
    use crate::{
        compute_reduce::ComputeReduce,
        compute_reduce2::ComputeReduce2,
        compute_with_reduction::ComputeWithReduction,
        type_sequence::{Many, One},
        *,
    };

    #[derive(Default)]
    struct And;

    impl Reduce for And {
        type Unit = bool;

        fn identity(&self) -> Self::Unit {
            true
        }

        fn reduce(&self, a: Self::Unit, b: Self::Unit) -> Self::Unit {
            a && b
        }
    }

    impl Compute for u32 {
        type In<'i> = u32;

        type Out = bool;

        fn compute(&self, input: Self::In<'_>) -> Self::Out {
            todo!()
        }
    }
    impl Compute for char {
        type In<'i> = char;

        type Out = bool;

        fn compute(&self, input: Self::In<'_>) -> Self::Out {
            todo!()
        }
    }
    impl Compute for bool {
        type In<'i> = bool;

        type Out = bool;

        fn compute(&self, input: Self::In<'_>) -> Self::Out {
            todo!()
        }
    }
    impl Compute for String {
        type In<'i> = String;

        type Out = bool;

        fn compute(&self, input: Self::In<'_>) -> Self::Out {
            todo!()
        }
    }

    #[test]
    fn abc() {
        let health_rules = Composable::new(And)
            .compose(42u32)
            .compose('x')
            .compose(true)
            .compose(String::from("abc"));

        type A = ComputeReduce2<
            And,
            ComputeReduce2<
                And,
                ComputeReduce2<
                    And,
                    ComputeWithReduction<And, u32>,
                    ComputeWithReduction<And, char>,
                    Many<u32, One<char>>,
                >,
                ComputeWithReduction<And, bool>,
                Many<u32, Many<char, One<bool>>>,
            >,
            ComputeWithReduction<And, String>,
            Many<u32, Many<char, Many<bool, One<String>>>>,
        >;

        // type X<'i> = <A as ComputeReduce>::ComputeSequence;
        println!("{:?}", type_name::<<A as ComputeReduce>::ComputeSequence>());

        type X = Many<u32, Many<char, Many<bool, One<String>>>>;
        type Left = <X as TypeSequence>::SplitLeft;
        type Right = <X as TypeSequence>::SplitRight;

        let b = InputBuilder0::<Left, Right>::default();
        let b = b.compose(42u32);
        let b = b.compose('x');
        let b = b.compose(true);
        let b = b.compose(String::from("abc"));
        let value = b.value();

        let b = InputBuilder0::<Left, Right>::default();
        let value = b
            .compose(3)
            .compose('x')
            .compose(true)
            .compose(String::from("x"))
            .value();

        dbg!(value);

        assert_eq!(1, 2);
    }
}
