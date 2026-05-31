//! `RefreshStrategy` — marker trait for OAuth refresh strategy implementations.

/// Marker trait for OAuth refresh strategy implementations.
pub trait RefreshStrategy: Send + Sync {}
