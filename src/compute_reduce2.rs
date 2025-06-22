use crate::{
    compute::Compute, compute_reduce::ComputeReduce, compute_with_reduction::ComputeWithReduction,
    reduce::Reduce,
};
use std::marker::PhantomData;

pub struct ComputeReduce2<R, C1, C2>(pub(super) PhantomData<R>, pub(super) C1, pub(super) C2)
where
    R: Reduce,
    C1: ComputeReduce<R = R>,
    C2: ComputeReduce<R = R>;

impl<R, C1, C2> ComputeReduce for ComputeReduce2<R, C1, C2>
where
    R: Reduce,
    C1: ComputeReduce<R = R>,
    C2: ComputeReduce<R = R>,
{
    type In<'i> = (C1::In<'i>, C2::In<'i>);

    type R = R;

    type Composed<C>
        = ComputeReduce2<R, Self, C>
    where
        C: ComputeReduce<R = Self::R>;

    type Composed2<C>
        = ComputeReduce2<R, Self, ComputeWithReduction<R, C>>
    where
        C: Compute<Out = <Self::R as Reduce>::Unit>;

    fn compute_reduce<'i>(
        &self,
        reduce: &Self::R,
        (in1, in2): Self::In<'i>,
    ) -> <Self::R as Reduce>::Unit {
        reduce.reduce(
            self.1.compute_reduce(reduce, in1),
            self.2.compute_reduce(reduce, in2),
        )
    }

    fn compose<C>(self, other: C) -> Self::Composed<C>
    where
        C: ComputeReduce<R = Self::R>,
    {
        ComputeReduce2(PhantomData, self, other)
    }

    fn compose2<C>(self, other: C) -> Self::Composed2<C>
    where
        C: Compute<Out = <Self::R as Reduce>::Unit>,
    {
        ComputeReduce2(PhantomData, self, ComputeWithReduction(PhantomData, other))
    }
}
