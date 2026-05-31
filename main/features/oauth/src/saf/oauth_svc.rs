//! SAF service — public factory for OAuth middleware.

use crate::api::oauth_builder::OAuthBuilder;
use crate::api::types::OAuthSvc;

impl OAuthSvc {
    /// Create a builder for [`OAuthMiddleware`].
    ///
    /// Returns an empty [`OAuthBuilder`]; call
    /// [`with_token_source`](crate::api::oauth_builder_ops::OAuthBuilderOps::with_token_source)
    /// then [`build`](crate::api::oauth_builder_ops::OAuthBuilderOps::build).
    pub fn builder() -> OAuthBuilder {
        OAuthBuilder::new()
    }

    /// Return a config builder pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        swe_edge_configbuilder::ConfigBuilderImpl::for_crate(
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
        )
    }
}
