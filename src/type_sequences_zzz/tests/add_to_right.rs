use crate::type_sequences::{empty::EmptyTypeSeq, type_seq::TypeSeq};
use std::any::type_name_of_val;

fn compose<S, T>(_: S) -> <S as TypeSeq>::AddToRight<T>
where
    S: TypeSeq,
{
    Default::default()
}

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

#[test]
fn add_to_right_one() {
    let x1 = EmptyTypeSeq;
    let x2 = compose::<_, char>(x1);

    let expected = r"
NonEmptyTypeSeq<
    EmptyTypeSeq,
    char>";

    assert_eq!(format(type_name_of_val(&x2)), format(expected));
}

#[test]
fn add_to_right_two() {
    let x1 = EmptyTypeSeq;
    let x2 = compose::<_, char>(x1);
    let x3 = compose::<_, bool>(x2);

    let expected = r"
NonEmptyTypeSeq<
    NonEmptyTypeSeq<
        EmptyTypeSeq,
        char>,
    bool>";

    assert_eq!(format(type_name_of_val(&x3)), format(expected));
}

#[test]
fn add_to_right_many() {
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
