use crate::{
    compute::Compute, compute_reduce::ComputeReduce, compute_reduce1::ComputeReduce1,
    compute_with_reduction::ComputeWithReduction, reduce::Reduce,
};
use std::marker::PhantomData;

pub struct ComputeReduce0<R>(pub(super) PhantomData<R>)
where
    R: Reduce;

impl<R> ComputeReduce for ComputeReduce0<R>
where
    R: Reduce,
{
    type In<'i> = ();

    type R = R;

    type Composed<C>
        = ComputeReduce1<R, ComputeWithReduction<R, C>>
    where
        C: Compute<Out = <Self::R as Reduce>::Unit>;

    fn compute_reduce<'i>(&self, reduce: &Self::R, _: Self::In<'i>) -> <Self::R as Reduce>::Unit {
        reduce.identity()
    }

    fn compose<C>(self, other: C) -> Self::Composed<C>
    where
        C: Compute<Out = <Self::R as Reduce>::Unit>,
    {
        ComputeReduce1(PhantomData, ComputeWithReduction(PhantomData, other))
    }
}
