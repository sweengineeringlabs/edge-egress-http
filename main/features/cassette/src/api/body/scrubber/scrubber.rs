//! Interface counterpart for `core::body::scrubber::scrubber`.

/// Marker trait for body scrubber implementations.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait Scrubber: Send + Sync {}
