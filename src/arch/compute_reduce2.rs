use crate::{compute::Compute, compute_reduce::ComputeReduce, reduce::Reduce};

pub struct ComputeReduce2<R, C1, C2>(pub(super) R, pub(super) C1, pub(super) C2)
where
    R: Reduce,
    C1: ComputeReduce<R = R>,
    C2: ComputeReduce<R = R>;

impl<R, C1, C2> Compute for ComputeReduce2<R, C1, C2>
where
    R: Reduce,
    C1: ComputeReduce<R = R>,
    C2: ComputeReduce<R = R>,
{
    type In<'i> = (C1::In<'i>, C2::In<'i>);

    type Out = R::Unit;

    fn compute(&self, (in1, in2): Self::In<'_>) -> Self::Out {
        self.0.reduce(
            self.1.compute_reduce(&self.0, in1),
            self.2.compute_reduce(&self.0, in2),
        )
    }
}

impl<R, C1, C2> ComputeReduce2<R, C1, C2>
where
    R: Reduce,
    C1: ComputeReduce<R = R>,
    C2: ComputeReduce<R = R>,
{
    pub fn compose<C>(self, other: C)
    where
        C: ComputeReduce<R = R>,
    {
        //
    }
}
