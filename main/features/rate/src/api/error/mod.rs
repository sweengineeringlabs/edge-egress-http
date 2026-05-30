//! Rate error types.

pub mod rate_error;
pub use rate_error::RateError;

/// Error type alias for compatibility.
pub type Error = RateError;
