use crate::reduce::Reduce;
use std::marker::PhantomData;

pub trait ComputeReduce: Sized {
    type R: Reduce;

    type In<'i>;

    fn compute_reduce<'i>(
        &self,
        reduce: &Self::R,
        input: Self::In<'_>,
    ) -> <Self::R as Reduce>::Unit;

    fn compose<C>(self, other: C) -> impl ComputeReduce<R = Self::R>
    where
        C: ComputeReduce<R = Self::R>;
}

pub struct ComputeReduce0<R: Reduce>(PhantomData<R>);

impl<R: Reduce> ComputeReduce for ComputeReduce0<R> {
    type R = R;

    type In<'i> = ();

    fn compute_reduce<'i>(&self, reduce: &Self::R, _: Self::In<'_>) -> <Self::R as Reduce>::Unit {
        reduce.identity()
    }

    fn compose<C>(self, other: C) -> impl ComputeReduce<R = Self::R>
    where
        C: ComputeReduce<R = Self::R>,
    {
        ComputeReduce1(other, PhantomData)
    }
}

pub struct ComputeReduce1<R, C>(C, PhantomData<R>)
where
    R: Reduce,
    C: ComputeReduce<R = R>;

impl<R, C> ComputeReduce for ComputeReduce1<R, C>
where
    R: Reduce,
    C: ComputeReduce<R = R>,
{
    type R = R;

    type In<'i> = C::In<'i>;

    fn compute_reduce<'i>(
        &self,
        reduce: &Self::R,
        input: Self::In<'_>,
    ) -> <Self::R as Reduce>::Unit {
        self.0.compute_reduce(reduce, input)
    }

    fn compose<C2>(self, other: C2) -> impl ComputeReduce<R = Self::R>
    where
        C2: ComputeReduce<R = Self::R>,
    {
        ComputeReduce2(self.0, other, PhantomData)
    }
}

pub struct ComputeReduce2<R, C1, C2>(C1, C2, PhantomData<R>)
where
    R: Reduce,
    C1: ComputeReduce<R = R>,
    C2: ComputeReduce<R = R>;

impl<R, C1, C2> ComputeReduce for ComputeReduce2<R, C1, C2>
where
    R: Reduce,
    C1: ComputeReduce<R = R>,
    C2: ComputeReduce<R = R>,
{
    type R = R;

    type In<'i> = (C1::In<'i>, C2::In<'i>);

    fn compute_reduce<'i>(
        &self,
        reduce: &Self::R,
        (in1, in2): Self::In<'_>,
    ) -> <Self::R as Reduce>::Unit {
        reduce.reduce(
            self.0.compute_reduce(reduce, in1),
            self.1.compute_reduce(reduce, in2),
        )
    }

    fn compose<C>(self, other: C) -> impl ComputeReduce<R = Self::R>
    where
        C: ComputeReduce<R = Self::R>,
    {
        ComputeReduce2(self, other, PhantomData)
    }
}
