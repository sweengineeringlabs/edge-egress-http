//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for recorded request types.
#[allow(dead_code)]
pub trait Request: Send + Sync {}
