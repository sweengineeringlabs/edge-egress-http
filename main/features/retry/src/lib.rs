//! swe_edge_egress_retry — Opinionated retry middleware (wraps reqwest-retry with SWE defaults).
//!

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]

mod api;
mod core;
mod saf;

mod gateway;
pub use gateway::*;
