use crate::*;

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

pub struct Add;

impl Reduction for Add {
    type Unit = usize;

    fn identity(&self) -> Self::Unit {
        0
    }

    fn reduce(&self, a: Self::Unit, b: Self::Unit) -> Self::Unit {
        a + b
    }
}

pub struct StrLen;

impl Computation for StrLen {
    type In<'i> = &'i str;

    type Out = usize;

    fn compute(&self, str: Self::In<'_>) -> Self::Out {
        str.len()
    }
}

pub struct SliceLen;

impl Computation for SliceLen {
    type In<'i> = &'i [bool];

    type Out = usize;

    fn compute(&self, slice: Self::In<'_>) -> Self::Out {
        slice.len()
    }
}

pub struct NumEvens;

impl Computation for NumEvens {
    type In<'i> = Vec<u64>;

    type Out = usize;

    fn compute(&self, vec: Self::In<'_>) -> Self::Out {
        vec.iter().filter(|x| *x % 2 == 0).count()
    }
}

pub struct BoundedBy10;

impl Computation for BoundedBy10 {
    type In<'i> = usize;

    type Out = usize;

    fn compute(&self, input: Self::In<'_>) -> Self::Out {
        input.min(10)
    }
}
