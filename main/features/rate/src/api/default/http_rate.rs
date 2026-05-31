//! Interface counterpart for `core::default::http_rate`.

/// Marker trait for the default HTTP rate limiter.
pub trait HttpRate: Send + Sync {}
