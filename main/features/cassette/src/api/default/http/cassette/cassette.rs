//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for default HTTP cassette implementations.
pub trait Cassette: Send + Sync {}
