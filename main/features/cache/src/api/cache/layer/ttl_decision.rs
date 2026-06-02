//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for TTL decision types.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait TtlDecision: Send + Sync {}
