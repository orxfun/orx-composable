use crate::type_queues::type_queue::TypeQueue;
use std::marker::PhantomData;

pub struct MultiQueue<L, R>
where
    L: TypeQueue,
{
    p: PhantomData<(L, R)>,
}

impl<L, R> TypeQueue for MultiQueue<L, R>
where
    L: TypeQueue,
{
    type Push<X> = MultiQueue<Self, X>;

    type PoppedType = R;

    type QueueAfterPop = L;
}
