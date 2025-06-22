use crate::input_builder::InputBuilder;

pub struct InputBuilder2<In1, In2>(pub(super) In1, pub(super) In2);

impl<In1, In2> InputBuilder for InputBuilder2<In1, In2> {
    type In = (In1, In2);

    type Composed<In> = InputBuilder2<(In1, In2), In>;

    fn value(self) -> Self::In {
        (self.0, self.1)
    }

    fn compose<In>(self, other: In) -> Self::Composed<In> {
        InputBuilder2((self.0, self.1), other)
    }
}
