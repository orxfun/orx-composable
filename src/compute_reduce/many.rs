use super::com_red::ComRed;
use crate::{Computation, Reduction, compute_reduce::one::ComRed1};
use core::marker::PhantomData;

pub struct ComRedN<R, C1, C2> {
    c1: C1,
    c2: C2,
    p: PhantomData<R>,
}

impl<R, C1, C2> ComRedN<R, C1, C2> {
    pub(super) fn new(c1: C1, c2: C2) -> Self {
        Self {
            c1,
            c2,
            p: PhantomData,
        }
    }
}

impl<R, C1, C2> ComRed for ComRedN<R, C1, C2>
where
    R: Reduction,
    C1: ComRed<R = R>,
    C2: ComRed<R = R>,
{
    type In<'i> = (C1::In<'i>, C2::In<'i>);

    type R = R;

    type Compose<C>
        = ComRedN<R, Self, ComRed1<R, C>>
    where
        C: Computation<Out = <Self::R as Reduction>::Unit>;

    fn compose<C>(self, other: C) -> Self::Compose<C>
    where
        C: Computation<Out = <Self::R as Reduction>::Unit>,
    {
        ComRedN::new(self, ComRed1::<R, _>::new(other))
    }

    fn compute_reduce<'i>(
        &self,
        reduction: &Self::R,
        (in1, in2): Self::In<'i>,
    ) -> <Self::R as Reduction>::Unit {
        reduction.reduce(
            self.c1.compute_reduce(reduction, in1),
            self.c2.compute_reduce(reduction, in2),
        )
    }
}
