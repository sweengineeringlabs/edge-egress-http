//! `swe_edge_egress_http` — HTTP outbound domain.

mod api;
mod core;
mod saf;
mod spi;

mod gateway;
pub use gateway::*;
