//! Interface counterpart for core::token::bucket.

/// Marker trait for token bucket implementations.
pub trait Bucket: Send + Sync {}
