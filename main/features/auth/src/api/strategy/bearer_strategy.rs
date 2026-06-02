//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for bearer token auth strategy.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait BearerStrategy: Send + Sync {}
