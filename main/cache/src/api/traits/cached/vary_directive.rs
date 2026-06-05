//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for Vary header directive types.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait VaryDirective: Send + Sync {}
