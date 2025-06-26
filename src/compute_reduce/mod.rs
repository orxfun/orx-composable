#[cfg(test)]
mod tests;

mod com_red;
mod empty;
mod many;
mod one;

pub use com_red::ComputeReduce;
pub use empty::ComputeReduceEmpty;
pub use many::ComputeReduceMany;
pub use one::ComputeReduceOne;
