//! Interface counterpart for core::token::bucket.

/// Marker trait for token bucket implementations.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait Bucket: Send + Sync {}
