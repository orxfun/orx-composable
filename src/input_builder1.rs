use crate::{input_builder::InputBuilder, input_builder2::InputBuilder2};

pub struct InputBuilder1<In1>(pub(super) In1);

impl<In1> InputBuilder for InputBuilder1<In1> {
    type In = In1;

    type Composed<In> = InputBuilder2<In1, In>;

    fn value(self) -> Self::In {
        self.0
    }

    fn compose<In>(self, other: In) -> Self::Composed<In> {
        InputBuilder2(self.0, other)
    }
}
