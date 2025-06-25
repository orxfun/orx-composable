use crate::{
    Computation, Reduction,
    compute_reduce::{com_red::ComRed, one::ComRed1},
};
use core::marker::PhantomData;

pub struct ComRed0<R> {
    p: PhantomData<R>,
}

impl<R> ComRed0<R> {
    pub(super) fn new() -> Self {
        Self { p: PhantomData }
    }
}

impl<R> ComRed for ComRed0<R>
where
    R: Reduction,
{
    type In<'i> = ();

    type R = R;

    type Compose<C>
        = ComRed1<R, C>
    where
        C: Computation<Out = <Self::R as Reduction>::Unit>;

    fn compose<C>(self, other: C) -> Self::Compose<C>
    where
        C: Computation<Out = <Self::R as Reduction>::Unit>,
    {
        ComRed1::new(other)
    }

    fn compute_reduce<'i>(
        &self,
        reduction: &Self::R,
        _: Self::In<'i>,
    ) -> <Self::R as Reduction>::Unit {
        reduction.identity()
    }
}
