use crate::{
    Computation, Reduction,
    compute_reduce::{com_red::ComputeReduce, empty::ComputeReduceEmpty},
};

struct Add;

impl Reduction for Add {
    type Unit = usize;

    fn identity(&self) -> Self::Unit {
        0
    }

    fn reduce(&self, a: Self::Unit, b: Self::Unit) -> Self::Unit {
        a + b
    }
}

struct StrLen;

impl Computation for StrLen {
    type In<'i> = &'i str;

    type Out = usize;

    fn compute(&self, str: Self::In<'_>) -> Self::Out {
        str.len()
    }
}

struct SliceLen;

impl Computation for SliceLen {
    type In<'i> = &'i [bool];

    type Out = usize;

    fn compute(&self, slice: Self::In<'_>) -> Self::Out {
        slice.len()
    }
}

struct NumEvens;

impl Computation for NumEvens {
    type In<'i> = Vec<u64>;

    type Out = usize;

    fn compute(&self, vec: Self::In<'_>) -> Self::Out {
        vec.iter().filter(|x| *x % 2 == 0).count()
    }
}

struct BoundedBy10;

impl Computation for BoundedBy10 {
    type In<'i> = usize;

    type Out = usize;

    fn compute(&self, input: Self::In<'_>) -> Self::Out {
        input.min(10)
    }
}

#[test]
fn compose_zero() {
    let c = ComputeReduceEmpty::<Add>::new();
    assert_eq!(c.compute_reduce(&Add, ()), 0);
}

#[test]
fn compose_one() {
    let c = ComputeReduceEmpty::<Add>::new();
    let c = c.compose(NumEvens);

    assert_eq!(c.compute_reduce(&Add, vec![1, 2, 3, 4, 5]), 2);
}

#[test]
fn compose_two() {
    let c = ComputeReduceEmpty::<Add>::new();
    let c = c.compose(StrLen);
    let c = c.compose(SliceLen);

    assert_eq!(c.compute_reduce(&Add, ("xyz", &[true, false])), 5);
}

#[test]
fn compose_three() {
    let c = ComputeReduceEmpty::<Add>::new();
    let c = c.compose(StrLen);
    let c = c.compose(SliceLen);
    let c = c.compose(NumEvens);

    assert_eq!(
        c.compute_reduce(&Add, (("xyz", &[true, false]), vec![1, 2, 4, 4])),
        8
    );
}

#[test]
fn compose_four() {
    let c = ComputeReduceEmpty::<Add>::new();
    let c = c.compose(StrLen);
    let c = c.compose(SliceLen);
    let c = c.compose(NumEvens);
    let c = c.compose(BoundedBy10);

    assert_eq!(
        c.compute_reduce(&Add, ((("xyz", &[true, false]), vec![1, 2, 4, 4]), 42)),
        18
    );
}
