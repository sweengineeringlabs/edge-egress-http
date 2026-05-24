//! Public factory entry point.

use swe_edge_configbuilder::ConfigBuilder as _;

use crate::api::error::Error;
use crate::api::retry_config::RetryConfig;
use crate::api::retry_layer::RetryLayer;

/// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Build a [`RetryLayer`] from a caller-supplied [`RetryConfig`].
pub fn build_retry_layer(config: RetryConfig) -> Result<RetryLayer, Error> {
    Ok(RetryLayer::new(config))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: create_config_builder
    #[test]
    fn test_create_config_builder_builds_loader() {
        let _loader = create_config_builder().build_loader();
    }

    /// @covers: build_retry_layer
    #[test]
    fn test_build_retry_layer_with_default_config_returns_layer() {
        let layer = build_retry_layer(RetryConfig::default()).expect("build ok");
        let s = format!("{layer:?}");
        assert!(s.contains("RetryLayer"));
        assert!(s.contains("max_retries"));
    }
}
