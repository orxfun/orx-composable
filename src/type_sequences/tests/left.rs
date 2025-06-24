use crate::type_sequences::{empty::EmptyTypeSeq, type_seq::TypeSeq};
use std::any::type_name_of_val;

fn format(type_name: impl ToString) -> String {
    type_name
        .to_string()
        .replace("\n", &"")
        .replace(" ", &"")
        .trim()
        .replace("orx_composable::type_sequences::empty::", "")
        .replace("orx_composable::type_sequences::non_empty::", "")
        .replace("alloc::string::", "")
}

fn compose<S, T>(_: S) -> <S as TypeSeq>::AddToLeft<T>
where
    S: TypeSeq,
{
    Default::default()
}

fn left_right<S>(_: S) -> (<S as TypeSeq>::Left, <S as TypeSeq>::Right)
where
    S: TypeSeq,
    S::Left: Default,
    S::Right: Default,
{
    Default::default()
}

#[test]
fn left_one() {
    let x1 = EmptyTypeSeq;
    let x2 = compose::<_, char>(x1);

    let (l, r) = left_right(x2);
    assert_eq!(format(type_name_of_val(&l)), "char");
    assert_eq!(format(type_name_of_val(&r)), "EmptyTypeSeq");
}

#[test]
fn left_two() {
    let x1 = EmptyTypeSeq;
    let x2 = compose::<_, char>(x1);
    let x3 = compose::<_, bool>(x2);

    let (l, r) = left_right(x3);
    assert_eq!(format(type_name_of_val(&l)), "bool");
    assert_eq!(
        format(type_name_of_val(&r)),
        "NonEmptyTypeSeq<char,EmptyTypeSeq>"
    );

    let (l, r) = left_right(r);
    assert_eq!(format(type_name_of_val(&l)), "char");
    assert_eq!(format(type_name_of_val(&r)), "EmptyTypeSeq");
}

#[test]
fn left_many() {
    let x1 = EmptyTypeSeq;
    let x2 = compose::<_, char>(x1);
    let x3 = compose::<_, bool>(x2);
    let x4 = compose::<_, u32>(x3);
    let x5 = compose::<_, String>(x4);

    let (l, r) = left_right(x5);
    assert_eq!(format(type_name_of_val(&l)), "String");
    assert_eq!(
        format(type_name_of_val(&r)),
        "NonEmptyTypeSeq<u32,NonEmptyTypeSeq<bool,NonEmptyTypeSeq<char,EmptyTypeSeq>>>"
    );

    let (l, r) = left_right(r);
    assert_eq!(format(type_name_of_val(&l)), "u32");
    assert_eq!(
        format(type_name_of_val(&r)),
        "NonEmptyTypeSeq<bool,NonEmptyTypeSeq<char,EmptyTypeSeq>>"
    );

    let (l, r) = left_right(r);
    assert_eq!(format(type_name_of_val(&l)), "bool");
    assert_eq!(
        format(type_name_of_val(&r)),
        "NonEmptyTypeSeq<char,EmptyTypeSeq>"
    );

    let (l, r) = left_right(r);
    assert_eq!(format(type_name_of_val(&l)), "char");
    assert_eq!(format(type_name_of_val(&r)), "EmptyTypeSeq");
}
