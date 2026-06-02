//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for custom header auth strategy.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait HeaderStrategy: Send + Sync {}
