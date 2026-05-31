//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for cached OAuth token types.
pub trait CachedToken: Send + Sync {}
