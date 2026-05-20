//! Gateway layer — public surface for the HTTP outbound transport crate.
pub(crate) mod input;
pub(crate) mod output;

pub use output::*;
