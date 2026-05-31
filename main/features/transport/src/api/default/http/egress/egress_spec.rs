//! Interface contract for default HTTP egress marker.

/// Marker trait for default HTTP egress implementations.
pub trait HttpEgressSpec: Send + Sync {}
