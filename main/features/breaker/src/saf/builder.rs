//! Public factory entry point.

use swe_edge_configbuilder::ConfigBuilder as _;

use crate::api::breaker_config::BreakerConfig;
use crate::api::breaker_layer::BreakerLayer;
use crate::api::error::Error;

/// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Build a [`BreakerLayer`] from a caller-supplied [`BreakerConfig`].
///
/// The returned layer implements `reqwest_middleware::Middleware` and
/// carries its own per-host state cache (bounded moka cache).
pub fn build_breaker_layer(config: BreakerConfig) -> Result<BreakerLayer, Error> {
    Ok(BreakerLayer::new(config))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: create_config_builder
    #[test]
    fn test_create_config_builder_builds_loader() {
        let _loader = create_config_builder().build_loader();
    }

    /// @covers: build_breaker_layer
    #[test]
    fn test_build_breaker_layer_with_default_config_returns_layer() {
        let layer = build_breaker_layer(BreakerConfig::default()).expect("build ok");
        let s = format!("{layer:?}");
        assert!(s.contains("BreakerLayer"));
    }
}
