//! Interface counterpart for `core::default::http_rate`.

/// Marker trait for the default HTTP rate limiter.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait HttpRate: Send + Sync {}
