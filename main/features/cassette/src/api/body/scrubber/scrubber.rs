//! Interface counterpart for `core::body::scrubber::scrubber`.

/// Marker trait for body scrubber implementations.
pub trait Scrubber: Send + Sync {}
