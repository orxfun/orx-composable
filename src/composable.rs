use crate::{
    compute::Compute, compute_reduce::ComputeReduce, compute_reduce0::ComputeReduce0,
    input_builder0::InputBuilder0, reduce::Reduce,
};
use std::marker::PhantomData;

pub struct Composable<R, C>(R, C)
where
    R: Reduce,
    C: ComputeReduce<R = R>;

impl<R> Composable<R, ComputeReduce0<R>>
where
    R: Reduce,
{
    pub fn new(reduce: R) -> Self {
        Self(reduce, ComputeReduce0(PhantomData))
    }
}

impl<R, C> Composable<R, C>
where
    R: Reduce,
    C: ComputeReduce<R = R>,
{
    pub fn compose<C2>(self, other: C2) -> Composable<R, C::Composed<C2>>
    where
        C2: Compute<Out = R::Unit>,
    {
        Composable(self.0, self.1.compose(other))
    }

    pub fn input_builder(&self) -> InputBuilder0 {
        InputBuilder0
    }
}

impl<R, C> Compute for Composable<R, C>
where
    R: Reduce,
    C: ComputeReduce<R = R>,
{
    type In<'i> = C::In<'i>;

    type Out = R::Unit;

    fn compute(&self, input: Self::In<'_>) -> Self::Out {
        self.1.compute_reduce(&self.0, input)
    }
}
