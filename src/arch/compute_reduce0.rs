use crate::{
    compute::Compute, compute_reduce::ReducibleComputation, compute_reduce1::ReducibleComputation1,
    compute_with_reduction::ComputeWithReduction, reduce::Reduce, type_sequence::End,
};
use std::marker::PhantomData;

pub struct ReducibleComputation0<R>(pub(super) PhantomData<R>)
where
    R: Reduce;

impl<R> ReducibleComputation for ReducibleComputation0<R>
where
    R: Reduce,
{
    type In<'i> = ();

    type R = R;

    type Composed<C>
        = ReducibleComputation1<R, ComputeWithReduction<R, C>, C>
    where
        C: Compute<Out = <Self::R as Reduce>::Unit>;

    type ComputeSequence = End;

    fn compute_reduce<'i>(&self, reduce: &Self::R, _: Self::In<'i>) -> <Self::R as Reduce>::Unit {
        reduce.identity()
    }

    fn compose<C>(self, other: C) -> Self::Composed<C>
    where
        C: Compute<Out = <Self::R as Reduce>::Unit>,
    {
        ReducibleComputation1::<R, _, C>(PhantomData, ComputeWithReduction(PhantomData, other))
    }
}
