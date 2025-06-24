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

fn compose<S, T>(_: S) -> <S as TypeSeq>::AddToRight<T>
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
fn right_one() {
    let x1 = EmptyTypeSeq;
    let x2 = compose::<_, char>(x1);

    let (l, r) = left_right(x2);
    assert_eq!(format(type_name_of_val(&l)), "EmptyTypeSeq");
    assert_eq!(format(type_name_of_val(&r)), "char");
}

#[test]
fn right_two() {
    let x1 = EmptyTypeSeq;
    let x2 = compose::<_, char>(x1);
    let x3 = compose::<_, bool>(x2);

    let (l, r) = left_right(x3);
    assert_eq!(
        format(type_name_of_val(&l)),
        "NonEmptyTypeSeq<EmptyTypeSeq,char>"
    );
    assert_eq!(format(type_name_of_val(&r)), "bool");

    let (l, r) = left_right(l);
    assert_eq!(format(type_name_of_val(&l)), "EmptyTypeSeq");
    assert_eq!(format(type_name_of_val(&r)), "char");
}

#[test]
fn right_many() {
    let x1 = EmptyTypeSeq;
    let x2 = compose::<_, char>(x1);
    let x3 = compose::<_, bool>(x2);
    let x4 = compose::<_, u32>(x3);
    let x5 = compose::<_, String>(x4);

    let expected = r"
NonEmptyTypeSeq<
    NonEmptyTypeSeq<
        NonEmptyTypeSeq<
            NonEmptyTypeSeq<
                EmptyTypeSeq,
                char>,
            bool>,
        u32>,
    String>";

    assert_eq!(format(type_name_of_val(&x5)), format(expected));
}
