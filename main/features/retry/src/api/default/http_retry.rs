//! Interface counterpart for `core::default::http_retry`.

/// Marker trait for the default HTTP retry decorator.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait HttpRetry: Send + Sync {}
