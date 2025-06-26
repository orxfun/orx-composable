use orx_meta_queue::Empty;

use crate::{
    Computation, Reduction,
    compute_reduce::{com_red::ReducibleComputation, one::ReducibleComputationOne},
};
use core::marker::PhantomData;

/// A [`ReducibleComputation`] with no composed computation.
pub struct ReducibleComputationEmpty<R> {
    p: PhantomData<R>,
}

impl<R> Default for ReducibleComputationEmpty<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> ReducibleComputationEmpty<R> {
    /// Creates a new empty [`ReducibleComputation`], which is the starting point
    /// for composed computations.
    pub fn new() -> Self {
        Self { p: PhantomData }
    }
}

impl<R> ReducibleComputation for ReducibleComputationEmpty<R>
where
    R: Reduction,
{
    type R = R;

    type In<'i> = ();

    type InQueue<'i> = Empty;

    type Compose<C>
        = ReducibleComputationOne<R, C>
    where
        C: Computation<Out = <Self::R as Reduction>::Unit>;

    fn compose<C>(self, other: C) -> Self::Compose<C>
    where
        C: Computation<Out = <Self::R as Reduction>::Unit>,
    {
        ReducibleComputationOne::new(other)
    }

    fn compute_reduce<'i>(
        &self,
        reduction: &Self::R,
        _: Self::In<'i>,
    ) -> <Self::R as Reduction>::Unit {
        reduction.identity()
    }
}
