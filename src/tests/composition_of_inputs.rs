use crate::{
    compute_reduce::{
        ReducibleComputationEmpty, ReducibleComputationMany, ReducibleComputationOne,
    },
    *,
};
use core::any::type_name;

pub fn format(type_name: impl ToString) -> String {
    type_name
        .to_string()
        .replace("\n", &"")
        .replace(" ", &"")
        .trim()
        .replace("orx_meta_queue::", "")
        .replace("orx_composable::", "")
        .replace("alloc::vec::", "")
        .replace("empty::", "")
        .replace("one::", "")
        .replace("multi::", "")
        .replace("alloc::string::", "")
}

pub fn assert_input_queue_type<C>(expected: impl ToString)
where
    C: ReducibleComputation,
{
    let a = format(type_name::<<C as ReducibleComputation>::InQueue<'_>>());
    let b = format(expected);
    assert_eq!(a, b);
}

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
fn empty() {
    assert_input_queue_type::<ReducibleComputationEmpty<Add>>("Empty");
}

#[test]
fn one() {
    assert_input_queue_type::<ReducibleComputationOne<Add, StrLen>>("One<&str>");
}

#[test]
fn two() {
    assert_input_queue_type::<
        ReducibleComputationMany<
            Add,
            ReducibleComputationOne<Add, StrLen>,
            ReducibleComputationOne<Add, SliceLen>,
        >,
    >("Multi<&str,One<&[bool]>>");
}

#[test]
fn three() {
    assert_input_queue_type::<
        ReducibleComputationMany<
            Add,
            ReducibleComputationOne<Add, StrLen>,
            ReducibleComputationMany<
                Add,
                ReducibleComputationOne<Add, SliceLen>,
                ReducibleComputationOne<Add, NumEvens>,
            >,
        >,
    >("Multi<&str,Multi<&[bool],One<Vec<u64>>>>");
}

#[test]
fn four() {
    assert_input_queue_type::<
        ReducibleComputationMany<
            Add,
            ReducibleComputationOne<Add, StrLen>,
            ReducibleComputationMany<
                Add,
                ReducibleComputationOne<Add, SliceLen>,
                ReducibleComputationMany<
                    Add,
                    ReducibleComputationOne<Add, NumEvens>,
                    ReducibleComputationOne<Add, BoundedBy10>,
                >,
            >,
        >,
    >("Multi<&str,Multi<&[bool],Multi<Vec<u64>,One<usize>>>>");
}
