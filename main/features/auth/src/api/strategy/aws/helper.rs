//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for AWS SigV4 helper.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait Helper: Send + Sync {}
