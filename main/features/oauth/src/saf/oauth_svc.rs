//! SAF service — public factory for OAuth middleware.

use crate::api::oauth_builder::OAuthBuilder;
use crate::api::types::OAuthSvc;

/// Service factory for the OAuth middleware.
///
/// All public entry points are methods on this type so the saf/ layer
/// contains no free-standing functions (SEA rule 191).

impl OAuthSvc {
    /// Create a builder for [`OAuthMiddleware`].
    ///
    /// Call [`crate::api::oauth_builder_ops::OAuthBuilderOps::with_token_source`]
    /// followed by [`crate::api::oauth_builder_ops::OAuthBuilderOps::build`]
    /// to produce an [`OAuthMiddleware`] ready for use with
    /// `reqwest_middleware::ClientBuilder`.
    ///
    /// ```ignore
    /// use std::sync::Arc;
    /// let mw = swe_edge_egress_oauth::OAuthSvc::builder()
    ///     .with_token_source(Arc::new(my_token_source))
    ///     .build()?;
    /// ```
    pub fn builder() -> OAuthBuilder {
        // @allow: saf_no_wrapper_methods — OAuthBuilder source field is private; Default is the only constructor
        OAuthBuilder::default()
    }

    /// Return a config builder pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }
}
