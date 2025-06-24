use crate::type_queues::type_queue::TypeQueue;
use std::marker::PhantomData;

pub struct NonEmptyQueue<L, R> {
    p: PhantomData<(L, R)>,
}

impl<L, R> TypeQueue for NonEmptyQueue<L, R> {
    type Push<T> = Self;

    type Pop = Self;
}
