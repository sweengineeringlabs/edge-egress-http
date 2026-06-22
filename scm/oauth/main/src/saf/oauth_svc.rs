//! SAF service — public factory for OAuth middleware.

use crate::api::refresh::types::OAuthBuilder;
use crate::api::refresh::types::OAuthSvc;

impl OAuthSvc {
    /// Create a builder for [`OAuthMiddleware`](crate::OAuthMiddleware).
    ///
    /// Returns an empty [`OAuthBuilder`](crate::OAuthBuilder); call
    /// [`OAuthBuilderOps::with_token_source`](crate::OAuthBuilderOps::with_token_source)
    /// then [`OAuthBuilderOps::build`](crate::OAuthBuilderOps::build).
    pub fn builder() -> OAuthBuilder {
        OAuthBuilder { source: None }
    }

    /// Return a config builder pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }
}
