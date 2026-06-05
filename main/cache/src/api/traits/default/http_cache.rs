//! Interface counterpart for core::default::http::cache.

/// Marker trait for default HTTP cache implementations.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait HttpCache: Send + Sync {}
