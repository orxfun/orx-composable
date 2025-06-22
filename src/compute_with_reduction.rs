use crate::{
    compute::Compute, compute_reduce::ComputeReduce, compute_reduce2::ComputeReduce2,
    reduce::Reduce,
};
use std::marker::PhantomData;

pub struct ComputeWithReduction<R, C>(PhantomData<R>, C)
where
    R: Reduce,
    C: Compute<Out = R::Unit>;

impl<R, C> ComputeReduce for ComputeWithReduction<R, C>
where
    R: Reduce,
    C: Compute<Out = R::Unit>,
{
    type In<'i> = C::In<'i>;

    type R = R;

    type Composed<C2>
        = ComputeReduce2<R, Self, C2>
    where
        C2: ComputeReduce<R = Self::R>;

    fn compute_reduce<'i>(&self, _: &Self::R, input: Self::In<'i>) -> <Self::R as Reduce>::Unit {
        self.1.compute(input)
    }

    fn compose<C2>(self, other: C2) -> Self::Composed<C2>
    where
        C2: ComputeReduce<R = Self::R>,
    {
        ComputeReduce2(PhantomData, self, other)
    }
}
