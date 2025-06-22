use crate::reduction::Reduction;

pub trait Computation<R>: Sized
where
    R: Reduction,
{
    type In<'i>
    where
        Self: 'i,
        R: 'i;

    fn compute<'i>(&self, input: Self::In<'i>) -> R::Out
    where
        R: 'i;

    fn compute_reduce<'i>(&self, _: &R, input: Self::In<'i>) -> R::Out
    where
        R: 'i,
    {
        self.compute(input)
    }
}
