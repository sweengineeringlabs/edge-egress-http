//! Public factory entry point.
//!
//! Cassette is slightly different from the other middleware crates —
//! each test case typically has its own named cassette file, so
//! `build_cassette_layer` takes a `cassette_name` that becomes part
//! of the on-disk path.

use swe_edge_configbuilder::ConfigBuilder as _;

use crate::api::cassette_config::CassetteConfig;
use crate::api::cassette_layer::CassetteLayer;
use crate::api::error::Error;

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
) -> Result<CassetteLayer, Error> {
    CassetteLayer::new(config, cassette_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: create_config_builder
    #[test]
    fn test_create_config_builder_builds_loader() {
        let _loader = create_config_builder().build_loader();
    }

    /// @covers: build_cassette_layer
    #[test]
    fn test_build_cassette_layer_with_nonexistent_cassette_file_starts_empty() {
        let tmpdir = tempfile::tempdir().unwrap();
        let dir_toml_safe = tmpdir.path().to_str().unwrap().replace('\\', "/");
        let toml = format!(
            r#"
                mode = "auto"
                cassette_dir = "{dir_toml_safe}"
                match_on = ["method", "url"]
                scrub_headers = []
                scrub_body_paths = []
            "#
        );
        let cfg = CassetteConfig::from_config(&toml).unwrap();
        let _layer = build_cassette_layer(cfg, "fresh_case").expect("build ok");
    }
}
