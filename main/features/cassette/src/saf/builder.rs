//! Public factory entry point.
//!
//! Cassette is slightly different from the other middleware crates —
//! each test case typically has its own named cassette file, so
//! `build_cassette_layer` takes a `cassette_name` that becomes part
//! of the on-disk path.

use swe_edge_configbuilder::ConfigBuilder as _;

use crate::api::error::CassetteError;
use crate::api::types::cassette_config::CassetteConfig;
use crate::api::types::cassette_layer::CassetteLayer;

/// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Build a [`CassetteLayer`] from a caller-supplied [`CassetteConfig`].
///
/// `cassette_name` identifies the on-disk fixture file (usually one per
/// test case). Appended to `config.cassette_dir` as `<cassette_name>.yaml`.
pub fn build_cassette_layer(
    config: CassetteConfig,
    cassette_name: &str,
) -> Result<CassetteLayer, CassetteError> {
    CassetteLayer::new(config, cassette_name)
}
