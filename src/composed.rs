use crate::{
    compute::Compute,
    compute_reduce::{ComputeReduce, ComputeReduce0, ComputeReduce1},
    reduce::Reduce,
};

pub struct Composed<R, C>
where
    R: Reduce,
    C: ComputeReduce<R = R>,
{
    reduction: R,
    computation: C,
}

impl<R> Composed<R, ComputeReduce0<R>>
where
    R: Reduce,
{
    pub fn new(reduction: R) -> Self {
        Self {
            reduction,
            computation: Default::default(),
        }
    }
}

impl<R, C> Composed<R, C>
where
    R: Reduce,
    C: ComputeReduce<R = R>,
{
    pub fn compose<O>(self, other: O)
    where
        O: Compute<Out = R::Unit>,
    {
        // let other = ComputeReduce1::new(other);
        //
    }
}
