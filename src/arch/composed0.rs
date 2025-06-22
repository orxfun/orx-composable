use crate::{compute::Compute, reduce::Reduce};

pub struct Composed0<R>(pub(super) R)
where
    R: Reduce;

impl<R> Compute for Composed0<R>
where
    R: Reduce,
{
    type In<'i> = ();

    type Out = R::Unit;

    fn compute(&self, _: Self::In<'_>) -> Self::Out {
        self.0.identity()
    }
}

impl<R> Composed0<R>
where
    R: Reduce,
{
    // pub fn compose<C>(self, other: C) -> ComputeReduce1<R, C>
    // where
    //     C: Compute<Out = R::Unit>,
    // {
    //     ComputeReduce1(self.0, other)
    // }
}
