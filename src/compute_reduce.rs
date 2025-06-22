use crate::{compute::Compute, reduce::Reduce};

pub trait ComputeReduce: Sized {
    type In<'i>;

    type R: Reduce;

    type Composed<C>
    where
        C: ComputeReduce<R = Self::R>;

    type Composed2<C>
    where
        C: Compute<Out = <Self::R as Reduce>::Unit>;

    fn compute_reduce<'i>(
        &self,
        reduce: &Self::R,
        input: Self::In<'i>,
    ) -> <Self::R as Reduce>::Unit;

    fn compose<C>(self, other: C) -> Self::Composed<C>
    where
        C: ComputeReduce<R = Self::R>;

    fn compose2<C>(self, other: C) -> Self::Composed2<C>
    where
        C: Compute<Out = <Self::R as Reduce>::Unit>,
    {
        todo!()
    }
}
