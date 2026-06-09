//! Public factory entry point for `swe-edge-egress-auth`.

use crate::api::auth::types::AuthSvc;

use std::sync::Arc;

use crate::api::auth::errors::AuthError;
use crate::api::auth::types::auth::auth_config::AuthConfig;
use crate::api::auth::types::AuthMiddleware;
use crate::core::credential::EnvCredentialResolver;
use crate::core::default::DefaultHttpAuth;

impl AuthSvc {
    /// Return a config builder pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Build an [`AuthMiddleware`] from a caller-supplied [`AuthConfig`].
    ///
    /// Uses the default `EnvCredentialResolver` to resolve every env-var
    /// reference in the config at call time. A missing env var fails with
    /// [`AuthError::MissingEnvVar`] so startup (not the first request) surfaces
    /// the misconfiguration.
    ///
    /// The returned [`AuthMiddleware`] implements
    /// `reqwest_middleware::Middleware` — plug into a
    /// `reqwest_middleware::ClientBuilder` via `.with(mw)`.
    pub fn build_auth_middleware(config: AuthConfig) -> Result<AuthMiddleware, AuthError> {
        let resolver = EnvCredentialResolver;
        let processor = DefaultHttpAuth::new(config, &resolver)?;
        Ok(AuthMiddleware::new(Arc::new(processor)))
    }
}
