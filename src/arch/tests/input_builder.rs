use crate::{Composition, Computation, tests::common::*};

#[test]
fn empty() {
    let c = Composition::new(Add);

    let i = c.input_builder();
    let input = i.value();

    let result = c.compute(input);
    assert_eq!(result, 0);
}

#[test]
fn one() {
    let c = Composition::new(Add).compose(StrLen);

    let i = c.input_builder().add("abc");
    let input = i.value();

    let result = c.compute(input);
    assert_eq!(result, 3);
}

#[test]
fn two() {
    let c = Composition::new(Add).compose(StrLen).compose(SliceLen);

    let i = c.input_builder().add("abc").add(&[true, false]);
    let input = i.value();

    let result = c.compute(input);
    assert_eq!(result, 5);
}

#[test]
fn three() {
    let c = Composition::new(Add)
        .compose(StrLen)
        .compose(SliceLen)
        .compose(NumEvens);

    let i = c
        .input_builder()
        .add("abc")
        .add(&[true, false])
        .add(vec![1, 2, 4, 6]);
    let input = i.value();

    let result = c.compute(input);
    assert_eq!(result, 8);
}

#[test]
fn four() {
    let c = Composition::new(Add)
        .compose(StrLen)
        .compose(SliceLen)
        .compose(NumEvens)
        .compose(BoundedBy10);

    let i = c
        .input_builder()
        .add("abc")
        .add(&[true, false])
        .add(vec![1, 2, 4, 6])
        .add(42);
    let input = i.value();

    let result = c.compute(input);
    assert_eq!(result, 18);
}
