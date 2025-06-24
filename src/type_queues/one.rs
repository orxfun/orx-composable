use crate::type_queues::{empty::EmptyQueue, multi::MultiQueue, type_queue::TypeQueue};
use std::marker::PhantomData;

pub struct OneQueue<T> {
    p: PhantomData<T>,
}

impl<T> TypeQueue for OneQueue<T> {
    type Push<X> = MultiQueue<Self, T>;

    type PoppedType = T;

    type QueueAfterPop = EmptyQueue;
}
