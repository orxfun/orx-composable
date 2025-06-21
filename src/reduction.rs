pub trait Reduction {
    type Out;

    fn identity(&self) -> Self::Out;

    fn reduce(&self, a: Self::Out, b: Self::Out) -> Self::Out;
}
