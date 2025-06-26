use crate::{
    Computation, Reduction,
    compute_reduce::{com_red::ComputeReduce, one::ComputeReduceOne},
};
use core::marker::PhantomData;

/// A [`ComputeReduce`] with no composed computation.
pub struct ComputeReduceEmpty<R> {
    p: PhantomData<R>,
}

impl<R> Default for ComputeReduceEmpty<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> ComputeReduceEmpty<R> {
    /// Creates a new empty [`ComputeReduce`], which is the starting point
    /// for composed computations.
    pub fn new() -> Self {
        Self { p: PhantomData }
    }
}

impl<R> ComputeReduce for ComputeReduceEmpty<R>
where
    R: Reduction,
{
    type In<'i> = ();

    type R = R;

    type Compose<C>
        = ComputeReduceOne<R, C>
    where
        C: Computation<Out = <Self::R as Reduction>::Unit>;

    fn compose<C>(self, other: C) -> Self::Compose<C>
    where
        C: Computation<Out = <Self::R as Reduction>::Unit>,
    {
        ComputeReduceOne::new(other)
    }

    fn compute_reduce<'i>(
        &self,
        reduction: &Self::R,
        _: Self::In<'i>,
    ) -> <Self::R as Reduction>::Unit {
        reduction.identity()
    }
}
