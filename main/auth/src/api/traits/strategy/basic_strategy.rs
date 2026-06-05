//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for basic auth strategy.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait BasicStrategy: Send + Sync {}
