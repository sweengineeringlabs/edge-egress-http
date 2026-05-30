//! HTTP breaker SAF — factory methods on [`HttpBreakerSvc`].

use swe_edge_configbuilder::ConfigLoaderFactory;

use crate::api::error::BreakerError;
use crate::api::types::breaker::breaker_config::BreakerConfig;
use crate::api::types::breaker::breaker_layer::BreakerLayer;
use crate::api::types::breaker::http_breaker_svc::HttpBreakerSvc;

impl HttpBreakerSvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let builder = ConfigLoaderFactory::create_config_builder();
        builder
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Build a [`BreakerLayer`] from a caller-supplied [`BreakerConfig`].
    pub fn build_breaker_layer(config: BreakerConfig) -> Result<BreakerLayer, BreakerError> {
        let layer = BreakerLayer::new(config);
        Ok(layer)
    }
}
