//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for the AWS SigV4 signing strategy.
pub trait Strategy: Send + Sync {}
