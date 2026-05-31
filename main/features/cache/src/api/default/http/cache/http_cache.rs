//! Interface counterpart for core::default::http::cache.

/// Marker trait for default HTTP cache implementations.
pub trait HttpCache: Send + Sync {}
