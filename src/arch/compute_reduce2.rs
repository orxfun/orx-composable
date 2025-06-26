use crate::{
    compute::Compute,
    compute_reduce::ReducibleComputation,
    compute_with_reduction::ComputeWithReduction,
    reduce::Reduce,
    type_sequence::{End, TypeSequence},
};
use std::marker::PhantomData;

pub struct ReducibleComputation2<R, C1, C2, S>(
    pub(super) PhantomData<(R, S)>,
    pub(super) C1,
    pub(super) C2,
)
where
    R: Reduce,
    C1: ReducibleComputation<R = R>,
    C2: ReducibleComputation<R = R>,
    S: TypeSequence;

impl<R, C1, C2, S> ReducibleComputation for ReducibleComputation2<R, C1, C2, S>
where
    R: Reduce,
    C1: ReducibleComputation<R = R>,
    C2: ReducibleComputation<R = R>,
    S: TypeSequence,
{
    type In<'i> = (C1::In<'i>, C2::In<'i>);

    type R = R;

    type Composed<C>
        = ReducibleComputation2<R, Self, ComputeWithReduction<R, C>, S::ComposeWith<C>>
    where
        C: Compute<Out = <Self::R as Reduce>::Unit>;

    type ComputeSequence = S;

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
        C: Compute<Out = <Self::R as Reduce>::Unit>,
    {
        ReducibleComputation2(PhantomData, self, ComputeWithReduction(PhantomData, other))
    }
}
