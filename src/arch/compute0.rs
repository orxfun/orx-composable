use crate::{compute::Compute, compute_reduce::ComputeReduce, compute1::Compute1, reduce::Reduce};
use std::marker::PhantomData;

pub struct Compute0<R>(pub(super) PhantomData<R>)
where
    R: Reduce;

impl<R> ComputeReduce for Compute0<R>
where
    R: Reduce,
{
    type In<'i> = ();

    type R = R;

    type Composed<C>
        = Compute1<R, C>
    where
        C: Compute<Out = <Self::R as Reduce>::Unit>;

    fn compute_reduce<'i>(&self, reduce: &Self::R, _: Self::In<'i>) -> <Self::R as Reduce>::Unit {
        reduce.identity()
    }

    // fn compose<C>(self, other: C) -> Compute1<R, C>
    // where
    //     C: Compute<Out = <Self::R as Reduce>::Unit>,
    // {
    //     Compute1(self.0, other)
    // }
}
