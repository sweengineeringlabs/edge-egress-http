//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for auth strategy factories.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait StrategyFactory: Send + Sync {}
