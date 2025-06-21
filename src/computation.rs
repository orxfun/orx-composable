use crate::reduction::Reduction;

pub trait Computation<R>: Sized
where
    R: Reduction,
{
    type In<'i>
    where
        Self: 'i,
        R: 'i;

    fn compute_reduce<'i>(&self, reduction: &R, input: Self::In<'i>) -> R::Out
    where
        R: 'i;
}
