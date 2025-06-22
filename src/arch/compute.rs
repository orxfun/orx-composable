pub trait Compute {
    type In<'i>;

    type Out;

    fn compute(&self, input: Self::In<'_>) -> Self::Out;

    // fn compose<'i, C>(
    //     self,
    //     other: C,
    // ) -> impl Compute<In<'i> = (Self::In<'i>, C::In<'i>), Out = Self::Out>
    // where
    //     C: Compute<Out = Self::Out>;
}
