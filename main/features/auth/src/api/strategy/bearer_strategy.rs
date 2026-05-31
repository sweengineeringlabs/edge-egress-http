//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for bearer token auth strategy.
pub trait BearerStrategy: Send + Sync {}
