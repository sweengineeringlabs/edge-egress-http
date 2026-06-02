//! Interface counterpart for `core::default::http_breaker`.

/// Marker trait for the default HTTP breaker.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait HttpBreaker: Send + Sync {}
