use crate::type_queues_zzz::{empty::EmptyQueue, multi::MultiQueue, type_queue::TypeQueue};
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
    type Push<X> = MultiQueue<Self, X>;

    type QueueAfterPop = EmptyQueue;

    type PoppedType = T;
}
