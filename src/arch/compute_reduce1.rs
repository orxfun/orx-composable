use crate::{compute::Compute, compute_reduce::ComputeReduce, reduce::Reduce};

pub struct ComputeReduce1<R, C1>(pub(super) R, pub(super) C1)
where
    R: Reduce,
    C1: ComputeReduce<R = R>;

impl<R, C1> Compute for ComputeReduce1<R, C1>
where
    R: Reduce,
    C1: ComputeReduce<R = R>,
{
    type In<'i> = C1::In<'i>;

    type Out = R::Unit;

    fn compute(&self, input: Self::In<'_>) -> Self::Out {
        self.1.compute_reduce(&self.0, input)
    }
}

// impl<R, C1> Compute for ComputeReduce1<R, C1>
// where
//     R: Reduce,
//     C1: Compute<Out = R::Unit>,
// {
//     type In<'i> = C1::In<'i>;

//     type Out = R::Unit;

//     fn compute(&self, input: Self::In<'_>) -> Self::Out {
//         self.1.compute(input)
//     }
// }

// impl<R, C1> ComputeReduce1<R, C1>
// where
//     R: Reduce,
//     C1: Compute<Out = R::Unit>,
// {
//     pub fn compose<C>(self, other: C) -> ComputeReduce2<R, C1, C>
//     where
//         C: Compute<Out = R::Unit>,
//     {
//         ComputeReduce2(self.0, self.1, other)
//     }
// }
