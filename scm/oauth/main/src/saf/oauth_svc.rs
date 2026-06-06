//! SAF service — public factory for OAuth middleware.

use crate::api::types::OAuthBuilder;
use crate::api::types::OAuthSvc;

impl OAuthSvc {
    /// Create a builder for [`OAuthMiddleware`].
    ///
    /// Returns an empty [`OAuthBuilder`]; call
    /// [`OAuthBuilderOps::with_token_source`](crate::api::traits::OAuthBuilderOps::with_token_source)
    /// then [`OAuthBuilderOps::build`](crate::api::traits::OAuthBuilderOps::build).
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
