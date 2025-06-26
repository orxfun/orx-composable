use super::com_red::ReducibleComputation;
use crate::{Computation, Reduction, compute_reduce::one::ReducibleComputationOne};
use core::marker::PhantomData;
use orx_meta_queue::MetaQueue;

/// A [`ReducibleComputation`] with two or more composed computation.
pub struct ReducibleComputationMany<R, C1, C2> {
    c1: C1,
    c2: C2,
    p: PhantomData<R>,
}

impl<R, C1, C2> ReducibleComputationMany<R, C1, C2> {
    pub(super) fn new(c1: C1, c2: C2) -> Self {
        Self {
            c1,
            c2,
            p: PhantomData,
        }
    }
}

impl<R, C1, C2> ReducibleComputation for ReducibleComputationMany<R, C1, C2>
where
    R: Reduction,
    C1: ReducibleComputation<R = R>,
    C2: ReducibleComputation<R = R>,
{
    type R = R;

    type In<'i> = (C1::In<'i>, C2::In<'i>);

    type InQueue<'i> = <C1::InQueue<'i> as MetaQueue>::Extend<C2::InQueue<'i>>;

    type Compose<C>
        = ReducibleComputationMany<R, Self, ReducibleComputationOne<R, C>>
    where
        C: Computation<Out = <Self::R as Reduction>::Unit>;

    fn compose<C>(self, other: C) -> Self::Compose<C>
    where
        C: Computation<Out = <Self::R as Reduction>::Unit>,
    {
        ReducibleComputationMany::new(self, ReducibleComputationOne::<R, _>::new(other))
    }

    fn compute_reduce<'i>(
        &self,
        reduction: &Self::R,
        (in1, in2): Self::In<'i>,
    ) -> <Self::R as Reduction>::Unit {
        reduction.reduce(
            self.c1.compute_reduce(reduction, in1),
            self.c2.compute_reduce(reduction, in2),
        )
    }
}
