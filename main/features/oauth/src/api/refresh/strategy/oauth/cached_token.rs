//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for cached OAuth token types.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait CachedToken: Send + Sync {}
