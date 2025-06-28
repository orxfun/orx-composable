use crate::{compute_reduce::*, tests::common::*};
use core::any::type_name;

pub fn assert_input_queue_type<C>(expected: impl ToString)
where
    C: ReducibleComputation,
{
    let a = format(type_name::<<C as ReducibleComputation>::InQueue<'_>>());
    let b = format(expected);
    assert_eq!(a, b);
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
