use crate::{compute::Compute, reduce::Reduce};

pub struct ComputeReduce<R, C1, C2>
where
    R: Reduce,
    C1: Compute<Out = R::Unit>,
    C2: Compute<Out = R::Unit>,
{
    reduce: R,
    c1: C1,
    c2: C2,
}

impl<R, C1, C2> ComputeReduce<R, C1, C2>
where
    R: Reduce,
    C1: Compute<Out = R::Unit>,
    C2: Compute<Out = R::Unit>,
{
    pub(super) fn new(reduce: R, c1: C1, c2: C2) -> Self {
        Self { reduce, c1, c2 }
    }
}

impl<R, C1, C2> Compute for ComputeReduce<R, C1, C2>
where
    R: Reduce,
    C1: Compute<Out = R::Unit>,
    C2: Compute<Out = R::Unit>,
{
    type In<'i> = (C1::In<'i>, C2::In<'i>);

    type Out = R::Unit;

    fn compute(&self, (in1, in2): Self::In<'_>) -> Self::Out {
        self.reduce
            .reduce(self.c1.compute(in1), self.c2.compute(in2))
    }
}
