//! Interface counterpart for `core::cache::layer::request_snapshot`.

/// Marker trait for request snapshot types.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait RequestSnapshot: Send + Sync {}
