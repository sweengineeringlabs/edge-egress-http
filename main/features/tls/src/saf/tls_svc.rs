//! HTTP TLS SAF — factory methods on [`HttpTlsSvc`].

use std::sync::Arc;

use swe_edge_configbuilder::ConfigLoaderFactory;

use crate::api::error::TlsError;
use crate::api::tls_config::TlsConfig;
use crate::api::types::tls_layer::TlsLayer;
use crate::api::types::tls_svc::HttpTlsSvc;
use crate::core::identity::build_provider;

impl HttpTlsSvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let builder = ConfigLoaderFactory::create_config_builder();
        builder
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Build a [`TlsLayer`] from a caller-supplied [`TlsConfig`].
    pub fn build_tls_layer(config: TlsConfig) -> Result<TlsLayer, TlsError> {
        let provider = build_provider(&config)?;
        let layer = TlsLayer::new(Arc::from(provider));
        Ok(layer)
    }
}
