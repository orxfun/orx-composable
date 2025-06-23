pub trait InputBuilderZzz {
    type In;

    type Composed<In>: InputBuilderZzz;

    fn value(self) -> Self::In;

    fn compose<In2>(self, other: In2) -> Self::Composed<In2>;
}
