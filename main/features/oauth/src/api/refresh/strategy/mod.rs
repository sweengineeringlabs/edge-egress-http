//! Interface counterpart for core::refresh::strategy.

/// Marker trait for OAuth refresh strategy implementations.
pub trait RefreshStrategy: Send + Sync {}
pub(crate) mod oauth;
