//! Public factory entry point.

use std::sync::Arc;

use swe_edge_configbuilder::ConfigBuilder as _;

use crate::api::error::Error;
use crate::api::tls_config::TlsConfig;
use crate::api::tls_layer::TlsLayer;
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
pub fn build_tls_layer(config: TlsConfig) -> Result<TlsLayer, Error> {
    let provider = build_provider(&config)?;
    Ok(TlsLayer::new(Arc::from(provider)))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: create_config_builder
    #[test]
    fn test_create_config_builder_builds_loader() {
        let _loader = create_config_builder().build_loader();
    }

    /// @covers: build_tls_layer
    #[test]
    fn test_build_tls_layer_with_none_config_returns_pass_through_layer() {
        let layer = build_tls_layer(TlsConfig::None).expect("build ok");
        let s = format!("{layer:?}");
        assert!(s.contains("noop"));
    }

    /// @covers: build_tls_layer
    #[test]
    fn test_build_tls_layer_with_missing_pem_file_fails_fast() {
        let cfg = TlsConfig::Pem {
            path: "/this/path/does/not/exist.pem".into(),
        };
        let err = build_tls_layer(cfg).unwrap_err();
        match err {
            Error::FileReadFailed { path, .. } => assert!(path.contains("does/not/exist")),
            other => panic!("expected FileReadFailed, got {other:?}"),
        }
    }
}
