#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]
#![cfg_attr(not(test), no_std)]

#[cfg(test)]
mod tests;

mod composition;
mod computation;
/// Module defining the [`ReducibleComputation`] trait and its implementations.
pub mod compute_reduce;
mod input_builder;
mod reduction;

pub use composition::Composition;
pub use computation::Computation;
pub use compute_reduce::ReducibleComputation;
pub use input_builder::InputBuilder;
pub use reduction::Reduction;
