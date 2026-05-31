//! Interface counterpart for `core::refresh`.

/// Marker trait for OAuth refresh strategy implementations.
pub trait RefreshSpec: Send + Sync {}
