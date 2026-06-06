//! `swe_edge_egress_http` — HTTP outbound domain.

// `unwrap`/`expect` are denied in production code (Cargo.toml `[lints.clippy]`)
// but are the idiomatic assertion mechanism inside the crate's inline
// `#[cfg(test)]` modules — allow them only under `cfg(test)`.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;
mod spi;

mod gateway;
pub use gateway::*;
