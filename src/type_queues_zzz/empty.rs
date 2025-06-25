use crate::type_queues_zzz::{never::Never, one::OneQueue, type_queue::TypeQueue};

#[derive(Default)]
pub struct EmptyQueue;

impl TypeQueue for EmptyQueue {
    type Push<X> = OneQueue<X>;

    type PoppedType = Never;

    type QueueAfterPop = EmptyQueue;
}
