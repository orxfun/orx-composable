use super::com_red::ComRed;
use crate::{Computation, Reduction, compute_reduce::many::ComRedN};
use core::marker::PhantomData;

pub struct ComRed1<R, C1> {
    c1: C1,
    p: PhantomData<R>,
}

impl<R, C1> ComRed1<R, C1> {
    pub(super) fn new(computation: C1) -> Self {
        Self {
            c1: computation,
            p: PhantomData,
        }
    }
}

impl<R, C1> ComRed for ComRed1<R, C1>
where
    R: Reduction,
    C1: Computation<Out = R::Unit>,
{
    type In<'i> = C1::In<'i>;

    type R = R;

    type Compose<C2>
        = ComRedN<R, Self, ComRed1<R, C2>>
    where
        C2: Computation<Out = <Self::R as Reduction>::Unit>;

    fn compose<C>(self, other: C) -> Self::Compose<C>
    where
        C: Computation<Out = <Self::R as Reduction>::Unit>,
    {
        ComRedN::new(self, ComRed1::new(other))
    }

    #[inline(always)]
    fn compute_reduce<'i>(&self, _: &Self::R, input: Self::In<'i>) -> <Self::R as Reduction>::Unit {
        self.c1.compute(input)
    }
}
