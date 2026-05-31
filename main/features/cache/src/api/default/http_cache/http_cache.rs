//! Interface counterpart for core::default::http_cache.

/// Marker trait for default HTTP cache implementations.
pub trait HttpCache: Send + Sync {}
