use crate::{
    compute::Compute, compute_reduce::ComputeReduce, compute_reduce2::ComputeReduce2,
    compute_with_reduction::ComputeWithReduction, reduce::Reduce,
};
use std::marker::PhantomData;

pub struct ComputeReduce1<R, C1>(pub(super) PhantomData<R>, pub(super) C1)
where
    R: Reduce,
    C1: ComputeReduce<R = R>;

impl<R, C1> ComputeReduce for ComputeReduce1<R, C1>
where
    R: Reduce,
    C1: ComputeReduce<R = R>,
{
    type In<'i> = C1::In<'i>;

    type R = R;

    type Composed<C>
        = ComputeReduce2<R, C1, C>
    where
        C: ComputeReduce<R = Self::R>;

    type Composed2<C>
        = ComputeReduce2<R, C1, ComputeWithReduction<R, C>>
    where
        C: Compute<Out = <Self::R as Reduce>::Unit>;

    fn compute_reduce<'i>(
        &self,
        reduce: &Self::R,
        input: Self::In<'i>,
    ) -> <Self::R as Reduce>::Unit {
        self.1.compute_reduce(reduce, input)
    }

    fn compose<C>(self, other: C) -> Self::Composed<C>
    where
        C: ComputeReduce<R = Self::R>,
    {
        ComputeReduce2(PhantomData, self.1, other)
    }

    fn compose2<C>(self, other: C) -> Self::Composed2<C>
    where
        C: Compute<Out = <Self::R as Reduce>::Unit>,
    {
        ComputeReduce2(
            PhantomData,
            self.1,
            ComputeWithReduction(PhantomData, other),
        )
    }
}
