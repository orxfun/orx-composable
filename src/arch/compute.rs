pub trait Compute {
    type In<'i>;

    type Out;

    fn compute(&self, input: Self::In<'_>) -> Self::Out;
}
