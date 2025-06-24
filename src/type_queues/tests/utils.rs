use crate::type_queues::type_queue::TypeQueue;
use std::any::type_name_of_val;

pub fn format(type_name: impl ToString) -> String {
    type_name
        .to_string()
        .replace("\n", &"")
        .replace(" ", &"")
        .trim()
        .replace("orx_composable::type_sequences::empty::", "")
        .replace("orx_composable::type_sequences::non_empty::", "")
        .replace("alloc::string::", "")
}

pub fn push<Q, T>(_: Q) -> <Q as TypeQueue>::Push<T>
where
    Q: TypeQueue,
{
    Default::default()
}

pub fn pop<Q>(
    _: Q,
) -> (
    <Q as TypeQueue>::PoppedType,
    <Q as TypeQueue>::QueueAfterPop,
)
where
    Q: TypeQueue,
    <Q as TypeQueue>::PoppedType: Default,
{
    Default::default()
}
