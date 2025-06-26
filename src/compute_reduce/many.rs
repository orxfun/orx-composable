use super::com_red::ComputeReduce;
use crate::{Computation, Reduction, compute_reduce::one::ComputeReduceOne};
use core::marker::PhantomData;

/// A [`ComputeReduce`] with two or more composed computation.
pub struct ComputeReduceMany<R, C1, C2> {
    c1: C1,
    c2: C2,
    p: PhantomData<R>,
}

impl<R, C1, C2> ComputeReduceMany<R, C1, C2> {
    pub(super) fn new(c1: C1, c2: C2) -> Self {
        Self {
            c1,
            c2,
            p: PhantomData,
        }
    }
}

impl<R, C1, C2> ComputeReduce for ComputeReduceMany<R, C1, C2>
where
    R: Reduction,
    C1: ComputeReduce<R = R>,
    C2: ComputeReduce<R = R>,
{
    type In<'i> = (C1::In<'i>, C2::In<'i>);

    type R = R;

    type Compose<C>
        = ComputeReduceMany<R, Self, ComputeReduceOne<R, C>>
    where
        C: Computation<Out = <Self::R as Reduction>::Unit>;

    fn compose<C>(self, other: C) -> Self::Compose<C>
    where
        C: Computation<Out = <Self::R as Reduction>::Unit>,
    {
        ComputeReduceMany::new(self, ComputeReduceOne::<R, _>::new(other))
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
