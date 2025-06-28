use super::com_red::ReducibleComputation;
use crate::{Computation, Reduction, compute_reduce::many::ReducibleComputationMany};
use core::marker::PhantomData;
use orx_meta::queue::One;

/// A [`ReducibleComputation`] with one composed computation.
pub struct ReducibleComputationOne<R, C1> {
    c1: C1,
    p: PhantomData<R>,
}

impl<R, C1> ReducibleComputationOne<R, C1> {
    pub(super) fn new(computation: C1) -> Self {
        Self {
            c1: computation,
            p: PhantomData,
        }
    }
}

impl<R, C1> ReducibleComputation for ReducibleComputationOne<R, C1>
where
    R: Reduction,
    C1: Computation<Out = R::Unit>,
{
    type R = R;

    type In<'i> = C1::In<'i>;

    type InQueue<'i> = One<Self::In<'i>>;

    type Compose<C2>
        = ReducibleComputationMany<R, Self, ReducibleComputationOne<R, C2>>
    where
        C2: Computation<Out = <Self::R as Reduction>::Unit>;

    fn compose<C>(self, other: C) -> Self::Compose<C>
    where
        C: Computation<Out = <Self::R as Reduction>::Unit>,
    {
        ReducibleComputationMany::new(self, ReducibleComputationOne::new(other))
    }

    #[inline(always)]
    fn compute_reduce<'i>(&self, _: &Self::R, input: Self::In<'i>) -> <Self::R as Reduction>::Unit {
        self.c1.compute(input)
    }
}
