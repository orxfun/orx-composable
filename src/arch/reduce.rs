pub trait Reduce {
    type Unit;

    fn identity(&self) -> Self::Unit;

    fn reduce(&self, a: Self::Unit, b: Self::Unit) -> Self::Unit;
}
