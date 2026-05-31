//! Interface counterpart for core::metrics_http_egress.

/// Marker trait for metrics HTTP egress implementations.
pub trait MetricsHttpEgressSpec: Send + Sync {}
pub(crate) mod metrics_http_egress;
