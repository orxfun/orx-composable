use crate::reduce::Reduce;

pub trait ComputeReduce: Sized {
    type In<'i>;

    type R: Reduce;

    type Composed<C>
    where
        C: ComputeReduce<R = Self::R>;

    fn compute_reduce<'i>(
        &self,
        reduce: &Self::R,
        input: Self::In<'i>,
    ) -> <Self::R as Reduce>::Unit;

    fn compose<C>(self, other: C) -> Self::Composed<C>
    where
        C: ComputeReduce<R = Self::R>;
}
