//! Interface counterpart for core::refresh.

/// Marker trait for OAuth refresh implementations.
pub trait RefreshSpec: Send + Sync {}
