use crate::{input_builder_zzz::InputBuilderZzz, input_builder2_zzz::InputBuilder2Zzz};

pub struct InputBuilder1Zzz<In1>(pub(super) In1);

impl<In1> InputBuilderZzz for InputBuilder1Zzz<In1> {
    type In = In1;

    type Composed<In> = InputBuilder2Zzz<In1, In>;

    fn value(self) -> Self::In {
        self.0
    }

    fn compose<In>(self, other: In) -> Self::Composed<In> {
        InputBuilder2Zzz(self.0, other)
    }
}
