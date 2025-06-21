use crate::{
    computation::Computation, computation0::Computation0, computation2::Computation2, input::Input,
    reduction::Reduction,
};

pub struct Composable<R, C>
where
    R: Reduction,
    C: Computation<R>,
{
    reduction: R,
    computation: C,
}

impl<R> Composable<R, Computation0>
where
    R: Reduction,
{
    pub fn new(reduction: R) -> Self {
        Self {
            reduction,
            computation: Default::default(),
        }
    }
}

impl<R, C> Composable<R, C>
where
    R: Reduction,
    C: Computation<R>,
{
    pub fn compose<D>(self, other: D) -> Composable<R, Computation2<R, C, D>>
    where
        D: Computation<R>,
    {
        let computation = Computation2::new(self.computation, other);
        Composable {
            reduction: self.reduction,
            computation,
        }
    }

    pub fn input_builder(&self) -> Input<()> {
        Input(())
    }
}
