use crate::{input_builder::InputBuilder, input_builder1::InputBuilder1};

pub struct InputBuilder0;

impl InputBuilder for InputBuilder0 {
    type In = ();

    type Composed<In> = InputBuilder1<In>;

    fn value(self) -> Self::In {}

    fn compose<In2>(self, other: In2) -> Self::Composed<In2> {
        InputBuilder1(other)
    }
}
