use crate::type_queues_zzz::type_queue::TypeQueue;
use std::marker::PhantomData;

pub struct MultiQueue<L, R> {
    p: PhantomData<(L, R)>,
}

impl<L, R> Default for MultiQueue<L, R> {
    fn default() -> Self {
        Self {
            p: Default::default(),
        }
    }
}

impl<L, R> TypeQueue for MultiQueue<L, R> {
    type Push<X> = MultiQueue<X, Self>;

    type QueueAfterPop = R;

    type PoppedType = L;
}
