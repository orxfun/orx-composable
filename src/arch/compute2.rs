// use crate::{compute::Compute, compute_reduce::ComputeReduce, reduce::Reduce};
// use std::marker::PhantomData;

// pub struct Compute2<R, C1, C2>(pub(super) PhantomData<R>, pub(super) C1, pub(super) C2)
// where
//     R: Reduce,
//     C1: Compute<Out = R::Unit>,
//     C2: Compute<Out = R::Unit>;

// impl<R, C1, C2> ComputeReduce for Compute2<R, C1, C2>
// where
//     R: Reduce,
//     C1: Compute<Out = R::Unit>,
//     C2: Compute<Out = R::Unit>,
// {
//     type In<'i> = (C1::In<'i>, C2::In<'i>);

//     type R = R;

//     type Composed<C> = Compute2<R, >
//         where
//             C: Compute<Out = <Self::R as Reduce>::Unit>;

//     fn compute_reduce<'i>(
//         &self,
//         reduce: &Self::R,
//         (in1, in2): Self::In<'i>,
//     ) -> <Self::R as Reduce>::Unit {
//         reduce.reduce(self.1.compute(in1), self.2.compute(in2))
//     }
// }
