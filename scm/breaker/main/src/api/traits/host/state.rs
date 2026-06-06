//! Interface counterpart for core::host::breaker::state.

/// Marker trait for breaker state types.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait State: Send + Sync {}
