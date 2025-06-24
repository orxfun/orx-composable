use crate::type_queues::type_queue::TypeQueue;
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

impl<L, R> TypeQueue for MultiQueue<L, R>
where
    L: TypeQueue,
{
    type Push<X> = MultiQueue<Self, X>;

    type QueueAfterPop = L;

    type PoppedType = R;
}
