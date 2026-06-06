//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for recorded HTTP interaction types.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait Interaction: Send + Sync {}
