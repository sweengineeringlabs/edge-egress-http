//! `MetricsHttpEgressSpec` — marker trait for metrics HTTP egress implementations.

/// Marker trait for metrics HTTP egress implementations.
pub trait MetricsHttpEgressSpec: Send + Sync {}
