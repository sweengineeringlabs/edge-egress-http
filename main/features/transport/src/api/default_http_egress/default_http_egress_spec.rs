//! Interface counterpart for core::default_http_egress.

/// Marker trait for default HTTP egress implementations.
pub trait DefaultHttpEgressSpec: Send + Sync {}
