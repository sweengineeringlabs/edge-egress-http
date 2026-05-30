//! HTTP rate SAF — factory methods on [`HttpRateSvc`].

use swe_edge_configbuilder::ConfigLoaderFactory;

use crate::api::error::RateError;
use crate::api::types::rate_config::RateConfig;
use crate::api::types::rate_layer::RateLayer;
use crate::api::types::rate_svc::HttpRateSvc;

impl HttpRateSvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let builder = ConfigLoaderFactory::create_config_builder();
        builder
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Build a [`RateLayer`] from a caller-supplied [`RateConfig`].
    pub fn build_rate_layer(config: RateConfig) -> Result<RateLayer, RateError> {
        let layer = RateLayer::new(config);
        Ok(layer)
    }
}
