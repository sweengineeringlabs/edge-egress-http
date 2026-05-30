//! SAF service — public factory for OAuth middleware.

use crate::api::oauth_builder::OAuthBuilder;

/// Service factory for the OAuth middleware.
///
/// All public entry points are methods on this type so the saf/ layer
/// contains no free-standing functions (SEA rule 191).
pub struct OAuthSvc;

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
        OAuthBuilder::default()
    }

    /// Return a config builder pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::oauth_builder_ops::OAuthBuilderOps as _;

    /// @covers: OAuthSvc::builder — missing token source returns Configuration error.
    #[test]
    fn test_builder_without_source_returns_configuration_error() {
        let result = OAuthSvc::builder().build();
        assert!(result.is_err(), "build without token source must fail");
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains("no OAuthTokenSource"),
            "error must identify missing source: {msg}",
        );
    }

    /// @covers: OAuthSvc::create_config_builder — returns a config builder.
    #[test]
    fn test_create_config_builder_builds_loader() {
        let _loader = OAuthSvc::create_config_builder().build_loader();
    }
}
