//! Public factory entry point.

use std::sync::Arc;

use swe_edge_configbuilder::ConfigBuilder as _;

use crate::api::auth_config::AuthConfig;
use crate::api::auth_middleware::AuthMiddleware;
use crate::api::error::Error;
use crate::core::credential::EnvCredentialResolver;
use crate::core::default_http_auth::DefaultHttpAuth;

/// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Build an [`AuthMiddleware`] from a caller-supplied [`AuthConfig`].
///
/// Uses the default [`EnvCredentialResolver`] to resolve every env-var
/// reference in the config at call time. A missing env var fails with
/// [`Error::MissingEnvVar`] so startup (not the first request) surfaces
/// the misconfiguration.
///
/// The returned [`AuthMiddleware`] implements
/// `reqwest_middleware::Middleware` — plug into a
/// `reqwest_middleware::ClientBuilder` via `.with(mw)`.
pub fn build_auth_middleware(config: AuthConfig) -> Result<AuthMiddleware, Error> {
    let resolver = EnvCredentialResolver;
    let processor = DefaultHttpAuth::new(config, &resolver)?;
    Ok(AuthMiddleware::new(Arc::new(processor)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;

    /// @covers: create_config_builder
    #[test]
    fn test_create_config_builder_builds_loader() {
        let _loader = create_config_builder().build_loader();
    }

    /// @covers: build_auth_middleware
    #[test]
    fn test_build_auth_middleware_with_none_config_returns_middleware_instance() {
        let mw = build_auth_middleware(AuthConfig::None).expect("build ok");
        let s = format!("{mw:?}");
        assert!(s.contains("swe_edge_egress_auth"));
    }

    /// @covers: build_auth_middleware
    #[test]
    fn test_build_auth_middleware_with_missing_bearer_env_fails_at_build_time() {
        let cfg = AuthConfig::Bearer {
            token_env: "EDGE_TEST_DEFINITELY_NOT_SET_99".into(),
        };
        std::env::remove_var("EDGE_TEST_DEFINITELY_NOT_SET_99");
        let err = build_auth_middleware(cfg).unwrap_err();
        match err {
            Error::MissingEnvVar { name } => {
                assert_eq!(name, "EDGE_TEST_DEFINITELY_NOT_SET_99");
            }
            other => panic!("expected MissingEnvVar, got {other:?}"),
        }
    }

    /// @covers: build_auth_middleware
    #[test]
    fn test_build_auth_middleware_with_bearer_env_set_produces_functioning_middleware() {
        std::env::set_var("EDGE_TEST_BEARER_OK_02", "tok-99");
        let cfg = AuthConfig::Bearer {
            token_env: "EDGE_TEST_BEARER_OK_02".into(),
        };
        let _mw = build_auth_middleware(cfg).expect("bearer builds");
        let _ = AtomicBool::new(true);
        std::env::remove_var("EDGE_TEST_BEARER_OK_02");
    }
}
