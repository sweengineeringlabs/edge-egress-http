//! Interface counterpart for the corresponding core/ implementation.

/// Trait for per-host breaker state machines.
///
/// Provides observable state inspection methods that consumers can rely on
/// without depending on the concrete `core::host::breaker::HostBreaker` type.
#[cfg_attr(not(feature = "loadbalancer"), allow(dead_code))]
pub trait HostBreaker: Send + Sync {
    /// Returns `true` when the breaker is in the Open (fail-fast) state.
    fn is_open(&self) -> bool;

    /// Returns `true` when the breaker is in the HalfOpen (probe) state.
    fn is_half_open(&self) -> bool;

    /// Returns `true` when the breaker is in the Closed (normal) state.
    fn is_closed(&self) -> bool;
}
