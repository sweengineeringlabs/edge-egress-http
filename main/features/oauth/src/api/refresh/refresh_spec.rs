//! Interface counterpart for core::refresh.

/// Marker trait for OAuth refresh implementations.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait RefreshSpec: Send + Sync {}
