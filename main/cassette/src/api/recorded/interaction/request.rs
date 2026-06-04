//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for recorded request types.
pub trait Request: Send + Sync {}
