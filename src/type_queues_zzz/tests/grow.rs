use crate::type_queues_zzz::{
    empty::EmptyQueue,
    tests::utils::{assert_type, push},
};

#[test]
fn grow_none() {
    let x = EmptyQueue;

    assert_type(&x, "EmptyQueue");
}

#[test]
fn grow_one() {
    let x = EmptyQueue;
    let x = push::<_, char>(x);

    assert_type(&x, "OneQueue<char>");
}

#[test]
fn grow_two() {
    let x = EmptyQueue;
    let x = push::<_, char>(x);
    let x = push::<_, u32>(x);

    assert_type(&x, "MultiQueue<OneQueue<char>,u32>");
}

#[test]
fn grow_many() {
    let x = EmptyQueue;
    let x = push::<_, char>(x);
    let x = push::<_, u32>(x);
    let x = push::<_, String>(x);
    let x = push::<_, bool>(x);

    assert_type(
        &x,
        "MultiQueue<bool,MultiQueue<String,MultiQueue<OneQueue<char>,u32>>>",
    );
}
