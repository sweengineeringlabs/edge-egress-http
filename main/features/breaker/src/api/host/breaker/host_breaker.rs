//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for per-host breaker state machines.
pub trait HostBreaker: Send + Sync {}
