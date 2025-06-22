use crate::{input_builder_zzz::InputBuilderZzz, input_builder1_zzz::InputBuilder1Zzz};

pub struct InputBuilder0Zzz;

impl InputBuilderZzz for InputBuilder0Zzz {
    type In = ();

    type Composed<In> = InputBuilder1Zzz<In>;

    fn value(self) -> Self::In {}

    fn compose<In2>(self, other: In2) -> Self::Composed<In2> {
        InputBuilder1Zzz(other)
    }
}
