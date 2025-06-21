pub trait Reduction: Default {
    type Out: Default;

    fn identity(&self) -> Self::Out;

    fn reduce(&self, a: Self::Out, b: Self::Out) -> Self::Out;
}
