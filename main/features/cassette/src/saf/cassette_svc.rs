//! HTTP cassette SAF — factory methods on [`HttpCassetteSvc`].

use swe_edge_configbuilder::ConfigLoaderFactory;

use crate::api::error::CassetteError;
use crate::api::types::cassette::cassette_config::CassetteConfig;
use crate::api::types::cassette::layer::CassetteLayer;
use crate::api::types::cassette::svc::HttpCassetteSvc;

impl HttpCassetteSvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let builder = ConfigLoaderFactory::create_config_builder();
        builder
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Build a [`CassetteLayer`] from a caller-supplied config and cassette name.
    pub fn build_cassette_layer(
        config: CassetteConfig,
        cassette_name: &str,
    ) -> Result<CassetteLayer, CassetteError> {
        CassetteLayer::new(config, cassette_name)
    }
}
