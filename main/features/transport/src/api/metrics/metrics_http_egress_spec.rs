//! `MetricsHttpEgressSpec` — marker trait for metrics HTTP egress implementations.

/// Marker trait for metrics HTTP egress implementations.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait MetricsHttpEgressSpec: Send + Sync {}
