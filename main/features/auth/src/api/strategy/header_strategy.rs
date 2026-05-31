//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for custom header auth strategy.
pub trait HeaderStrategy: Send + Sync {}
