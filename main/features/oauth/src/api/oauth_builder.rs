//! `OAuthBuilder` — fluent builder for [`OAuthMiddleware`].

use std::sync::Arc;

use crate::api::error::OAuthError;
use crate::api::oauth_builder_ops::OAuthBuilderOps;
use crate::api::oauth_middleware::OAuthMiddleware;
use crate::api::oauth_token_source::OAuthTokenSource;

/// Fluent builder for [`OAuthMiddleware`].
#[derive(Default)]
pub struct OAuthBuilder {
    source: Option<Arc<dyn OAuthTokenSource>>,
}

impl OAuthBuilderOps for OAuthBuilder {
    fn with_token_source(mut self, source: Arc<dyn OAuthTokenSource>) -> Self {
        self.source = Some(source);
        self
    }

    fn build(self) -> std::result::Result<OAuthMiddleware, OAuthError> {
        let source = self.source.ok_or_else(|| {
            OAuthError::Configuration(
                "no OAuthTokenSource provided — call with_token_source first".into(),
            )
        })?;
        let strategy = crate::core::refresh_strategy::OAuthRefreshStrategy::new(source);
        Ok(OAuthMiddleware {
            inner: Arc::new(strategy),
        })
    }
}
