//! Public factory entry point.

use swe_edge_configbuilder::ConfigBuilder as _;

use crate::api::cache_config::CacheConfig;
use crate::api::cache_layer::CacheLayer;
use crate::api::error::Error;

/// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Build a [`CacheLayer`] from a caller-supplied [`CacheConfig`].
pub fn build_cache_layer(config: CacheConfig) -> Result<CacheLayer, Error> {
    Ok(CacheLayer::new(config))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: create_config_builder
    #[test]
    fn test_create_config_builder_builds_loader() {
        let _loader = create_config_builder().build_loader();
    }

    /// @covers: build_cache_layer
    #[test]
    fn test_build_cache_layer_with_default_config_returns_layer() {
        let layer = build_cache_layer(CacheConfig::default()).expect("build ok");
        let s = format!("{layer:?}");
        assert!(s.contains("CacheLayer"));
    }
}
