//! HTTP retry SAF — factory methods on [`HttpRetrySvc`].

use crate::api::error::RetryError;
use crate::api::types::retry::http_retry_svc::HttpRetrySvc;
use crate::api::types::retry::retry_config::RetryConfig;
use crate::api::types::retry::retry_layer::RetryLayer;

impl HttpRetrySvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Build a [`RetryLayer`] from a caller-supplied [`RetryConfig`].
    pub fn build_retry_layer(config: RetryConfig) -> Result<RetryLayer, RetryError> {
        let layer = RetryLayer::new(config);
        Ok(layer)
    }
}
