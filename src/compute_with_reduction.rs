use crate::{
    compute::Compute,
    compute_reduce::ComputeReduce,
    compute_reduce2::ComputeReduce2,
    reduce::Reduce,
    type_sequence::{End, One, TypeSequence},
};
use std::marker::PhantomData;

pub struct ComputeWithReduction<R, C1>(pub(super) PhantomData<R>, pub(super) C1)
where
    R: Reduce,
    C1: Compute<Out = R::Unit>;

impl<R, C1> ComputeReduce for ComputeWithReduction<R, C1>
where
    R: Reduce,
    C1: Compute<Out = R::Unit>,
{
    type In<'i> = C1::In<'i>;

    type R = R;

    type Composed<C2>
        = ComputeReduce2<
        R,
        Self,
        ComputeWithReduction<R, C2>,
        <One<C1> as TypeSequence>::ComposeWith<C2>,
    >
    where
        C2: Compute<Out = <Self::R as Reduce>::Unit>;

    type ComputeSequence = One<C1>;

    fn compute_reduce<'i>(&self, _: &Self::R, input: Self::In<'i>) -> <Self::R as Reduce>::Unit {
        self.1.compute(input)
    }

    fn compose<C2>(self, other: C2) -> Self::Composed<C2>
    where
        C2: Compute<Out = <Self::R as Reduce>::Unit>,
    {
        ComputeReduce2(PhantomData, self, ComputeWithReduction(PhantomData, other))
    }
}
