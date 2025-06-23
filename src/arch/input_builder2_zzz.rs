use crate::input_builder_zzz::InputBuilderZzz;

pub struct InputBuilder2Zzz<In1, In2>(pub(super) In1, pub(super) In2);

impl<In1, In2> InputBuilderZzz for InputBuilder2Zzz<In1, In2> {
    type In = (In1, In2);

    type Composed<In> = InputBuilder2Zzz<(In1, In2), In>;

    fn value(self) -> Self::In {
        (self.0, self.1)
    }

    fn compose<In>(self, other: In) -> Self::Composed<In> {
        InputBuilder2Zzz((self.0, self.1), other)
    }
}
