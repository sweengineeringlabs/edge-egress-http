//! Public factory entry point.

use swe_edge_configbuilder::ConfigBuilder as _;

use crate::api::error::Error;
use crate::api::rate_config::RateConfig;
use crate::api::rate_layer::RateLayer;

/// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Build a [`RateLayer`] from a caller-supplied [`RateConfig`].
pub fn build_rate_layer(config: RateConfig) -> Result<RateLayer, Error> {
    Ok(RateLayer::new(config))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: create_config_builder
    #[test]
    fn test_create_config_builder_builds_loader() {
        let _loader = create_config_builder().build_loader();
    }

    /// @covers: build_rate_layer
    #[test]
    fn test_build_rate_layer_with_default_config_returns_layer() {
        let layer = build_rate_layer(RateConfig::default()).expect("build ok");
        let s = format!("{layer:?}");
        assert!(s.contains("RateLayer"));
    }
}
