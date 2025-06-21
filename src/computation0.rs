use crate::{computation::Computation, reduction::Reduction};
pub struct Computation0;

impl<R> Computation<R> for Computation0
where
    R: Reduction,
{
    type In = ();

    fn compute(&self, reduction: &R, _: Self::In) -> <R as Reduction>::Out {
        reduction.identity()
    }
}
