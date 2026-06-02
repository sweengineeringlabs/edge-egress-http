//! API interface types for the metrics HTTP outbound decorator.
#[allow(clippy::module_inception)]
pub(crate) mod metrics_http_egress;
pub(crate) mod metrics_http_egress_spec;
pub(crate) mod observation_config;
pub use metrics_http_egress::MetricsHttpEgress;
pub use observation_config::ObservationConfig;
