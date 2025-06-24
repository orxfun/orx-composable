pub trait TypeQueue: Default {
    type Push<X>: TypeQueue;

    type QueueAfterPop: TypeQueue;

    type PoppedType;
}
