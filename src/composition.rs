use crate::{Computation, ComputeReduce, Reduction, compute_reduce::ComputeReduceEmpty};

pub struct Composition<R, C>
where
    R: Reduction,
    C: ComputeReduce<R = R>,
{
    reduction: R,
    computation: C,
}

impl<R> Composition<R, ComputeReduceEmpty<R>>
where
    R: Reduction,
{
    pub fn new(reduction: R) -> Self {
        Self {
            reduction,
            computation: Default::default(),
        }
    }
}

impl<R, C> Composition<R, C>
where
    R: Reduction,
    C: ComputeReduce<R = R>,
{
    pub fn compose<C2>(self, other: C2) -> Composition<R, C::Compose<C2>>
    where
        C2: Computation<Out = R::Unit>,
    {
        Composition {
            reduction: self.reduction,
            computation: self.computation.compose(other),
        }
    }
}
