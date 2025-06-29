#[cfg(test)]
mod tests;

mod com_red;
mod empty;
mod many;
mod one;

pub use com_red::ReducibleComputation;
pub use empty::ReducibleComputationEmpty;
pub use many::ReducibleComputationMany;
pub use one::ReducibleComputationOne;
