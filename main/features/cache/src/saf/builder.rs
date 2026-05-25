//! Public factory entry point.

use swe_edge_configbuilder::ConfigBuilder as _;

use crate::api::error::CacheError;
use crate::api::types::cache_config::CacheConfig;
use crate::api::types::cache_layer::CacheLayer;

/// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Build a [`CacheLayer`] from a caller-supplied [`CacheConfig`].
pub fn build_cache_layer(config: CacheConfig) -> Result<CacheLayer, CacheError> {
    Ok(CacheLayer::new(config))
}
