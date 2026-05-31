//! Interface counterpart for `core::cache::layer::request_snapshot`.

/// Marker trait for request snapshot types.
pub trait RequestSnapshot: Send + Sync {}
