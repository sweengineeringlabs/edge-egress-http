//! Domain error types for `swe_edge_egress_breaker`.

pub mod breaker_error;
pub mod error_alias;

pub use breaker_error::BreakerError;
pub use error_alias::Error;
