use crate::{compute::Compute, compute_reduce::ComputeReduce, reduce::Reduce};
use std::marker::PhantomData;

pub struct Compute1<R, C1>(pub(super) PhantomData<R>, pub(super) C1)
where
    R: Reduce,
    C1: Compute<Out = R::Unit>;

impl<R, C1> ComputeReduce for Compute1<R, C1>
where
    R: Reduce,
    C1: Compute<Out = R::Unit>,
{
    type In<'i> = C1::In<'i>;

    type R = R;

    type Composed<C>
        = Compute2<R, C1, C>
    where
        C: Compute<Out = <Self::R as Reduce>::Unit>;

    fn compute_reduce<'i>(&self, _: &Self::R, input: Self::In<'i>) -> <Self::R as Reduce>::Unit {
        self.1.compute(input)
    }

    // fn compose<'i, C>(self, other: C) -> impl ComputeReduce<R = Self::R>
    // where
    //     C: Compute<Out = <Self::R as Reduce>::Unit>,
    // {
    // }
}
