//! HTTP cache SAF — factory methods on [`HttpCacheSvc`].

use swe_edge_configbuilder::ConfigLoaderFactory;

use crate::api::error::CacheError;
use crate::api::types::cache_config::CacheConfig;
use crate::api::types::cache_layer::CacheLayer;
use crate::api::types::cache_svc::HttpCacheSvc;

impl HttpCacheSvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let builder = ConfigLoaderFactory::create_config_builder();
        builder
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Build a [`CacheLayer`] from a caller-supplied [`CacheConfig`].
    pub fn build_cache_layer(config: CacheConfig) -> Result<CacheLayer, CacheError> {
        let layer = CacheLayer::new(config);
        Ok(layer)
    }
}
