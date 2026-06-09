//! HTTP breaker SAF — factory methods on [`HttpBreakerSvc`].

use crate::api::error::BreakerError;
use crate::api::types::breaker_config::BreakerConfig;
use crate::api::types::breaker_layer::BreakerLayer;
use crate::api::types::http_breaker_svc::HttpBreakerSvc;

/// Returns the failure threshold configured on a [`BreakerLayer`].
pub fn get_failure_threshold(layer: &BreakerLayer) -> u32 {
    use crate::api::traits::BreakerMetrics;
    layer.failure_threshold()
}

impl HttpBreakerSvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Build a [`BreakerLayer`] from a caller-supplied [`BreakerConfig`].
    pub fn build_breaker_layer(config: BreakerConfig) -> Result<BreakerLayer, BreakerError> {
        let layer = BreakerLayer::new(config);
        Ok(layer)
    }
}
