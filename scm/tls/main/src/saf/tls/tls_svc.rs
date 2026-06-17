//! HTTP TLS SAF — factory methods on [`HttpTlsSvc`].

use std::sync::Arc;

use crate::api::error::TlsError;
use crate::api::types::HttpTlsSvc;
use crate::api::types::TlsConfig;
use crate::api::types::TlsLayer;
use crate::core::identity::TlsProviderFactory;

/// Returns the identity label for this TLS service provider.
pub fn describe_tls_provider(svc: &HttpTlsSvc) -> &'static str {
    use crate::api::traits::Provider;
    svc.describe()
}

/// Validates a [`TlsConfig`], returning a human-readable error on failure.
pub fn validate_tls_config(config: &TlsConfig) -> Result<(), String> {
    use crate::api::traits::Validator;
    config.validate()
}

impl HttpTlsSvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Build a [`TlsLayer`] from a caller-supplied [`TlsConfig`].
    pub fn build_tls_layer(config: TlsConfig) -> Result<TlsLayer, TlsError> {
        let provider = TlsProviderFactory::build_provider(&config)?;
        let layer = TlsLayer::new(Arc::from(provider));
        Ok(layer)
    }
}
