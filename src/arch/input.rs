pub struct Input<In>(pub(super) In);

impl<In> Input<In> {
    pub fn compose<In2>(self, other: In2) -> Input<(In, In2)> {
        Input((self.0, other))
    }

    pub fn value(self) -> In {
        self.0
    }
}

impl<In> From<In> for Input<In> {
    fn from(value: In) -> Self {
        Self(value)
    }
}
