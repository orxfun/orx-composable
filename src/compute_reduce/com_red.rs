use crate::{Computation, Reduction};

pub trait ComRed {
    type In<'i>;

    type R: Reduction;

    type Compose<C>
    where
        C: Computation<Out = <Self::R as Reduction>::Unit>;

    fn compose<C>(self, other: C) -> Self::Compose<C>
    where
        C: Computation<Out = <Self::R as Reduction>::Unit>;

    fn compute_reduce<'i>(
        &self,
        reduction: &Self::R,
        input: Self::In<'i>,
    ) -> <Self::R as Reduction>::Unit;
}
