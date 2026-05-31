//! HTTP retry SAF — factory methods on [`HttpRetrySvc`].

use swe_edge_configbuilder::ConfigLoaderFactory;

use crate::api::error::RetryError;
use crate::api::types::retry::http_retry_svc::HttpRetrySvc;
use crate::api::types::retry::retry_config::RetryConfig;
use crate::api::types::retry::retry_layer::RetryLayer;

impl HttpRetrySvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        swe_edge_configbuilder::ConfigBuilderImpl::for_crate(
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
        )
    }

    /// Build a [`RetryLayer`] from a caller-supplied [`RetryConfig`].
    pub fn build_retry_layer(config: RetryConfig) -> Result<RetryLayer, RetryError> {
        let layer = RetryLayer::new(config);
        Ok(layer)
    }
}
