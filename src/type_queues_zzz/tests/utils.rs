use crate::type_queues_zzz::type_queue::TypeQueue;
use std::any::type_name_of_val;

pub fn format(type_name: impl ToString) -> String {
    type_name
        .to_string()
        .replace("\n", &"")
        .replace(" ", &"")
        .trim()
        .replace("orx_composable::type_queues::empty::", "")
        .replace("orx_composable::type_queues::one::", "")
        .replace("orx_composable::type_queues::multi::", "")
        .replace("alloc::string::", "")
}

pub fn assert_type<T>(t: &T, type_name: impl ToString) {
    let a = format(type_name_of_val(t));
    let b = format(type_name);
    assert_eq!(a, b);
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
    <Q as TypeQueue>::QueueAfterPop: Default,
{
    Default::default()
}
