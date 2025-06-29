pub trait Criterion {
    type On<'i>;

    type In<'i>;

    type Out<'i>;

    fn evaluate<'i>(&self, on: Self::On<'i>, input: Self::In<'i>) -> Self::Out<'i>;
}
