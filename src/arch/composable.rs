use crate::{
    compute::Compute, compute_reduce::ReducibleComputation, compute_reduce0::ReducibleComputation0,
    input_builder0_zzz::InputBuilder0Zzz, reduce::Reduce,
};
use std::marker::PhantomData;

pub struct Composable<R, C>(R, C)
where
    R: Reduce,
    C: ReducibleComputation<R = R>;

impl<R> Composable<R, ReducibleComputation0<R>>
where
    R: Reduce,
{
    pub fn new(reduce: R) -> Self {
        Self(reduce, ReducibleComputation0(PhantomData))
    }
}

impl<R, C> Composable<R, C>
where
    R: Reduce,
    C: ReducibleComputation<R = R>,
{
    pub fn compose<C2>(self, other: C2) -> Composable<R, C::Composed<C2>>
    where
        C2: Compute<Out = R::Unit>,
    {
        Composable(self.0, self.1.compose(other))
    }

    pub fn input_builder(&self) -> InputBuilder0Zzz {
        InputBuilder0Zzz
    }
}

impl<R, C> Compute for Composable<R, C>
where
    R: Reduce,
    C: ReducibleComputation<R = R>,
{
    type In<'i> = C::In<'i>;

    type Out = R::Unit;

    fn compute(&self, input: Self::In<'_>) -> Self::Out {
        self.1.compute_reduce(&self.0, input)
    }
}
