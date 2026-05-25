//! Public factory entry point.

use swe_edge_configbuilder::ConfigBuilder as _;

use crate::api::error::RateError;
use crate::api::types::rate_config::RateConfig;
use crate::api::types::rate_layer::RateLayer;

/// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Build a [`RateLayer`] from a caller-supplied [`RateConfig`].
pub fn build_rate_layer(config: RateConfig) -> Result<RateLayer, RateError> {
    Ok(RateLayer::new(config))
}
