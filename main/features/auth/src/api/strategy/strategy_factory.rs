//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for auth strategy factories.
pub trait StrategyFactory: Send + Sync {}
