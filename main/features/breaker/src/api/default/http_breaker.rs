//! Interface counterpart for `core::default::http_breaker`.

/// Marker trait for the default HTTP breaker.
pub trait HttpBreaker: Send + Sync {}
