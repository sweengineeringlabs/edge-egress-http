//! `RefreshStrategy` — marker trait for OAuth refresh strategy implementations.

/// Marker trait for OAuth refresh strategy implementations.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait RefreshStrategy: Send + Sync {}
