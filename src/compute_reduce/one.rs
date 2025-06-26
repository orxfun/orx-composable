use super::com_red::ComputeReduce;
use crate::{Computation, Reduction, compute_reduce::many::ComputeReduceMany};
use core::marker::PhantomData;

/// A [`ComputeReduce`] with one composed computation.
pub struct ComputeReduceOne<R, C1> {
    c1: C1,
    p: PhantomData<R>,
}

impl<R, C1> ComputeReduceOne<R, C1> {
    pub(super) fn new(computation: C1) -> Self {
        Self {
            c1: computation,
            p: PhantomData,
        }
    }
}

impl<R, C1> ComputeReduce for ComputeReduceOne<R, C1>
where
    R: Reduction,
    C1: Computation<Out = R::Unit>,
{
    type In<'i> = C1::In<'i>;

    type R = R;

    type Compose<C2>
        = ComputeReduceMany<R, Self, ComputeReduceOne<R, C2>>
    where
        C2: Computation<Out = <Self::R as Reduction>::Unit>;

    fn compose<C>(self, other: C) -> Self::Compose<C>
    where
        C: Computation<Out = <Self::R as Reduction>::Unit>,
    {
        ComputeReduceMany::new(self, ComputeReduceOne::new(other))
    }

    #[inline(always)]
    fn compute_reduce<'i>(&self, _: &Self::R, input: Self::In<'i>) -> <Self::R as Reduction>::Unit {
        self.c1.compute(input)
    }
}
