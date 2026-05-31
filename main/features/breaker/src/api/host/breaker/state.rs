//! Interface counterpart for core::host::breaker::state.

/// Marker trait for breaker state types.
pub trait State: Send + Sync {}
