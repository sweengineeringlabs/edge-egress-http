//! HTTP cache SAF — factory methods on [`HttpCacheSvc`].

use crate::api::error::CacheError;
use crate::api::types::cache_config::CacheConfig;
use crate::api::types::cache_layer::CacheLayer;
use crate::api::types::cache_svc::HttpCacheSvc;

impl HttpCacheSvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Build a [`CacheLayer`] from a caller-supplied [`CacheConfig`].
    pub fn build_cache_layer(config: CacheConfig) -> Result<CacheLayer, CacheError> {
        let layer = CacheLayer::new(config);
        Ok(layer)
    }
}
