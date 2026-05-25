//! Public factory entry point.

use std::sync::Arc;

use swe_edge_configbuilder::ConfigBuilder as _;

use crate::api::error::TlsError;
use crate::api::tls_config::TlsConfig;
use crate::api::types::tls_layer::TlsLayer;
use crate::core::identity::build_provider;

/// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Build a [`TlsLayer`] from a caller-supplied [`TlsConfig`].
///
/// Resolves file paths and env-var-backed passwords at call time so
/// that missing files or unset passwords fail startup rather than
/// the first request.
pub fn build_tls_layer(config: TlsConfig) -> Result<TlsLayer, TlsError> {
    let provider = build_provider(&config)?;
    Ok(TlsLayer::new(Arc::from(provider)))
}
