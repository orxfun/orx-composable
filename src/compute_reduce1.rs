use crate::{
    compute::Compute,
    compute_reduce::ComputeReduce,
    compute_reduce2::ComputeReduce2,
    compute_with_reduction::ComputeWithReduction,
    reduce::Reduce,
    type_sequence::{One, TypeSequence},
};
use std::marker::PhantomData;

pub struct ComputeReduce1<R, C1, C>(pub(super) PhantomData<(R, C)>, pub(super) C1)
where
    R: Reduce,
    C1: ComputeReduce<R = R>,
    C: Compute<Out = R::Unit>;

impl<R, C1, C> ComputeReduce for ComputeReduce1<R, C1, C>
where
    R: Reduce,
    C1: ComputeReduce<R = R>,
    C: Compute<Out = R::Unit>,
{
    type In<'i> = C1::In<'i>;

    type R = R;

    type Composed<C2>
        = ComputeReduce2<
        R,
        C1,
        ComputeWithReduction<R, C2>,
        <One<C> as TypeSequence>::ComposeWith<C2>,
    >
    where
        C2: Compute<Out = <Self::R as Reduce>::Unit>;

    type ComputeSequence = One<C>;

    fn compute_reduce<'i>(
        &self,
        reduce: &Self::R,
        input: Self::In<'i>,
    ) -> <Self::R as Reduce>::Unit {
        self.1.compute_reduce(reduce, input)
    }

    fn compose<C2>(self, other: C2) -> Self::Composed<C2>
    where
        C2: Compute<Out = <Self::R as Reduce>::Unit>,
    {
        ComputeReduce2(
            PhantomData,
            self.1,
            ComputeWithReduction(PhantomData, other),
        )
    }
}
