use crate::{computation::Computation, reduction::Reduction};

#[derive(Default)]
pub struct Computation0;

impl<R> Computation<R> for Computation0
where
    R: Reduction,
{
    type In<'i>
        = ()
    where
        Self: 'i,
        R: 'i;

    fn compute_reduce<'i>(&self, reduction: &R, _: Self::In<'i>) -> <R as Reduction>::Out
    where
        R: 'i,
    {
        reduction.identity()
    }
}
