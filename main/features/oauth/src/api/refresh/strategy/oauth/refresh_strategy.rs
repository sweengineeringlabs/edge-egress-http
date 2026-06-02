//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for OAuth token refresh strategies.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait RefreshStrategy: Send + Sync {}
