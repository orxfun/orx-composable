use crate::type_sequences::{empty::EmptyTypeSeq, type_seq::TypeSeq};
use std::any::type_name_of_val;

fn compose<S, X>(_: S) -> <S as TypeSeq>::AddToLeft<X>
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
fn add_to_left_one() {
    let x1 = EmptyTypeSeq;
    let x2 = compose::<_, char>(x1);

    let expected = r"
NonEmptyTypeSeq<
    char,
    EmptyTypeSeq>";

    assert_eq!(format(type_name_of_val(&x2)), format(expected));
}

#[test]
fn add_to_left_two() {
    let x1 = EmptyTypeSeq;
    let x2 = compose::<_, char>(x1);
    let x3 = compose::<_, bool>(x2);

    let expected = r"
NonEmptyTypeSeq<
    bool,
    NonEmptyTypeSeq<
        char,
        EmptyTypeSeq>>";

    assert_eq!(format(type_name_of_val(&x3)), format(expected));
}

#[test]
fn add_to_left_many() {
    let x1 = EmptyTypeSeq;
    let x2 = compose::<_, char>(x1);
    let x3 = compose::<_, bool>(x2);
    let x4 = compose::<_, u32>(x3);
    let x5 = compose::<_, String>(x4);

    let expected = r"
NonEmptyTypeSeq<
    String,
    NonEmptyTypeSeq<
        u32,
        NonEmptyTypeSeq<
            bool,
            NonEmptyTypeSeq<
                char,
                EmptyTypeSeq>>>>";

    assert_eq!(format(type_name_of_val(&x5)), format(expected));
}
