//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for OAuth token refresh strategies.
pub trait RefreshStrategy: Send + Sync {}
