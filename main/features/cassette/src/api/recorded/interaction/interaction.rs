//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for recorded HTTP interaction types.
pub trait Interaction: Send + Sync {}
