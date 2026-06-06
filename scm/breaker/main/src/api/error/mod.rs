//! Domain error types for `swe_edge_egress_breaker`.

pub mod breaker_error;

pub use breaker_error::BreakerError;
/// Error type alias.
pub type Error = BreakerError;
