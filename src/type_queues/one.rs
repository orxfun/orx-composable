use crate::type_queues::{empty::EmptyQueue, multi::MultiQueue, type_queue::TypeQueue};
use std::marker::PhantomData;

pub struct OneQueue<T> {
    p: PhantomData<T>,
}

impl<T> Default for OneQueue<T> {
    fn default() -> Self {
        Self {
            p: Default::default(),
        }
    }
}

impl<T> TypeQueue for OneQueue<T> {
    type Push<X> = MultiQueue<Self, T>;

    type QueueAfterPop = EmptyQueue;

    type PoppedType = T;
}
