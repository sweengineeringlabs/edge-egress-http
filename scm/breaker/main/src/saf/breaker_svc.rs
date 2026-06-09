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

    /// Build a [`BreakerLayer`] that reports circuit-trip and recovery events
    /// back to a `BackendPool`.
    ///
    /// Requires the `loadbalancer` feature. When the circuit opens (trip) the
    /// layer reports `Outcome::Failure { reason: "circuit open" }` to the pool
    /// for the affected backend, removing it from rotation. When a half-open
    /// probe succeeds the layer reports `Outcome::Success`, restoring the
    /// backend.
    #[cfg(feature = "loadbalancer")]
    pub fn build_breaker_layer_with_pool(
        config: BreakerConfig,
        pool: std::sync::Arc<swe_edge_loadbalancer::BackendPoolInstance>,
    ) -> Result<BreakerLayer, BreakerError> {
        let layer = BreakerLayer::new_with_pool(config, pool);
        Ok(layer)
    }
}
