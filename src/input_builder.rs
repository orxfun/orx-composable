pub trait InputBuilder {
    type In;

    type Composed<In>: InputBuilder;

    fn value(self) -> Self::In;

    fn compose<In2>(self, other: In2) -> Self::Composed<In2>;
}
