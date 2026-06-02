//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for default HTTP cassette implementations.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait Cassette: Send + Sync {}
