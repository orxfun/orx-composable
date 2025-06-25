pub trait TypeQueue: Default {
    type Push<X>: TypeQueue;

    type QueueAfterPop;

    type PoppedType;
}
