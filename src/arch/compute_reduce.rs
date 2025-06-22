use crate::{compute::Compute, reduce::Reduce};

pub trait ComputeReduce: Sized {
    type In<'i>;

    type R: Reduce;

    type Composed<C>
    where
        C: Compute<Out = <Self::R as Reduce>::Unit>;

    fn compute_reduce<'i>(
        &self,
        reduce: &Self::R,
        input: Self::In<'i>,
    ) -> <Self::R as Reduce>::Unit;

    // fn compose<C>(self, other: C) -> impl ComputeReduce<R = Self::R>
    // where
    //     C: Compute<Out = <Self::R as Reduce>::Unit>,
    // {
    // }
}
