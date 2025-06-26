use orx_meta_queue::{MetaQueue, TupleQueue};

pub type InputBuilder<M: MetaQueue> = TupleQueue<M>;
