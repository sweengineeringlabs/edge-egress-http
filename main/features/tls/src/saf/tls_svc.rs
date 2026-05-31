//! HTTP TLS SAF — factory methods on [`HttpTlsSvc`].

use std::sync::Arc;

use swe_edge_configbuilder::ConfigLoaderFactory;

use crate::api::error::TlsError;
use crate::api::types::HttpTlsSvc;
use crate::api::types::TlsConfig;
use crate::api::types::TlsLayer;
use crate::core::identity::TlsProviderFactory;

impl HttpTlsSvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        swe_edge_configbuilder::ConfigBuilderImpl::for_crate(
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
        )
    }

    /// Build a [`TlsLayer`] from a caller-supplied [`TlsConfig`].
    pub fn build_tls_layer(config: TlsConfig) -> Result<TlsLayer, TlsError> {
        let provider = TlsProviderFactory::build_provider(&config)?;
        let layer = TlsLayer::new(Arc::from(provider));
        Ok(layer)
    }
}
