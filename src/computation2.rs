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
    type In = (C1::In, C2::In);

    fn compute(&self, reduction: &R, (in1, in2): Self::In) -> <R as Reduction>::Out {
        reduction.reduce(
            self.c1.compute(reduction, in1),
            self.c2.compute(reduction, in2),
        )
    }
}
