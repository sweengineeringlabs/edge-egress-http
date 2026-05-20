//! API interface types for the metrics HTTP outbound decorator.
#[allow(clippy::module_inception)]
pub mod metrics_http_outbound;
pub mod observation_config;
pub use metrics_http_outbound::MetricsHttpOutbound;
pub use observation_config::ObservationConfig;
