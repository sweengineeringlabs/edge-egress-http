//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for cache entry helpers.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait CacheEntryHelper: Send + Sync {}
