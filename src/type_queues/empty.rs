use crate::type_queues::type_queue::TypeQueue;

#[derive(Default)]
pub struct EmptyQueue;

impl TypeQueue for EmptyQueue {
    type Push<T> = Self;

    type Pop = Self;
}
