use crate::{compute_reduce::ComputeReduce, compute_reduce1::ComputeReduce1, reduce::Reduce};
use std::marker::PhantomData;

pub struct ComputeReduce0<R>(pub(super) PhantomData<R>)
where
    R: Reduce;

impl<R> ComputeReduce for ComputeReduce0<R>
where
    R: Reduce,
{
    type In<'i> = ();

    type R = R;

    type Composed<C>
        = ComputeReduce1<R, C>
    where
        C: ComputeReduce<R = Self::R>;

    fn compute_reduce<'i>(&self, reduce: &Self::R, _: Self::In<'i>) -> <Self::R as Reduce>::Unit {
        reduce.identity()
    }

    fn compose<C>(self, other: C) -> Self::Composed<C>
    where
        C: ComputeReduce<R = Self::R>,
    {
        ComputeReduce1(PhantomData, other)
    }
}
