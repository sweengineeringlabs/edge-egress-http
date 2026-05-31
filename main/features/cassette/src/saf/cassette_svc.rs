//! HTTP cassette SAF — factory methods on [`HttpCassetteSvc`].

use std::path::PathBuf;
use std::sync::Arc;

use swe_edge_configbuilder::ConfigLoaderFactory;
use tokio::sync::Mutex;

use crate::api::error::CassetteError;
use crate::api::types::cassette::cassette_config::CassetteConfig;
use crate::api::types::cassette::cassette_layer::CassetteLayer;
use crate::api::types::cassette::http_cassette_svc::HttpCassetteSvc;

impl HttpCassetteSvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        swe_edge_configbuilder::ConfigBuilderImpl::for_crate(
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
        )
    }

    /// Build a [`CassetteLayer`] from a caller-supplied config and cassette name.
    ///
    /// Resolves the cassette path (`<cassette_dir>/<cassette_name>.yaml`),
    /// loads any pre-recorded fixtures from disk, and returns a ready layer.
    pub fn build_cassette_layer(
        config: CassetteConfig,
        cassette_name: &str,
    ) -> Result<CassetteLayer, CassetteError> {
        let path = PathBuf::from(&config.cassette_dir).join(format!("{cassette_name}.yaml"));
        let fixtures = CassetteLayer::load_fixtures_from_disk(&path)?;
        Ok(CassetteLayer {
            config: Arc::new(config),
            cassette_path: path,
            fixtures: Arc::new(Mutex::new(fixtures)),
        })
    }
}
