use crate::{computation::Computation, reduction::Reduction};
use std::marker::PhantomData;

pub struct Computation2<R, C1, C2>
where
    R: Reduction,
    C1: Computation<R>,
    C2: Computation<R>,
{
    c1: C1,
    c2: C2,
    phantom: PhantomData<R>,
}

impl<R, C1, C2> Computation2<R, C1, C2>
where
    R: Reduction,
    C1: Computation<R>,
    C2: Computation<R>,
{
    pub(super) fn new(c1: C1, c2: C2) -> Self {
        Self {
            c1,
            c2,
            phantom: PhantomData,
        }
    }
}

impl<R, C1, C2> Computation<R> for Computation2<R, C1, C2>
where
    R: Reduction,
    C1: Computation<R>,
    C2: Computation<R>,
{
    type In<'i>
        = (C1::In<'i>, C2::In<'i>)
    where
        Self: 'i,
        R: 'i;

    fn compute_reduce<'i>(&self, reduction: &R, (in1, in2): Self::In<'i>) -> <R as Reduction>::Out
    where
        R: 'i,
    {
        reduction.reduce(
            self.c1.compute_reduce(reduction, in1),
            self.c2.compute_reduce(reduction, in2),
        )
    }
}
