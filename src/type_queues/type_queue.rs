pub trait TypeQueue {
    type Push<X>;

    type PoppedType;

    type QueueAfterPop: TypeQueue;
}
