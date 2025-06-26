use crate::Reduction;

pub struct Composition<R, C>
where
    R: Reduction,
{
    reduction: R,
    computation: C,
}
