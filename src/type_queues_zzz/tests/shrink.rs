use crate::type_queues_zzz::{
    empty::EmptyQueue,
    tests::utils::{assert_type, pop, push},
};

#[test]
fn shrink_one() {
    let x = EmptyQueue;
    let x = push::<_, char>(x);

    let (p, x) = pop(x);
    assert_type(&p, "char");
    assert_type(&x, "EmptyQueue");
}

#[test]
fn shrink_two() {
    let x = EmptyQueue;
    let x = push::<_, char>(x);
    let x = push::<_, u32>(x);

    let (p, x) = pop(x);
    assert_type(&p, "u32");
    assert_type(&x, "OneQueue<char>");

    // let (p, x) = pop(x);
    // assert_type(&p, "char");
    // assert_type(&x, "EmptyQueue");
}

#[test]
fn shrink_many() {
    let x = EmptyQueue;
    let x = push::<_, char>(x);
    let x = push::<_, u32>(x);
    let x = push::<_, String>(x);
    let x = push::<_, bool>(x);

    let (p, x) = pop(x);
    assert_type(&p, "bool");
    assert_type(&x, "MultiQueue<MultiQueue<OneQueue<char>,u32>,String>");
}
