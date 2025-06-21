use crate::reduction::Reduction;

pub trait Computation<R>: Sized
where
    R: Reduction,
{
    type In;

    fn compute(&self, reduction: &R, input: Self::In) -> R::Out;
}
